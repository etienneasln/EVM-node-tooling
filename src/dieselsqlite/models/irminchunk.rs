use crate::dieselsqlite::schema::{irmin_chunks, irmin_chunks::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = irmin_chunks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IrminChunk {
    pub level: i32,
    pub timestamp: i32,
}

impl IrminChunk {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = self.insert_into(irmin_chunks).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn nth(connection: &mut SqliteConnection, offset: i64) -> QueryResult<(i32, i32)> {
        let nth = irmin_chunks
            .select((level, timestamp))
            .order_by(level.desc())
            .limit(1)
            .offset(offset)
            .get_result(connection)?;
        Ok(nth)
    }

    pub fn latest(connection: &mut SqliteConnection) -> QueryResult<(i32, i32)> {
        let latest = irmin_chunks
            .select((level, timestamp))
            .order_by(level.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(latest)
    }

    pub fn clear(connection: &mut SqliteConnection) -> QueryResult<usize> {
        let cleared_rows = delete(irmin_chunks).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows =
            delete(irmin_chunks.filter(level.gt(queried_level))).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before_included(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows =
            delete(irmin_chunks.filter(level.le(queried_level))).execute(connection)?;
        Ok(cleared_rows)
    }
}
