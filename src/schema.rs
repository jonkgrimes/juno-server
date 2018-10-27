table! {
    agents (id) {
        id -> Int4,
        uuid -> Nullable<Uuid>,
        hostname -> Nullable<Varchar>,
        ip -> Nullable<Cidr>,
        mac -> Nullable<Macaddr>,
    }
}
