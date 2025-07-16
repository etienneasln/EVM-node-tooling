use crate::dieselsqlite::{models::cast_hash_comparison, schema::blocks};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = blocks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Block {
    pub level: i32,
    pub hash: Vec<u8>,
    pub block: Vec<u8>,
}

impl Block {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let inserted_rows = self.insert_into(blocks).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_with_level(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<Vec<u8>> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let b = blocks
            .find(queried_level)
            .select(block)
            .get_result(connection)?;
        Ok(b)
    }

    pub fn select_with_hash(
        connection: &mut SqliteConnection,
        queried_hash: &Vec<u8>,
    ) -> QueryResult<Vec<u8>> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let b = blocks
            .filter(cast_hash_comparison(queried_hash))
            .select(block)
            .get_result(connection)?;

        Ok(b)
    }

    pub fn select_hash_of_number(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<Vec<u8>> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let h = blocks
            .find(queried_level)
            .select(hash)
            .get_result(connection)?;
        Ok(h)
    }

    pub fn select_number_of_hash(
        connection: &mut SqliteConnection,
        queried_hash: &Vec<u8>,
    ) -> QueryResult<i32> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let n = blocks
            .filter(cast_hash_comparison(queried_hash))
            .select(level)
            .get_result(connection)?;
        Ok(n)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let cleared_rows = delete(blocks.filter(level.gt(queried_level))).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let cleared_rows = delete(blocks.filter(level.lt(queried_level))).execute(connection)?;
        Ok(cleared_rows)
    }

    //For testing

    pub fn count(connection: &mut SqliteConnection) -> QueryResult<i64> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let count = blocks.select(count(level)).first(connection)?;
        Ok(count)
    }

    pub fn base_level(connection: &mut SqliteConnection) -> QueryResult<i32> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let base_level = blocks
            .select(level)
            .order(level.asc())
            .limit(1)
            .get_result(connection)?;
        Ok(base_level)
    }

    pub fn top_level(connection: &mut SqliteConnection) -> QueryResult<i32> {
        use crate::dieselsqlite::schema::blocks::dsl::*;
        let base_level = blocks
            .select(level)
            .order(level.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(base_level)
    }
}

#[cfg(test)]
mod block_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;
    #[test]
    fn test_block_insert_selects_clearafter() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let inserted_hash = "hash".as_bytes().to_vec();
            let inserted_block = "block".as_bytes().to_vec();
            let base_insert_index = Block::top_level(conn)?;

            let insert_block = Block {
                level: base_insert_index + 1,
                hash: inserted_hash.clone(),
                block: inserted_block.clone(),
            };

            let _ = insert_block.insert(conn)?;

            let block_from_level = Block::select_with_level(conn, base_insert_index + 1)?;
            let hash_of_number = Block::select_hash_of_number(conn, base_insert_index + 1)?;
            let number_of_hash = Block::select_number_of_hash(conn, &hash_of_number)?;
            let block_from_hash = Block::select_with_hash(conn, &hash_of_number)?;

            assert_eq!(block_from_level, inserted_block);
            assert_eq!(hash_of_number, inserted_hash);
            assert_eq!(number_of_hash, base_insert_index + 1);
            assert_eq!(block_from_hash, inserted_block);

            let expected_rows_cleared: usize = 1;

            let rows_cleared = Block::clear_after(conn, base_insert_index)?;

            assert_eq!(rows_cleared, expected_rows_cleared);
            Ok(())
        })
    }

    #[test]
    fn test_block_selects() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let select_index = Block::top_level(conn)?;

            let block_from_level = Block::select_with_level(conn, select_index)?;
            let hash_of_number = Block::select_hash_of_number(conn, select_index)?;
            let number_of_hash = Block::select_number_of_hash(conn, &hash_of_number)?;
            let block_from_hash = Block::select_with_hash(conn, &hash_of_number)?;

            assert_eq!(block_from_hash, block_from_level);
            assert_eq!(number_of_hash, select_index);

            Ok(())
        })
    }
}
