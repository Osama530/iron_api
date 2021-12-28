use serde::{Serialize, Deserialize};

use crate::models::{Post, Product};

#[derive(Serialize, Deserialize)]
pub struct Database {
    posts: Vec<Post>,
}

impl Database {
    pub fn create_database()-> Database {
        Database {
            posts: Vec::new(),
        }
    }

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn all_posts(&self)-> &Vec<Post> {
        &self.posts
    }
}

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new()-> Inventory {
        Inventory {
            products: Vec::new(),
        }
    }

    fn add_product (&mut self, product: Product) {
        self.products.push(product);
    }

    fn list_all(&self)-> &Vec<Product> {
        &self.products
    }
}