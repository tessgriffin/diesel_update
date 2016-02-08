use diesel::prelude::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::posts;

#[insertable_into(posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body : &'a str,
}

#[changeset_for(posts)]
pub struct UpdatePost<'a> {
    pub id: i32,
    pub title: &'a str,
    pub body: Option<&'a str>,
}

impl<'a> UpdatePost<'a> {
    pub fn new(id: i32, title: &'a str, body: Option<&'a str>) -> Self {
        UpdatePost {
            id: id,
            title: title,
            body: body,
        }
    }
}