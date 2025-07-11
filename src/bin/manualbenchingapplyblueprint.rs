use std::time::Instant;

use diesel::{Connection, result::Error};
use evmnodetooling::dieselsqlite::{models::*, *};

fn main() {
    let connection = &mut establish_connection().unwrap();
    let select_index = load_block_number();
    let clear_index = Blueprint::top_level(connection).unwrap();
    let insert_index = clear_index + 1;
    // println!("Insert_index:{}",insert_index);
    let (payload, timestamp) = Blueprint::select(connection, select_index).unwrap();
    let blueprint = Blueprint {
        id: insert_index,
        payload,
        timestamp,
    };
    let block = Block::select_with_level(connection, select_index).unwrap();
    let mut bytes = [0u8; 32];
    rand::fill(&mut bytes);
    let block = Block {
        level: insert_index,
        hash: Vec::from(bytes),
        block,
    };
    let transactions_receipts =
        Transaction::select_receipts_from_block_number(connection, select_index).unwrap();
    let transaction_objects =
        Transaction::select_objects_from_block_number(connection, select_index).unwrap();

    let transactions = transactions_receipts
        .into_iter()
        .zip(transaction_objects.into_iter())
        .map(
            |((block_hash, index_, _, from_, to_, receipt_fields), (_, _, _, _, object_fields))| {
                Transaction {
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
                }
            },
        )
        .collect::<Vec<Transaction>>();
    let context_hash = ContextHash {
        id: insert_index,
        context_hash: ContextHash::select(connection, select_index).unwrap(),
    };
    let now = Instant::now();
    connection
        .transaction::<_, Error, _>(|conn| {
            let _ = PendingConfirmation::select_with_level(conn, blueprint.id);
            // println!("blueprint_id:{}",blueprint.id);
            blueprint.insert(conn)?;
            block.insert(conn)?;
            for tx in transactions {
                tx.insert(conn)?;
            }
            context_hash.insert(conn)?;
            let _history_mode = Metadata::get_history_mode(conn)?;

            Ok(())
        })
        .unwrap();
    let elapsed = now.elapsed();

    let _ = Blueprint::clear_after(connection, clear_index).unwrap();
    let _ = Block::clear_after(connection, clear_index).unwrap();
    let _ = ContextHash::clear_after(connection, clear_index).unwrap();
    let _ = Transaction::clear_after(connection, clear_index).unwrap();
    println!("Single iteration apply_blueprint:{:?}", elapsed);
}
