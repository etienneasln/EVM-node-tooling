use crate::dieselsqlite::schema::sqlite_schema;
use diesel::{prelude::*, sql_query};

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
            "SELECT * FROM sqlite_schema
             WHERE name NOT LIKE 'sqlite_%' AND name != 'migrations'",
        )
        .load::<Schema>(connection)?;
        // let schemas=sqlite_schema
        //     .filter(name.not_like("sqlite_%").and(name.ne("migrations")))
        //     .select(sql)
        //     .load(connection)?;
        let sqls = schemas.into_iter().map(|s| s.sql).collect::<Vec<String>>();
        Ok(sqls)
    }
}

#[cfg(test)]
mod schema_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use crate::dieselsqlite::schema::sqlite_schema::dsl::*;
    use diesel::{debug_query, result::Error, sqlite::Sqlite};

    #[test]
    fn test_schema() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            println!(
                "{:?}",
                debug_query::<Sqlite, _>(
                    &sqlite_schema
                        .filter(name.not_like("sqlite_%").and(name.ne("migrations")))
                        .select(sql)
                )
            );
            println!("Get_all:{:?}", Schema::get_all(conn).unwrap());
            assert_eq!(0, 1);
            Ok(())
        })
    }
}
