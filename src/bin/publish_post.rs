extern crate diesel_update;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_update::*;
use self::diesel_update::models::Post;
use std::env::args;

fn main() {
    use diesel_update::schema::posts::dsl::{posts, published, title};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set((published.eq(true), title.eq("I've been updated")))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}