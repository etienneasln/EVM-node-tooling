use crate::dieselsqlite::schema::{context_hashes, context_hashes::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = context_hashes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ContextHash {
    pub id: i32,
    pub context_hash: Vec<u8>,
}

impl ContextHash {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = replace_into(context_hashes)
            .values(&self)
            .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select(connection: &mut SqliteConnection, queried_id: i32) -> QueryResult<Vec<u8>> {
        let hash = context_hashes
            .find(queried_id)
            .select(context_hash)
            .get_result(connection)?;
        Ok(hash)
    }

    pub fn get_latest(connection: &mut SqliteConnection) -> QueryResult<(i32, Vec<u8>)> {
        let latest_context = context_hashes
            .select((id, context_hash))
            .order(id.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(latest_context)
    }

    pub fn get_earliest(connection: &mut SqliteConnection) -> QueryResult<(i32, Vec<u8>)> {
        let earliest_context = context_hashes
            .filter(id.ge(0))
            .select((id, context_hash))
            .order(id.asc())
            .limit(1)
            .get_result(connection)?;
        Ok(earliest_context)
    }

    pub fn clear_after(connection: &mut SqliteConnection, queried_id: i32) -> QueryResult<usize> {
        let cleared_rows = delete(context_hashes.filter(id.gt(queried_id))).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection: &mut SqliteConnection, queried_id: i32) -> QueryResult<usize> {
        let cleared_rows = delete(context_hashes.filter(id.lt(queried_id))).execute(connection)?;
        Ok(cleared_rows)
    }
}

#[cfg(test)]
mod context_hash_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_context_hash_insert_select_get_clear() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let iter = 10;
            for i in -1..iter {
                let inserted_level = i;
                let inserted_hash = format!("hash {i}").as_bytes().to_vec();

                let contexthash = ContextHash {
                    id: inserted_level,
                    context_hash: inserted_hash.clone(),
                };
                contexthash.insert(conn)?;
            }

            let earliest_id = 0;
            let earliest_hash = "hash 0".as_bytes().to_vec();

            let _ = ContextHash::clear_after(conn, iter)?;

            let expected_earliest = (earliest_id, earliest_hash);

            let earliest = ContextHash::get_earliest(conn)?;

            assert_eq!(earliest, expected_earliest);

            let new_earliest_hash = "Replaced hash".as_bytes().to_vec();

            let replace_earliest = ContextHash {
                id: earliest_id,
                context_hash: new_earliest_hash.clone(),
            };

            replace_earliest.insert(conn)?;

            let new_expected_earliest = (earliest_id, new_earliest_hash);

            let new_earliest = ContextHash::get_earliest(conn)?;

            assert_eq!(new_earliest, new_expected_earliest);

            let expected_latest = (iter - 1, format!("hash {}", iter - 1).as_bytes().to_vec());

            let latest = ContextHash::get_latest(conn)?;

            assert_eq!(latest, expected_latest);

            let expected_clear: usize = (iter + 1) as usize;

            let clear = ContextHash::clear_before(conn, iter)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
