use super::models::*;
pub use diesel::result::Error as DbErr;
use diesel::{
    self,
    prelude::*,
    r2d2::{self, ConnectionManager},
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, DbErr>;

/// Get all groups
pub fn get_all_groups(conn: &PgConnection) -> DbResult<Vec<Group>> {
    use super::schema::groups;

    let all_groups = groups::table.load::<Group>(conn)?;
    return Ok(all_groups);
}

pub fn get_group_by_id(conn: &PgConnection, id: i32) -> DbResult<Group> {
    use super::schema::groups;

    groups::table.find(id).first::<Group>(conn)
}

pub fn create_group(conn: &PgConnection, group: NewGroup) -> DbResult<()> {
    use super::schema::groups;

    diesel::insert_into(groups::table)
        .values(group)
        .execute(conn)?;
    Ok(())
}

/// Get all events
pub fn get_all_events(conn: &PgConnection) -> DbResult<Vec<Event>> {
    use super::schema::events;

    let all_events = events::table.load::<Event>(conn)?;
    return Ok(all_events);
}

pub fn create_event(conn: &PgConnection, event: NewEvent) -> DbResult<()> {
    use super::schema::events;

    diesel::insert_into(events::table)
        .values(event)
        .execute(conn)?;
    Ok(())
}

pub fn get_event_by_id(conn: &PgConnection, id: i32) -> DbResult<Event> {
    use super::schema::events;

    events::table.find(id).first::<Event>(conn)
}

/// Get all users
pub fn get_all_users(conn: &PgConnection) -> DbResult<Vec<User>> {
    use super::schema::users;

    let all_users = users::table.load::<User>(conn)?;
    return Ok(all_users);
}

pub fn create_user(conn: &PgConnection, user: NewUser) -> DbResult<()> {
    use super::schema::users;

    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)?;
    Ok(())
}

pub fn get_user_by_id(conn: &PgConnection, id: i32) -> DbResult<User> {
    use super::schema::users;

    users::table.find(id).first::<User>(conn)
}

pub fn create_membership(conn: &PgConnection, mship: NewMembership) -> DbResult<()> {
    use super::schema::memberships;

    diesel::insert_into(memberships::table)
        .values(mship)
        .execute(conn)?;
    Ok(())
}

pub fn get_users_in_group(conn: &PgConnection, group_id: i32) -> DbResult<Vec<PermissionedUser>> {
    use super::schema::{groups, memberships, users};

    let group = groups::table.find(group_id).first::<Group>(conn)?;
    Membership::belonging_to(&group)
        .inner_join(users::table)
        .select((
            users::id,
            users::username,
            users::fullname,
            users::email,
            users::profile_img,
            memberships::membership_role,
        ))
        .load(conn)
}

pub fn get_groups_of_user(conn: &PgConnection, user_id: i32) -> DbResult<Vec<PermissionedGroup>> {
    use super::schema::{groups, memberships, users};

    let user = users::table.find(user_id).first::<User>(conn)?;
    Membership::belonging_to(&user)
        .inner_join(groups::table)
        .select((
            groups::id,
            groups::name,
            groups::description,
            groups::profile_img,
            groups::cover_img,
            memberships::membership_role,
        ))
        .load(conn)
}
