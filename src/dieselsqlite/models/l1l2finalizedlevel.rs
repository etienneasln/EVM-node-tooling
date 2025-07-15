use crate::dieselsqlite::schema::{l1_l2_finalized_levels, l1_l2_finalized_levels::dsl::*};
use diesel::{dsl::*, prelude::*, result::Error::NotFound};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = l1_l2_finalized_levels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct L1L2FinalizedLevel {
    pub l1_level: i32,
    pub start_l2_level: i32,
    pub end_l2_level: i32,
}

impl L1L2FinalizedLevel {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = replace_into(l1_l2_finalized_levels)
            .values(&self)
            .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn get(
        connection: &mut SqliteConnection,
        queried_l1_level: i32,
    ) -> QueryResult<(i32, i32)> {
        let get = l1_l2_finalized_levels
            .find(queried_l1_level)
            .select((start_l2_level, end_l2_level))
            .get_result(connection)?;
        Ok(get)
    }

    pub fn last_l2_level(connection: &mut SqliteConnection) -> QueryResult<i32> {
        let max: Option<i32> = l1_l2_finalized_levels
            .select(max(end_l2_level))
            .get_result(connection)?;
        max.ok_or_else(|| NotFound)
    }

    pub fn last(connection: &mut SqliteConnection) -> QueryResult<(i32, i32, i32)> {
        let last = l1_l2_finalized_levels
            .select((l1_level, start_l2_level, end_l2_level))
            .order_by(l1_level.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(last)
    }

    pub fn find_l1_level(
        connection: &mut SqliteConnection,
        queried_l2_level: i32,
    ) -> QueryResult<i32> {
        let find = l1_l2_finalized_levels
            .filter(
                start_l2_level
                    .lt(queried_l2_level)
                    .and(end_l2_level.ge(queried_l2_level)),
            )
            .select(l1_level)
            .order_by(l1_level.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(find)
    }

    pub fn list_by_l2_levels(
        connection: &mut SqliteConnection,
        start_l2: i32,
        end_l2: i32,
    ) -> QueryResult<Vec<(i32, i32, i32)>> {
        let list = l1_l2_finalized_levels
            .filter(start_l2_level.ge(start_l2).and(end_l2_level.le(end_l2)))
            .select((l1_level, start_l2_level, end_l2_level))
            .order_by(l1_level.asc())
            .load(connection)?;
        Ok(list)
    }

    pub fn list_by_l1_levels(
        connection: &mut SqliteConnection,
        start_l1: i32,
        end_l1: i32,
    ) -> QueryResult<Vec<(i32, i32, i32)>> {
        let list = l1_l2_finalized_levels
            .filter(l1_level.between(start_l1, end_l1))
            .select((l1_level, start_l2_level, end_l2_level))
            .order_by(l1_level.asc())
            .load(connection)?;
        Ok(list)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows = delete(l1_l2_finalized_levels.filter(end_l2_level.gt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows = delete(l1_l2_finalized_levels.filter(start_l2_level.lt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }
}
