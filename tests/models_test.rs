use diesel::{result::Error, Connection};
use evmnodetooling::dieselsqlite::{establish_connection,models::*};

#[test]
fn test_blueprint_insert_select_clearafter(){
    let connection=&mut establish_connection().unwrap();


    connection.test_transaction::<_,Error,_>(|conn| {
        let inserted_payload="payload".as_bytes().to_vec();
        let inserted_timestamp=1000;
        let base_insert_index=Blueprint::top_level(conn)?;

        let blueprint=Blueprint{
            id:base_insert_index+1,
            payload:inserted_payload.clone(),
            timestamp:inserted_timestamp
        };

        let _=blueprint.insert(conn);

        let (payload,timestamp)=Blueprint::select(conn, base_insert_index+1)?;

        assert_eq!(payload,inserted_payload);
        assert_eq!(timestamp,inserted_timestamp);

        let expected_rows_cleared:usize=1;

        let rows_cleared=Blueprint::clear_after(conn, base_insert_index)?;

        assert_eq!(rows_cleared,expected_rows_cleared);
        Ok(())
    })

    
}

#[test]
fn test_blueprint_insert_selectrange_clearafter(){
    let connection=&mut establish_connection().unwrap();

    connection.test_transaction::<_,Error,_>(|conn|{
        let inserted_payloads=vec!["payload1".as_bytes().to_vec(),"payload2".as_bytes().to_vec(),"payload3".as_bytes().to_vec()];
        let inserted_timestamps=vec![1000,1001,1002];
        let base_insert_index=Blueprint::top_level(conn)?;

        let blueprint1=Blueprint{
            id:base_insert_index+1,
            payload:inserted_payloads[0].clone(),
            timestamp:inserted_timestamps[0]
        };
        let blueprint2=Blueprint{
            id:base_insert_index+2,
            payload:inserted_payloads[1].clone(),
            timestamp:inserted_timestamps[1]
        };
        let blueprint3=Blueprint{
            id:base_insert_index+3,
            payload:inserted_payloads[2].clone(),
            timestamp:inserted_timestamps[2]
        };
        let _=blueprint1.insert(conn)?;
        let _=blueprint2.insert(conn)?;
        let _=blueprint3.insert(conn)?;


        let expected_vector=vec![base_insert_index+1,base_insert_index+2,base_insert_index+3]
        .into_iter()
        .zip(inserted_payloads)
        .collect::<Vec<(i32,Vec<u8>)>>();
        let vector=Blueprint::select_range(conn, base_insert_index+1,base_insert_index+3)?;



        assert_eq!(vector,expected_vector);

        let expected_rows_cleared:usize=3;
        
        let rows_cleared=Blueprint::clear_after(conn, base_insert_index)?;

        assert_eq!(rows_cleared,expected_rows_cleared);

        Ok(())
    })
    
    
}

#[test]
fn test_block_insert_selects_clearafter(){
    let connection=&mut establish_connection().unwrap();

    connection.test_transaction::<_,Error,_>(|conn|{
        let inserted_hash="hash".as_bytes().to_vec();
        let inserted_block="block".as_bytes().to_vec();
        let base_insert_index=Block::top_level(conn)?;

        let block=Block{
            level:base_insert_index+1,
            hash:inserted_hash.clone(),
            block:inserted_block.clone()
        };


        let _=block.insert(conn)?;
        
        let block_from_level=Block
    ::select_with_level(conn, base_insert_index+1)?;
        let hash_of_number=Block
    ::select_hash_of_number(conn, base_insert_index+1)?;
        let number_of_hash=Block
    ::select_number_of_hash(conn, &hash_of_number)?;
        let block_from_hash=Block
    ::select_with_hash(conn, &hash_of_number)?;
        
        assert_eq!(block_from_level,inserted_block);
        assert_eq!(hash_of_number,inserted_hash);
        assert_eq!(number_of_hash,base_insert_index+1);
        assert_eq!(block_from_hash,inserted_block);


        let expected_rows_cleared:usize=1;
        
        let rows_cleared=Block
    ::clear_after(conn, base_insert_index)?;

        assert_eq!(rows_cleared,expected_rows_cleared);
        Ok(())
    })

    
    
    
}

#[test]
fn test_block_selects(){
    let connection=&mut establish_connection().unwrap();

    connection.test_transaction::<_,Error,_>(|conn|{
        let select_index=Block::top_level(conn)?;

        let block_from_level=Block
    ::select_with_level(conn, select_index)?;
        let hash_of_number=Block
    ::select_hash_of_number(conn, select_index)?;
        let number_of_hash=Block
    ::select_number_of_hash(conn, &hash_of_number)?;
        let block_from_hash=Block
    ::select_with_hash(conn, &hash_of_number)?;

        assert_eq!(block_from_hash,block_from_level);
        assert_eq!(number_of_hash,select_index);
        Ok(())
    })

    
}

#[test]
fn test_transaction_select_insert_clear(){
    let connection=&mut establish_connection().unwrap();

    connection.test_transaction::<_,Error,_>(|conn|{
        let inserted_block_hash:Vec<u8>="block_hash".as_bytes().to_vec();
        let inserted_block_number=Block::top_level(conn)?+1;
        let inserted_index_=0;
        let inserted_hash:Vec<u8>="transactionHash".as_bytes().to_vec();
        let inserted_from_="from_".as_bytes().to_vec();
        let inserted_to_=Some("to_".as_bytes().to_vec());
        let inserted_receipt_fields="receipt_fields".as_bytes().to_vec();
        let inserted_object_fields="object_fields".as_bytes().to_vec();

        let transaction=Transaction{
            block_hash:inserted_block_hash.clone(),
            block_number: inserted_block_number,
            index_: inserted_index_,
            hash:inserted_hash.clone(),
            from_:inserted_from_.clone(),
            to_:inserted_to_.clone(),
            receipt_fields:inserted_receipt_fields.clone(),
            object_fields:inserted_object_fields.clone()
        };

        let _=transaction.insert(conn);

        let (block_hash,block_number, index_, 
            hash, from_, to_, receipt_fields)
            =Transaction::select_receipt(conn, &inserted_hash)?;
        
        assert_eq!(block_hash,inserted_block_hash);
        assert_eq!(block_number,inserted_block_number);
        assert_eq!(index_,inserted_index_);
        assert_eq!(hash,inserted_hash);
        assert_eq!(from_,inserted_from_);
        assert_eq!(to_,inserted_to_);
        assert_eq!(receipt_fields, inserted_receipt_fields);

        let expected_rows_cleared=1;

        let rows_cleared=Transaction::clear_after(conn,inserted_block_number-1)?;

        assert_eq!(rows_cleared,expected_rows_cleared);
        Ok(())
    })

    

    
}

#[test]
fn test_transaction_selects(){
    let connection=&mut establish_connection().unwrap();

    connection.test_transaction::<_,Error,_>(|conn|{
        let select_block_level=Block::top_level(conn)?;
        
        let receipts=Transaction::select_receipts_from_block_number(conn, select_block_level)?;


        let objects=Transaction::select_objects_from_block_number(conn, select_block_level)?;

        let length=receipts.len();
        for i in 0..length{
            let (vec_block_hash,vec_index_,vec_hash,vec_from_,vec_to_,vec_receipt_fields)=(&receipts[i]).clone();
            let (_,_,_,_,vec_object_fields)=(&objects[i]).clone();

            let (block_hash,block_number,index_,hash,from_,to_,receipt_fields)=Transaction::select_receipt(conn, &vec_hash)?;
            let (_,_,_,_,_,_,object_fields)=Transaction::select_object(conn, &vec_hash)?;


            assert_eq!(block_hash,vec_block_hash);
            assert_eq!(block_number,select_block_level);
            assert_eq!(index_, vec_index_);
            assert_eq!(hash,vec_hash);
            assert_eq!(from_,vec_from_);
            assert_eq!(to_,vec_to_);
            assert_eq!(receipt_fields,vec_receipt_fields);
            assert_eq!(object_fields,vec_object_fields);
        }
        
        Ok(())
    })
    
}



#[test]
fn test_apply_blueprint_iterations(){
    let connection=&mut establish_connection().unwrap();

    connection.test_transaction::<_,Error,_>(|conn|{ 
        let select_index=Blueprint::base_level(conn)?;
        let clear_index=Blueprint::top_level(conn)?;
        
        let (payload,timestamp)=Blueprint::select(conn, select_index)?;
        let mut bytes=[0u8;32];
        let block_vector=Block::select_with_level(conn, select_index)?;

        let transactions_receipts=Transaction::select_receipts_from_block_number(conn, select_index)?;
        let transaction_objects=Transaction::select_objects_from_block_number(conn, select_index)?;

        let context_hash_vector=ContextHash::select(conn, select_index)?;
        
        for _i in 0..10{
            let insert_index=Blueprint::top_level(conn)?+1;
        
            
            let blueprint=Blueprint{
                id:insert_index,payload:payload.clone(),timestamp
            };
            
            
            rand::fill(&mut bytes);
            let hash=Vec::from(bytes);
            
            
            
            let block=Block{
                level:insert_index,hash:hash.clone(),block:block_vector.clone()
            };
            

            let transactions=transactions_receipts
                .clone()
                .into_iter()
                .zip(transaction_objects.clone()
                    .into_iter())
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
                context_hash:context_hash_vector.clone()
            };

            let _=PendingConfirmation::select_with_level(conn, blueprint.id);
            // println!("blueprint_id:{}",blueprint.id);
            blueprint.insert(conn)?;
            block.insert(conn)?;
            for tx in transactions{
                tx.insert(conn)?;
            }
            context_hash.insert(conn)?;
            let _history_mode=Metadata::get_history_mode(conn)?;

            let (insertedpayload, insertedtimestamp)=Blueprint::select(conn, insert_index)?;
            let insertedhash=Block::select_hash_of_number(conn, insert_index)?;
            let insertedblock=Block::select_with_level(conn, insert_index)?;

            let _inserted_transactions_receipts=Transaction::select_receipts_from_block_number(conn, insert_index)?;
            let _inserted_transaction_objects=Transaction::select_objects_from_block_number(conn, insert_index)?;
            
            let inserted_context_hash=ContextHash::select(conn,insert_index)?;


            assert_eq!(payload,insertedpayload);
            assert_eq!(timestamp,insertedtimestamp);
            assert_eq!(hash,insertedhash);
            assert_eq!(block_vector,insertedblock);
            // assert_eq!(transactions_receipts,inserted_transactions_receipts);
            // assert_eq!(transaction_objects,inserted_transaction_objects);
            assert_eq!(context_hash_vector,inserted_context_hash);
        }
        

        Blueprint::clear_after(conn, clear_index)?;
        Block::clear_after(conn, clear_index)?;
        ContextHash::clear_after(conn, clear_index)?;
        Transaction::clear_after(conn, clear_index)?;

        Ok(())
    })
   
}