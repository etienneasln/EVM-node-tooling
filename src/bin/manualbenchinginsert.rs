use std::time::Instant;

use evmnodetooling::dieselsqlite::{establish_connection, models::{Block, Blueprint}};
fn main(){
    let mut connection=establish_connection();
    let id=18987875;
    let (payload,timestamp)=Blueprint::select(&mut connection, id);
    let hash=Block::select_hash_of_number(&mut connection, id);
    let block=Block::select_with_level(&mut connection, id);
    let _=Blueprint::clear_after(&mut connection, id-1);
    let blueprint=Blueprint{
        id,payload,timestamp
    };
    let block=Block{
        level:id,hash,block
    };



    let _=Blueprint::clear_after(&mut connection, id);
    let start=Instant::now();
    let _=blueprint.insert(&mut connection);
    let _=block.insert(&mut connection);
    let duration=start.elapsed();

    println!("Duration of block_select_with_level:{:?}",duration);
    
}