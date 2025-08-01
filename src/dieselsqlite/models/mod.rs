mod block;
mod blockstoragemode;
mod blueprint;
mod contexthash;
mod delayedtransaction;
mod irminchunk;
mod kernelupgrade;
mod l1l2finalizedlevel;
mod l1l2levelrelationship;
mod metadata;
mod migration;
mod pendingconfirmation;
mod schema;
mod sequencerupgrade;
mod transaction;

pub use block::*;
pub use blockstoragemode::*;
pub use blueprint::*;
pub use contexthash::*;
pub use delayedtransaction::*;
pub use irminchunk::*;
pub use kernelupgrade::*;
pub use l1l2finalizedlevel::*;
pub use l1l2levelrelationship::*;
pub use metadata::*;
pub use migration::*;
pub use pendingconfirmation::*;
pub use schema::*;
pub use sequencerupgrade::*;
pub use transaction::*;

use diesel::{
    dsl::sql,
    expression::{AsExpression, SqlLiteral, UncheckedBind},
    prelude::*,
    sql_types::{Binary, Bool},
};

use crate::dieselsqlite::schema::{blocks, context_hashes};

const CASTINGLITERALSQL: &str = "CAST(hash as BLOB) = ";

pub fn cast_hash_comparison(
    queried_hash: &Vec<u8>,
) -> UncheckedBind<SqlLiteral<Bool>, <&Vec<u8> as AsExpression<Binary>>::Expression> {
    sql(CASTINGLITERALSQL).bind::<Binary, &Vec<u8>>(queried_hash)
}

pub fn context_hash_of_block_hash(
    connection: &mut SqliteConnection,
    queried_block_hash: &Vec<u8>,
) -> QueryResult<Vec<u8>> {
    let c_h = context_hashes::table
        .inner_join(blocks::table)
        .filter(blocks::hash.eq(queried_block_hash))
        .select(context_hashes::context_hash)
        .get_result(connection)?;
    Ok(c_h)
}

#[cfg(test)]
mod mod_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::{Connection, result::Error};

    #[test]
    fn test_join_context_hash_block_hash() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let id = 10;

            let inserted_context_hash = "context_hash".as_bytes().to_vec();
            let inserted_block_hash = "block_hash".as_bytes().to_vec();
            let inserted_block = "block".as_bytes().to_vec();

            let block = Block {
                level: id,
                hash: inserted_block_hash.clone(),
                block: inserted_block.clone(),
            };

            let context_hash = ContextHash {
                id,
                context_hash: inserted_context_hash.clone(),
            };

            block.insert(conn)?;
            context_hash.insert(conn)?;

            let select_context_hash = context_hash_of_block_hash(conn, &inserted_block_hash)?;

            assert_eq!(inserted_context_hash, select_context_hash);

            Ok(())
        })
    }
}
