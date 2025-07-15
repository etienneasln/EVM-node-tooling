use crate::dieselsqlite::schema::{block_storage_mode, block_storage_mode::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = block_storage_mode)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BlockStorageMode {
    pub legacy: i32,
}

impl BlockStorageMode {
    pub fn legacy(connection: &mut SqliteConnection) -> QueryResult<i32> {
        let leg = block_storage_mode.select(legacy).get_result(connection)?;
        Ok(leg)
    }

    pub fn force_legacy(connection: &mut SqliteConnection) -> QueryResult<usize> {
        let updated_rows = update(block_storage_mode)
            .set(legacy.eq(1))
            .execute(connection)?;
        Ok(updated_rows)
    }
}
