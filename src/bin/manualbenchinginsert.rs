use std::time::Instant;

use diesel::RunQueryDsl;
use evmnodetooling::dieselsqlite::{establish_connection, load_database_url, models::Blueprint, rusqlite_connection, set_journal_mode_to_wal, set_synchronous_mode_to_full, CLEAR_AFTER_BLUEPRINTS_QUERY, CREATE_TABLE_BLUEPRINTS_QUERY, INSERT_INTO_BLUEPRINTS_QUERY};
use rusqlite::params;

fn main(){
    let database_url=load_database_url();
    let dieselconnection=&mut establish_connection().unwrap();
    let rusqliteconnection=rusqlite_connection().unwrap();

    let id=18987875;
    
    let (payload,timestamp)=(vec!(0, 0, 1, 47, 0, 0, 1, 43, 0, 116, 248, 149, 46, 122, 40, 125, 120, 232, 220, 238, 198, 117, 71, 189, 0, 162, 120, 171, 191, 3, 249, 1, 18, 184, 167, 248, 165, 160, 119, 122, 26, 196, 131, 37, 205, 229, 178, 4, 242, 149, 158, 89, 152, 209, 10, 222, 46, 149, 142, 180, 216, 165, 16, 215, 140, 62, 87, 20, 166, 168, 192, 248, 120, 184, 118, 2, 248, 115, 130, 167, 41, 131, 5, 145, 24, 128, 132, 125, 43, 117, 0, 131, 9, 132, 150, 148, 219, 99, 44, 223, 246, 126, 40, 110, 101, 178, 60, 48, 144, 94, 22, 165, 112, 187, 160, 180, 135, 35, 134, 242, 111, 193, 0, 0, 128, 192, 1, 160, 150, 215, 220, 134, 5, 139, 136, 251, 193, 94, 102, 144, 248, 142, 41, 20, 182, 143, 102, 63, 123, 255, 79, 22, 130, 247, 139, 144, 34, 98, 208, 8, 160, 88, 140, 91, 202, 180, 122, 101, 30, 180, 64, 16, 180, 45, 211, 8, 203, 70, 194, 85, 87, 140, 71, 150, 246, 123, 181, 63, 129, 137, 78, 144, 103, 136, 58, 105, 89, 104, 0, 0, 0, 0, 160, 29, 189, 30, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 1, 0, 130, 0, 0, 184, 64, 55, 60, 97, 214, 146, 178, 26, 113, 28, 222, 226, 182, 81, 223, 131, 111, 0, 8, 249, 190, 17, 201, 158, 252, 177, 42, 185, 142, 34, 199, 251, 44, 41, 161, 104, 14, 187, 46, 158, 198, 163, 128, 187, 212, 203, 166, 75, 141, 224, 221, 110, 71, 22, 97, 123, 198, 22, 32, 244, 204, 97, 254, 38, 13),1750690106);
    
    if database_url.as_str() == ":memory:"{

        let _ = rusqliteconnection.execute(
        CREATE_TABLE_BLUEPRINTS_QUERY,
        (), );

        let _ = diesel::sql_query(CREATE_TABLE_BLUEPRINTS_QUERY).execute(dieselconnection);
    }else{
        set_journal_mode_to_wal(dieselconnection).unwrap();
        set_synchronous_mode_to_full(dieselconnection).unwrap();
        
        let _=Blueprint::clear_after(dieselconnection, id-1);
    }

    
    let blueprint=Blueprint{
        id,payload:payload.clone(),timestamp
    };
    

    
    let start=Instant::now();
    let _=blueprint.insert(dieselconnection);
    let duration=start.elapsed();

    let _=Blueprint::clear_after(dieselconnection, id-1);


    let start2=Instant::now();
    rusqliteconnection.execute(INSERT_INTO_BLUEPRINTS_QUERY,(id,payload,timestamp)).expect("Error");
    let duration2=start2.elapsed();

    let _ = rusqliteconnection.execute(CLEAR_AFTER_BLUEPRINTS_QUERY,params![id-1]);


    println!("Duration of inserting for apply_blueprint diesel:{:?}",duration);
    println!("Duration of inserting for apply_blueprint rusqlite:{:?}",duration2);

    
}