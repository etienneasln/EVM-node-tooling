use crate::dieselsqlite::schema::blocks;
use diesel::{dsl::*, prelude::*, sql_types::Binary};

#[derive(Queryable, Selectable, QueryableByName, Insertable)]
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
        let b = sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
            .bind::<Binary, _>(queried_hash)
            .get_result::<Block>(connection)?;
        Ok(b.block)
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
        let b = sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
            .bind::<Binary, _>(queried_hash)
            .get_result::<Block>(connection)?;
        Ok(b.level)
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
