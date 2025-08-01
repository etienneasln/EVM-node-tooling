use diesel::{Connection, result::Error};
use evmnodetooling::dieselsqlite::{establish_connection, models::*};

#[test]
fn test_apply_blueprint_iterations() {
    let connection = &mut establish_connection().unwrap();

    connection.test_transaction::<_, Error, _>(|conn| {
        let iter = 10;
        let select_index = Blueprint::base_level(conn)?;
        let clear_index = Blueprint::top_level(conn)?;

        let (payload, timestamp) = Blueprint::select(conn, select_index)?;
        let mut bytes = [0u8; 32];
        let block_vector = Block::select_with_level(conn, select_index)?;

        let transaction_receipts =
            Transaction::select_receipts_from_block_number(conn, select_index)?;
        let transaction_objects =
            Transaction::select_objects_from_block_number(conn, select_index)?;

        let transactions_len = transaction_receipts.len();

        let context_hash_vector = ContextHash::select(conn, select_index)?;

        for _ in 0..iter {
            let insert_index = Blueprint::top_level(conn)? + 1;

            let blueprint = Blueprint {
                id: insert_index,
                payload: payload.clone(),
                timestamp,
            };

            rand::fill(&mut bytes);
            let hash = Vec::from(bytes);

            let transactions_hash = (0..transactions_len)
                .map(|_| {
                    rand::fill(&mut bytes);
                    Vec::from(bytes)
                })
                .collect::<Vec<Vec<u8>>>();

            let block = Block {
                level: insert_index,
                hash: hash.clone(),
                block: block_vector.clone(),
            };

            let transactions = transaction_receipts
                .clone()
                .into_iter()
                .zip(transactions_hash.clone().into_iter())
                .zip(transaction_objects.clone().into_iter())
                .map(
                    |(
                        ((block_hash, index_, _, from_, to_, receipt_fields), hash),
                        (_, _, _, _, object_fields),
                    )| Transaction {
                        block_hash,
                        block_number: insert_index,
                        index_,
                        hash,
                        from_,
                        to_,
                        receipt_fields,
                        object_fields,
                    },
                )
                .collect::<Vec<Transaction>>();

            let context_hash = ContextHash {
                id: insert_index,
                context_hash: context_hash_vector.clone(),
            };

            let _ = PendingConfirmation::select_with_level(conn, blueprint.id);
            blueprint.insert(conn)?;
            block.insert(conn)?;
            Transaction::batch_insert(conn, &transactions)?;
            context_hash.insert(conn)?;
            let _history_mode = Metadata::get_history_mode(conn)?;

            let (insertedpayload, insertedtimestamp) = Blueprint::select(conn, insert_index)?;
            let insertedhash = Block::select_hash_of_number(conn, insert_index)?;
            let insertedblock = Block::select_with_level(conn, insert_index)?;

            let inserted_transaction_receipts =
                Transaction::select_receipts_from_block_number(conn, insert_index)?;
            let inserted_transaction_objects =
                Transaction::select_objects_from_block_number(conn, insert_index)?;

            let transaction_receipts = transaction_receipts
                .clone()
                .into_iter()
                .zip(transactions_hash.clone().into_iter())
                .map(
                    |((block_hash, index_, _, from_, to_, receipt_fields), hash)| {
                        (block_hash, index_, hash, from_, to_, receipt_fields)
                    },
                )
                .collect::<Vec<(Vec<u8>, i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>>();

            let transaction_objects = transaction_objects
                .clone()
                .into_iter()
                .zip(transactions_hash.clone().into_iter())
                .map(|((index_, _, from_, to_, object_fields), hash)| {
                    (index_, hash, from_, to_, object_fields)
                })
                .collect::<Vec<(i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>>();

            let inserted_context_hash = ContextHash::select(conn, insert_index)?;

            assert_eq!(payload, insertedpayload);
            assert_eq!(timestamp, insertedtimestamp);
            assert_eq!(hash, insertedhash);
            assert_eq!(block_vector, insertedblock);
            assert_eq!(transaction_receipts, inserted_transaction_receipts);
            assert_eq!(transaction_objects, inserted_transaction_objects);
            assert_eq!(context_hash_vector, inserted_context_hash);
        }

        Blueprint::clear_after(conn, clear_index)?;
        Block::clear_after(conn, clear_index)?;
        ContextHash::clear_after(conn, clear_index)?;
        Transaction::clear_after(conn, clear_index)?;

        Ok(())
    })
}
