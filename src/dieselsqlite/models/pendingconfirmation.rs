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
