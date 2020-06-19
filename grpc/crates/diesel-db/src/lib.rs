#[macro_use]
extern crate diesel;
extern crate dotenv;


use self::diesel::prelude::*;
use std::env;
use crate::models::{Book, NewBook};
use crate::schema::books::dsl::*;
use std::error::Error;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn get_books(conn: &PgConnection, limit: i64, offset: i64) -> Vec<Book> {
    return books
        .limit(limit)
        .offset(offset)
        .load::<Book>(conn).expect("Error loading books");
}

pub fn create_book<'a>(conn: &PgConnection, title1: &'a str, author1: &'a str) -> Book {
    use schema::books;

    let new_book = NewBook {
        title: title1, author: author1
    };


    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error saving book")
}


