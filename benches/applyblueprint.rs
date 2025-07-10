use criterion::{criterion_group, criterion_main, Criterion};
use diesel::{result::Error, Connection, SqliteConnection};
use evmnodetooling::dieselsqlite::{models::*, *};




fn criterion_insert_blueprint(c:&mut Criterion){

    let connection=&mut establish_connection().unwrap();
    let block_number=load_block_number();

    let select_id=block_number;
    let clear_id=Blueprint::top_level(connection).unwrap();
    let mut insert_id=clear_id+1;

    let (payload,timestamp)=Blueprint::select(connection, select_id).unwrap();
  
    
    connection.transaction::<_,Error,_>(|conn|{
        c.bench_function("step Insert blueprint", |b| b.iter(|| run_insert_blueprint(conn, &mut insert_id, &payload, timestamp)));
        let _=Blueprint::clear_after(conn, clear_id);
        Ok(())
    }).unwrap(); 


}


fn run_insert_blueprint(connection:&mut SqliteConnection,id:&mut i32,payload:&Vec<u8>,timestamp:i32){
    let blueprint=Blueprint{
        id:*id,payload:payload.clone(),timestamp
    };
    let _=blueprint.insert(connection);

    *id=*id+1;
}




fn criterion_insert_block(c:&mut Criterion){
    let connection=&mut establish_connection().unwrap();
    let block_number=load_block_number();

    let select_id=block_number;
    let clear_id=Block::top_level(connection).unwrap();
    let mut insert_id=clear_id+1;


    let block= Block::select_with_level(connection, select_id).unwrap();
    let bytes=[0u8;32];

  
    
    connection.transaction::<_,Error,_>(|conn|{
        c.bench_function("step Insert block", |b| b.iter(|| run_insert_block(conn, &mut insert_id,bytes,&block)));
        let _=Block::clear_after(conn, clear_id);
        Ok(())
    }).unwrap(); 



}

fn run_insert_block(connection:&mut SqliteConnection,level:&mut i32,bytes:[u8;32],block:&Vec<u8>){
    let block=Block{
        level:*level,
        hash:rand_32_bytes_vec(bytes),
        block:block.clone()
    };
    block.insert(connection).unwrap();
    *level+=1;

}

fn criterion_insert_transactions(c:&mut Criterion){
    let connection=&mut establish_connection().unwrap();
    let block_number=load_block_number();

    let select_id=block_number;
    let clear_id=Block::top_level(connection).unwrap();
    let mut insert_id=clear_id+1;
    let bytes=[0u8;32];

    connection.transaction::<_,Error,_>(|conn|{
        c.bench_function("step Insert transactions", |b| b.iter(|| run_insert_transaction(conn, select_id,&mut insert_id,bytes)));
        let _=Block::clear_after(conn, clear_id);
        Ok(())
    }).unwrap();
}

fn run_insert_transaction(connection:&mut SqliteConnection,select_id:i32,insert_id:&mut i32,bytes:[u8;32]){
    let transactions=transactions(connection, select_id, *insert_id, bytes);

    for tx in transactions{
        let _ = tx.insert(connection);
    }
    *insert_id+=1;
}

fn criterion_insert_context_hash(c:&mut Criterion){
    let connection=&mut establish_connection().unwrap();
    let block_number=load_block_number();

    let select_id=block_number;
    let clear_id=Block::top_level(connection).unwrap();
    let mut insert_id=clear_id+1;


    let context_hash= ContextHash::select(connection, select_id).unwrap();

  
    
    connection.transaction::<_,Error,_>(|conn|{
        c.bench_function("step Insert Context Hash", |b| b.iter(|| run_insert_context_hash(conn, &mut insert_id,&context_hash)));
        let _=ContextHash::clear_after(conn, clear_id);
        Ok(())
    }).unwrap(); 
}

fn run_insert_context_hash(connection:&mut SqliteConnection,insert_id:&mut i32,context_hash:&Vec<u8>){
    let context_hash=ContextHash{
        id:*insert_id,
        context_hash:context_hash.clone()
    };
    context_hash.insert(connection).unwrap();
    *insert_id+=1;

}

fn criterion_select_history_mode(c:&mut Criterion){
    let connection=&mut establish_connection().unwrap();

  
    
    connection.transaction::<_,Error,_>(|conn|{
        c.bench_function("step Select history mode", |b| b.iter(|| run_select_history_mode(conn)));
        Ok(())
    }).unwrap(); 
}

fn run_select_history_mode(connection:&mut SqliteConnection){
    let _=Metadata::get_history_mode(connection);
}


fn criterion_apply_blueprint(c:&mut Criterion){
    let block_number=load_block_number();
    
        
    let mut connection=establish_connection().unwrap();
    let select_id=block_number;
    let clear_id=Blueprint::top_level(&mut connection).unwrap();
    let insert_id= clear_id+1;
    

    let setup=||{
        let mut connection=establish_connection().unwrap();
        
        
        let (payload,timestamp)=Blueprint::select(&mut connection, select_id).unwrap();
        let blueprint=Blueprint{
            id:insert_id,payload,timestamp
        };
        let block=Block::select_with_level(&mut connection, select_id).unwrap();
        let bytes=[0u8;32];
        
        let block=Block{
            level:insert_id,hash:rand_32_bytes_vec(bytes),block
        };
        let transactions=transactions(&mut connection, select_id, insert_id, bytes);
        let context_hash=ContextHash{
            id:insert_id,
            context_hash:ContextHash::select(&mut connection, select_id).unwrap()
        };
        clear_after_apply_blueprint(&mut connection, clear_id);
        (connection,blueprint,block,transactions,context_hash)

    };

    let routine=|(mut connection,blueprint,block,transactions,context_hash)| 
            run_apply_blueprint(&mut connection, blueprint, block, transactions, context_hash);


    
    

    
    c.bench_function("Apply blueprint",  |b| 
        b.iter_batched(setup,routine,
        criterion::BatchSize::PerIteration));
        
    
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
    criterion_insert_block,
    criterion_insert_transactions,
    criterion_insert_context_hash,
    criterion_select_history_mode,
    criterion_apply_blueprint,
);
criterion_main!(benches);

//Helper functions

fn transactions(connection:&mut SqliteConnection,select_id:i32,insert_id:i32,bytes:[u8;32])->Vec<Transaction>{
    let transactions_receipts=Transaction::select_receipts_from_block_number(connection,select_id).unwrap();
    let transaction_objects=Transaction::select_objects_from_block_number(connection, select_id).unwrap();


    transactions_receipts.into_iter()
    .zip(transaction_objects.into_iter())
    .map(|((block_hash,index_,_,from_,to_,receipt_fields),
    (_,_,_,_,object_fields))|
        Transaction{
            block_hash,
            block_number:insert_id,
            index_,
            hash:rand_32_bytes_vec(bytes),
            from_,
            to_,
            receipt_fields,
            object_fields,
        }
    ).collect::<Vec<Transaction>>()
}

fn clear_after_apply_blueprint(connection:&mut SqliteConnection,clear_id:i32){
    let _ = Blueprint::clear_after(connection, clear_id);
    let _ = Block::clear_after(connection, clear_id);
    let _ = ContextHash::clear_after(connection, clear_id);
    let _ = Transaction::clear_after(connection, clear_id);
}

fn rand_32_bytes_vec(mut bytes:[u8;32])->Vec<u8>{
    rand::fill(&mut bytes);
    Vec::from(bytes)
}


