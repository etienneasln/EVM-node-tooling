use diesel::{prelude::*, result::Error};
use dotenvy::dotenv;
use rusqlite::Connection as RusqliteConnection;
use std::env;

pub mod models;
pub mod schema;

pub const BASE_LEVEL:i32=18791709;
pub const TOP_LEVEL:i32=18990601;

pub fn load_database_url()->String{
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish_connection() -> SqliteConnection {
    let database_url=load_database_url();

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn set_journal_mode_to_wal(conn:&mut SqliteConnection)->usize{
    diesel::sql_query("PRAGMA journal_mode=WAL;")
    .execute(conn)
    .unwrap_or_else(|_| panic!("Error changing journal mode to WAL"))
}

pub fn set_synchronous_mode_to_full(conn:&mut SqliteConnection)->usize{
    diesel::sql_query("PRAGMA synchronous = FULL;")
    .execute(conn)
    .unwrap_or_else(|_| panic!("Error changing synchronous mode to FULL"))
}

pub fn set_synchronous_mode_to_normal(conn:&mut SqliteConnection)->usize{
    diesel::sql_query("PRAGMA synchronous = NORMAL;")
    .execute(conn)
    .unwrap_or_else(|_| panic!("Error changing synchronous mode to NORMAL"))
}

//For benchmarking purposes
pub fn rusqlite_connection()->RusqliteConnection{
    let database_url=load_database_url();
    
    RusqliteConnection::open(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub const CREATE_TABLE_BLUEPRINTS_QUERY:&str="CREATE TABLE blueprints (
        id SERIAL PRIMARY KEY,
        payload BLOB NOT NULL,
        timestamp DATETIME NOT NULL
        );";

pub const INSERT_INTO_BLUEPRINTS_QUERY:&str="INSERT INTO blueprints (id,payload,timestamp) VALUES (?1,?2,?3)";

pub const CLEAR_AFTER_BLUEPRINTS_QUERY:&str="DELETE FROM blueprints WHERE id > ?1";