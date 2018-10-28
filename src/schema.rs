table! {
    agents (id) {
        id -> Int4,
        uuid -> Uuid,
        hostname -> Varchar,
        ip -> Cidr,
        mac -> Macaddr,
    }
}
