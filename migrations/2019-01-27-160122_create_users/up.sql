-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    encrypted_password VARCHAR NOT NULL,
    confirmation_token VARCHAR,
    remember_token VARCHAR NOT NULL
);

CREATE UNIQUE INDEX users_email ON users (email);
CREATE INDEX users_remember_token ON users (remember_token);