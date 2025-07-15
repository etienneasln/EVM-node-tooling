use super::*;
use crate::dieselsqlite::schema::{blueprints, blueprints::dsl::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = blueprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blueprint {
    pub id: i32,
    pub payload: Vec<u8>,
    pub timestamp: i32,
}

impl Blueprint {
    pub fn select(
        connection: &mut SqliteConnection,
        queried_id: i32,
    ) -> QueryResult<(Vec<u8>, i32)> {
        let tuple = blueprints
            .find(queried_id)
            .select((payload, timestamp))
            .get_result(connection)?;
        Ok(tuple)
    }

    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = self.insert_into(blueprints).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_range(
        connection: &mut SqliteConnection,
        lowerlevel: i32,
        upperlevel: i32,
    ) -> QueryResult<Vec<(i32, Vec<u8>)>> {
        let vec = blueprints
            .filter(id.ge(lowerlevel).and(id.le(upperlevel)))
            .order(id.asc())
            .select((id, payload))
            .load(connection)?;
        Ok(vec)
    }

    pub fn clear_after(connection: &mut SqliteConnection, level: i32) -> QueryResult<usize> {
        let cleared_rows = delete(blueprints.filter(id.gt(level))).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection: &mut SqliteConnection, level: i32) -> QueryResult<usize> {
        let cleared_rows = delete(blueprints.filter(id.lt(level))).execute(connection)?;
        Ok(cleared_rows)
    }
    //For testing

    pub fn count(connection: &mut SqliteConnection) -> QueryResult<i64> {
        let count = blueprints.select(count(id)).first(connection)?;
        Ok(count)
    }

    pub fn base_level(connection: &mut SqliteConnection) -> QueryResult<i32> {
        let base_level = blueprints
            .select(id)
            .order(id.asc())
            .limit(1)
            .get_result(connection)?;
        Ok(base_level)
    }

    pub fn top_level(connection: &mut SqliteConnection) -> QueryResult<i32> {
        let top_level = blueprints
            .select(id)
            .order(id.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(top_level)
    }
}
