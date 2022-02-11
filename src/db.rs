use std::ops::Deref;

use diesel::PgConnection;

use rocket::request::FromRequest;
use rocket::http::Status;
use rocket::{Outcome, State};

use r2d2;
use r2d2_diesel::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn pool_init(db_url: String)-> Pool {
    let manager = ConnectionManager::new(db_url);
    r2d2::Pool::new(manager).expect("error creating pool")
}

type PoolConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub struct Conn(PoolConn);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();
    fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
        
    }
} 



impl Deref for Conn {
    type Target = PgConnection;
    
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0    
    }
}
