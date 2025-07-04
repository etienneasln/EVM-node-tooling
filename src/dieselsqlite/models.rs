use diesel::{dsl::{count, delete}, prelude::*, result::Error, sql_query, sql_types::Binary, upsert::excluded};


#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = super::schema::blueprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blueprint{
    pub id: i32,
    pub payload:Vec<u8>,
    pub timestamp:i32
}

impl Blueprint{


    
    
    pub fn select(connection:&mut SqliteConnection,queried_id:i32)->Result<(Vec<u8>,i32),Error>{
        use super::schema::blueprints::dsl::*;
        let tuple= blueprints
        .find(queried_id)
        .select((payload,timestamp))
        .get_result(connection)?;
        Ok(tuple)
    }

    pub fn insert(self,connection:&mut SqliteConnection)->Result<usize, Error>{
        use super::schema::blueprints::dsl::*;
        let inserted_rows=diesel::insert_into(blueprints)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
        
    }


    pub fn select_range(connection:&mut SqliteConnection,lowerlevel:i32,upperlevel:i32)->Result<Vec<(i32,Vec<u8>)>,Error>{
        use super::schema::blueprints::dsl::*;

        let vec=blueprints
        .filter(id.ge(lowerlevel).and(id.le(upperlevel)))
        .order(id.asc())
        .select((id,payload))
        .load(connection)?;
        Ok(vec)
    }

    pub fn clear_after(connection:&mut SqliteConnection,level:i32)->Result<usize,Error>{
        use super::schema::blueprints::dsl::*;
        let cleared_rows=delete(blueprints.filter(id.gt(level)))
        .execute(connection)?;
        Ok(cleared_rows)

        
    }

    pub fn clear_before(connection:&mut SqliteConnection,level:i32)->Result<usize,Error>{
        use super::schema::blueprints::dsl::*;
        let cleared_rows=delete(blueprints.filter(id.lt(level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }
    //For testing

    pub fn count(connection:&mut SqliteConnection)->Result<i64,Error>{
        use super::schema::blueprints::dsl::*;
            
        let count=blueprints
        .select(count(id))
        .first(connection)?;
        Ok(count)
    }

    pub fn base_level(connection:&mut SqliteConnection)->Result<i32,Error>{
        use super::schema::blueprints::dsl::*;
        let base_level=
        blueprints
        .select(id)
        .order(id.asc())
        .limit(1)
        .get_result(connection)?;
        Ok(base_level)
    }

    pub fn top_level(connection:&mut SqliteConnection)->Result<i32,Error>{
        use super::schema::blueprints::dsl::*;
        let top_level=
        blueprints
        .select(id)
        .order(id.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(top_level)
    }


}



#[derive(Queryable, Selectable,QueryableByName,Insertable)]
#[diesel(table_name = super::schema::blocks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Block {
    pub level: i32,
    pub hash:Vec<u8>,
    pub block:Vec<u8>
}

impl Block {

    pub fn insert(self,connection:&mut SqliteConnection)->Result<usize,Error>{
        use super::schema::blocks::dsl::*;
        let inserted_rows=diesel::insert_into(blocks)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
        
    }

    pub fn select_with_level(connection:&mut SqliteConnection,queried_level:i32)->Result<Vec<u8>,Error>{
        use super::schema::blocks::dsl::*;
        let b=
        blocks
        .find(queried_level)
        .select(block)
        .get_result(connection)?;
        Ok(b)
    }


    pub fn select_with_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->Result<Vec<u8>,Error>{
        let b=
        sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Block>(connection)?;
        Ok(b.block)

    }

    pub fn select_hash_of_number(connection:&mut SqliteConnection,queried_level:i32)->Result<Vec<u8>,Error>{
        use super::schema::blocks::dsl::*;
        let h=
        blocks
        .find(queried_level)
        .select(hash)
        .get_result(connection)?;
        Ok(h)
    }


    pub fn select_number_of_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->Result<i32,Error>{
        let b=
        sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Block>(connection)?;
        Ok(b.level)
    }

    

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->Result<usize,Error>{
        use super::schema::blocks::dsl::*;
        let cleared_rows=
        delete(blocks.filter(level.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->Result<usize,Error>{
        use super::schema::blocks::dsl::*;
        let cleared_rows=
        delete(blocks.filter(level.lt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    //For testing

    pub fn count(connection:&mut SqliteConnection)->Result<i64,Error>{
        use super::schema::blocks::dsl::*;

        let count=  
        blocks
        .select(count(level))
        .first(connection)?;
        Ok(count)
    }

    pub fn base_level(connection:&mut SqliteConnection)->Result<i32,Error>{
        use super::schema::blocks::dsl::*;
        let base_level=
        blocks
        .select(level)
        .order(level.asc())
        .limit(1)
        .get_result(connection)?;
        Ok(base_level)
    }

    pub fn top_level(connection:&mut SqliteConnection)->Result<i32,Error>{
        use super::schema::blocks::dsl::*;
        let base_level=
        blocks
        .select(level)
        .order(level.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(base_level)
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
    pub fn insert(self,connection:&mut SqliteConnection)->Result<usize,Error>{
        use super::schema::pending_confirmations::dsl::*;
        let inserted_rows=
        diesel::insert_into(pending_confirmations)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_with_level(connection:&mut SqliteConnection,queried_level:i32)->Result<Vec<u8>,Error>{
        use super::schema::pending_confirmations::dsl::*;
        let h=pending_confirmations
        .find(queried_level)
        .select(hash)
        .get_result(connection)?;
        Ok(h)
    }

    pub fn delete_with_level(connection:&mut SqliteConnection,queried_level:i32)->Result<usize,Error>{
        use super::schema::pending_confirmations::dsl::*;
        let deleted_rows=
        delete(pending_confirmations.filter(level.eq(queried_level)))
        .execute(connection)?;
        Ok(deleted_rows)
    }

    pub fn clear(connection:&mut SqliteConnection)->Result<usize,Error>{
        use super::schema::pending_confirmations::dsl::*;
        let deleted_rows=delete(pending_confirmations)
        .execute(connection)?;
        Ok(deleted_rows)
    }

    pub fn count(connection:&mut SqliteConnection)->Result<i64,Error>{
        use super::schema::pending_confirmations::dsl::*;
        let count=pending_confirmations
        .select(count(level))
        .first(connection)?;
        Ok(count)
    }

}

#[derive(Queryable, Selectable, Insertable,QueryableByName)]
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
    pub fn insert(self,connection:&mut SqliteConnection)->Result<usize,Error>{
        use super::schema::transactions::dsl::*;
        let inserted_rows=
        diesel::insert_into(transactions)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_receipt(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->Result<(Vec<u8>,i32,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>),Error>{
        let receipt=sql_query("SELECT * FROM transactions WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Transaction>(connection)?;
        Ok((receipt.block_hash,
        receipt.block_number,
        receipt.index_,
        receipt.hash,
        receipt.from_,
        receipt.to_,
        receipt.receipt_fields))
        
    }

    pub fn select_receipts_from_block_number(connection:&mut SqliteConnection,queried_block_number:i32)->Result<Vec<(Vec<u8>,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>,Error>{
        use super::schema::transactions::dsl::*;
        let receipts=
        transactions
        .filter(block_number.eq(queried_block_number))
        .select((block_hash,
                index_,
                hash,
                from_,
                to_,
                receipt_fields))
        .load(connection)?;
        Ok(receipts)
    }

    pub fn select_object(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->Result<(Vec<u8>,i32,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>),Error>{
        let object=sql_query("SELECT * FROM transactions WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Transaction>(connection)?;
        Ok((object.block_hash,
        object.block_number,
        object.index_,
        object.hash,
        object.from_,
        object.to_,
        object.object_fields))
    }
    
    pub fn select_objects_from_block_number(connection:&mut SqliteConnection,queried_block_number:i32)->Result<Vec<(i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>,Error>{
        use super::schema::transactions::dsl::*;
        let objects=transactions
        .filter(block_number.eq(queried_block_number))
        .select((index_,
                hash,
                from_,
                to_,
                object_fields))
        .load(connection)?;
        Ok(objects)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_block_number:i32)->Result<usize,Error>{
        use super::schema::transactions::dsl::*;
        let cleared_rows=
        delete(transactions.filter(block_number.gt(queried_block_number)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_block_number:i32)->Result<usize,Error>{
        use super::schema::transactions::dsl::*;
        let cleared_rows=
        delete(transactions.filter(block_number.lt(queried_block_number)))
        .execute(connection)?;
        Ok(cleared_rows)
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
    pub fn insert(self,connection:&mut SqliteConnection)->Result<usize,Error>{
        use super::schema::context_hashes::dsl::*;
        let inserted_rows=
        diesel::insert_into(context_hashes)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select(connection:&mut SqliteConnection, queried_id:i32)->Result<Vec<u8>,Error>{
        use super::schema::context_hashes::dsl::*;
        let hash=
        context_hashes
        .find(queried_id)
        .select(context_hash)
        .get_result(connection)?;
        Ok(hash)
    }

    pub fn get_latest(connection:&mut SqliteConnection)->Result<(i32,Vec<u8>),Error>{
        use super::schema::context_hashes::dsl::*;
        let latest_context=
        context_hashes
        .select((id,context_hash))
        .order(id.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(latest_context)
    }

    pub fn get_earliest(connection:&mut SqliteConnection)->Result<(i32,Vec<u8>),Error>{
        use super::schema::context_hashes::dsl::*;
        let earliest_context=
        context_hashes
        .filter(id.ge(0))
        .select((id,context_hash))
        .order(id.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(earliest_context)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_id:i32)->Result<usize,Error>{
        use super::schema::context_hashes::dsl::*;
        let cleared_rows=
        delete(context_hashes.filter(id.gt(queried_id)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_id:i32)->Result<usize,Error>{
        use super::schema::context_hashes::dsl::*;
        let cleared_rows=delete(context_hashes.filter(id.lt(queried_id)))
        .execute(connection)?;
        Ok(cleared_rows)
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
    pub fn insert_smart_rollup_address(connection:&mut SqliteConnection,inserted_value:&str) -> Result<usize,Error>{
        use super::schema::metadata::dsl::*;
        let metadata_object=Metadata{
            key:"smart_rollup_address".to_string(),
            value:inserted_value.to_string()
        };
        let inserted_rows=
        diesel::insert_into(metadata)
        .values(metadata_object)
        .on_conflict(key)
        .do_update()
        .set(value.eq(excluded(value)))
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn get_smart_rollup_address(connection:&mut SqliteConnection)->Result<String,Error>{
        use super::schema::metadata::dsl::*;
        let address=
        metadata
        .find("smart_rollup_address")
        .select(value)
        .get_result(connection)?;
        Ok(address)
    }

    pub fn insert_history_mode(connection:&mut SqliteConnection,inserted_value:&str) -> Result<usize,Error>{
        use super::schema::metadata::dsl::*;
        let metadata_object=Metadata{
            key:"history_mode".to_string(),
            value:inserted_value.to_string()
        };
        let inserted_rows=diesel::insert_into(metadata)
        .values(metadata_object)
        .on_conflict(key)
        .do_update()
        .set(value.eq(excluded(value)))
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn get_history_mode(connection:&mut SqliteConnection)->Result<String,Error>{
        use super::schema::metadata::dsl::*;
        let history_mode=
        metadata
        .find("history_mode")
        .select(value)
        .get_result(connection)?;
        Ok(history_mode)
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

