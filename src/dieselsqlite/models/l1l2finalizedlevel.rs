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

#[cfg(test)]
mod l1_l2_finalized_level_test {
    use super::*;
    use crate::dieselsqlite::{Block, establish_connection};
    use diesel::result::Error;

    #[test]
    fn test_l1_l2_finalized_level_all() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            L1L2FinalizedLevel::clear_after(conn, 0)?;
            let iter = 3;
            let l1_level_base = 6000;
            let start_l2_level_base = Block::top_level(conn)?;
            let span = iter;
            let end_l2_level_base = start_l2_level_base + span;

            for i in 0..iter {
                let finalizedlevel = L1L2FinalizedLevel {
                    l1_level: l1_level_base + i,
                    start_l2_level: start_l2_level_base + i,
                    end_l2_level: end_l2_level_base + i,
                };
                finalizedlevel.insert(conn)?;
            }

            let expected_get = (start_l2_level_base, end_l2_level_base);

            let get = L1L2FinalizedLevel::get(conn, l1_level_base)?;

            assert_eq!(get, expected_get);

            let expected_last_l2_level = end_l2_level_base + iter - 1;

            let last_l2_level = L1L2FinalizedLevel::last_l2_level(conn)?;

            assert_eq!(last_l2_level, expected_last_l2_level);

            let expected_last = (
                l1_level_base + iter - 1,
                start_l2_level_base + iter - 1,
                end_l2_level_base + iter - 1,
            );

            let last = L1L2FinalizedLevel::last(conn)?;

            assert_eq!(last, expected_last);

            let expected_l1_level = l1_level_base + iter - 1;

            let find_l1_level = L1L2FinalizedLevel::find_l1_level(conn, end_l2_level_base)?;

            assert_eq!(find_l1_level, expected_l1_level);

            let expected_list = (0..iter)
                .map(|i| {
                    (
                        l1_level_base + i,
                        start_l2_level_base + i,
                        end_l2_level_base + i,
                    )
                })
                .collect::<Vec<(i32, i32, i32)>>();

            let list_by_l2 = L1L2FinalizedLevel::list_by_l2_levels(
                conn,
                start_l2_level_base,
                end_l2_level_base + iter - 1,
            )?;

            let list_by_l1 = L1L2FinalizedLevel::list_by_l1_levels(
                conn,
                l1_level_base,
                l1_level_base + iter - 1,
            )?;

            assert_eq!(list_by_l2, expected_list);
            assert_eq!(list_by_l1, expected_list);

            let expected_clear = iter as usize;

            let clear = L1L2FinalizedLevel::clear_after(conn, end_l2_level_base - 1)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
