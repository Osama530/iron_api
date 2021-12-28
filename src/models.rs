use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
   pub title: String,
   pub body: String,
   pub auther: String,
   pub datetime: DateTime<Utc>,
   pub uuid: Uuid,
}

impl Post {
    fn create_post(title: String, body: String, auther: String) -> Post {
        Post {
            title,
            body,
            auther,
            datetime: Utc::now(),
            uuid: Uuid::new_v4(),
        }
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

}

#[derive(Clone, Serialize, Deserialize)]
pub struct Product {
    name: String,
    qty: u32,
    price: u32,
    uuid: Uuid,
}

impl Product {
    fn new_product(name: String, qty: u32, price: u32) -> Product {
        Product {
            name,
            qty,
            price,
            uuid: Uuid::new_v4(),
        }
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }


}