use uuid::Uuid;

pub struct NewAgent {
    pub uuid: Uuid,
    pub hostname: String,
    pub ip: String,
    pub mac: String
}

pub struct Agent {
    pub id: u32,
    pub uuid: Uuid,
    pub hostname: String,
    pub ip: String,
    pub mac: String
}