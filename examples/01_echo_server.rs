use log::info;
use maelstrom_rs::message::{Request, Response};
use maelstrom_rs::node::Node;
use maelstrom_rs::runtime::Runtime;

use serde_json::{Map, Value};
use std::fmt::Error;

fn main() {
    let node = EchoNode { node_id: None };
    let mut runtime = Runtime::new(Box::new(node));
    runtime.start();
}

struct EchoNode {
    node_id: Option<String>,
}

impl Node for EchoNode {
    fn init(&mut self, node_id: &str, _node_ids: Vec<String>) -> Result<(), Error> {
        self.node_id = Some(String::from(node_id));
        info!("node {} initiated", node_id);
        Ok(())
    }

    fn receive(&mut self, message: &Request) -> Result<Vec<Response>, Error> {
        match message.message_type.as_str() {
            "echo" => self.handle_echo(message),
            _ => unimplemented!(
                "unimplemented message type {}",
                message.message_type.as_str()
            ),
        }
    }
}

impl EchoNode {
    pub(crate) fn handle_echo(&self, request: &Request) -> Result<Vec<Response>, Error> {
        let echo = request.body.get("echo").unwrap().as_str().unwrap();
        let mut body = Map::new();
        body.insert("echo".to_string(), Value::from(String::from(echo)));

        Ok(vec![Response {
            message_type: "echo_ok".to_string(),
            message_id: request.message_id.map(|id| id + 1),
            in_reply_to: request.message_id,
            body,
        }])
    }
}
