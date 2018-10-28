#![allow(proc_macro_derive_resolution_fallback)]
use uuid::Uuid;
use ipnetwork::IpNetwork;

use super::schema::agents;

#[derive(Insertable)]
#[table_name="agents"]
pub struct NewAgent<'a> {
    pub uuid: Uuid,
    pub hostname: &'a str,
    pub ip: IpNetwork,
}

#[derive(Queryable)]
pub struct Agent {
    pub id: i32,
    pub uuid: Uuid,
    pub hostname: String,
    pub ip: IpNetwork,
}