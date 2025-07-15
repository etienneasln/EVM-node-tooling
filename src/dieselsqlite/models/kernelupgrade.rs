use crate::dieselsqlite::schema::{kernel_upgrades, kernel_upgrades::dsl::*};
use diesel::{dsl::*, prelude::*};

#[derive(Queryable, Selectable)]
#[diesel(table_name = kernel_upgrades)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct KernelUpgrade {
    pub injected_before: i32,
    pub root_hash: Vec<u8>,
    pub activation_timestamp: i32,
    pub applied_before: Option<i32>,
}

impl KernelUpgrade {
    pub fn insert(self, connection: &mut SqliteConnection) -> QueryResult<usize> {
        let inserted_rows = replace_into(kernel_upgrades)
            .values((
                injected_before.eq(self.injected_before),
                root_hash.eq(self.root_hash),
                activation_timestamp.eq(self.activation_timestamp),
            ))
            .execute(connection)?;
        Ok(inserted_rows)
    }

    pub fn activation_levels(connection: &mut SqliteConnection) -> QueryResult<Vec<i32>> {
        let activation_levels = kernel_upgrades
            .filter(applied_before.is_not_null())
            .select(applied_before.assume_not_null())
            .order_by(applied_before.desc())
            .load(connection)?;
        Ok(activation_levels)
    }

    pub fn get_latest_unapplied(
        connection: &mut SqliteConnection,
    ) -> QueryResult<(i32, Vec<u8>, i32)> {
        let latest_unapplied = kernel_upgrades
            .filter(applied_before.is_null())
            .select((injected_before, root_hash, activation_timestamp))
            .order_by(injected_before.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(latest_unapplied)
    }

    pub fn find_injected_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<(Vec<u8>, i32)> {
        let result = kernel_upgrades
            .filter(injected_before.eq(queried_level))
            .select((root_hash, activation_timestamp))
            .get_result(connection)?;
        Ok(result)
    }

    pub fn find_latest_injected_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<(Vec<u8>, i32)> {
        let latest_injected_after = kernel_upgrades
            .filter(injected_before.gt(queried_level))
            .select((root_hash, activation_timestamp))
            .order_by(injected_before.desc())
            .limit(1)
            .get_result(connection)?;
        Ok(latest_injected_after)
    }

    pub fn record_apply(connection: &mut SqliteConnection, level: i32) -> QueryResult<usize> {
        let updated_rows = update(kernel_upgrades.filter(applied_before.is_null()))
            .set(applied_before.eq(level))
            .execute(connection)?;
        Ok(updated_rows)
    }

    pub fn clear_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows = delete(kernel_upgrades.filter(injected_before.gt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }

    pub fn nullify_after(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let nullified_rows = update(kernel_upgrades.filter(applied_before.gt(queried_level)))
            .set(applied_before.eq::<Option<i32>>(None))
            .execute(connection)?;
        Ok(nullified_rows)
    }

    pub fn clear_before(
        connection: &mut SqliteConnection,
        queried_level: i32,
    ) -> QueryResult<usize> {
        let cleared_rows = delete(kernel_upgrades.filter(injected_before.lt(queried_level)))
            .execute(connection)?;
        Ok(cleared_rows)
    }
}

#[cfg(test)]
mod kernel_upgrade_test {
    use super::*;
    use crate::dieselsqlite::establish_connection;
    use diesel::result::Error;

    #[test]
    fn test_kernel_upgrade_all() {
        let connection = &mut establish_connection().unwrap();

        connection.test_transaction::<_, Error, _>(|conn| {
            let injected_before_base = 5000;
            let iter = 10;
            let applied_before_base = 6000;
            let mut expected_activation_levels: Vec<i32> = Vec::new();
            for i in 0..iter {
                let inserted_injected_before = injected_before_base + i;
                let inserted_root_hash = format!("root_hash {}", i).as_bytes().to_vec();
                let inserted_activation_timestamp = i;
                let inserted_applied_before = None;

                let kernel_upgrade = KernelUpgrade {
                    injected_before: inserted_injected_before,
                    root_hash: inserted_root_hash.clone(),
                    activation_timestamp: inserted_activation_timestamp,
                    applied_before: inserted_applied_before,
                };

                kernel_upgrade.insert(conn)?;

                let applied_before_value = applied_before_base + i;

                KernelUpgrade::record_apply(conn, applied_before_value)?;

                expected_activation_levels.push(applied_before_value);
            }

            expected_activation_levels.reverse();

            let inserted_injected_before = injected_before_base + iter;
            let inserted_root_hash = format!("root_hash {}", iter).as_bytes().to_vec();
            let inserted_activation_timestamp = iter;
            let inserted_applied_before = None;

            let kernel_upgrade = KernelUpgrade {
                injected_before: inserted_injected_before,
                root_hash: inserted_root_hash.clone(),
                activation_timestamp: inserted_activation_timestamp,
                applied_before: inserted_applied_before,
            };

            kernel_upgrade.insert(conn)?;

            let (
                latest_unapplied_injected_before,
                latest_unapplied_root_hash,
                latest_unapplied_activation_timestamp,
            ) = KernelUpgrade::get_latest_unapplied(conn)?;

            assert_eq!(latest_unapplied_injected_before, inserted_injected_before);
            assert_eq!(latest_unapplied_root_hash, inserted_root_hash);
            assert_eq!(
                latest_unapplied_activation_timestamp,
                inserted_activation_timestamp
            );

            let queried_injected_before = injected_before_base;
            let expected_root_hash = "root_hash 0".as_bytes().to_vec();
            let expected_activation_timestamp = 0;

            let (injected_before_root_hash, injected_before_activation_timestamp) =
                KernelUpgrade::find_injected_before(conn, queried_injected_before)?;

            assert_eq!(injected_before_root_hash, expected_root_hash);
            assert_eq!(
                injected_before_activation_timestamp,
                expected_activation_timestamp
            );

            let activations_levels = KernelUpgrade::activation_levels(conn)?;

            assert_eq!(activations_levels, expected_activation_levels);

            let (latest_injected_after_root_hash, latest_injected_after_activation_timestamp) =
                KernelUpgrade::find_latest_injected_after(conn, inserted_injected_before - 4)?;

            assert_eq!(latest_injected_after_root_hash, inserted_root_hash);
            assert_eq!(
                latest_injected_after_activation_timestamp,
                inserted_activation_timestamp
            );

            KernelUpgrade::record_apply(conn, applied_before_base + iter)?;

            let result = KernelUpgrade::get_latest_unapplied(conn);

            assert_eq!(result, Err(Error::NotFound));

            let expected_nullified_rows = 1;
            let nullified_rows = KernelUpgrade::nullify_after(conn, applied_before_base + 9)?;

            assert_eq!(nullified_rows, expected_nullified_rows);

            let (
                latest_unapplied_injected_before,
                latest_unapplied_root_hash,
                latest_unapplied_activation_timestamp,
            ) = KernelUpgrade::get_latest_unapplied(conn)?;

            assert_eq!(latest_unapplied_injected_before, inserted_injected_before);
            assert_eq!(latest_unapplied_root_hash, inserted_root_hash);
            assert_eq!(
                latest_unapplied_activation_timestamp,
                inserted_activation_timestamp
            );

            let expected_clear: usize = (iter + 1) as usize;

            let clear = KernelUpgrade::clear_before(conn, injected_before_base + iter + 1)?;

            assert_eq!(clear, expected_clear);

            Ok(())
        })
    }
}
