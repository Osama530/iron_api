use diesel::{self, Insertable, Queryable, PgConnection};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use super::schema::students;
use students::dsl::students as all_students;

#[derive(Serialize, Queryable, Deserialize)]
pub struct Student {
    id: i32,
    student_name: String,
    sub_name: Option<String>,
    gender: String,
    age: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "students"]
pub struct NewStudent {
    pub student_name: String,
    pub sub_name: Option<String>,
    gender: String,
    age: i32,
}

impl NewStudent {
    pub fn new(name: String, sub: Option<String>, gender: String, age: i32)-> NewStudent {
        NewStudent {
             student_name: name,
             sub_name: sub,
             gender,
             age,
        }

    }
}

impl Student {
    pub fn add(new_student: NewStudent, conn: &PgConnection)-> bool {
        diesel::insert_into(students::table)
            .values(&new_student)
            .execute(conn)
            .is_ok()

    }
    
    pub fn show_all(conn: &PgConnection)-> Vec<Student> {
        all_students
            .order(students::id.desc())
            .load::<Student>(conn)
            .expect("error loading students")
    }

    pub fn show_id(id: i32, conn: &PgConnection)-> Vec<Student> {
        all_students
            .find(id)
            .load::<Student>(conn)
            .expect("error finding id")
    }

    pub fn remove_id(id: i32, conn: &PgConnection)-> bool {
        if Student::show_id(id, conn).is_empty() {
            return false;
        }
        
        diesel::delete(all_students.find(id))
            .execute(conn)
            .is_ok()
    }

    pub fn update_id(id: i32, conn: &PgConnection, name: String)-> bool {
        diesel::update(all_students.find(id))
            .set(
                students::dsl::student_name.eq(name))
                // students::dsl::sub_name.eq(sub) ))
            .execute(conn)
            .is_ok()
    }
}
