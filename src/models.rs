use super::schema::*;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, pg::Pg, serialize::{self, ToSql, IsNull, Output}, deserialize::{self, FromSql}, sql_types::Varchar};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupRole {
    Admin,
    Member,
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

#[derive(Queryable, Serialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cover_img: Option<String>,
    pub address: String,
    pub loc_name: String,
    pub lat: f64,
    pub long: f64,
}

#[derive(Insertable)]
#[table_name = "events"]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub cover_img: Option<&'a str>,
    pub address: &'a str,
    pub loc_name: Option<&'a str>,
    pub lat: f64,
    pub long: f64,
}

#[derive(Queryable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub profile_img: Option<String>,
    pub cover_img: Option<String>,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub profile_img: Option<&'a str>,
    pub cover_img: Option<&'a str>,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub profile_img: Option<String>,
}

#[derive(Queryable, Serialize, Associations)]
#[belongs_to(User)]
#[belongs_to(Group)]
#[belongs_to(Event)]
pub struct Post {
    pub id: i32,
    pub title: Option<String>,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub user_id: i32,
    pub group_id: i32,
    pub event_id: Option<i32>,
}

#[derive(Queryable, Serialize, Associations)]
#[belongs_to(User)]
#[belongs_to(Group)]
pub struct Membership {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub membership_role: GroupRole,
}
