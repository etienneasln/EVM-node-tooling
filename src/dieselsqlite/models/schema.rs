use crate::dieselsqlite::schema::sqlite_schema;
use diesel::{prelude::*, sql_query, sql_types::Text};

#[derive(Queryable, Selectable, QueryableByName)]
#[diesel(table_name = sqlite_schema)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Schema {
    pub schema_type: String,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: i32,
    pub sql: String,
}

impl Schema {
    pub fn get_all(connection: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        let schemas = sql_query(
            "SELECT type AS schema_type,name,tbl_name,rootpage,sql FROM sqlite_schema
             WHERE name NOT LIKE 'sqlite_%' AND name != 'migrations'",
        )
        .load::<Schema>(connection)?;
        let sqls = schemas.into_iter().map(|s| s.sql).collect::<Vec<String>>();

        Ok(sqls)
    }

    pub fn table_exists(connection: &mut SqliteConnection, table_name: &str) -> QueryResult<bool> {
        let schema_option = sql_query(
            "SELECT type AS schema_type,name,tbl_name,rootpage,sql FROM sqlite_schema
             WHERE schema_type='table' AND name = ?",
        )
        .bind::<Text, _>(table_name)
        .get_result::<Schema>(connection)
        .optional()?;
        Ok(schema_option.is_some())
    }
}

#[cfg(test)]
mod schema_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_schema() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let schema = Schema::get_all(conn).unwrap();

            let create_in_string = schema.iter().any(|s| s.contains("CREATE"));
            assert!(create_in_string);

            let blocks_exists = Schema::table_exists(conn, "blocks").unwrap();
            let no_table_exists = Schema::table_exists(conn, "no table").unwrap();

            assert!(blocks_exists);
            assert!(!no_table_exists);
            Ok(())
        })
    }
}
