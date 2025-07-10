use diesel::{dsl::*, prelude::*, result::Error::*, sql_query, sql_types::Binary, upsert::excluded};




#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = super::schema::blueprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blueprint{
    pub id: i32,
    pub payload:Vec<u8>,
    pub timestamp:i32
}

impl Blueprint{


    
    
    pub fn select(connection:&mut SqliteConnection,queried_id:i32)->QueryResult<(Vec<u8>,i32)>{
        use super::schema::blueprints::dsl::*;
        let tuple= blueprints
        .find(queried_id)
        .select((payload,timestamp))
        .get_result(connection)?;
        Ok(tuple)
    }

    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::blueprints::dsl::*;
        let inserted_rows=
        self.insert_into(blueprints)
        .execute(connection)?;
        Ok(inserted_rows)
        
    }


    pub fn select_range(connection:&mut SqliteConnection,lowerlevel:i32,upperlevel:i32)->QueryResult<Vec<(i32,Vec<u8>)>>{
        use super::schema::blueprints::dsl::*;

        let vec=blueprints
        .filter(id.ge(lowerlevel).and(id.le(upperlevel)))
        .order(id.asc())
        .select((id,payload))
        .load(connection)?;
        Ok(vec)
    }

    pub fn clear_after(connection:&mut SqliteConnection,level:i32)->QueryResult<usize>{
        use super::schema::blueprints::dsl::*;
        let cleared_rows=delete(blueprints.filter(id.gt(level)))
        .execute(connection)?;
        Ok(cleared_rows)

        
    }

    pub fn clear_before(connection:&mut SqliteConnection,level:i32)->QueryResult<usize>{
        use super::schema::blueprints::dsl::*;
        let cleared_rows=delete(blueprints.filter(id.lt(level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }
    //For testing

    pub fn count(connection:&mut SqliteConnection)->QueryResult<i64>{
        use super::schema::blueprints::dsl::*;
            
        let count=blueprints
        .select(count(id))
        .first(connection)?;
        Ok(count)
    }

    pub fn base_level(connection:&mut SqliteConnection)->QueryResult<i32>{
        use super::schema::blueprints::dsl::*;
        let base_level=
        blueprints
        .select(id)
        .order(id.asc())
        .limit(1)
        .get_result(connection)?;
        Ok(base_level)
    }

    pub fn top_level(connection:&mut SqliteConnection)->QueryResult<i32>{
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

    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::blocks::dsl::*;
        let inserted_rows=
        self.insert_into(blocks)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_with_level(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<Vec<u8>>{
        use super::schema::blocks::dsl::*;
        let b=
        blocks
        .find(queried_level)
        .select(block)
        .get_result(connection)?;
        Ok(b)
    }


    pub fn select_with_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->QueryResult<Vec<u8>>{
        let b=
        sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Block>(connection)?;
        Ok(b.block)

    }

    pub fn select_hash_of_number(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<Vec<u8>>{
        use super::schema::blocks::dsl::*;
        let h=
        blocks
        .find(queried_level)
        .select(hash)
        .get_result(connection)?;
        Ok(h)
    }


    pub fn select_number_of_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->QueryResult<i32>{
        let b=
        sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Block>(connection)?;
        Ok(b.level)
    }

    

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::blocks::dsl::*;
        let cleared_rows=
        delete(blocks.filter(level.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::blocks::dsl::*;
        let cleared_rows=
        delete(blocks.filter(level.lt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    //For testing

    pub fn count(connection:&mut SqliteConnection)->QueryResult<i64>{
        use super::schema::blocks::dsl::*;

        let count=  
        blocks
        .select(count(level))
        .first(connection)?;
        Ok(count)
    }

    pub fn base_level(connection:&mut SqliteConnection)->QueryResult<i32>{
        use super::schema::blocks::dsl::*;
        let base_level=
        blocks
        .select(level)
        .order(level.asc())
        .limit(1)
        .get_result(connection)?;
        Ok(base_level)
    }

    pub fn top_level(connection:&mut SqliteConnection)->QueryResult<i32>{
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
    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::pending_confirmations::dsl::*;
        let inserted_rows=
        self.insert_into(pending_confirmations)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_with_level(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<Vec<u8>>{
        use super::schema::pending_confirmations::dsl::*;
        let h=pending_confirmations
        .find(queried_level)
        .select(hash)
        .get_result(connection)?;
        Ok(h)
    }

    pub fn delete_with_level(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::pending_confirmations::dsl::*;
        let deleted_rows=
        delete(pending_confirmations.filter(level.eq(queried_level)))
        .execute(connection)?;
        Ok(deleted_rows)
    }

    pub fn clear(connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::pending_confirmations::dsl::*;
        let deleted_rows=delete(pending_confirmations)
        .execute(connection)?;
        Ok(deleted_rows)
    }

    pub fn count(connection:&mut SqliteConnection)->QueryResult<i64>{
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
    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::transactions::dsl::*;
        let inserted_rows=
        self.insert_into(transactions)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_receipt(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->QueryResult<(Vec<u8>,i32,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>{
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

    pub fn select_receipts_from_block_number(connection:&mut SqliteConnection,queried_block_number:i32)->QueryResult<Vec<(Vec<u8>,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>>{
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

    pub fn select_object(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->QueryResult<(Vec<u8>,i32,i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>{
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
    
    pub fn select_objects_from_block_number(connection:&mut SqliteConnection,queried_block_number:i32)->QueryResult<Vec<(i32,Vec<u8>,Vec<u8>,Option<Vec<u8>>,Vec<u8>)>>{
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

    pub fn clear_after(connection:&mut SqliteConnection,queried_block_number:i32)->QueryResult<usize>{
        use super::schema::transactions::dsl::*;
        let cleared_rows=
        delete(transactions.filter(block_number.gt(queried_block_number)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_block_number:i32)->QueryResult<usize>{
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
    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::context_hashes::dsl::*;
        let inserted_rows=
        replace_into(context_hashes)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select(connection:&mut SqliteConnection, queried_id:i32)->QueryResult<Vec<u8>>{
        use super::schema::context_hashes::dsl::*;
        let hash=
        context_hashes
        .find(queried_id)
        .select(context_hash)
        .get_result(connection)?;
        Ok(hash)
    }

    pub fn get_latest(connection:&mut SqliteConnection)->QueryResult<(i32,Vec<u8>)>{
        use super::schema::context_hashes::dsl::*;
        let latest_context=
        context_hashes
        .select((id,context_hash))
        .order(id.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(latest_context)
    }

    pub fn get_earliest(connection:&mut SqliteConnection)->QueryResult<(i32,Vec<u8>)>{
        use super::schema::context_hashes::dsl::*;
        let earliest_context=
        context_hashes
        .filter(id.ge(0))
        .select((id,context_hash))
        .order(id.asc())
        .limit(1)
        .get_result(connection)?;
        Ok(earliest_context)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_id:i32)->QueryResult<usize>{
        use super::schema::context_hashes::dsl::*;
        let cleared_rows=
        delete(context_hashes.filter(id.gt(queried_id)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_id:i32)->QueryResult<usize>{
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
    
    pub fn insert_smart_rollup_address(connection:&mut SqliteConnection,inserted_value:&str) -> QueryResult<usize>{
        Metadata::insert_key_value(connection,"smart_rollup_address",inserted_value)
    }

    pub fn get_smart_rollup_address(connection:&mut SqliteConnection)->QueryResult<String>{
        Metadata::get_value(connection, "smart_rollup_address")
    }

    pub fn insert_history_mode(connection:&mut SqliteConnection,inserted_value:&str) -> QueryResult<usize>{
        Metadata::insert_key_value(connection, "history_mode", inserted_value)
    }

    pub fn get_history_mode(connection:&mut SqliteConnection)->QueryResult<String>{
       Metadata::get_value(connection, "history_mode")
    }

    fn insert_key_value(connection:&mut SqliteConnection,inserted_key:&str,inserted_value:&str)->QueryResult<usize>{
        use super::schema::metadata::dsl::*;
        let metadata_object=Metadata{
            key:inserted_key.to_string(),
            value:inserted_value.to_string()
        };
        let inserted_rows=
        metadata_object.insert_into(metadata)
        .on_conflict(key)
        .do_update()
        .set(value.eq(excluded(value)))
        .execute(connection)?;
        Ok(inserted_rows)
    }

    fn get_value(connection:&mut SqliteConnection,queried_key:&str)->QueryResult<String>{
        use super::schema::metadata::dsl::*;
        let returned_value=
        metadata
        .find(queried_key)
        .select(value)
        .get_result(connection)?;
        Ok(returned_value)
    }
}

//TODO: Check if root_hash should be Vec<u8> for serialization
#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::kernel_upgrades)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct KernelUpgrade{
    pub injected_before:i32,
    pub root_hash:Vec<u8>,
    pub activation_timestamp:i32,
    pub applied_before:Option<i32>
}

impl KernelUpgrade{
    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::kernel_upgrades::dsl::*;
        let inserted_rows=
        replace_into(kernel_upgrades)
        .values((injected_before.eq(self.injected_before),
                        root_hash.eq(self.root_hash),
                        activation_timestamp.eq(self.activation_timestamp)
                        )
        )
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn activation_levels(connection:&mut SqliteConnection)->QueryResult<Vec<i32>>{
        use super::schema::kernel_upgrades::dsl::*;
        let activation_levels=
        kernel_upgrades
        .filter(applied_before.is_not_null())
        .select(applied_before.assume_not_null())
        .order_by(applied_before.desc())
        .load(connection)?;
        Ok(activation_levels)
    }

    pub fn get_latest_unapplied(connection:&mut SqliteConnection)->QueryResult<(i32,Vec<u8>,i32)>{
        use super::schema::kernel_upgrades::dsl::*;
        let latest_unapplied=
        kernel_upgrades
        .filter(applied_before.is_null())
        .select((injected_before,root_hash,activation_timestamp))
        .order_by(injected_before.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(latest_unapplied)
    }

    pub fn find_injected_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<(Vec<u8>,i32)>{
        use super::schema::kernel_upgrades::dsl::*;
        let result=
        kernel_upgrades
        .filter(injected_before.eq(queried_level))
        .select((root_hash, activation_timestamp))
        .get_result(connection)?;
        Ok(result)
    }

    pub fn find_latest_injected_after(connection:&mut SqliteConnection, queried_level:i32)->QueryResult<(Vec<u8>,i32)>{
        use super::schema::kernel_upgrades::dsl::*;
        let latest_injected_after=
        kernel_upgrades
        .filter(injected_before.gt(queried_level))
        .select((root_hash,activation_timestamp))
        .order_by(injected_before.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(latest_injected_after)
    }

    pub fn record_apply(connection:&mut SqliteConnection, level:i32)->QueryResult<usize>{
        use super::schema::kernel_upgrades::dsl::*;
        let updated_rows=
        update(kernel_upgrades.filter(applied_before.is_null()))
        .set(applied_before.eq(level))
        .execute(connection)?;
        Ok(updated_rows)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::kernel_upgrades::dsl::*;
        let cleared_rows=
        delete(kernel_upgrades.filter(injected_before.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn nullify_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::kernel_upgrades::dsl::*;
        let nullified_rows=
        update(kernel_upgrades.filter(applied_before.gt(queried_level)))
        .set(applied_before.eq::<Option<i32>>(None))
        .execute(connection)?;
        Ok(nullified_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::kernel_upgrades::dsl::*;
        let cleared_rows=
        delete(kernel_upgrades.filter(injected_before.lt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::sequencer_upgrades)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SequencerUpgrade{
    pub injected_before:i32,
    pub sequencer:Vec<u8>,
    pub pool_address:Vec<u8>,
    pub activation_timestamp:i32,
    pub applied_before:Option<i32>
}

impl SequencerUpgrade{
    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::sequencer_upgrades::dsl::*;
        let inserted_rows=
        replace_into(sequencer_upgrades)
        .values((injected_before.eq(self.injected_before),
                        sequencer.eq(self.sequencer),
                        pool_address.eq(self.pool_address),
                        activation_timestamp.eq(self.activation_timestamp)
                        )
        )
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn activation_levels(connection:&mut SqliteConnection)->QueryResult<Vec<i32>>{
        use super::schema::sequencer_upgrades::dsl::*;
        let activation_levels=
        sequencer_upgrades
        .filter(applied_before.is_not_null())
        .select(applied_before.assume_not_null())
        .order_by(applied_before.desc())
        .load(connection)?;
        Ok(activation_levels)
    }

    pub fn get_latest_unapplied(connection:&mut SqliteConnection)->QueryResult<(i32,Vec<u8>,Vec<u8>,i32)>{
        use super::schema::sequencer_upgrades::dsl::*;
        let latest_unapplied=
        sequencer_upgrades
        .filter(applied_before.is_null())
        .select((injected_before,sequencer,pool_address,activation_timestamp))
        .order_by(injected_before.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(latest_unapplied)
    }

    pub fn find_injected_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<(Vec<u8>,Vec<u8>,i32)>{
        use super::schema::sequencer_upgrades::dsl::*;
        let result=
        sequencer_upgrades
        .filter(injected_before.eq(queried_level))
        .select((sequencer, pool_address, activation_timestamp))
        .get_result(connection)?;
        Ok(result)
    }

    pub fn find_latest_injected_after(connection:&mut SqliteConnection, queried_level:i32)->QueryResult<(Vec<u8>,Vec<u8>,i32)>{
        use super::schema::sequencer_upgrades::dsl::*;
        let latest_injected_after=
        sequencer_upgrades
        .filter(injected_before.gt(queried_level))
        .select((sequencer,pool_address,activation_timestamp))
        .order_by(injected_before.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(latest_injected_after)
    }

    pub fn record_apply(connection:&mut SqliteConnection, level:i32)->QueryResult<usize>{
        use super::schema::sequencer_upgrades::dsl::*;
        let updated_rows=
        update(sequencer_upgrades.filter(applied_before.is_null()))
        .set(applied_before.eq(level))
        .execute(connection)?;
        Ok(updated_rows)
    }


    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::sequencer_upgrades::dsl::*;
        let cleared_rows=
        delete(sequencer_upgrades.filter(injected_before.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn nullify_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::sequencer_upgrades::dsl::*;
        let nullified_rows=
        update(sequencer_upgrades.filter(applied_before.gt(queried_level)))
        .set(applied_before.eq::<Option<i32>>(None))
        .execute(connection)?;
        Ok(nullified_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::sequencer_upgrades::dsl::*;
        let cleared_rows=
        delete(sequencer_upgrades.filter(injected_before.lt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }
}


#[derive(Queryable, Selectable, Insertable,QueryableByName)]
#[diesel(table_name = super::schema::delayed_transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DelayedTransaction{
    pub injected_before:i32,
    pub hash:Vec<u8>,
    pub payload:Vec<u8>
}

impl DelayedTransaction{
    pub fn insert(self, connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::delayed_transactions::dsl::*;
        let inserted_rows=
        self.insert_into(delayed_transactions)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_at_level(connection:&mut SqliteConnection,queried_injected_before:i32)->QueryResult<Vec<u8>>{
        use super::schema::delayed_transactions::dsl::*;
        let p=
        delayed_transactions
        .filter(injected_before.eq(queried_injected_before))
        .select(payload)
        .get_result(connection)?;
        Ok(p)
    }

    pub fn select_at_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->QueryResult<Vec<u8>>{
        let dt=
        sql_query("SELECT * FROM delayed_transactions WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<DelayedTransaction>(connection)?;
        Ok(dt.payload)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::delayed_transactions::dsl::*;
        let cleared_rows=
        delete(delayed_transactions.filter(injected_before.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::delayed_transactions::dsl::*;
        let cleared_rows=
        delete(delayed_transactions.filter(injected_before.lt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::l1_l2_levels_relationships)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct L1L2LevelRelationship{
    pub latest_l2_level:i32,
    pub l1_level:i32
}

impl L1L2LevelRelationship{

    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::l1_l2_levels_relationships::dsl::*;
        let inserted_rows=
        self.insert_into(l1_l2_levels_relationships)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn get(connection:&mut SqliteConnection)->QueryResult<(i32,i32)>{
        use super::schema::l1_l2_levels_relationships::dsl::*;
        let get=l1_l2_levels_relationships
        .select((latest_l2_level,l1_level))
        .order_by(latest_l2_level.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(get)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::l1_l2_levels_relationships::dsl::*;
        let cleared_rows=
        delete(l1_l2_levels_relationships.filter(latest_l2_level.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::l1_l2_levels_relationships::dsl::*;
        let cleared_rows=
        delete(l1_l2_levels_relationships.filter(latest_l2_level.lt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::l1_l2_finalized_levels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct L1L2FinalizedLevel{
    pub l1_level:i32,
    pub start_l2_level:i32,
    pub end_l2_level:i32
}

impl L1L2FinalizedLevel{
    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let inserted_rows=
        replace_into(l1_l2_finalized_levels)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn get(connection:&mut SqliteConnection,queried_l1_level:i32)->QueryResult<(i32,i32)>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let get=l1_l2_finalized_levels
        .find(queried_l1_level)
        .select((start_l2_level,end_l2_level))
        .get_result(connection)?;
        Ok(get)
    }

    pub fn last_l2_level(connection:&mut SqliteConnection)->QueryResult<i32>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let max:Option<i32>=
        l1_l2_finalized_levels
        .select(max(end_l2_level))
        .get_result(connection)?;
        max.ok_or_else(|| NotFound)
    }

    pub fn last(connection:&mut SqliteConnection)->QueryResult<(i32,i32,i32)>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let last=
        l1_l2_finalized_levels
        .select((l1_level,start_l2_level,end_l2_level))
        .order_by(l1_level.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(last)
    }

    pub fn find_l1_level(connection:&mut SqliteConnection,queried_l2_level:i32)->QueryResult<i32>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let find=
        l1_l2_finalized_levels
        .filter(start_l2_level.lt(queried_l2_level)
                .and(end_l2_level.ge(queried_l2_level)))
        .select(l1_level)
        .order_by(l1_level.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(find)
    }

    pub fn list_by_l2_levels(connection:&mut SqliteConnection,start_l2:i32,end_l2:i32)->QueryResult<Vec<(i32,i32,i32)>>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let list=
        l1_l2_finalized_levels
        .filter(start_l2_level.ge(start_l2)
                .and(end_l2_level.le(end_l2)))
        .select((l1_level,start_l2_level,end_l2_level))
        .order_by(l1_level.asc())
        .load(connection)?;
        Ok(list)
    }

    pub fn list_by_l1_levels(connection:&mut SqliteConnection,start_l1:i32,end_l1:i32)->QueryResult<Vec<(i32,i32,i32)>>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let list=l1_l2_finalized_levels
        .filter(l1_level.between(start_l1,end_l1))
        .select((l1_level,start_l2_level,end_l2_level))
        .order_by(l1_level.asc())
        .load(connection)?;
        Ok(list)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let cleared_rows=
        delete(l1_l2_finalized_levels.filter(end_l2_level.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::l1_l2_finalized_levels::dsl::*;
        let cleared_rows=
        delete(l1_l2_finalized_levels.filter(start_l2_level.lt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::irmin_chunks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IrminChunk{
    pub level:i32,
    pub timestamp:i32
}

impl IrminChunk{
    pub fn insert(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::irmin_chunks::dsl::*;
        let inserted_rows=
        self.insert_into(irmin_chunks)
        .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn nth(connection:&mut SqliteConnection,offset:i64)->QueryResult<(i32,i32)>{
        use super::schema::irmin_chunks::dsl::*;
        let nth=
        irmin_chunks
        .select((level,timestamp))
        .order_by(level.desc())
        .limit(1)
        .offset(offset)
        .get_result(connection)?;
        Ok(nth)
    }

    pub fn latest(connection:&mut SqliteConnection)->QueryResult<(i32,i32)>{
        use super::schema::irmin_chunks::dsl::*;
        let latest=
        irmin_chunks
        .select((level,timestamp))
        .order_by(level.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(latest)
    }

    pub fn clear(connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::irmin_chunks::dsl::*;
        let cleared_rows=
        delete(irmin_chunks)
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_after(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::irmin_chunks::dsl::*;
        let cleared_rows=
        delete(irmin_chunks.filter(level.gt(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before_included(connection:&mut SqliteConnection,queried_level:i32)->QueryResult<usize>{
        use super::schema::irmin_chunks::dsl::*;
        let cleared_rows=
        delete(irmin_chunks.filter(level.le(queried_level)))
        .execute(connection)?;
        Ok(cleared_rows)
    }

}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::block_storage_mode)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BlockStorageMode{
    pub legacy:i32
}

impl BlockStorageMode{
    pub fn legacy(connection:&mut SqliteConnection)->QueryResult<i32>{
        use super::schema::block_storage_mode::dsl::*;
        let leg=
        block_storage_mode
        .select(legacy)
        .get_result(connection)?;
        Ok(leg)
    }

    pub fn force_legacy(connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::block_storage_mode::dsl::*;
        let updated_rows=
        update(block_storage_mode)
        .set(legacy.eq(1))
        .execute(connection)?;
        Ok(updated_rows)
    }   
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::migrations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Migration{
    pub id:i32,
    pub name:Option<String>
}

impl Migration{
    pub fn create_table(connection:&mut SqliteConnection)->QueryResult<usize>{
        let create=sql_query("CREATE TABLE migrations (
        id SERIAL PRIMARY KEY,
        name TEXT
        );")
        .execute(connection)?;
        Ok(create)
    }

    pub fn current_migration(connection:&mut SqliteConnection)->QueryResult<i32>{
        use super::schema::migrations::dsl::*;
        let current_id=
        migrations
        .select(id)
        .order_by(id.desc())
        .limit(1)
        .get_result(connection)?;
        Ok(current_id)
    }

    pub fn register_migration(self,connection:&mut SqliteConnection)->QueryResult<usize>{
        use super::schema::migrations::dsl::*;
        let inserted_rows=
        insert_into(migrations)
        .values(&self)
        .execute(connection)?;
        Ok(inserted_rows)
    }
}




