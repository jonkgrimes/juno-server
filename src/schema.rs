#![allow(proc_macro_derive_resolution_fallback)]
table! {
    agents (id) {
        id -> Int4,
        uuid -> Uuid,
        hostname -> Varchar,
        ip -> Cidr,
    }
}
