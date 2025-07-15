use super::*;
use crate::dieselsqlite::schema::delayed_transactions;

#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = delayed_transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DelayedTransaction {
    pub injected_before: i32,
    pub hash: Vec<u8>,
    pub payload: Vec<u8>,
}

impl DelayedTransaction {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::delayed_transactions::dsl::*;
        let inserted_rows = self.insert_into(delayed_transactions).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_at_level(
        connection: &mut SqliteConnection,
        queried_injected_before: i32,
    ) -> QueryResult<Vec<u8>> {
        use crate::dieselsqlite::schema::delayed_transactions::dsl::*;
        let p = delayed_transactions
            .filter(injected_before.eq(queried_injected_before))
            .select(payload)
            .get_result(connection)?;
        Ok(p)
    }

    pub fn select_at_hash(
        connection: &mut SqliteConnection,
        queried_hash: &Vec<u8>,
    ) -> QueryResult<Vec<u8>> {
        let dt = sql_query("SELECT * FROM delayed_transactions WHERE CAST(hash as BLOB)=?1")
            .bind::<Binary, _>(queried_hash)
            .get_result::<DelayedTransaction>(connection)?;
        Ok(dt.payload)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::delayed_transactions::dsl::*;
        let cleared_rows = delete(delayed_transactions.filter(injected_before.gt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        use crate::dieselsqlite::schema::delayed_transactions::dsl::*;
        let cleared_rows = delete(delayed_transactions.filter(injected_before.lt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }
}
