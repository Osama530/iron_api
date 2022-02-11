-- Your SQL goes here
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    student_name VARCHAR(20) NOT NULL,
    sub_name VARCHAR(20),
    gender VARCHAR(10) NOT NULL,
    age INT NOT NULL
);