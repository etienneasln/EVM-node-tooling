use crate::dieselsqlite::schema::{sqlite_schema};
use diesel::{prelude::*};

#[derive(Queryable, Selectable)]
#[diesel(table_name = sqlite_schema)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Schema {
    pub type_: String,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: i32,
    pub sql: String,
}

impl Schema {
    pub fn get_all(connection: &mut SqliteConnection) -> QueryResult<Vec<String>> {
        use crate::dieselsqlite::schema::sqlite_schema::dsl::*;
        let schemas=sqlite_schema
            .filter(name.not_like("sqlite_%").and(name.ne("migrations")))
            .select(sql)
            .load(connection)?;
        Ok(schemas)
    }
}

#[cfg(test)]
mod schema_test{
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::{debug_query, result::Error, sqlite::Sqlite};
    use crate::dieselsqlite::schema::sqlite_schema::dsl::*;

    #[test]
    fn test_schema(){
        let connection=&mut establish_connection().unwrap();

        connection.test_transaction::<_,Error,_>(|conn|{
            println!("{:?}", debug_query::<Sqlite,_>(&sqlite_schema
            .filter(name.not_like("sqlite_%").and(name.ne("migrations")))
            .select(sql)));
            println!("Get_all:{:?}", Schema::get_all(conn).unwrap());
            assert_eq!(0,1);
            Ok(())
        })
        
    }
}
