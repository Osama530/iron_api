use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize,Serialize};

use super::schema::books;
use books::dsl::books as all_books;

#[derive(Queryable, Serialize, Clone, Deserialize)]
pub struct Book {
    id: i32,
    title: String,
    auther: String,
    published: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub auther: String,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, conn: &PgConnection)-> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("error loading book")
    }

    pub fn all(conn: &PgConnection)->Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("error loading books")
    }
    
    pub fn insert(book: NewBook, conn: &PgConnection)-> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(id: i32, conn: &PgConnection)-> bool {
        if Book::show(id, conn).is_empty() {
            return false
        };
        diesel::delete(all_books.find(id))
            .execute(conn)
            .is_ok()
    }
}