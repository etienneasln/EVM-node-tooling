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

#[cfg(test)]
mod block_storage_mode_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_block_storage_mode() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let mut expected_legacy = 0;

            let current_legacy = BlockStorageMode::legacy(conn)?;

            assert_eq!(current_legacy, expected_legacy);

            BlockStorageMode::force_legacy(conn)?;

            expected_legacy = 1;

            let new_legacy = BlockStorageMode::legacy(conn)?;

            assert_eq!(new_legacy, expected_legacy);

            Ok(())
        });
    }
}
