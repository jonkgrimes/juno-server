use actix::prelude::*;

// Server sends these messages to the session
#[derive(Message)]
pub struct Message(pub String);

#[derive(Message)]
pub struct ClientMessage {
    pub msg: String,
}

#[derive(Message)]
pub struct Connect {
  pub addr: Recipient<Message>,
}

pub struct AgentServer {
    sessions: Vec<Recipient<Message>>,
}

impl Default for AgentServer {
    fn default() -> AgentServer {
        AgentServer {
            sessions: Vec::new(),
        }
    }
}

impl AgentServer {
    fn send_message(&self, message: &str) {
        for session in &self.sessions {
            session.do_send(Message(message.to_owned()));
        }
    }
}

impl Actor for AgentServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for AgentServer {
  type Result = ();

  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
    println!("Someone joined!");
    self.sessions.push(msg.addr);
  }
}

impl Handler<ClientMessage> for AgentServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        println!("Received a message: {}", msg.msg);
        self.send_message(msg.msg.as_str());
    }
}
