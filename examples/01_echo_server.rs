use log::info;
use maelstrom_rs::actor::Actor;
use maelstrom_rs::error::Error;
use maelstrom_rs::message::{Request, Response};
use maelstrom_rs::runtime::Runtime;
use serde_json::{Map, Value};

fn main() {
    let node = EchoActor { node_id: None };
    let mut runtime = Runtime::new(Box::new(node));
    runtime.start();
}

struct EchoActor {
    node_id: Option<String>,
}

impl Actor for EchoActor {
    fn init(&mut self, node_id: &str, _node_ids: Vec<String>) -> Result<(), Error> {
        self.node_id = Some(String::from(node_id));
        info!("node {} initiated", node_id);
        Ok(())
    }

    fn receive(&mut self, message: &Request) -> Result<Option<Response>, Error> {
        match message.message_type.as_str() {
            "echo" => self.handle_echo(message),
            _ => unimplemented!(
                "unimplemented message type {}",
                message.message_type.as_str()
            ),
        }
    }
}

impl EchoActor {
    pub(crate) fn handle_echo(&self, request: &Request) -> Result<Option<Response>, Error> {
        let echo = request.body.get("echo").unwrap().as_str().unwrap();
        let mut body = Map::new();
        body.insert("echo".to_string(), Value::from(String::from(echo)));

        Ok(Some(Response {
            message_type: "echo_ok".to_string(),
            message_id: request.message_id.map(|id| id + 1),
            in_reply_to: request.message_id,
            body,
        }))
    }
}
