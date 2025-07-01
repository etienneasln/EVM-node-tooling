use evmnodetooling::dieselsqlite::{establish_connection, models::{Block, Blueprint}};

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

    assert!(block==blockfromhash);
    assert!(idfromhash==base_level);

    let block=Block{
        level:top_level+1,hash:"Random hash".as_bytes().to_vec(),block:block
    };

    let _=block.insert(&mut connection);

    let block = Block::select_with_level(&mut connection, top_level+1);
    println!("Block:{:?}",block);

    
    let _=Block::clear_after(&mut connection, top_level);

    println!("Block count:{}", Block::count(&mut connection));
    println!("Blueprint count:{}",Blueprint::count(&mut connection));

    
}

