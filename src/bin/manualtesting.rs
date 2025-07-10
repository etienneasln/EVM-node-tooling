use diesel::{debug_query, dsl::*, sqlite::Sqlite, ExpressionMethods};
use evmnodetooling::dieselsqlite::{establish_connection, models::*, schema::kernel_upgrades::dsl::*};

fn main(){
    let connection=&mut establish_connection().unwrap();

    let base_level=Block::base_level(connection).unwrap();
    let top_level=Block::top_level(connection).unwrap();
    
    let (payload,timestamp)=Blueprint::select(connection, base_level).unwrap();
    
    println!("Payload:{:?}, Timestamp:{timestamp}", payload);

    let blueprint=Blueprint{
        id:top_level+1,payload,timestamp
    };
    
    let _=blueprint.insert(connection).unwrap();
    
    
    let (payload,timestamp)=Blueprint::select(connection, top_level+1).unwrap();
    
    println!("Payload:{:?}, Timestamp:{timestamp}",payload);
    
    let _=Blueprint::clear_after(connection, top_level);
    
    let tuplevec=Blueprint::select_range(connection, top_level-2, top_level).unwrap();
    
    for (id,payload) in tuplevec{
        println!("Id:{id}, payload:{:?}",payload);
        println!("------------------------------");
    }


    let block = Block::select_with_level(connection, base_level).unwrap();
    println!("Block:{:?}",block);

    let hash=Block::select_hash_of_number(connection, base_level).unwrap();
    println!("Block hash:{:?}",hash);

    let blockfromhash=Block::select_with_hash(connection, &hash).unwrap();
    println!("Block from hash:{:?}",blockfromhash);

    let idfromhash=Block::select_number_of_hash(connection, &hash).unwrap();
    println!("Block from hash:{:?}",blockfromhash);

    assert_eq!(block,blockfromhash);
    assert_eq!(idfromhash,base_level);

    let block=Block{
        level:top_level+1,hash:"Random hash".as_bytes().to_vec(),block:block
    };

    let _=block.insert(connection).unwrap();

    let block = Block::select_with_level(connection, top_level+1).unwrap();
    println!("Block:{:?}",block);

    
    let _=Block::clear_after(connection, top_level);

    println!("Block count:{}", Block::count(connection).unwrap());
    println!("Blueprint count:{}",Blueprint::count(connection).unwrap());

    let receipts=Transaction::select_receipts_from_block_number(connection, top_level).unwrap();

    println!("Transaction receipts top level block:{:?}",receipts);

    let objects=Transaction::select_objects_from_block_number(connection, top_level).unwrap();

    println!("Transaction objects top level block:{:?}",objects);

    let (vec_block_hash,vec_index_,vec_hash,vec_from_,vec_to_,vec_receipt_fields)=(&receipts[0]).clone();
    let (_,_,_,_,vec_object_fields)=(&objects[0]).clone();

    let (block_hash,block_number,index_,hash,from_,to_,receipt_fields)=Transaction::select_receipt(connection, &vec_hash).unwrap();
    let (_,_,_,_,_,_,object_fields)=Transaction::select_object(connection, &vec_hash).unwrap();


    assert_eq!(block_hash,vec_block_hash);
    assert_eq!(block_number,top_level);
    assert_eq!(index_, vec_index_);
    assert_eq!(hash,vec_hash);
    assert_eq!(from_,vec_from_);
    assert_eq!(to_,vec_to_);
    assert_eq!(receipt_fields,vec_receipt_fields);
    assert_eq!(object_fields,vec_object_fields);

    
    let kernel_upgrade=KernelUpgrade{
        injected_before:1000,
        root_hash:"hash".to_string(),
        activation_timestamp:2000,
        applied_before:Some(1004)
    };

    let binding = replace_into(kernel_upgrades)
    .values((injected_before.eq(kernel_upgrade.injected_before),
                        root_hash.eq(kernel_upgrade.root_hash.clone()),
                        activation_timestamp.eq(kernel_upgrade.activation_timestamp)
                        ));
    let sql=debug_query::<Sqlite,_>(&binding);
    println!("SQL:{:?}",sql);

    let _ = kernel_upgrade.insert(connection).unwrap();
    let _=KernelUpgrade::nullify_after(connection, 1003);
    let latest_unapplied=KernelUpgrade::get_latest_unapplied(connection).unwrap();
    println!("Latest unapplied:{:?}",latest_unapplied);
    let _=KernelUpgrade::clear_after(connection,999);
    




}

