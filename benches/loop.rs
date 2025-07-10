use criterion::{criterion_group, criterion_main, Criterion};
use diesel::{result::Error, Connection as _, RunQueryDsl, SqliteConnection};
use evmnodetooling::dieselsqlite::{models::*, *};
use rusqlite::{params, Connection};




fn criterion_insert_blueprint(c:&mut Criterion){
    let database_url=load_database_url();
    let rusqliteconnection=rusqlite_connection().unwrap();
    let dieselconnection=&mut establish_connection().unwrap();
    let block_number=load_block_number();

    let mut id=block_number;
    let (payload,timestamp)=(vec!(0, 0, 1, 47, 0, 0, 1, 43, 0, 116, 248, 149, 46, 122, 40, 125, 120, 232, 220, 238, 198, 117, 71, 189, 0, 162, 120, 171, 191, 3, 249, 1, 18, 184, 167, 248, 165, 160, 119, 122, 26, 196, 131, 37, 205, 229, 178, 4, 242, 149, 158, 89, 152, 209, 10, 222, 46, 149, 142, 180, 216, 165, 16, 215, 140, 62, 87, 20, 166, 168, 192, 248, 120, 184, 118, 2, 248, 115, 130, 167, 41, 131, 5, 145, 24, 128, 132, 125, 43, 117, 0, 131, 9, 132, 150, 148, 219, 99, 44, 223, 246, 126, 40, 110, 101, 178, 60, 48, 144, 94, 22, 165, 112, 187, 160, 180, 135, 35, 134, 242, 111, 193, 0, 0, 128, 192, 1, 160, 150, 215, 220, 134, 5, 139, 136, 251, 193, 94, 102, 144, 248, 142, 41, 20, 182, 143, 102, 63, 123, 255, 79, 22, 130, 247, 139, 144, 34, 98, 208, 8, 160, 88, 140, 91, 202, 180, 122, 101, 30, 180, 64, 16, 180, 45, 211, 8, 203, 70, 194, 85, 87, 140, 71, 150, 246, 123, 181, 63, 129, 137, 78, 144, 103, 136, 58, 105, 89, 104, 0, 0, 0, 0, 160, 29, 189, 30, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 1, 0, 130, 0, 0, 184, 64, 55, 60, 97, 214, 146, 178, 26, 113, 28, 222, 226, 182, 81, 223, 131, 111, 0, 8, 249, 190, 17, 201, 158, 252, 177, 42, 185, 142, 34, 199, 251, 44, 41, 161, 104, 14, 187, 46, 158, 198, 163, 128, 187, 212, 203, 166, 75, 141, 224, 221, 110, 71, 22, 97, 123, 198, 22, 32, 244, 204, 97, 254, 38, 13),1750690106);



    let mut group_name=String::from("Insert blueprint");


    if database_url.as_str() == ":memory:"{
        group_name.push_str(" in memory");

        let _ = rusqliteconnection.execute(
        CREATE_TABLE_BLUEPRINTS_QUERY,
        (), );

        let _ = diesel::sql_query(CREATE_TABLE_BLUEPRINTS_QUERY).execute(dieselconnection);
    }else{
        set_journal_mode_to_wal(dieselconnection).unwrap();
        set_synchronous_mode_to_full(dieselconnection).unwrap();
        
        let _=Blueprint::clear_after(dieselconnection, id-1);
    }

    
    let mut group=c.benchmark_group(group_name);
    
    

    group.bench_function("Insert blueprint diesel", |b| b.iter(|| run_insert_blueprint_diesel(dieselconnection,&mut id,&payload,timestamp)));

    id=block_number;
    let _=Blueprint::clear_after(dieselconnection, id-1);
    

    group.bench_function("Insert blueprint rusqlite",|b| b.iter(|| run_insert_blueprint_rusqlite(&rusqliteconnection,&mut id,&payload,timestamp)));
        
    id=block_number;
    let _ = rusqliteconnection.execute(CLEAR_AFTER_BLUEPRINTS_QUERY,params![id-1]);

}

fn run_insert_blueprint_diesel(connection:&mut SqliteConnection,id:&mut i32,payload:&Vec<u8>,timestamp:i32){
    let blueprint=Blueprint{
        id:*id,payload:payload.clone(),timestamp
    };
    let _=blueprint.insert(connection);

    *id=*id+1;
}

fn run_insert_blueprint_rusqlite(connection:&Connection,id:&mut i32,payload:&Vec<u8>,timestamp:i32){
    connection.execute(INSERT_INTO_BLUEPRINTS_QUERY,(*id,payload,timestamp)).expect("Error");

    *id=*id+1;
}



fn criterion_insert_then_clear_blueprint(c:&mut Criterion){
    let database_url=load_database_url();
    let rusqliteconnection=rusqlite_connection().unwrap();
    let dieselconnection=&mut establish_connection().unwrap();
    let block_number=load_block_number();

    let id=block_number;
    let (payload,timestamp)=(vec!(0, 0, 1, 47, 0, 0, 1, 43, 0, 116, 248, 149, 46, 122, 40, 125, 120, 232, 220, 238, 198, 117, 71, 189, 0, 162, 120, 171, 191, 3, 249, 1, 18, 184, 167, 248, 165, 160, 119, 122, 26, 196, 131, 37, 205, 229, 178, 4, 242, 149, 158, 89, 152, 209, 10, 222, 46, 149, 142, 180, 216, 165, 16, 215, 140, 62, 87, 20, 166, 168, 192, 248, 120, 184, 118, 2, 248, 115, 130, 167, 41, 131, 5, 145, 24, 128, 132, 125, 43, 117, 0, 131, 9, 132, 150, 148, 219, 99, 44, 223, 246, 126, 40, 110, 101, 178, 60, 48, 144, 94, 22, 165, 112, 187, 160, 180, 135, 35, 134, 242, 111, 193, 0, 0, 128, 192, 1, 160, 150, 215, 220, 134, 5, 139, 136, 251, 193, 94, 102, 144, 248, 142, 41, 20, 182, 143, 102, 63, 123, 255, 79, 22, 130, 247, 139, 144, 34, 98, 208, 8, 160, 88, 140, 91, 202, 180, 122, 101, 30, 180, 64, 16, 180, 45, 211, 8, 203, 70, 194, 85, 87, 140, 71, 150, 246, 123, 181, 63, 129, 137, 78, 144, 103, 136, 58, 105, 89, 104, 0, 0, 0, 0, 160, 29, 189, 30, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 1, 0, 130, 0, 0, 184, 64, 55, 60, 97, 214, 146, 178, 26, 113, 28, 222, 226, 182, 81, 223, 131, 111, 0, 8, 249, 190, 17, 201, 158, 252, 177, 42, 185, 142, 34, 199, 251, 44, 41, 161, 104, 14, 187, 46, 158, 198, 163, 128, 187, 212, 203, 166, 75, 141, 224, 221, 110, 71, 22, 97, 123, 198, 22, 32, 244, 204, 97, 254, 38, 13),1750690106);



    let mut group_name=String::from("Insert then clear blueprint");


    if database_url.as_str() == ":memory:"{
        group_name.push_str(" in memory");

        let _ = rusqliteconnection.execute(
        CREATE_TABLE_BLUEPRINTS_QUERY,
        (), );

        let _ = diesel::sql_query(CREATE_TABLE_BLUEPRINTS_QUERY).execute(dieselconnection);
    }else{
        set_journal_mode_to_wal(dieselconnection).unwrap();
        set_synchronous_mode_to_full(dieselconnection).unwrap();
        
        let _=Blueprint::clear_after(dieselconnection, id-1);
    }

    
    let mut group=c.benchmark_group(group_name);
    
    

    
    group.bench_function("Insert then clear blueprint diesel", |b| b.iter(|| run_insert_then_clear_blueprint_diesel(dieselconnection,id,&payload,timestamp)));

    group.bench_function("Insert then clear blueprint rusqlite",|b| b.iter(|| run_insert_then_clear_blueprint_rusqlite(&rusqliteconnection,id,&payload,timestamp)));


}








fn run_insert_then_clear_blueprint_diesel(connection:&mut SqliteConnection,id:i32,payload:&Vec<u8>,timestamp:i32){
    let blueprint=Blueprint{
        id,payload:payload.clone(),timestamp
    };
    blueprint.insert(connection).unwrap();
    Blueprint::clear_after(connection, id-1).unwrap();
}

fn run_insert_then_clear_blueprint_rusqlite(connection:&Connection,id:i32,payload:&Vec<u8>,timestamp:i32){
    connection.execute(INSERT_INTO_BLUEPRINTS_QUERY,(id,payload,timestamp)).expect("Error");
    let _ = connection.execute(CLEAR_AFTER_BLUEPRINTS_QUERY,params![id-1]);
}

fn criterion_insert_then_clear_block(c:&mut Criterion){
    let database_url=load_database_url();
    let rusqliteconnection=rusqlite_connection().unwrap();
    let dieselconnection=&mut establish_connection().unwrap();
    let block_number=load_block_number();

    let id=block_number;

    let hash=vec!(189, 14, 79, 14, 107, 86, 150, 87, 170, 173, 153, 114, 87, 220, 255, 144, 167, 170, 145, 31, 62, 46, 232, 248, 84, 5, 80, 158, 234, 20, 136, 110);
    let block=vec!(0, 0, 0, 9, 48, 120, 49, 49, 101, 98, 100, 49, 100, 0, 0, 0, 66, 48, 120, 98, 100, 48, 101, 52, 102, 48, 101, 54, 98, 53, 54, 57, 54, 53, 55, 97, 97, 97, 100, 57, 57, 55, 50, 53, 55, 100, 99, 102, 102, 57, 48, 97, 55, 97, 97, 57, 49, 49, 102, 51, 101, 50, 101, 101, 56, 102, 56, 53, 52, 48, 53, 53, 48, 57, 101, 101, 97, 49, 52, 56, 56, 54, 101, 0, 0, 0, 66, 48, 120, 55, 55, 55, 97, 49, 97, 99, 52, 56, 51, 50, 53, 99, 100, 101, 53, 98, 50, 48, 52, 102, 50, 57, 53, 57, 101, 53, 57, 57, 56, 100, 49, 48, 97, 100, 101, 50, 101, 57, 53, 56, 101, 98, 52, 100, 56, 97, 53, 49, 48, 100, 55, 56, 99, 51, 101, 53, 55, 49, 52, 97, 54, 97, 56, 0, 0, 0, 18, 48, 120, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 0, 0, 0, 66, 48, 120, 49, 100, 99, 99, 52, 100, 101, 56, 100, 101, 99, 55, 53, 100, 55, 97, 97, 98, 56, 53, 98, 53, 54, 55, 98, 54, 99, 99, 100, 52, 49, 97, 100, 51, 49, 50, 52, 53, 49, 98, 57, 52, 56, 97, 55, 52, 49, 51, 102, 48, 97, 49, 52, 50, 102, 100, 52, 48, 100, 52, 57, 51, 52, 55, 0, 0, 2, 2, 48, 120, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 0, 0, 0, 66, 48, 120, 55, 50, 55, 50, 101, 52, 49, 52, 102, 98, 52, 100, 100, 97, 97, 97, 102, 100, 50, 53, 56, 57, 57, 102, 99, 49, 53, 55, 55, 55, 57, 54, 48, 100, 100, 51, 98, 50, 52, 50, 50, 97, 49, 50, 102, 56, 98, 52, 102, 50, 102, 52, 100, 49, 101, 97, 56, 51, 49, 53, 98, 51, 102, 101, 0, 0, 0, 66, 48, 120, 99, 98, 53, 50, 50, 49, 98, 57, 98, 54, 55, 49, 48, 50, 99, 49, 49, 57, 53, 55, 55, 98, 99, 98, 52, 57, 97, 56, 57, 49, 52, 56, 48, 55, 50, 50, 54, 100, 98, 97, 48, 99, 57, 56, 101, 97, 50, 53, 53, 55, 48, 98, 52, 56, 101, 100, 52, 56, 100, 49, 51, 49, 57, 51, 0, 0, 0, 66, 48, 120, 49, 55, 53, 48, 98, 101, 99, 52, 57, 48, 54, 52, 50, 56, 52, 48, 57, 102, 50, 56, 52, 48, 101, 56, 57, 101, 48, 49, 98, 99, 49, 57, 100, 55, 55, 57, 102, 49, 99, 98, 50, 99, 48, 102, 54, 48, 53, 49, 49, 57, 53, 102, 98, 97, 97, 100, 48, 54, 102, 101, 56, 49, 51, 101, 0, 0, 0, 42, 48, 120, 99, 102, 48, 50, 98, 57, 99, 97, 52, 56, 56, 102, 56, 102, 54, 102, 52, 101, 50, 56, 101, 51, 55, 97, 97, 49, 98, 100, 100, 49, 54, 98, 51, 102, 49, 98, 50, 97, 100, 56, 0, 0, 0, 3, 48, 120, 48, 0, 0, 0, 3, 48, 120, 48, 0, 0, 0, 2, 48, 120, 0, 0, 0, 5, 48, 120, 50, 55, 57, 0, 0, 0, 15, 48, 120, 52, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 0, 0, 0, 7, 48, 120, 57, 55, 57, 99, 56, 0, 0, 0, 10, 48, 120, 54, 56, 53, 57, 54, 57, 51, 97, 0, 0, 0, 0, 70, 0, 0, 0, 66, 48, 120, 51, 50, 53, 97, 56, 100, 57, 57, 100, 101, 55, 100, 97, 51, 49, 48, 50, 50, 49, 54, 98, 51, 97, 99, 52, 55, 102, 98, 53, 98, 49, 50, 102, 98, 56, 99, 98, 100, 49, 100, 98, 100, 49, 50, 48, 97, 48, 48, 53, 55, 100, 99, 57, 48, 55, 52, 97, 50, 99, 97, 48, 56, 55, 49, 0, 0, 0, 0, 255, 0, 0, 0, 10, 48, 120, 51, 98, 57, 97, 99, 97, 48, 48, 255, 0, 0, 0, 66, 48, 120, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 255, 0, 0, 0, 0, 255, 0, 0, 0, 66, 48, 120, 53, 54, 101, 56, 49, 102, 49, 55, 49, 98, 99, 99, 53, 53, 97, 54, 102, 102, 56, 51, 52, 53, 101, 54, 57, 50, 99, 48, 102, 56, 54, 101, 53, 98, 52, 56, 101, 48, 49, 98, 57, 57, 54, 99, 97, 100, 99, 48, 48, 49, 54, 50, 50, 102, 98, 53, 101, 51, 54, 51, 98, 52, 50, 49, 255, 0, 0, 0, 3, 48, 120, 48, 255, 0, 0, 0, 3, 48, 120, 48, 255, 0, 0, 0, 66, 48, 120, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48);


    let mut group_name=String::from("Insert then clear block");


    if database_url.as_str() == ":memory:"{
        group_name.push_str(" in memory");

        let _ = rusqliteconnection.execute(
        CREATE_TABLE_BLOCKS_QUERY,
        (), );

        let _ = diesel::sql_query(CREATE_TABLE_BLOCKS_QUERY).execute(dieselconnection);
    }else{
        set_journal_mode_to_wal(dieselconnection).unwrap();
        set_synchronous_mode_to_full(dieselconnection).unwrap();
        
        let _=Block::clear_after(dieselconnection, id-1);
    }

    
    let mut group=c.benchmark_group(group_name);
    
    

    
    group.bench_function("Insert then clear blueprint diesel", |b| b.iter(|| run_insert_then_clear_block_diesel(dieselconnection,id,&hash,&block)));

    group.bench_function("Insert then clear blueprint rusqlite",|b| b.iter(|| run_insert_then_clear_block_rusqlite(&rusqliteconnection,id,&hash,&block)));


}

fn run_insert_then_clear_block_diesel(connection:&mut SqliteConnection,level:i32,hash:&Vec<u8>,block:&Vec<u8>){
    let block=Block{
        level,hash:hash.clone(),block:block.clone()
    };
    block.insert(connection).unwrap();
    Block::clear_after(connection, level-1).unwrap();
}

fn run_insert_then_clear_block_rusqlite(connection:&Connection,level:i32,hash:&Vec<u8>,block:&Vec<u8>){
    connection.execute(INSERT_INTO_BLOCKS_QUERY,(level,hash,block)).expect("Error");
    let _ = connection.execute(CLEAR_AFTER_BLOCKS_QUERY,params![level-1]);
}



fn criterion_select_then_insert_blueprint(c:&mut Criterion){
    let database_url=load_database_url();
    let block_number=load_block_number();
    if database_url.as_str() != ":memory:"{
        let connection=&mut establish_connection().unwrap();
        let mut id=block_number;
        let (payload,timestamp)=Blueprint::select(connection, id).unwrap();
        let _=Blueprint::clear_after(connection, block_number-1);
        


        c.bench_function("select then insert", |b| b.iter(|| run_insert_blueprint_diesel(connection,&mut id,&payload,timestamp)));

        let _=Blueprint::clear_after(connection, block_number);
    }
}



fn criterion_block_select_with_level(c:&mut Criterion){
    let database_url=load_database_url();
    let block_number=load_block_number();
    if database_url.as_str() != ":memory:"{
        let connection=&mut establish_connection().unwrap();
        let id=block_number;

        c.bench_function("block_select_with_level",  |b| b.iter(|| run_select_block_with_level(connection,id)));
    }

}

fn run_select_block_with_level(connection:&mut SqliteConnection,level:i32){
    let _=Block::select_with_level(connection, level);
}

fn criterion_apply_blueprint(c:&mut Criterion){
    let block_number=load_block_number();
    let database_url=load_database_url();
    if database_url.as_str()!=":memory:"{
        
        let mut connection=establish_connection().unwrap();
        // let select_index=Blueprint::base_level(&mut connection).unwrap();//Change to custom number to benchmark against
        let select_index=block_number;
        let clear_index=Blueprint::top_level(&mut connection).unwrap();
        

        let setup=||{
            let mut connection=establish_connection().unwrap();
            
            let insert_index= Blueprint::top_level(&mut connection).unwrap()+1;
            // println!("Insert_index:{}",insert_index);
            let (payload,timestamp)=Blueprint::select(&mut connection, select_index).unwrap();
            let blueprint=Blueprint{
                id:insert_index,payload,timestamp
            };
            let block=Block::select_with_level(&mut connection, select_index).unwrap();
            let mut bytes=[0u8;32];
            rand::fill(&mut bytes);
            let block=Block{
                level:insert_index,hash:Vec::from(bytes),block
            };
            let transactions_receipts=Transaction::select_receipts_from_block_number(&mut connection, select_index).unwrap();
            let transaction_objects=Transaction::select_objects_from_block_number(&mut connection, select_index).unwrap();


            let transactions=transactions_receipts.into_iter()
            .zip(transaction_objects.into_iter())
            .map(|((block_hash,index_,_,from_,to_,receipt_fields),
            (_,_,_,_,object_fields))|
                Transaction{
                    block_hash,
                    block_number:insert_index,
                    index_,
                    hash:{rand::fill(&mut bytes);
                        Vec::from(bytes)},
                    from_,
                    to_,
                    receipt_fields,
                    object_fields,
                }
            ).collect::<Vec<Transaction>>();
            let context_hash=ContextHash{
                id:insert_index,
                context_hash:ContextHash::select(&mut connection, select_index).unwrap()
            };
            (connection,blueprint,block,transactions,context_hash)

        };

        let routine=|(mut connection,blueprint,block,transactions,context_hash)| 
                run_apply_blueprint(&mut connection, blueprint, block, transactions, context_hash);


        
        

        
        c.bench_function(&format!("apply_blueprint {}",block_number),  move |b| 
            b.iter_batched(setup,routine,
            criterion::BatchSize::PerIteration));
        
        let _ = Blueprint::clear_after(&mut connection, clear_index);
        let _ = Block::clear_after(&mut connection, clear_index);
        let _ = ContextHash::clear_after(&mut connection, clear_index);
        let _ = Transaction::clear_after(&mut connection, clear_index);
    }


}




fn run_apply_blueprint(connection:&mut SqliteConnection,blueprint:Blueprint,block:Block,transactions:Vec<Transaction>,context_hash:ContextHash){
    
    connection.transaction::<_,Error,_>(|conn|{
        let _=PendingConfirmation::select_with_level(conn, blueprint.id);
        blueprint.insert(conn)?;
        block.insert(conn)?;
        for tx in transactions{
            tx.insert(conn)?;
        }
        context_hash.insert(conn)?;
        let _history_mode=Metadata::get_history_mode(conn)?;
        Ok(())
    }).unwrap();
}




criterion_group!(benches, 
    criterion_insert_blueprint,
    criterion_insert_then_clear_blueprint,
    criterion_insert_then_clear_block,
    criterion_apply_blueprint,
    criterion_block_select_with_level,
    criterion_select_then_insert_blueprint,
);
criterion_main!(benches);