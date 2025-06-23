use diesel::{dsl::delete, prelude::*};

use super::schema::blueprints::dsl::blueprints;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::blueprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blueprint{
    pub id: i32,
    pub payload:Vec<u8>,
    pub timestamp:i32
}

impl Blueprint{

    
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



#[cfg(test)]
mod query_tests{
    use crate::dieselsqlite::{establish_connection};
    // pub const BASE_LEVEL:i32=19595657;
    pub const TOP_LEVEL:i32=19630993;

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

    
}