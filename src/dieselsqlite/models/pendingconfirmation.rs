use crate::dieselsqlite::schema::{pending_confirmations, pending_confirmations::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = pending_confirmations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PendingConfirmation {
    pub level: i32,
    pub hash: Vec<u8>,
}

impl PendingConfirmation {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = self
            .insert_into(pending_confirmations)
            .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_with_level(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<Vec<u8>> {
        let h = pending_confirmations
            .find(queried_level)
            .select(hash)
            .get_result(connection)?;
        Ok(h)
    }

    pub fn delete_with_level(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let deleted_rows =
            delete(pending_confirmations.filter(level.eq(queried_level))).execute(connection)?;
        Ok(deleted_rows)
    }

    pub fn clear(connection: &mut SqliteConnection) -> QueryResult<usize> {
        let deleted_rows = delete(pending_confirmations).execute(connection)?;
        Ok(deleted_rows)
    }

    pub fn count(connection: &mut SqliteConnection) -> QueryResult<i64> {
        let count = pending_confirmations
            .select(count(level))
            .first(connection)?;
        Ok(count)
    }
}

#[cfg(test)]
mod pending_confirmation_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_pending_confirmation_insert_select_delete() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let inserted_level = 0;
            let inserted_hash = "hash".as_bytes().to_vec();

            let pendingconfirmation = PendingConfirmation {
                level: inserted_level,
                hash: inserted_hash.clone(),
            };
            pendingconfirmation.insert(conn)?;

            let selected_hash = PendingConfirmation::select_with_level(conn, inserted_level)?;

            assert_eq!(selected_hash, inserted_hash);

            let expected_delete_row = 1;

            let delete_row = PendingConfirmation::delete_with_level(conn, inserted_level)?;

            assert_eq!(delete_row, expected_delete_row);

            Ok(())
        })
    }

    #[test]
    fn test_pending_confirmation_insert_count_clear() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let iter = 10;
            for i in 0..iter {
                let inserted_level = i;
                let inserted_hash = format!("hash {i}").as_bytes().to_vec();

                let pendingconfirmation = PendingConfirmation {
                    level: inserted_level,
                    hash: inserted_hash.clone(),
                };
                pendingconfirmation.insert(conn)?;
            }

            let expected_count: i64 = iter.into();

            let count = PendingConfirmation::count(conn)?;

            assert_eq!(count, expected_count);

            let expected_clear: usize = iter as usize;

            let clear = PendingConfirmation::clear(conn)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
