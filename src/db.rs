use actix::prelude::*;
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::*;

use models;
use schema;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub struct RegisterAgent {
    pub hostname: String,
    pub ip: String
}

impl Message for  RegisterAgent {
    type Result = Result<models::Agent, actix_web::Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<RegisterAgent> for DbExecutor {
    type Result = Result<models::Agent, actix_web::Error>;

    fn handle(&mut self, msg: RegisterAgent, _: &mut Self::Context) -> Self::Result {
        use self::schema::agents::dsl::*;

        let new_agent = new_agent(&msg);

        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(agents)
            .values(&new_agent)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Couldn't register agent"))?;

        let mut items = agents
            .filter(uuid.eq(&new_agent.uuid))
            .load::<models::Agent>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading newly created agent"))?;

        Ok(items.pop().unwrap())
    }
}

fn new_agent<'a>(msg: &'a RegisterAgent) -> models::NewAgent<'a> {
    let uuid = uuid::Uuid::new_v4();
    let new_agent = models::NewAgent {
        uuid: uuid,
        hostname: &msg.hostname,
        ip: msg.ip.parse().unwrap()
    };
    new_agent
}
