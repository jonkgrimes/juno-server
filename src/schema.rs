table! {
    agents (id) {
        id -> Int4,
        uuid -> Uuid,
        hostname -> Varchar,
        ip -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        encrypted_password -> Varchar,
        confirmation_token -> Nullable<Varchar>,
        remember_token -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    agents,
    users,
);
