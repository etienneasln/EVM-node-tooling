use crate::dieselsqlite::schema::{blueprints, blueprints::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = blueprints)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blueprint {
    pub id: i32,
    pub payload: Vec<u8>,
    pub timestamp: i32,
}

impl Blueprint {
    pub fn select(
        connection: &mut SqliteConnection,
        queried_id: i32,
    ) -> QueryResult<(Vec<u8>, i32)> {
        let tuple = blueprints
            .find(queried_id)
            .select((payload, timestamp))
            .get_result(connection)?;
        Ok(tuple)
    }

    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = self.insert_into(blueprints).execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn select_range(
        connection: &mut SqliteConnection,
        lowerlevel: i32,
        upperlevel: i32,
    ) -> QueryResult<Vec<(i32, Vec<u8>)>> {
        let vec = blueprints
            .filter(id.ge(lowerlevel).and(id.le(upperlevel)))
            .order(id.asc())
            .select((id, payload))
            .load(connection)?;
        Ok(vec)
    }

    pub fn clear_after(connection: &mut SqliteConnection, level: i32) -> QueryResult<usize> {
        let cleared_rows = delete(blueprints.filter(id.gt(level))).execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn clear_before(connection: &mut SqliteConnection, level: i32) -> QueryResult<usize> {
        let cleared_rows = delete(blueprints.filter(id.lt(level))).execute(connection)?;
        Ok(cleared_rows)
    }
    //For testing

    pub fn count(connection: &mut SqliteConnection) -> QueryResult<i64> {
        let count = blueprints.select(count(id)).first(connection)?;
        Ok(count)
    }

    pub fn base_level(connection: &mut SqliteConnection) -> QueryResult<i32> {
        let base_level = blueprints
            .select(id)
            .order(id.asc())
            .limit(1)
            .get_result(connection)?;
        Ok(base_level)
    }

    pub fn top_level(connection: &mut SqliteConnection) -> QueryResult<i32> {
        let top_level = blueprints
            .select(id)
            .order(id.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(top_level)
    }
}

#[cfg(test)]
mod blueprint_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;
    #[test]
    fn test_blueprint_insert_select_clearafter() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let inserted_payload = "payload".as_bytes().to_vec();
            let inserted_timestamp = 1000;
            let base_insert_index = Blueprint::top_level(conn)?;

            let blueprint = Blueprint {
                id: base_insert_index + 1,
                payload: inserted_payload.clone(),
                timestamp: inserted_timestamp,
            };

            let _ = blueprint.insert(conn);

            let (selected_payload, selected_timestamp) =
                Blueprint::select(conn, base_insert_index + 1)?;

            assert_eq!(selected_payload, inserted_payload);
            assert_eq!(selected_timestamp, inserted_timestamp);

            let expected_rows_cleared: usize = 1;

            let rows_cleared = Blueprint::clear_after(conn, base_insert_index)?;

            assert_eq!(rows_cleared, expected_rows_cleared);
            Ok(())
        })
    }

    #[test]
    fn test_blueprint_insert_selectrange_clearafter() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let inserted_payloads = vec![
                "payload1".as_bytes().to_vec(),
                "payload2".as_bytes().to_vec(),
                "payload3".as_bytes().to_vec(),
            ];
            let inserted_timestamps = vec![1000, 1001, 1002];
            let base_insert_index = Blueprint::top_level(conn)?;

            let blueprint1 = Blueprint {
                id: base_insert_index + 1,
                payload: inserted_payloads[0].clone(),
                timestamp: inserted_timestamps[0],
            };
            let blueprint2 = Blueprint {
                id: base_insert_index + 2,
                payload: inserted_payloads[1].clone(),
                timestamp: inserted_timestamps[1],
            };
            let blueprint3 = Blueprint {
                id: base_insert_index + 3,
                payload: inserted_payloads[2].clone(),
                timestamp: inserted_timestamps[2],
            };
            let _ = blueprint1.insert(conn)?;
            let _ = blueprint2.insert(conn)?;
            let _ = blueprint3.insert(conn)?;

            let expected_vector = vec![
                base_insert_index + 1,
                base_insert_index + 2,
                base_insert_index + 3,
            ]
            .into_iter()
            .zip(inserted_payloads)
            .collect::<Vec<(i32, Vec<u8>)>>();
            let vector =
                Blueprint::select_range(conn, base_insert_index + 1, base_insert_index + 3)?;

            assert_eq!(vector, expected_vector);

            let expected_rows_cleared: usize = 3;

            let rows_cleared = Blueprint::clear_after(conn, base_insert_index)?;

            assert_eq!(rows_cleared, expected_rows_cleared);

            Ok(())
        })
    }
}
