use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::dieselsqlite::models::Block;

pub mod models;
pub mod schema;

pub const DATABASE_URL_KEY: &str = "DATABASE_URL";
pub const DEFAULT_DATABASE_PATH: &str = "./store.sqlite";
pub const BLOCK_NUMBER_KEY: &str = "BLOCK_NUMBER";

pub fn load_database_url() -> String {
    dotenv().ok();
    env::var(DATABASE_URL_KEY).unwrap_or_else(|_| DEFAULT_DATABASE_PATH.to_string())
}

pub fn load_block_number() -> i32 {
    dotenv().ok();
    env::var(BLOCK_NUMBER_KEY)
        .map(|s| s.parse::<i32>().expect("Provide a valid block number"))
        .unwrap_or_else(|_| {
            let connection = &mut establish_connection().expect("Provide a valid database URL");
            Block::top_level(connection)
                .expect("Can't obtain default block value (top level block)")
        })
}

pub fn establish_connection() -> Result<SqliteConnection, ConnectionError> {
    let database_url = &load_database_url();

    SqliteConnection::establish(database_url)
}

