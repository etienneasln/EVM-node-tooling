use super::*;
use crate::dieselsqlite::schema::{l1_l2_levels_relationships, l1_l2_levels_relationships::dsl::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = l1_l2_levels_relationships)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct L1L2LevelRelationship {
    pub latest_l2_level: i32,
    pub l1_level: i32,
}

impl L1L2LevelRelationship {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = self
            .insert_into(l1_l2_levels_relationships)
            .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn get(connection: &mut SqliteConnection) -> QueryResult<(i32, i32)> {
        let get = l1_l2_levels_relationships
            .select((latest_l2_level, l1_level))
            .order_by(latest_l2_level.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(get)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows =
            delete(l1_l2_levels_relationships.filter(latest_l2_level.gt(queried_level)))
                .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows =
            delete(l1_l2_levels_relationships.filter(latest_l2_level.lt(queried_level)))
                .execute(connection)?;
        Ok(cleared_rows)
    }
}
