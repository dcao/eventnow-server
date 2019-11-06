use super::schema::*;
use diesel::{Insertable, Queryable};
use serde::Serialize;

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
