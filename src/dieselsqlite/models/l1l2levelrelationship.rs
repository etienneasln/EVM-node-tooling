use crate::dieselsqlite::schema::{l1_l2_levels_relationships, l1_l2_levels_relationships::dsl::*};
use diesel::{dsl::*, prelude::*};

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

#[cfg(test)]
mod l1_l2_level_relationship_test {
    use super::*;
    use crate::dieselsqlite::{Block, establish_connection};
    use diesel::result::Error;

    #[test]
    fn test_l1_l2_level_relationship_all() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let iter = 3;
            let latest_l2_level_base = Block::top_level(conn)?;
            let l1_level_base = 6000;
            for i in 0..iter {
                let relationship = L1L2LevelRelationship {
                    latest_l2_level: latest_l2_level_base + i,
                    l1_level: l1_level_base + i,
                };
                relationship.insert(conn)?;
            }

            let expected_get = (latest_l2_level_base + iter - 1, l1_level_base + iter - 1);

            let get = L1L2LevelRelationship::get(conn)?;

            assert_eq!(get, expected_get);

            let expected_clear = iter as usize;

            let clear = L1L2LevelRelationship::clear_after(conn, latest_l2_level_base - 1)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
