use evmnodetooling::dieselsqlite::{establish_connection, models::*};

fn main(){
    let mut connection=establish_connection();

    let base_level=Block::base_level(&mut connection);
    let top_level=Block::top_level(&mut connection);
    
    let (payload,timestamp)=Blueprint::select(&mut connection, base_level);
    
    println!("Payload:{:?}, Timestamp:{timestamp}", payload);

    let blueprint=Blueprint{
        id:top_level+1,payload,timestamp
    };
    
    let _=blueprint.insert(&mut connection);
    
    
    let (payload,timestamp)=Blueprint::select(&mut connection, top_level+1);
    
    println!("Payload:{:?}, Timestamp:{timestamp}",payload);
    
    let _=Blueprint::clear_after(&mut connection, top_level);
    
    let tuplevec=Blueprint::select_range(&mut connection, top_level-2, top_level);
    
    for (id,payload) in tuplevec{
        println!("Id:{id}, payload:{:?}",payload);
        println!("------------------------------");
    }


    let block = Block::select_with_level(&mut connection, base_level);
    println!("Block:{:?}",block);

    let hash=Block::select_hash_of_number(&mut connection, base_level);
    println!("Block hash:{:?}",hash);

    let blockfromhash=Block::select_with_hash(&mut connection, &hash);
    println!("Block from hash:{:?}",blockfromhash);

    let idfromhash=Block::select_number_of_hash(&mut connection, &hash);
    println!("Block from hash:{:?}",blockfromhash);

    assert_eq!(block,blockfromhash);
    assert_eq!(idfromhash,base_level);

    let block=Block{
        level:top_level+1,hash:"Random hash".as_bytes().to_vec(),block:block
    };

    let _=block.insert(&mut connection);

    let block = Block::select_with_level(&mut connection, top_level+1);
    println!("Block:{:?}",block);

    
    let _=Block::clear_after(&mut connection, top_level);

    println!("Block count:{}", Block::count(&mut connection));
    println!("Blueprint count:{}",Blueprint::count(&mut connection));

    let receipts=Transaction::select_receipts_from_block_number(&mut connection, top_level);

    println!("Transaction receipts top level block:{:?}",receipts);

    let objects=Transaction::select_objects_from_block_number(&mut connection, top_level);

    println!("Transaction objects top level block:{:?}",objects);

    let (vec_block_hash,vec_index_,vec_hash,vec_from_,vec_to_,vec_receipt_fields)=(&receipts[0]).clone();

    let (block_hash,block_number,index_,hash,from_,to_,receipt_fields)=Transaction::select_receipt(&mut connection, &vec_hash);

    assert_eq!(block_hash,vec_block_hash);
    assert_eq!(block_number,top_level);
    assert_eq!(index_, vec_index_);
    assert_eq!(hash,vec_hash);
    assert_eq!(from_,vec_from_);
    assert_eq!(to_,vec_to_);
    assert_eq!(receipt_fields,vec_receipt_fields);




}

