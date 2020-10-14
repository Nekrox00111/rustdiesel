#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::PgConnection;
use models::Post;
use std::io::{stdin, Read};

pub struct DieselDemo{
    database_connection: PgConnection,
}

impl DieselDemo{
    pub fn new(database_url: String) -> DieselDemo{
        let database_connection = PgConnection::establish(&database_url).expect("Error connecting to database");
        DieselDemo{
            database_connection,
        }
    }

    pub fn run(&self){
        self.display_all_posts();
    }

    fn display_all_posts(&self){
        use schema::posts::dsl::*;
        let all_post = posts
        .load::<Post>(&self.database_connection)
        .expect("Error getting our posts");
        println!("Displaying all posts");
        for post in all_post {
            println!("-------------------");
            println!("{}",post.title);
            println!("-------------------");
            println!("{}", post.body);
            println!();
        }
    }

    fn add_new_post(&self){
        println!("Creating new post");
        println!("Title: ");
        let mut title = String::new();
        stdin().read_line(&mut title).unwrap();
        println!("Body: ");
        let mut body = String::new();
        stdin().read_line(&mut body).unwrap();

    }

}