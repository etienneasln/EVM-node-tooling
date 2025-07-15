use diesel::{Connection, result::Error};
use evmnodetooling::dieselsqlite::{establish_connection, models::*};

#[test]
fn test_apply_blueprint_iterations() {
    let connection = &mut establish_connection().unwrap();

    connection.test_transaction::<_, Error, _>(|conn| {
        let select_index = Blueprint::base_level(conn)?;
        let clear_index = Blueprint::top_level(conn)?;

        let (payload, timestamp) = Blueprint::select(conn, select_index)?;
        let mut bytes = [0u8; 32];
        let block_vector = Block::select_with_level(conn, select_index)?;

        let transactions_receipts =
            Transaction::select_receipts_from_block_number(conn, select_index)?;
        let transaction_objects =
            Transaction::select_objects_from_block_number(conn, select_index)?;

        let context_hash_vector = ContextHash::select(conn, select_index)?;

        for _i in 0..10 {
            let insert_index = Blueprint::top_level(conn)? + 1;

            let blueprint = Blueprint {
                id: insert_index,
                payload: payload.clone(),
                timestamp,
            };

            rand::fill(&mut bytes);
            let hash = Vec::from(bytes);

            let block = Block {
                level: insert_index,
                hash: hash.clone(),
                block: block_vector.clone(),
            };

            let transactions = transactions_receipts
                .clone()
                .into_iter()
                .zip(transaction_objects.clone().into_iter())
                .map(
                    |(
                        (block_hash, index_, _, from_, to_, receipt_fields),
                        (_, _, _, _, object_fields),
                    )| Transaction {
                        block_hash,
                        block_number: insert_index,
                        index_,
                        hash: {
                            rand::fill(&mut bytes);
                            Vec::from(bytes)
                        },
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
            // println!("blueprint_id:{}",blueprint.id);
            blueprint.insert(conn)?;
            block.insert(conn)?;
            for tx in transactions {
                tx.insert(conn)?;
            }
            context_hash.insert(conn)?;
            let _history_mode = Metadata::get_history_mode(conn)?;

            let (insertedpayload, insertedtimestamp) = Blueprint::select(conn, insert_index)?;
            let insertedhash = Block::select_hash_of_number(conn, insert_index)?;
            let insertedblock = Block::select_with_level(conn, insert_index)?;

            let _inserted_transactions_receipts =
                Transaction::select_receipts_from_block_number(conn, insert_index)?;
            let _inserted_transaction_objects =
                Transaction::select_objects_from_block_number(conn, insert_index)?;

            let inserted_context_hash = ContextHash::select(conn, insert_index)?;

            assert_eq!(payload, insertedpayload);
            assert_eq!(timestamp, insertedtimestamp);
            assert_eq!(hash, insertedhash);
            assert_eq!(block_vector, insertedblock);
            // assert_eq!(transactions_receipts,inserted_transactions_receipts);
            // assert_eq!(transaction_objects,inserted_transaction_objects);
            assert_eq!(context_hash_vector, inserted_context_hash);
        }

        Blueprint::clear_after(conn, clear_index)?;
        Block::clear_after(conn, clear_index)?;
        ContextHash::clear_after(conn, clear_index)?;
        Transaction::clear_after(conn, clear_index)?;

        Ok(())
    })
}
