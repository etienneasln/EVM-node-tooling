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
