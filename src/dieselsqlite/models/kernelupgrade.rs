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
