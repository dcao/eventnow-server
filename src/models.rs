use super::schema::*;
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Varchar,
    Insertable, Queryable,
};
use serde::{Deserialize, Serialize};
use std::{io::Write, str::FromStr};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, AsExpression)]
#[sql_type = "Varchar"]
#[serde(rename_all = "lowercase")]
pub enum GroupRole {
    Admin,
    Member,
}

impl FromStr for GroupRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(GroupRole::Admin),
            "member" => Ok(GroupRole::Member),
            _ => Err(()),
        }
    }
}

impl ToSql<Varchar, Pg> for GroupRole {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            GroupRole::Admin => out.write_all(b"admin")?,
            GroupRole::Member => out.write_all(b"member")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Pg> for GroupRole {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"admin" => Ok(GroupRole::Admin),
            b"member" => Ok(GroupRole::Member),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl Queryable<Varchar, Pg> for GroupRole {
    type Row = <String as Queryable<Varchar, Pg>>::Row;

    fn build(row: Self::Row) -> Self {
        GroupRole::from_str(&row).unwrap()
    }
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cover_img: Option<String>,
    pub address: String,
    pub loc_name: Option<String>,
    pub lat: f64,
    pub long: f64,
}

#[derive(Deserialize, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub cover_img: Option<String>,
    pub address: String,
    pub loc_name: Option<String>,
    pub lat: f64,
    pub long: f64,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "groups"]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub profile_img: Option<String>,
    pub cover_img: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct PermissionedGroup {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub profile_img: Option<String>,
    pub cover_img: Option<String>,
    pub membership_role: GroupRole,
}

#[derive(Deserialize, Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub description: Option<String>,
    pub profile_img: Option<String>,
    pub cover_img: Option<String>,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub profile_img: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct PermissionedUser {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub profile_img: Option<String>,
    pub membership_role: GroupRole,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub profile_img: Option<String>,
}

#[derive(Queryable, Serialize, Associations, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Group)]
#[belongs_to(Event)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub user_id: Option<i32>,
    pub group_id: i32,
    pub event_id: Option<i32>,
}

#[derive(Serialize, Associations, Identifiable, Queryable)]
#[belongs_to(User)]
#[belongs_to(Group)]
#[table_name = "memberships"]
pub struct Membership {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub membership_role: GroupRole,
}

#[derive(Deserialize, Insertable)]
#[table_name = "memberships"]
pub struct NewMembership {
    pub user_id: i32,
    pub group_id: i32,
    pub membership_role: GroupRole,
}
