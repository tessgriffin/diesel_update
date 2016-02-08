#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

extern crate diesel_update;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_update::*;
use self::diesel_update::models::Post;
use self::diesel_update::schema::posts;
use std::env::args;

#[changeset_for(posts)]
struct UpdatePost {
    id: i32,
    title: String,
    body: Option<String>,
}

fn main() {
    use diesel_update::schema::posts::dsl::{posts, title, body};

    let id = args().nth(1).expect("update_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let changes = UpdatePost { id: id, title: "Updated title".into(), body: None };

    let updated_post = changes.save_changes::<Post>(&connection);
}