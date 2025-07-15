use crate::dieselsqlite::schema::{migrations, migrations::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = migrations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Migration {
    pub id: i32,
    pub name: Option<String>,
}

impl Migration {
    pub fn create_table(connection: &mut SqliteConnection) -> QueryResult<usize> {
        let create = sql_query(
            "CREATE TABLE migrations (
        id SERIAL PRIMARY KEY,
        name TEXT
        );",
        )
        .execute(connection)?;
        Ok(create)
    }

    pub fn current_migration(connection: &mut SqliteConnection) -> QueryResult<i32> {
        let current_id = migrations
            .select(id)
            .order_by(id.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(current_id)
    }

    pub fn register_migration(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = insert_into(migrations).values(&self).execute(connection)?;
        Ok(inserted_rows)
    }
}

#[cfg(test)]
mod migration_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_migration_all() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let _ = Migration::create_table(conn);

            let current_migration = Migration::current_migration(conn)?;

            let current_id = current_migration + 1;
            let current_name = Some(String::from("test text"));

            let migration = Migration {
                id: current_id,
                name: current_name,
            };

            migration.register_migration(conn)?;

            let new_current_migration = Migration::current_migration(conn)?;

            assert_eq!(new_current_migration, current_id);

            Ok(())
        });
    }
}
