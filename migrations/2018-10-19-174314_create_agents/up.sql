CREATE TABLE agents (
    id SERIAL PRIMARY KEY,
    uuid uuid,
    hostname VARCHAR,
    ip cidr,
    mac macaddr
)