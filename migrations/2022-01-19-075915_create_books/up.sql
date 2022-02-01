-- Your SQL goes here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(20) NOT NULL,
    auther VARCHAR(20)  NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);