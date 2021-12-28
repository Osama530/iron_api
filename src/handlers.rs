
use std::io::Read;
use std::sync::{Arc, Mutex};

use iron::headers::ContentType;
use iron::{Handler, Request, Response, IronResult, status, AfterMiddleware};
use router::Router;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json;

use crate::database::{Database, Inventory};
use crate::models::{Post, Product};

macro_rules! try_handler {
    ($e: expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, "error parsing jason")))
        }        
    };
}

macro_rules! get_http_params {
    ($r: expr, $p: expr) => {
        match $r.extensions.get::<Router>() {
            Some(route) => {
                match route.find($p) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest)),
                    }
                },
            None => return Ok(Response::with(status::InternalServerError)),
            }
        }       
}

macro_rules! lock {
    ($e: expr) => { $e.lock().unwrap() }
}

// #[derive(Serialize, Deserialize)]
pub struct Handlers {
    pub post_feed: PostFeedHandler,
    pub post_post: PostPostHandler,
    pub post: PostHandler, 
}

impl Handlers {
    pub fn new(db: Database) -> Handlers {
        let database = Arc::new(Mutex::new(db));
        Handlers {
            post_feed: PostFeedHandler::new(database.clone()),
            post_post: PostPostHandler::new(database.clone()),
            post: PostHandler::new(database.clone()),
        }
    } 
}

pub struct PostFeedHandler {
    database: Arc<Mutex<Database>>,
}

impl PostFeedHandler {
    fn new(db: Arc<Mutex<Database>>)-> PostFeedHandler{
        PostFeedHandler {
            database: db,
        }
    }
}

impl Handler for PostFeedHandler {
    fn handle(&self, _:&mut Request)-> IronResult<Response> {
        let payload = try_handler!(serde_json::to_string(lock!(&self.database).all_posts()));
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct PostPostHandler {
    database: Arc<Mutex<Database>>,
}

impl PostPostHandler {
    fn new(db: Arc<Mutex<Database>>)-> PostPostHandler {
        PostPostHandler {
            database: db,
        }
    }
}

impl Handler for PostPostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let post = serde_json::from_str(&payload).unwrap();

        lock!(self.database).add_post(post);
        Ok(Response::with((status::Created, payload))) // *payload chage to post and try

    }
}
pub struct PostHandler {
    database: Arc<Mutex<Database>>,
}

impl PostHandler {
    fn new(db: Arc<Mutex<Database>>)-> PostHandler {
        PostHandler {
            database: db,
        }
    }

    fn find_post(&self, id: &Uuid)-> Option<Post>{

        let db = lock!(&self.database);
        let mut iterator = db.all_posts().iter();

        iterator.find(|p| p.uuid() == *id).map(|p| p.clone())
    }
}

impl Handler for PostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref post_id = get_http_params!(&req, "id");

        let id = try_handler!(Uuid::parse_str(post_id));

        if let Some(post) = self.find_post(&id) {
            let payload = try_handler!(serde_json::to_string(&post));
            Ok(Response::with((status::Ok, payload)))
        }
        else {
            Ok(Response::with(status::NotFound))
        }
    }
}

pub struct JsonAfterMiddelware;

impl AfterMiddleware for JsonAfterMiddelware   {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)

    }
}