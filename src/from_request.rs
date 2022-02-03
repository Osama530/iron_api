use rocket::http::Status;
use rocket::request::{Outcome, FromRequest, self};

pub struct ApiKey<'r>(&'r str);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey<'a>{
    type Error = ();

    fn from_request(request: &'a rocket::Request<'r>) -> Outcome<Self, Self::Error> {

        let key = request.headers().get_one("here_is_the_key").unwrap(); 
        match key {
            "osama" => Outcome::Success(ApiKey(key)),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}