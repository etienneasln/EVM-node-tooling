use super::*;
use crate::dieselsqlite::schema::{migrations, migrations::dsl::*};

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
