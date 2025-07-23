use crate::dieselsqlite::schema::sqlite_schema;
use diesel::{
    dsl::{exists, select},
    prelude::*,
};

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
        use crate::dieselsqlite::schema::sqlite_schema::dsl::*;
        let sqls = sqlite_schema
            .filter(name.not_like("sqlite_%").and(name.ne("migrations")))
            .select(sql)
            .load(connection)?;
        Ok(sqls)
    }

    pub fn table_exists(connection: &mut SqliteConnection, table_name: &str) -> QueryResult<bool> {
        use crate::dieselsqlite::schema::sqlite_schema::dsl::*;
        let exists_bool = select(exists(
            sqlite_schema.filter(schema_type.eq("table").and(name.eq(table_name))),
        ))
        .get_result(connection)?;
        Ok(exists_bool)
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
