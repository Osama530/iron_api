use crate::db::Conn as DbConn;
use rocket_contrib::json::{JsonValue, Json};
use super::modle::{Book,NewBook};

#[get("/books", format = "application/json")]
pub fn index(conn: DbConn)-> Json<JsonValue> { 
    let books = Book::all(&conn);

    Json(json!({
        "status": 200,
        "result": books,
    }))
}

#[post("/book", format = "application/json", data = "<new_book>")]
pub fn new(conn: DbConn, new_book: Json<NewBook>)-> Json<JsonValue> {

    Json(json!({
        "status": Book::insert(new_book.into_inner(), &conn),
        "result": Book::all(&conn).first(),
    }))    
}

#[get("/book/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32)-> Json<JsonValue> {
    let result = Book::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };
    Json(json!({
        "sataus": status,
        "result": result.get(0),
    }))
}



#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}