use std::time::Instant;

use evmnodetooling::dieselsqlite::{establish_connection, models::Block};
fn main(){
    let connection=&mut establish_connection(None).unwrap();
    let id=19100196;
    let start=Instant::now();

    let _=Block::select_with_level(connection, id);
    let duration=start.elapsed();

    println!("Duration of block_select_with_level:{:?}",duration);
    
}