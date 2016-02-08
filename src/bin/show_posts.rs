extern crate diesel_update;
extern crate diesel;

use self::diesel_update::*;
use self::diesel_update::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_update::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}