#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[macro_use]
extern crate rocket_contrib;


mod from_request;

use from_request::*;

#[get("/sensitive")]
fn sensitive(key: ApiKey) -> &'static str {
    "Sensitive data."
}

// Part 2:
fn main()  {

    rocket::ignite()
        // .mount("/api", routes![])
        .mount("/", routes![sensitive])
        .launch();
}





