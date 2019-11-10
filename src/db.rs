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
/// Delete a post
pub fn delete_post(conn: &PgConnection, post_id: i32) -> DbResult<()> {
    use super::schema::posts;

    diesel::delete(posts::table.filter(posts::id.eq(post_id))).execute(conn)?;
    Ok(())
}

/// Delete a group
pub fn delete_group(conn: &PgConnection, group_id: i32) -> DbResult<()> {
    use super::schema::groups;
    use super::schema::posts;
    // Delete the group
    diesel::delete(groups::table.filter(groups::id.eq(group_id))).execute(conn)?;
    // Delete posts associated with the group
    diesel::delete(posts::table.filter(posts::id.eq(group_id))).execute(conn)?;
    Ok(())
    
}

/// Delete an event
pub fn delete_event(conn: &PgConnection, event_id: i32) -> DbResult<()> {
    use super::schema::events;

    diesel::delete(events::table.filter(events::id.eq(event_id))).execute(conn)?;
    Ok(())
}

/// Delete a user
pub fn delete_user(conn: &PgConnection, user_id: i32) -> DbResult<()> {
    use super::schema::users;
    use super::schema::posts;

    // Delete the user
    diesel::delete(users::table.filter(users::id.eq(user_id))).execute(conn)?;
    // Updates deleted user's posts id to that of anonymous
    let target = posts::table.filter(posts::user_id.eq(Some(user_id)));
    diesel::update(target).set(posts::user_id.eq::<Option<i32>>(None)).execute(conn)?;
    // Deletes the user memberships
    delete_memberships(conn, user_id)?;
    Ok(())
}

/// Delete memberships
pub fn delete_memberships(conn: &PgConnection, user_id: i32) -> DbResult<()> {
    use super::schema::memberships;
    // Delete memberships
    diesel::delete(memberships::table.filter(memberships::user_id.eq(user_id))).execute(conn)?;
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
