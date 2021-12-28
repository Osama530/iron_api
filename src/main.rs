use iron;
use logger;
use env_logger;
use uuid::Uuid;
use router;
use serde::{Serialize,Deserialize};

use iron::Iron;
use iron::prelude::Chain;
use logger::Logger;
use router::Router;

mod models;
mod database;
mod handlers;

use models::Post;
use database::{Database, Inventory};
use handlers::*;

fn main() {
    env_logger::init();
    let (logger_before, logger_after) = Logger::new(None);

    let mut db = Database::create_database();
    let p1 = Post {
        title: "post#1".to_string(),
        auther: "osama".to_string(),
        body: "first post".to_string(),
        datetime: chrono::offset::Utc::now(),
        uuid: Uuid::new_v4(),
    };
    db.add_post(p1);

    let p2 = Post {
        title: "post#2".to_string(),
        auther: "Khizar".to_string(),
        body: "second post".to_string(),
        datetime: chrono::offset::Utc::now(),
        uuid: Uuid::new_v4(),
    };

    db.add_post(p2);

    let handlers = Handlers::new(db);
    
    let json_content_middleware = JsonAfterMiddelware;

    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed, "post_feed");
    router.post("/post_post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:8000").unwrap();

}
