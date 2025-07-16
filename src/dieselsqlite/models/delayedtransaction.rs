use crate::dieselsqlite::{models::cast_hash_comparison, schema::delayed_transactions};
use diesel::{dsl::*, prelude::*};

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
        use crate::dieselsqlite::schema::delayed_transactions::dsl::*;

        let pld = delayed_transactions
            .filter(cast_hash_comparison(queried_hash))
            .select(payload)
            .get_result(connection)?;
        Ok(pld)
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

#[cfg(test)]
mod delayed_transaction_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_delayed_transaction_all() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let inserted_injected_before = 5000;
            let inserted_hash = "hash".as_bytes().to_vec();
            let inserted_payload = "payload".as_bytes().to_vec();

            let delayed_transaction = DelayedTransaction {
                injected_before: inserted_injected_before,
                hash: inserted_hash.clone(),
                payload: inserted_payload.clone(),
            };

            delayed_transaction.insert(conn)?;

            let select_payload_at_level =
                DelayedTransaction::select_at_level(conn, inserted_injected_before)?;
            let select_payload_at_hash = DelayedTransaction::select_at_hash(conn, &inserted_hash)?;

            assert_eq!(select_payload_at_level, inserted_payload);
            assert_eq!(select_payload_at_hash, inserted_payload);

            let expected_clear: usize = 1;

            let clear = DelayedTransaction::clear_before(conn, inserted_injected_before + 1)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
