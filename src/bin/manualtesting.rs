use evmnodetooling::dieselsqlite::{establish_connection, models::{Block, Blueprint}, BASE_LEVEL, TOP_LEVEL};

fn main(){
    let mut connection=establish_connection();
    
    let (payload,timestamp)=Blueprint::select(&mut connection, BASE_LEVEL);
    
    println!("Payload:{:?}, Timestamp:{timestamp}", payload);
    
    let _=Blueprint::insert(&mut connection,TOP_LEVEL+1,&payload,timestamp);
    
    
    let (payload,timestamp)=Blueprint::select(&mut connection, TOP_LEVEL+1);
    
    println!("Payload:{:?}, Timestamp:{timestamp}",payload);
    
    let _=Blueprint::clear_after(&mut connection, TOP_LEVEL);
    
    let tuplevec=Blueprint::select_range(&mut connection, TOP_LEVEL-2, TOP_LEVEL);
    
    for (id,payload) in tuplevec{
        println!("Id:{id}, payload:{:?}",payload);
        println!("------------------------------");
    }


    let block = Block::select_with_level(&mut connection, BASE_LEVEL);
    println!("Block:{:?}",block);

    let hash=Block::select_hash_of_number(&mut connection, BASE_LEVEL);
    println!("Block hash:{:?}",hash);

    let blockfromhash=Block::select_with_hash(&mut connection, &hash);
    println!("Block from hash:{:?}",blockfromhash);

    let idfromhash=Block::select_number_of_hash(&mut connection, &hash);
    println!("Block from hash:{:?}",blockfromhash);

    assert!(block==blockfromhash);
    assert!(idfromhash==BASE_LEVEL);

    let _=Block::insert(&mut connection,TOP_LEVEL+1,&"Random hash".as_bytes().to_vec(),&block);

    let block = Block::select_with_level(&mut connection, TOP_LEVEL+1);
    println!("Block:{:?}",block);

    
    let _=Block::clear_after(&mut connection, TOP_LEVEL);

    
}

