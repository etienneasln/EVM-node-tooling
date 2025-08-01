use diesel::{ExpressionMethods, debug_query, dsl::*, sqlite::Sqlite};
use evmnodetooling::dieselsqlite::{
    establish_connection, models::*, schema::kernel_upgrades::dsl::*,
    schema::transactions::dsl::transactions,
};

fn main() {
    let connection = &mut establish_connection().unwrap();

    let base_level = Block::base_level(connection).unwrap();
    let top_level = Block::top_level(connection).unwrap();

    let (payload, timestamp) = Blueprint::select(connection, base_level).unwrap();

    println!("Payload:{:?}, Timestamp:{timestamp}", payload);

    let blueprint = Blueprint {
        id: top_level + 1,
        payload,
        timestamp,
    };

    let _ = blueprint.insert(connection).unwrap();

    let (payload, timestamp) = Blueprint::select(connection, top_level + 1).unwrap();

    println!("Payload:{:?}, Timestamp:{timestamp}", payload);

    let _ = Blueprint::clear_after(connection, top_level);

    let tuplevec = Blueprint::select_range(connection, top_level - 2, top_level).unwrap();

    for (id, payload) in tuplevec {
        println!("Id:{id}, payload:{:?}", payload);
        println!("------------------------------");
    }

    let block = Block::select_with_level(connection, base_level).unwrap();
    println!("Block:{:?}", block);

    let hash = Block::select_hash_of_number(connection, base_level).unwrap();
    println!("Block hash:{:?}", hash);

    let blockfromhash = Block::select_with_hash(connection, &hash).unwrap();
    println!("Block from hash:{:?}", blockfromhash);

    let idfromhash = Block::select_number_of_hash(connection, &hash).unwrap();
    println!("Block from hash:{:?}", blockfromhash);

    assert_eq!(block, blockfromhash);
    assert_eq!(idfromhash, base_level);

    let block = Block {
        level: top_level + 1,
        hash: "Random hash".as_bytes().to_vec(),
        block: block,
    };

    let _ = block.insert(connection).unwrap();

    let block = Block::select_with_level(connection, top_level + 1).unwrap();
    println!("Block:{:?}", block);

    let _ = Block::clear_after(connection, top_level);

    println!("Block count:{}", Block::count(connection).unwrap());
    println!("Blueprint count:{}", Blueprint::count(connection).unwrap());

    let receipts = Transaction::select_receipts_from_block_number(connection, top_level).unwrap();

    println!("Transaction receipts top level block:{:?}", receipts);

    let objects = Transaction::select_objects_from_block_number(connection, top_level).unwrap();

    println!("Transaction objects top level block:{:?}", objects);

    let (vec_block_hash, vec_index_, vec_hash, vec_from_, vec_to_, vec_receipt_fields) =
        (&receipts[0]).clone();
    let (_, _, _, _, vec_object_fields) = (&objects[0]).clone();

    let (block_hash, block_number, index_, hash, from_, to_, receipt_fields) =
        Transaction::select_receipt(connection, &vec_hash).unwrap();
    let (_, _, _, _, _, _, object_fields) =
        Transaction::select_object(connection, &vec_hash).unwrap();

    assert_eq!(block_hash, vec_block_hash);
    assert_eq!(block_number, top_level);
    assert_eq!(index_, vec_index_);
    assert_eq!(hash, vec_hash);
    assert_eq!(from_, vec_from_);
    assert_eq!(to_, vec_to_);
    assert_eq!(receipt_fields, vec_receipt_fields);
    assert_eq!(object_fields, vec_object_fields);

    let kernel_upgrade = KernelUpgrade {
        injected_before: 1000,
        root_hash: "hash".as_bytes().to_vec(),
        activation_timestamp: 2000,
        applied_before: None,
    };

    let binding1 = replace_into(kernel_upgrades).values((
        injected_before.eq(kernel_upgrade.injected_before),
        root_hash.eq(kernel_upgrade.root_hash.clone()),
        activation_timestamp.eq(kernel_upgrade.activation_timestamp),
    ));

    let binding2 = update(diesel::QueryDsl::filter(
        kernel_upgrades,
        applied_before.gt(1000),
    ))
    .set(applied_before.eq::<Option<i32>>(None));

    let sql = debug_query::<Sqlite, _>(&binding1);
    println!("SQL:{:?}", sql);
    let sql = debug_query::<Sqlite, _>(&binding2);
    println!("SQL:{:?}", sql);

    let _ = kernel_upgrade.insert(connection).unwrap();
    let _ = KernelUpgrade::record_apply(connection, 1004);
    let _ = KernelUpgrade::nullify_after(connection, 1003).unwrap();
    let latest_unapplied = KernelUpgrade::get_latest_unapplied(connection).unwrap();
    println!("Latest unapplied:{:?}", latest_unapplied);
    let _ = KernelUpgrade::clear_after(connection, 999);

    let iter = 5;
    let inserted_block_hash: Vec<u8> = "block_hash".as_bytes().to_vec();
    let inserted_block_number = Block::top_level(connection).unwrap() + 1;
    let inserted_index_ = 0;
    let inserted_from_ = "from_".as_bytes().to_vec();
    let inserted_to_ = Some("to_".as_bytes().to_vec());
    let inserted_receipt_fields = "receipt_fields".as_bytes().to_vec();
    let inserted_object_fields = "object_fields".as_bytes().to_vec();

    let mut batch = Vec::new();
    for i in 0..iter {
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

    let binding3 = insert_into(transactions).values(&batch);

    let sql = debug_query::<Sqlite, _>(&binding3);
    println!("Batch inserts:{:?}", sql);
}
