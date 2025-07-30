use crate::dieselsqlite::schema::{sequencer_upgrades, sequencer_upgrades::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable)]
#[diesel(table_name = sequencer_upgrades)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SequencerUpgrade {
    pub injected_before: i32,
    pub sequencer: Vec<u8>,
    pub pool_address: Vec<u8>,
    pub activation_timestamp: i64,
    pub applied_before: Option<i32>,
}

impl SequencerUpgrade {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = replace_into(sequencer_upgrades)
            .values((
                injected_before.eq(self.injected_before),
                sequencer.eq(self.sequencer),
                pool_address.eq(self.pool_address),
                activation_timestamp.eq(self.activation_timestamp),
            ))
            .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn activation_levels(connection: &mut SqliteConnection) -> QueryResult<Vec<i32>> {
        let activation_levels = sequencer_upgrades
            .filter(applied_before.is_not_null())
            .select(applied_before.assume_not_null())
            .order_by(applied_before.desc())
            .load(connection)?;
        Ok(activation_levels)
    }

    pub fn get_latest_unapplied(
        connection: &mut SqliteConnection,
    ) -> QueryResult<(i32, Vec<u8>, Vec<u8>, i64)> {
        let latest_unapplied = sequencer_upgrades
            .filter(applied_before.is_null())
            .select((
                injected_before,
                sequencer,
                pool_address,
                activation_timestamp,
            ))
            .order_by(injected_before.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(latest_unapplied)
    }

    pub fn find_injected_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<(Vec<u8>, Vec<u8>, i64)> {
        let result = sequencer_upgrades
            .filter(injected_before.eq(queried_level))
            .select((sequencer, pool_address, activation_timestamp))
            .get_result(connection)?;
        Ok(result)
    }

    pub fn find_latest_injected_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<(Vec<u8>, Vec<u8>, i64)> {
        let latest_injected_after = sequencer_upgrades
            .filter(injected_before.gt(queried_level))
            .select((sequencer, pool_address, activation_timestamp))
            .order_by(injected_before.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(latest_injected_after)
    }

    pub fn record_apply(connection: &mut SqliteConnection, level: i32) -> QueryResult<usize> {
        let updated_rows = update(sequencer_upgrades.filter(applied_before.is_null()))
            .set(applied_before.eq(level))
            .execute(connection)?;
        Ok(updated_rows)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows = delete(sequencer_upgrades.filter(injected_before.gt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn nullify_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let nullified_rows = update(sequencer_upgrades.filter(applied_before.gt(queried_level)))
            .set(applied_before.eq::<Option<i32>>(None))
            .execute(connection)?;
        Ok(nullified_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows = delete(sequencer_upgrades.filter(injected_before.lt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }
}

#[cfg(test)]
mod sequencer_upgrade_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_sequencer_upgrade_all() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let injected_before_base = 5000;
            let iter = 10;
            let applied_before_base = 6000;
            let mut expected_activation_levels: Vec<i32> = Vec::new();

            for i in 0..iter {
                let inserted_injected_before = injected_before_base + i;
                let inserted_sequencer = format!("sequencer {}", i).as_bytes().to_vec();
                let inserted_pool_address = format!("pool_address {}", i).as_bytes().to_vec();
                let inserted_activation_timestamp = i64::from(i);
                let inserted_applied_before = None;

                let sequencer_upgrade = SequencerUpgrade {
                    injected_before: inserted_injected_before,
                    sequencer: inserted_sequencer.clone(),
                    pool_address: inserted_pool_address.clone(),
                    activation_timestamp: inserted_activation_timestamp,
                    applied_before: inserted_applied_before,
                };

                sequencer_upgrade.insert(conn)?;

                let applied_before_value = applied_before_base + i;

                SequencerUpgrade::record_apply(conn, applied_before_value)?;

                expected_activation_levels.push(applied_before_value);
            }

            expected_activation_levels.reverse();

            let inserted_injected_before = injected_before_base + iter;
            let inserted_sequencer = format!("sequencer {}", iter).as_bytes().to_vec();
            let inserted_pool_address = format!("pool_address {}", iter).as_bytes().to_vec();
            let inserted_activation_timestamp = i64::from(iter);
            let inserted_applied_before = None;

            let sequencer_upgrade = SequencerUpgrade {
                injected_before: inserted_injected_before,
                sequencer: inserted_sequencer.clone(),
                pool_address: inserted_pool_address.clone(),
                activation_timestamp: inserted_activation_timestamp,
                applied_before: inserted_applied_before,
            };

            sequencer_upgrade.insert(conn)?;

            let (
                latest_unapplied_injected_before,
                latest_unapplied_sequencer,
                latest_unapplied_pool_address,
                latest_unapplied_activation_timestamp,
            ) = SequencerUpgrade::get_latest_unapplied(conn)?;

            assert_eq!(latest_unapplied_injected_before, inserted_injected_before);
            assert_eq!(latest_unapplied_sequencer, inserted_sequencer);
            assert_eq!(latest_unapplied_pool_address, inserted_pool_address);
            assert_eq!(
                latest_unapplied_activation_timestamp,
                inserted_activation_timestamp
            );

            let queried_injected_before = injected_before_base;
            let expected_sequencer = "sequencer 0".as_bytes().to_vec();
            let expected_pool_address = "pool_address 0".as_bytes().to_vec();
            let expected_activation_timestamp = 0;

            let (
                injected_before_sequencer,
                injected_before_pool_address,
                injected_before_activation_timestamp,
            ) = SequencerUpgrade::find_injected_before(conn, queried_injected_before)?;

            assert_eq!(injected_before_sequencer, expected_sequencer);
            assert_eq!(injected_before_pool_address, expected_pool_address);
            assert_eq!(
                injected_before_activation_timestamp,
                expected_activation_timestamp
            );

            let activations_levels = SequencerUpgrade::activation_levels(conn)?;

            assert_eq!(activations_levels, expected_activation_levels);

            let (
                latest_injected_after_sequencer,
                latest_injected_after_pool_address,
                latest_injected_after_activation_timestamp,
            ) = SequencerUpgrade::find_latest_injected_after(conn, inserted_injected_before - 4)?;

            assert_eq!(latest_injected_after_sequencer, inserted_sequencer);
            assert_eq!(latest_injected_after_pool_address, inserted_pool_address);
            assert_eq!(
                latest_injected_after_activation_timestamp,
                inserted_activation_timestamp
            );

            SequencerUpgrade::record_apply(conn, applied_before_base + iter)?;

            let result = SequencerUpgrade::get_latest_unapplied(conn);

            assert_eq!(result, Err(Error::NotFound));

            let expected_nullified_rows = 1;
            let nullified_rows = SequencerUpgrade::nullify_after(conn, applied_before_base + 9)?;

            assert_eq!(nullified_rows, expected_nullified_rows);

            let (
                latest_unapplied_injected_before,
                latest_unapplied_sequencer,
                latest_unapplied_pool_address,
                latest_unapplied_activation_timestamp,
            ) = SequencerUpgrade::get_latest_unapplied(conn)?;

            assert_eq!(latest_unapplied_injected_before, inserted_injected_before);
            assert_eq!(latest_unapplied_sequencer, inserted_sequencer);
            assert_eq!(latest_unapplied_pool_address, inserted_pool_address);
            assert_eq!(
                latest_unapplied_activation_timestamp,
                inserted_activation_timestamp
            );

            let expected_clear: usize = (iter + 1) as usize;

            let clear = SequencerUpgrade::clear_before(conn, injected_before_base + iter + 1)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
