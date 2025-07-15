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
