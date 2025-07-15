use crate::dieselsqlite::schema::{irmin_chunks, irmin_chunks::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = irmin_chunks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IrminChunk {
    pub level: i32,
    pub timestamp: i32,
}

impl IrminChunk {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = self.insert_into(irmin_chunks).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn nth(connection: &mut SqliteConnection, offset: i64) -> QueryResult<(i32, i32)> {
        let nth = irmin_chunks
            .select((level, timestamp))
            .order_by(level.desc())
            .limit(1)
            .offset(offset)
            .get_result(connection)?;
        Ok(nth)
    }

    pub fn latest(connection: &mut SqliteConnection) -> QueryResult<(i32, i32)> {
        let latest = irmin_chunks
            .select((level, timestamp))
            .order_by(level.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(latest)
    }

    pub fn clear(connection: &mut SqliteConnection) -> QueryResult<usize> {
        let cleared_rows = delete(irmin_chunks).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows =
            delete(irmin_chunks.filter(level.gt(queried_level))).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before_included(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows =
            delete(irmin_chunks.filter(level.le(queried_level))).execute(connection)?;
        Ok(cleared_rows)
    }
}

#[cfg(test)]
mod irmin_chunk_test {
    use super::*;
    use crate::dieselsqlite::{Block, establish_connection};
    use diesel::result::Error;

    #[test]
    fn test_irmin_chunk_all() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            IrminChunk::clear(conn)?;

            let iter = 3;
            let level_base = Block::top_level(conn)?;
            let timestamp_base = 6000;
            for i in 0..iter {
                let chunk = IrminChunk {
                    level: level_base + i,
                    timestamp: timestamp_base + i,
                };
                chunk.insert(conn)?;
            }

            let expected_nth = (level_base, timestamp_base);

            let nth = IrminChunk::nth(conn, (iter - 1) as i64)?;

            assert_eq!(nth, expected_nth);

            let expected_latest = (level_base + iter - 1, timestamp_base + iter - 1);

            let latest = IrminChunk::latest(conn)?;

            assert_eq!(latest, expected_latest);

            let expected_clear = iter as usize;

            let clear = IrminChunk::clear_after(conn, level_base - 1)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
