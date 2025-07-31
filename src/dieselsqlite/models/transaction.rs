use crate::dieselsqlite::{
    models::cast_hash_comparison,
    schema::{transactions, transactions::dsl::*},
};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_default_value = false)]
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
        let inserted_rows = self.insert_into(transactions).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn batch_insert(
        batch: &Vec<Self>,
        connection: &mut SqliteConnection,
    ) -> QueryResult<usize> {
        let inserted_rows = insert_into(transactions)
            .values(batch)
            .execute(connection)?;
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
        let (block_h, block_n, index, h, from, to, receipt_f) = transactions
            .filter(cast_hash_comparison(queried_hash))
            .select((
                block_hash,
                block_number,
                index_,
                hash,
                from_,
                to_,
                receipt_fields,
            ))
            .get_result(connection)?;
        Ok((block_h, block_n, index, h, from, to, receipt_f))
    }

    pub fn select_receipts_from_block_number(
        connection: &mut SqliteConnection,
        queried_block_number: i32,
    ) -> QueryResult<Vec<(Vec<u8>, i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>> {
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
        let (block_h, block_n, index, h, from, to, object_f) = transactions
            .filter(cast_hash_comparison(queried_hash))
            .select((
                block_hash,
                block_number,
                index_,
                hash,
                from_,
                to_,
                object_fields,
            ))
            .get_result(connection)?;
        Ok((block_h, block_n, index, h, from, to, object_f))
    }

    pub fn select_objects_from_block_number(
        connection: &mut SqliteConnection,
        queried_block_number: i32,
    ) -> QueryResult<Vec<(i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>> {
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
        let cleared_rows = delete(transactions.filter(block_number.gt(queried_block_number)))
            .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_block_number: i32,
    ) -> QueryResult<usize> {
        let cleared_rows = delete(transactions.filter(block_number.lt(queried_block_number)))
            .execute(connection)?;
        Ok(cleared_rows)
    }
}

#[cfg(test)]
mod transaction_test {
    use super::*;
    use crate::dieselsqlite::{establish_connection, models::Block};
    use diesel::result::Error;

    #[test]
    fn test_transaction_insert_select_clear() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let inserted_block_hash: Vec<u8> = "block_hash".as_bytes().to_vec();
            let inserted_block_number = Block::top_level(conn)? + 1;
            let inserted_index_ = 0;
            let inserted_hash: Vec<u8> = "transactionHash".as_bytes().to_vec();
            let inserted_from_ = "from_".as_bytes().to_vec();
            let inserted_to_ = Some("to_".as_bytes().to_vec());
            let inserted_receipt_fields = "receipt_fields".as_bytes().to_vec();
            let inserted_object_fields = "object_fields".as_bytes().to_vec();

            let transaction = Transaction {
                block_hash: inserted_block_hash.clone(),
                block_number: inserted_block_number,
                index_: inserted_index_,
                hash: inserted_hash.clone(),
                from_: inserted_from_.clone(),
                to_: inserted_to_.clone(),
                receipt_fields: inserted_receipt_fields.clone(),
                object_fields: inserted_object_fields.clone(),
            };

            let _ = transaction.insert(conn);

            let (
                select_block_hash,
                select_block_number,
                select_index_,
                select_hash,
                select_from_,
                select_to_,
                select_receipt_fields,
            ) = Transaction::select_receipt(conn, &inserted_hash)?;

            assert_eq!(select_block_hash, inserted_block_hash);
            assert_eq!(select_block_number, inserted_block_number);
            assert_eq!(select_index_, inserted_index_);
            assert_eq!(select_hash, inserted_hash);
            assert_eq!(select_from_, inserted_from_);
            assert_eq!(select_to_, inserted_to_);
            assert_eq!(select_receipt_fields, inserted_receipt_fields);

            let expected_rows_cleared = 1;

            let rows_cleared = Transaction::clear_after(conn, inserted_block_number - 1)?;

            assert_eq!(rows_cleared, expected_rows_cleared);
            Ok(())
        })
    }

    #[test]
    fn test_transaction_selects() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let select_block_level = Block::top_level(conn)?;

            let receipts =
                Transaction::select_receipts_from_block_number(conn, select_block_level)?;

            let objects = Transaction::select_objects_from_block_number(conn, select_block_level)?;

            let length = receipts.len();
            for i in 0..length {
                let (vec_block_hash, vec_index_, vec_hash, vec_from_, vec_to_, vec_receipt_fields) =
                    (&receipts[i]).clone();
                let (_, _, _, _, vec_object_fields) = (&objects[i]).clone();

                let (
                    select_block_hash,
                    select_block_number,
                    select_index_,
                    select_hash,
                    select_from_,
                    select_to_,
                    select_receipt_fields,
                ) = Transaction::select_receipt(conn, &vec_hash)?;
                let (_, _, _, _, _, _, select_object_fields) =
                    Transaction::select_object(conn, &vec_hash)?;

                assert_eq!(select_block_hash, vec_block_hash);
                assert_eq!(select_block_number, select_block_level);
                assert_eq!(select_index_, vec_index_);
                assert_eq!(select_hash, vec_hash);
                assert_eq!(select_from_, vec_from_);
                assert_eq!(select_to_, vec_to_);
                assert_eq!(select_receipt_fields, vec_receipt_fields);
                assert_eq!(select_object_fields, vec_object_fields);
            }

            Ok(())
        })
    }

    #[test]
    fn test_transaction_batch_insert_clear() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let iter=5;
            let inserted_block_hash: Vec<u8> = "block_hash".as_bytes().to_vec();
            let inserted_block_number = Block::top_level(conn)? + 1;
            let inserted_index_ = 0;
            let inserted_from_ = "from_".as_bytes().to_vec();
            let inserted_to_ = Some("to_".as_bytes().to_vec());
            let inserted_receipt_fields = "receipt_fields".as_bytes().to_vec();
            let inserted_object_fields = "object_fields".as_bytes().to_vec();

            let mut batch=Vec::new();
            for i in 0..iter{
                
                let inserted_hash: Vec<u8> = format!("transactionHash:{i}").as_bytes().to_vec();
               
                let transaction = Transaction {
                    block_hash: inserted_block_hash.clone(),
                    block_number: inserted_block_number,
                    index_: inserted_index_,
                    hash: inserted_hash.clone(),
                    from_: inserted_from_.clone(),
                    to_: inserted_to_.clone(),
                    receipt_fields: inserted_receipt_fields.clone(),
                    object_fields: inserted_object_fields.clone(),
                };

                batch.push(transaction);
            }

            Transaction::batch_insert(&batch, conn)?;
            

            
            let expected_rows_cleared = iter as usize;

            let rows_cleared = Transaction::clear_after(conn, inserted_block_number - 1)?;

            assert_eq!(rows_cleared, expected_rows_cleared);
            Ok(())
        })
    }

}
