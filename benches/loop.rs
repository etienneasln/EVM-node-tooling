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
        c.bench_function("Insert blueprint", |b| b.iter(|| run_insert_blueprint(conn, &mut insert_id, &payload, timestamp)));
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
        c.bench_function("Insert block", |b| b.iter(|| run_insert_block(conn, &mut insert_id,bytes,&block)));
        let _=Block::clear_after(conn, clear_id);
        Ok(())
    }).unwrap(); 



}

fn run_insert_block(connection:&mut SqliteConnection,level:&mut i32,mut bytes:[u8;32],block:&Vec<u8>){
    let block=Block{
        level:*level,
        hash:{rand::fill(&mut bytes);
            Vec::from(bytes)},
        block:block.clone()
    };
    block.insert(connection).unwrap();
    *level=*level+1;

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
        let mut bytes=[0u8;32];
        rand::fill(&mut bytes);
        let block=Block{
            level:insert_id,hash:Vec::from(bytes),block
        };
        let transactions_receipts=Transaction::select_receipts_from_block_number(&mut connection, select_id).unwrap();
        let transaction_objects=Transaction::select_objects_from_block_number(&mut connection, select_id).unwrap();


        let transactions=transactions_receipts.into_iter()
        .zip(transaction_objects.into_iter())
        .map(|((block_hash,index_,_,from_,to_,receipt_fields),
        (_,_,_,_,object_fields))|
            Transaction{
                block_hash,
                block_number:insert_id,
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
            id:insert_id,
            context_hash:ContextHash::select(&mut connection, select_id).unwrap()
        };
        let _ = Blueprint::clear_after(&mut connection, clear_id);
        let _ = Block::clear_after(&mut connection, clear_id);
        let _ = ContextHash::clear_after(&mut connection, clear_id);
        let _ = Transaction::clear_after(&mut connection, clear_id);
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
    criterion_apply_blueprint,
);
criterion_main!(benches);