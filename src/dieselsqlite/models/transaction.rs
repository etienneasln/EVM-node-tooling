use crate::dieselsqlite::schema::transactions;
use diesel::{dsl::*, prelude::*, sql_query, sql_types::Binary};

#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub block_hash: Vec<u8>,
    pub block_number: i32,
    pub index_: i32,
    pub hash: Vec<u8>,
    pub from_: Vec<u8>,
    pub to_: Option<Vec<u8>>,
    pub receipt_fields: Vec<u8>,
    pub object_fields: Vec<u8>,
}

impl Transaction {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::transactions::dsl::*;
        let inserted_rows = self.insert_into(transactions).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_receipt(
        connection: &mut SqliteConnection,
        queried_hash: &Vec<u8>,
    ) -> QueryResult<(
        Vec<u8>,
        i32,
        i32,
        Vec<u8>,
        Vec<u8>,
        Option<Vec<u8>>,
        Vec<u8>,
    )> {
        let receipt = sql_query("SELECT * FROM transactions WHERE CAST(hash as BLOB)=?1")
            .bind::<Binary, _>(queried_hash)
            .get_result::<Transaction>(connection)?;
        Ok((
            receipt.block_hash,
            receipt.block_number,
            receipt.index_,
            receipt.hash,
            receipt.from_,
            receipt.to_,
            receipt.receipt_fields,
        ))
    }

    pub fn select_receipts_from_block_number(
        connection: &mut SqliteConnection,
        queried_block_number: i32,
    ) -> QueryResult<Vec<(Vec<u8>, i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>> {
        use crate::dieselsqlite::schema::transactions::dsl::*;
        let receipts = transactions
            .filter(block_number.eq(queried_block_number))
            .select((block_hash, index_, hash, from_, to_, receipt_fields))
            .load(connection)?;
        Ok(receipts)
    }

    pub fn select_object(
        connection: &mut SqliteConnection,
        queried_hash: &Vec<u8>,
    ) -> QueryResult<(
        Vec<u8>,
        i32,
        i32,
        Vec<u8>,
        Vec<u8>,
        Option<Vec<u8>>,
        Vec<u8>,
    )> {
        let object = sql_query("SELECT * FROM transactions WHERE CAST(hash as BLOB)=?1")
            .bind::<Binary, _>(queried_hash)
            .get_result::<Transaction>(connection)?;
        Ok((
            object.block_hash,
            object.block_number,
            object.index_,
            object.hash,
            object.from_,
            object.to_,
            object.object_fields,
        ))
    }

    pub fn select_objects_from_block_number(
        connection: &mut SqliteConnection,
        queried_block_number: i32,
    ) -> QueryResult<Vec<(i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>> {
        use crate::dieselsqlite::schema::transactions::dsl::*;
        let objects = transactions
            .filter(block_number.eq(queried_block_number))
            .select((index_, hash, from_, to_, object_fields))
            .load(connection)?;
        Ok(objects)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_block_number: i32,
    ) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::transactions::dsl::*;
        let cleared_rows = delete(transactions.filter(block_number.gt(queried_block_number)))
            .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_block_number: i32,
    ) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::transactions::dsl::*;
        let cleared_rows = delete(transactions.filter(block_number.lt(queried_block_number)))
            .execute(connection)?;
        Ok(cleared_rows)
    }
}
