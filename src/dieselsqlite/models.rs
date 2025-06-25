use diesel::{dsl::{count, delete}, prelude::*, sql_query, sql_types::{Binary, Integer}};

use super::schema::{blueprints::dsl::blueprints,blocks::dsl::blocks};

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::blueprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blueprint{
    pub id: i32,
    pub payload:Vec<u8>,
    pub timestamp:i32

}

impl Blueprint{

    pub fn count(connection:&mut SqliteConnection)->i64{
        use super::schema::blueprints::dsl::id;
            
        blueprints
        .select(count(id))
        .first(connection)
        .unwrap_or_else(|e| panic!("Error counting blueprints:{}",e))
    }
    
    pub fn select(connection:&mut SqliteConnection,id:i32)->(Vec<u8>,i32){
            use super::schema::blueprints::dsl::{payload,timestamp};
            
            blueprints
            .find(id)
            .select((payload,timestamp))
            .get_result(connection)
            .unwrap_or_else(|e| panic!("Error selecting blueprint with id:{} :{}",id,e))
    }

    pub fn insert(connection:&mut SqliteConnection,id:i32,payload:&Vec<u8>, timestamp: i32)->usize{
        let new_blueprint=NewBlueprint{
            id,payload: payload.clone(),timestamp
        };
        diesel::insert_into(blueprints)
        .values(&new_blueprint)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error inserting blueprint with id:{} :{}",id,e))
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

    



}

#[derive(Insertable)]
#[diesel(table_name = super::schema::blueprints)]
pub struct NewBlueprint{
    pub id:i32,
    pub payload: Vec<u8>,
    pub timestamp: i32,
}

#[derive(Queryable, Selectable,QueryableByName)]
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

    pub fn count(connection:&mut SqliteConnection)->i64{
        use super::schema::blocks::dsl::level;
            
        blocks
        .select(count(level))
        .first(connection)
        .unwrap_or_else(|e| panic!("Error counting blueprints:{}",e))
    }

    pub fn insert(connection:&mut SqliteConnection,level:i32,hash:&Vec<u8>, block: &Vec<u8>)->usize{
        let new_block=NewBlock{
            level,hash: hash.clone(),block:block.clone()
        };
        diesel::insert_into(blocks)
        .values(&new_block)
        .execute(connection)
        .unwrap_or_else(|e| panic!("Error inserting block with level:{} :{}",level,e))
    }

    pub fn select_with_level(connection:&mut SqliteConnection,level:i32)->Vec<u8>{
        use super::schema::blocks::dsl::block;
            blocks
            .find(level)
            .select(block)
            .get_result(connection)
            .unwrap_or_else(|e| panic!("Error selecting block with level:{} :{}",level,e))
    }


    pub fn select_with_hash(connection:&mut SqliteConnection,queried_hash:&Vec<u8>)->Vec<u8>{
        sql_query("SELECT * FROM blocks WHERE CAST(hash as BLOB)=?1")
        .bind::<Binary,_>(queried_hash)
        .get_result::<Block>(connection)
        .unwrap_or_else(|e| panic!("Error selecting block with specified hash:{}",e))
        .block
    }

    pub fn select_hash_of_number(connection:&mut SqliteConnection,level:i32)->Vec<u8>{
        use super::schema::blocks::hash;
        blocks
        .find(level)
        .select(hash)
        .get_result(connection)
        .unwrap_or_else(|e| panic!("Error selecting block with level:{} :{}",level,e))
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
    
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::blocks)]
pub struct NewBlock{
    pub level:i32,
    pub hash: Vec<u8>,
    pub block:Vec<u8>,
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
    use crate::dieselsqlite::{establish_connection, TOP_LEVEL};

    use super::*;

    #[test]
    fn test_blueprint_insert_select_clearafter(){
        let mut connection=establish_connection();

        let inserted_payload="payload".as_bytes().to_vec();
        let inserted_timestamp=1000;
        let base_insert_index=TOP_LEVEL;

        let _=Blueprint::insert(&mut connection, base_insert_index+1,&inserted_payload,inserted_timestamp);
        
        let (payload,timestamp)=Blueprint::select(&mut connection, base_insert_index+1);
        
        assert_eq!(payload,inserted_payload);
        assert_eq!(timestamp,inserted_timestamp);

        let expected_rows_cleared:usize=1;
        
        let rows_cleared=Blueprint::clear_after(&mut connection, base_insert_index);

        assert_eq!(rows_cleared,expected_rows_cleared);
        
    }

    #[test]
    fn test_blueprint_insert_selectrange_clearafter(){
        let mut connection=establish_connection();

        let inserted_payloads=vec!["payload1".as_bytes().to_vec(),"payload2".as_bytes().to_vec(),"payload3".as_bytes().to_vec()];
        let inserted_timestamps=vec![1000,1001,1002];
        let base_insert_index=TOP_LEVEL;

        let _=Blueprint::insert(&mut connection, base_insert_index+1,&inserted_payloads[0],inserted_timestamps[0]);
        let _=Blueprint::insert(&mut connection, base_insert_index+2,&inserted_payloads[1],inserted_timestamps[0]);
        let _=Blueprint::insert(&mut connection, base_insert_index+3,&inserted_payloads[2],inserted_timestamps[0]);


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
        let base_insert_index=TOP_LEVEL;



        let _=Block::insert(&mut connection, base_insert_index+1,&inserted_hash,&inserted_block);
        
        let block_from_level=Block
    ::select_with_level(&mut connection, base_insert_index+1);
        let hash_of_number=Block
    ::select_hash_of_number(&mut connection, base_insert_index+1);
        let number_of_hash=Block
    ::select_number_of_hash(&mut connection, &hash_of_number);
        let block_from_hash=Block
    ::select_with_hash(&mut connection, &hash_of_number);
        
        assert_eq!(block_from_level,inserted_block);
        assert_eq!(hash_of_number,inserted_hash);
        assert_eq!(number_of_hash,base_insert_index+1);
        assert_eq!(block_from_hash,inserted_block);


        let expected_rows_cleared:usize=1;
        
        let rows_cleared=Block
    ::clear_after(&mut connection, base_insert_index);

        assert_eq!(rows_cleared,expected_rows_cleared);
        
    }

    
}