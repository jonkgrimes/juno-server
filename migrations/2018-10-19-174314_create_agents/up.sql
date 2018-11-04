CREATE TABLE agents (
    id SERIAL PRIMARY KEY,
    uuid uuid NOT NULL,
    hostname VARCHAR NOT NULL,
    ip VARCHAR NOT NULL
)