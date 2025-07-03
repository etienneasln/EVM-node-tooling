use diesel::{dsl::{count, delete}, prelude::*, sql_query, sql_types::{Binary, Integer}, upsert::excluded};


#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = super::schema::blueprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blueprint{
    pub id: i32,
    pub payload:Vec<u8>,
    pub timestamp:i32
}

impl Blueprint{


    
    
    pub fn select(connection:&mut SqliteConnection,queried_id:i32)->(Vec<u8>,i32){
        use super::schema::blueprints::dsl::*;
            
        blueprints
        .find(queried_id)
        .select((payload,timestamp))
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting blueprint with id:{} :{}",queried_id,e))
    }

    pub fn insert(self,connection:&mut SqliteConnection)->usize{
        use super::schema::blueprints::dsl::*;

        diesel::insert_into(blueprints)
        .values(&self)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error inserting blueprint with id:{} :{}",self.id,e))
    }


    pub fn select_range(connection:&mut SqliteConnection,lowerlevel:i32,upperlevel:i32)->Vec<(i32,Vec<u8>)>{
        use super::schema::blueprints::dsl::*;

        blueprints
        .filter(id.ge(lowerlevel).and(id.le(upperlevel)))
        .order(id.asc())
        .select((id,payload))
        .load(connection)
        .unwrap_or_else(|e| panic!("Error selecting blueprints from level {} to level {} (both included):{}",lowerlevel,upperlevel,e))

    
    }

    pub fn clear_after(connection:&mut SqliteConnection,level:i32)->usize{
        use super::schema::blueprints::dsl::*;
        delete(blueprints.filter(id.gt(level)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing blueprints after level {} (excluded):{}",level,e))
    }

    pub fn clear_before(connection:&mut SqliteConnection,level:i32)->usize{
        use super::schema::blueprints::dsl::*;

        delete(blueprints.filter(id.lt(level)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing blueprints before level {} (excluded):{}",level,e))
    }

    //For testing

    pub fn count(connection:&mut SqliteConnection)->i64{
        use super::schema::blueprints::dsl::*;
            
        blueprints
        .select(count(id))
        .first(connection)
        .unwrap_or_else(|e| panic!("Error counting blueprints:{}",e))
    }

    pub fn base_level(connection:&mut SqliteConnection)->i32{
        use super::schema::blueprints::dsl::*;
        blueprints
        .select(id)
        .order(id.asc())
        .limit(1)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting base level:{e}"))
    }

    pub fn top_level(connection:&mut SqliteConnection)->i32{
        use super::schema::blueprints::dsl::*;
        blueprints
        .select(id)
        .order(id.desc())
        .limit(1)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting top level:{e}"))
    }


}



#[derive(Queryable, Selectable,QueryableByName,Insertable)]
#[diesel(table_name = super::schema::blocks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Block {
    #[diesel(sql_type=Integer)]
    pub level: i32,
    #[diesel(sql_type=Binary)]
    pub hash:Vec<u8>,
    #[diesel(sql_type=Binary)]
    pub block:Vec<u8>
}

impl Block {

    

    pub fn insert(self,connection:&mut SqliteConnection)->usize{
        use super::schema::blocks::dsl::*;
        diesel::insert_into(blocks)
        .values(&self)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error inserting block with level:{} :{}",self.level,e))
    }

    pub fn select_with_level(connection:&mut SqliteConnection,queried_level:i32)->Vec<u8>{
        use super::schema::blocks::dsl::*;
        blocks
        .find(queried_level)
        .select(block)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting block with level:{} :{}",queried_level,e))
    }


    pub fn select_with_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->Vec<u8>{
        sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Block>(connection)
        .unwrap_or_else(|e| panic!("Error selecting block with specified hash:{}",e))
        .block
    }

    pub fn select_hash_of_number(connection:&mut SqliteConnection,queried_level:i32)->Vec<u8>{
        use super::schema::blocks::dsl::*;

        blocks
        .find(queried_level)
        .select(hash)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting block with level:{} :{}",queried_level,e))
    }


    pub fn select_number_of_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->i32{
        sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Block>(connection)
        .unwrap_or_else(|e| panic!("Error selecting level with specified hash:{}",e))
        .level
    }

    

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->usize{
        use super::schema::blocks::dsl::*;
        delete(blocks.filter(level.gt(queried_level)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing blocks after level {} (excluded):{}",queried_level,e))
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->usize{
        use super::schema::blocks::dsl::*;
        delete(blocks.filter(level.lt(queried_level)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing blocks before level {} (excluded):{}",queried_level,e))
    }

    //For testing

    pub fn count(connection:&mut SqliteConnection)->i64{
        use super::schema::blocks::dsl::*;

            
        blocks
        .select(count(level))
        .first(connection)
        .unwrap_or_else(|e| panic!("Error counting blueprints:{}",e))
    }

    pub fn base_level(connection:&mut SqliteConnection)->i32{
        use super::schema::blocks::dsl::*;
        blocks
        .select(level)
        .order(level.asc())
        .limit(1)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting base level:{e}"))
    }

    pub fn top_level(connection:&mut SqliteConnection)->i32{
        use super::schema::blocks::dsl::*;
        blocks
        .select(level)
        .order(level.desc())
        .limit(1)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting top level:{e}"))
    }
    
}


#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = super::schema::pending_confirmations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PendingConfirmation{
    pub level:i32,
    pub hash:Vec<u8>
}

impl PendingConfirmation{
    pub fn insert(self,connection:&mut SqliteConnection)->usize{
        use super::schema::pending_confirmations::dsl::*;
        diesel::insert_into(pending_confirmations)
        .values(&self)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error inserting pending confirmation with level:{}:{}",self.level,e))
    }

    pub fn select_with_level(connection:&mut SqliteConnection,queried_level:i32)->Vec<u8>{
        use super::schema::pending_confirmations::dsl::*;
        pending_confirmations
        .find(queried_level)
        .select(hash)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting pending confirmation with level:{}:{}",queried_level,e))
    }

    pub fn delete_with_level(connection:&mut SqliteConnection,queried_level:i32)->usize{
        use super::schema::pending_confirmations::dsl::*;
        delete(pending_confirmations.filter(level.eq(queried_level)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error deleting pending confirmation with level {} :{}",queried_level,e))
    }

    pub fn clear(connection:&mut SqliteConnection)->usize{
        use super::schema::pending_confirmations::dsl::*;
        delete(pending_confirmations)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing pending confirmations:{}",e))
    }

    pub fn count(connection:&mut SqliteConnection)->i64{
        use super::schema::pending_confirmations::dsl::*;
        pending_confirmations
        .select(count(level))
        .first(connection)
        .unwrap_or_else(|e| panic!("Error counting pending confirmations:{}",e))
    }

}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction{
    pub block_hash:Vec<u8>,
    pub block_number:i32,
    pub index_:i32,
    pub hash:Vec<u8>,
    pub from_:Vec<u8>,
    pub to_:Option<Vec<u8>>,
    pub receipt_fields:Vec<u8>,
    pub object_fields:Vec<u8>
}

impl Transaction{
    pub fn insert(self,connection:&mut SqliteConnection)->usize{
        use super::schema::transactions::dsl::*;
        diesel::insert_into(transactions)
        .values(&self)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error inserting transaction with hash:{:?}:{}",self.hash,e))
    }

    pub fn select_receipt(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->(Vec<u8>,i32,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>){
        use super::schema::transactions::dsl::*;
        transactions
        .find(queried_hash)
        .select((block_hash,
                block_number,
                index_,
                hash,
                from_,
                to_,
                receipt_fields))
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting transaction receipt with hash:{:?}:{}",queried_hash,e))
    }

    pub fn select_receipts_from_block_number(connection:&mut SqliteConnection,queried_block_number:i32)->Vec<(Vec<u8>,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>{
        use super::schema::transactions::dsl::*;
        transactions
        .filter(block_number.eq(queried_block_number))
        .select((block_hash,
                index_,
                hash,
                from_,
                to_,
                receipt_fields))
        .load(connection)
        .unwrap_or_else(|e| panic!("Error selecting transaction receipts from block number:{}:{}",queried_block_number,e))
    }

    pub fn select_object(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->(Vec<u8>,i32,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>){
        use super::schema::transactions::dsl::*;
        transactions
        .find(queried_hash)
        .select((block_hash,
                block_number,
                index_,
                hash,
                from_,
                to_,
                object_fields)
        )
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting transaction object with hash:{:?}:{}",queried_hash,e))
    }
    
    pub fn select_objects_from_block_number(connection:&mut SqliteConnection,queried_block_number:i32)->Vec<(i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>{
        use super::schema::transactions::dsl::*;
        transactions
        .filter(block_number.eq(queried_block_number))
        .select((index_,
                hash,
                from_,
                to_,
                object_fields))
        .load(connection)
        .unwrap_or_else(|e| panic!("Error selecting transaction objects from block number:{}:{}",queried_block_number,e))
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_block_number:i32)->usize{
        use super::schema::transactions::dsl::*;
        delete(transactions.filter(block_number.gt(queried_block_number)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing transactions from blocks with indices greater than {} (excluded):{}",queried_block_number,e))
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_block_number:i32)->usize{
        use super::schema::transactions::dsl::*;
        delete(transactions.filter(block_number.lt(queried_block_number)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing transactions from blocks with indices lesser than {} (excluded):{}",queried_block_number,e))
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::context_hashes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ContextHash{
    pub id:i32,
    pub context_hash:Vec<u8>
}

impl ContextHash{
    pub fn insert(self,connection:&mut SqliteConnection)->usize{
        use super::schema::context_hashes::dsl::*;
        
        diesel::insert_into(context_hashes)
        .values(&self)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error inserting context hash with id:{}:{}",self.id,e))
    }

    pub fn select(connection:&mut SqliteConnection, queried_id:i32)->Vec<u8>{
        use super::schema::context_hashes::dsl::*;

        context_hashes
        .find(queried_id)
        .select(context_hash)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting context hash:{}:{}",queried_id,e))
    }

    pub fn get_latest(connection:&mut SqliteConnection)->(i32,Vec<u8>){
        use super::schema::context_hashes::dsl::*;
        context_hashes
        .select((id,context_hash))
        .order(id.desc())
        .limit(1)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting latest context hash:{e}"))
    }

    pub fn get_earliest(connection:&mut SqliteConnection)->(i32,Vec<u8>){
        use super::schema::context_hashes::dsl::*;
        context_hashes
        .filter(id.ge(0))
        .select((id,context_hash))
        .order(id.desc())
        .limit(1)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting latest context hash:{e}"))
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_id:i32)->usize{
        use super::schema::context_hashes::dsl::*;
        delete(context_hashes.filter(id.gt(queried_id)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing context_hashes with id greater than {} (excluded):{}",queried_id,e))
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_id:i32)->usize{
        use super::schema::context_hashes::dsl::*;
        delete(context_hashes.filter(id.lt(queried_id)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error clearing context_hashes with lesser than {} (excluded):{}",queried_id,e))
    }


}


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::metadata)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Metadata{
    pub key:String,
    pub value:String
}

impl Metadata {
    pub fn insert_smart_rollup_address(connection:&mut SqliteConnection,inserted_value:&str) -> usize{
        use super::schema::metadata::dsl::*;
        let metadata_object=Metadata{
            key:"smart_rollup_address".to_string(),
            value:inserted_value.to_string()
        };
        diesel::insert_into(metadata)
        .values(metadata_object)
        .on_conflict(key)
        .do_update()
        .set(value.eq(excluded(value)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error upserting smart rollup address:{}:{}",inserted_value,e))
    }

    pub fn get_smart_rollup_address(connection:&mut SqliteConnection)->String{
        use super::schema::metadata::dsl::*;
        metadata
        .find("smart_rollup_address")
        .select(value)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error getting smart rollup address:{e}"))
    }

    pub fn insert_history_mode(connection:&mut SqliteConnection,inserted_value:&str) -> usize{
        use super::schema::metadata::dsl::*;
        let metadata_object=Metadata{
            key:"history_mode".to_string(),
            value:inserted_value.to_string()
        };
        
        diesel::insert_into(metadata)
        .values(metadata_object)
        .on_conflict(key)
        .do_update()
        .set(value.eq(excluded(value)))
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error upserting smart rollup address:{}:{}",inserted_value,e))
    }

    pub fn get_history_mode(connection:&mut SqliteConnection)->String{
        use super::schema::metadata::dsl::*;
        metadata
        .find("history_mode")
        .select(value)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error getting smart rollup address:{e}"))
    }

}


// pub fn insert<T>(conn:&mut SqliteConnection,object:impl Insertable<T>,table:impl Table)->usize{
//     diesel::insert_into(table)
//         .values(&object)
//         .execute(conn)
//         .unwrap_or_else(|e| panic!("Error inserting object:{}",e))
// }

// pub fn select_with_primary_key<'a,PK,T,S,U>(conn:&mut SqliteConnection,pk:PK,selection:S,table:T)->U
// where T:Table + FindDsl<PK>,
// S: Expression,
// <T as FindDsl<PK>>::Output:SelectDsl<S>,
// <<T as FindDsl<PK>>::Output as SelectDsl<S>>::Output:LoadQuery<'a,SqliteConnection,U>{
//     table.find(pk)
//     .select(selection)
//     .get_result(conn)
//     .unwrap_or_else(|e| panic!("Error selecting object with primary key:{e}"))
// }

#[cfg(test)]
mod query_tests{
    use crate::dieselsqlite::establish_connection;

    use super::*;

    #[test]
    fn test_blueprint_insert_select_clearafter(){
        let mut connection=establish_connection();

        

        let inserted_payload="payload".as_bytes().to_vec();
        let inserted_timestamp=1000;
        let base_insert_index=Blueprint::top_level(&mut connection)+1;

        let blueprint=Blueprint{
            id:base_insert_index,
            payload:inserted_payload.clone(),
            timestamp:inserted_timestamp
        };

        let _=blueprint.insert(&mut connection);
        
        let (payload,timestamp)=Blueprint::select(&mut connection, base_insert_index);
        
        assert_eq!(payload,inserted_payload);
        assert_eq!(timestamp,inserted_timestamp);

        let expected_rows_cleared:usize=1;
        
        let rows_cleared=Blueprint::clear_after(&mut connection, base_insert_index-1);

        assert_eq!(rows_cleared,expected_rows_cleared);
        
    }

    #[test]
    fn test_blueprint_insert_selectrange_clearafter(){
        let mut connection=establish_connection();

        let inserted_payloads=vec!["payload1".as_bytes().to_vec(),"payload2".as_bytes().to_vec(),"payload3".as_bytes().to_vec()];
        let inserted_timestamps=vec![1000,1001,1002];
        let base_insert_index=Blueprint::top_level(&mut connection);

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
        let _=blueprint1.insert(&mut connection);
        let _=blueprint2.insert(&mut connection);
        let _=blueprint3.insert(&mut connection);


        let expected_vector=vec![base_insert_index+1,base_insert_index+2,base_insert_index+3]
        .into_iter()
        .zip(inserted_payloads)
        .collect::<Vec<(i32,Vec<u8>)>>();
        let vector=Blueprint::select_range(&mut connection, base_insert_index+1,base_insert_index+3);



        assert_eq!(vector,expected_vector);

        let expected_rows_cleared:usize=3;
        
        let rows_cleared=Blueprint::clear_after(&mut connection, base_insert_index);

        assert_eq!(rows_cleared,expected_rows_cleared);

        
    }

    #[test]
    fn test_block_insert_selects_clearafter(){
        let mut connection=establish_connection();

        
        let inserted_hash="hash".as_bytes().to_vec();
        let inserted_block="block".as_bytes().to_vec();
        let base_insert_index=Block::top_level(&mut connection)+1;

        let block=Block{
            level:base_insert_index,
            hash:inserted_hash.clone(),
            block:inserted_block.clone()
        };


        let _=block.insert(&mut connection);
        
        let block_from_level=Block
    ::select_with_level(&mut connection, base_insert_index);
        let hash_of_number=Block
    ::select_hash_of_number(&mut connection, base_insert_index);
        let number_of_hash=Block
    ::select_number_of_hash(&mut connection, &hash_of_number);
        let block_from_hash=Block
    ::select_with_hash(&mut connection, &hash_of_number);
        
        assert_eq!(block_from_level,inserted_block);
        assert_eq!(hash_of_number,inserted_hash);
        assert_eq!(number_of_hash,base_insert_index);
        assert_eq!(block_from_hash,inserted_block);


        let expected_rows_cleared:usize=1;
        
        let rows_cleared=Block
    ::clear_after(&mut connection, base_insert_index-1);

        assert_eq!(rows_cleared,expected_rows_cleared);
        
    }
    

}