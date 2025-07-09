use diesel::prelude::*;
use dotenvy::dotenv;
use rusqlite::Connection as RusqliteConnection;
use std::env;

pub mod models;
pub mod schema;

pub fn load_database_url()->String{
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish_connection(path:Option<&str>) -> Result<SqliteConnection, ConnectionError> {
    let database_url=
    match path{
        None=>&load_database_url(),
        Some(p)=>p
    };
    
    SqliteConnection::establish(database_url)
}

pub fn set_journal_mode_to_wal(conn:&mut SqliteConnection)->QueryResult<usize>{
    diesel::sql_query("PRAGMA journal_mode=WAL;")
    .execute(conn)
    
}

pub fn set_synchronous_mode_to_full(conn:&mut SqliteConnection)->QueryResult<usize>{
    diesel::sql_query("PRAGMA synchronous = FULL;")
    .execute(conn)
    
}

pub fn set_synchronous_mode_to_normal(conn:&mut SqliteConnection)->QueryResult<usize>{
    diesel::sql_query("PRAGMA synchronous = NORMAL;")
    .execute(conn)
    
}

//For benchmarking purposes
pub fn rusqlite_connection()->Result<RusqliteConnection, rusqlite::Error>{
    let database_url=load_database_url();
    
    RusqliteConnection::open(&database_url)
}

pub const CREATE_TABLE_BLUEPRINTS_QUERY:&str="CREATE TABLE blueprints (
        id SERIAL PRIMARY KEY,
        payload BLOB NOT NULL,
        timestamp DATETIME NOT NULL
        );";

pub const INSERT_INTO_BLUEPRINTS_QUERY:&str="INSERT INTO blueprints (id,payload,timestamp) VALUES (?1,?2,?3)";

pub const CLEAR_AFTER_BLUEPRINTS_QUERY:&str="DELETE FROM blueprints WHERE id > ?1";

pub const CREATE_TABLE_BLOCKS_QUERY:&str="CREATE TABLE blocks (
  level serial PRIMARY KEY,
  hash VARCHAR(32) NOT NULL,
  block BLOB NOT NULL
);";

pub const INSERT_INTO_BLOCKS_QUERY:&str="INSERT INTO blocks (level,hash,block) VALUES (?1,?2,?3)";

pub const CLEAR_AFTER_BLOCKS_QUERY:&str="DELETE FROM blocks WHERE level > ?1";