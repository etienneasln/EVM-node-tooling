use criterion::{black_box, criterion_group, criterion_main, Criterion};
use diesel::SqliteConnection;
use evmnodetooling::dieselsqlite::{establish_connection, models::{Block, Blueprint}, BASE_LEVEL};

fn run_select_block_with_level(connection:&mut SqliteConnection,level:i32){
    let _=Block::select_with_level(connection, level);
}

fn run_insert_blueprint(connection:&mut SqliteConnection,id:&mut i32,payload:&Vec<u8>,timestamp:i32){
    let _=Blueprint::insert(connection, *id, payload, timestamp);

    *id=*id+1;
}

fn run_change_to_mut_ref(connection:&mut SqliteConnection,id:&mut i32){
    *id=*id+1;
}

fn criterion_block_select_with_level_19100196(c:&mut Criterion){
    let mut connection=establish_connection();
    let id=19100196;
    
    c.bench_function("block_select_with_level_19100196", |b| b.iter(|| black_box(run_select_block_with_level(&mut connection,id))));
}

fn criterion_apply_blueprint_18987875(c:&mut Criterion){
    let mut connection=establish_connection();
    let mut insertindex=18987875;
    let (payload,timestamp)=Blueprint::select(&mut connection, insertindex);

    let _=Blueprint::clear_after(&mut connection, insertindex-1);


    c.bench_function("apply_blueprint_18987875", |b| b.iter(|| run_insert_blueprint(&mut connection,&mut insertindex,&payload,timestamp)));

    let _=Blueprint::clear_after(&mut connection, insertindex);
}

fn criterion_apply_blueprint_18989033(c:&mut Criterion){
    let mut connection=establish_connection();
    let mut insertindex=18989033;
    let (payload,timestamp)=Blueprint::select(&mut connection, insertindex);

    let _=Blueprint::clear_after(&mut connection, insertindex-1);


    c.bench_function("apply_blueprint_18989033", |b| b.iter(|| run_insert_blueprint(&mut connection,&mut insertindex,&payload,timestamp)));

    let _=Blueprint::clear_after(&mut connection, insertindex);
}

fn criterion_apply_blueprint_18989013(c:&mut Criterion){
    let mut connection=establish_connection();
    let mut insertindex=18989013;
    let (payload,timestamp)=Blueprint::select(&mut connection, insertindex);

    let _=Blueprint::clear_after(&mut connection, insertindex-1);


    c.bench_function("apply_blueprint_18989013", |b| b.iter(|| run_insert_blueprint(&mut connection,&mut insertindex,&payload,timestamp)));

    let _=Blueprint::clear_after(&mut connection, insertindex);
}

fn criterion_test_change_to_mutable_reference(c:&mut Criterion){
    let mut connection=establish_connection();
    let mut insertindex=18987875;
    

    c.bench_function("incrementing mutable reference", |b| b.iter(|| run_change_to_mut_ref(&mut connection,&mut insertindex)));
}


criterion_group!(benches, 
    criterion_block_select_with_level_19100196,
    // criterion_apply_blueprint_18989033,
    // criterion_apply_blueprint_18989013,
    // criterion_apply_blueprint_18987875,
    // criterion_test_change_to_mutable_reference,
);
criterion_main!(benches);