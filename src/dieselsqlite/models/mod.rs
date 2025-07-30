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
    sql_types::{Binary, Bool},
};

const CASTINGLITERALSQL: &str = "CAST(hash as BLOB) = ";

pub fn cast_hash_comparison(
    queried_hash: &Vec<u8>,
) -> UncheckedBind<SqlLiteral<Bool>, <&Vec<u8> as AsExpression<Binary>>::Expression> {
    sql(CASTINGLITERALSQL).bind::<Binary, &Vec<u8>>(queried_hash)
}
