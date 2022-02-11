#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;

use dotenv::dotenv;
use std::env;

mod schema;
mod model;
mod db;
mod routes;

use routes::*;

fn main()  {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("ulr does not match");
    let db_pool = db::pool_init(db_url);
    

    rocket::ignite()
        .manage(db_pool)
        .mount("/api", routes![show_all, add_student, show_id, update_name, delete_id])
        .launch();
}





