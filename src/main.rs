#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;
extern crate r2d2_diesel;

use std::env;
use dotenv::dotenv;

mod modle;
mod schema;
// mod db;
mod static_files;
mod db;
mod routes;

use routes::*;

// Part 2:
fn main()  {
    dotenv().ok();
    let database_url =  env::var("DATABASE_URL")
        .expect("database url must be provided");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes![index, new, show])
        .mount("/", routes![static_files::index, static_files::all])
        .register(catchers![not_found])
        .launch();
    }


// // Part 1:
// fn main() {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").unwrap();
//     let conn = PgConnection::establish(&database_url).unwrap();

//     let book = NewBook {
//         title: "harry potter".to_string(),
//         auther: "krisin".to_string(),
//         published: true,
//     };

//     if modle::Book::insert(book, &conn) {
//         println!("insert to database successfully");
//     }

//     else {
//         println!("error inserting to database");
//     }
// }


