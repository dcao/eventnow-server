use super::models::*;
use diesel::{
    self,
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::Error as DbErr,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, DbErr>;

/// Get all groups
pub fn groups(conn: &PgConnection) -> DbResult<Vec<Group>> {
    use super::schema::groups;

    let all_groups = groups::table.load::<Group>(conn)?;
    return Ok(all_groups);
}

pub fn create_group<'a>(conn: &PgConnection, group: NewGroup<'a>) -> DbResult<()> {
    use super::schema::groups;

    diesel::insert_into(groups::table)
        .values(group)
        .execute(conn)?;
    Ok(())
}
