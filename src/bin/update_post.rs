extern crate diesel_update;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_update::*;
use self::diesel_update::models::{Post, UpdatePost};
use std::env::args;

fn main() {
    use diesel_update::schema::posts::dsl::{posts, title, body};

    let id = args().nth(1).expect("update_post requires a post id")
        .parse().expect("Invalid ID");
    let connection = establish_connection();

    let changes = UpdatePost::new(id, "Wowowowie", Some("Stuff"));


    let updated_post = changes.save_changes::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));;

    println!("Updated Post {}", updated_post.title);
}