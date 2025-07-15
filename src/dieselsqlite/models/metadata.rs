use crate::dieselsqlite::schema::{metadata, metadata::dsl::*};
use diesel::{prelude::*, upsert::excluded};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = metadata)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Metadata {
    pub key: String,
    pub value: String,
}

impl Metadata {
    pub fn insert_smart_rollup_address(
        connection: &mut SqliteConnection,
        inserted_value: &str,
    ) -> QueryResult<usize> {
        Metadata::insert_key_value(connection, "smart_rollup_address", inserted_value)
    }

    pub fn get_smart_rollup_address(connection: &mut SqliteConnection) -> QueryResult<String> {
        Metadata::get_value(connection, "smart_rollup_address")
    }

    pub fn insert_history_mode(
        connection: &mut SqliteConnection,
        inserted_value: &str,
    ) -> QueryResult<usize> {
        Metadata::insert_key_value(connection, "history_mode", inserted_value)
    }

    pub fn get_history_mode(connection: &mut SqliteConnection) -> QueryResult<String> {
        Metadata::get_value(connection, "history_mode")
    }

    fn insert_key_value(
        connection: &mut SqliteConnection,
        inserted_key: &str,
        inserted_value: &str,
    ) -> QueryResult<usize> {
        let metadata_object = Metadata {
            key: inserted_key.to_string(),
            value: inserted_value.to_string(),
        };
        let inserted_rows = metadata_object
            .insert_into(metadata)
            .on_conflict(key)
            .do_update()
            .set(value.eq(excluded(value)))
            .execute(connection)?;
        Ok(inserted_rows)
    }

    fn get_value(connection: &mut SqliteConnection, queried_key: &str) -> QueryResult<String> {
        let returned_value = metadata
            .find(queried_key)
            .select(value)
            .get_result(connection)?;
        Ok(returned_value)
    }
}

#[cfg(test)]
mod metadata_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_metadata_insert_select() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let history_mode = "new history mode";

            Metadata::insert_history_mode(conn, history_mode)?;

            let new_history_mode = Metadata::get_history_mode(conn)?;

            assert_eq!(new_history_mode, history_mode);

            let smart_rollup_node_address = "new smart rollup node address";

            Metadata::insert_smart_rollup_address(conn, smart_rollup_node_address)?;

            let new_smart_rollup_node_address = Metadata::get_smart_rollup_address(conn)?;

            assert_eq!(new_smart_rollup_node_address, smart_rollup_node_address);

            Ok(())
        })
    }
}
