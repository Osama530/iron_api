use rocket_contrib::json::{Json, JsonValue};
use crate::db::Conn as DbConn;
use super::model::{Student, NewStudent};

#[get("/students", format = "application/json")]
pub fn show_all(conn: DbConn)-> Json<JsonValue> {
    let students = Student::show_all(&conn);

    Json(json!({
        "status": 200,
        "result": students,
    }))
}

// reminder: do this with forms
#[post("/add", format = "application/json", data = "<new_student>")]
pub fn add_student(conn: DbConn, new_student: Json<NewStudent>)-> Json<JsonValue> {
    Json(json!({
        "result" : Student::add(new_student.into_inner(), &conn),
        "status" : Student::show_all(&conn).first(),
    }))
}

#[get("/student/<id>")]
pub fn show_id(conn: DbConn, id: i32)-> Json<JsonValue> {
    let student = Student::show_id(id, &conn);

    Json(json!({
        "status" : 200,
        "result" : student,
    }))
}

#[delete("/student/<id>")]
pub fn delete_id(id: i32, conn: DbConn)-> Json<JsonValue> {
    let status = if Student::remove_id(id, &conn) { 
        "200"
    }
    else {
        "404"
    };
    Json(json!({
        "status": status,
        "result": "null", 
    }))
}

#[put("/student/<id>", format = "application/json", data = "<student>")]
pub fn update_name(id: i32, conn: DbConn, student: Json<NewStudent>)-> Json<JsonValue> {
    let ref name = student.into_inner().student_name;
    let status = if Student::update_id(id, &conn, name.to_string()){
        "200"
    }
    else {
        "404"
    };
    Json(json!({
        "staus": status,
        "result" : "sucssesfully updated",
    }))
}