use maelstrom_rs::actor::Actor;
use maelstrom_rs::error::Error;
use maelstrom_rs::message::{Request, Response};
use maelstrom_rs::runtime::Runtime;
use serde_json::{Map, Value};

// https://github.com/jepsen-io/maelstrom/blob/main/doc/03-broadcast/index.md
fn main() {
    let node = BroadcastActor {
        node_id: None,
        node_ids: vec![],
        neighbors: vec![],
        messages: vec![],
    };
    let mut runtime = Runtime::new(Box::new(node));
    runtime.start();
}

struct BroadcastActor {
    node_id: Option<String>,
    node_ids: Vec<String>,
    neighbors: Vec<String>,
    messages: Vec<Value>,
}

impl BroadcastActor {
    pub(crate) fn handle_read(&self, request: &Request) -> Result<Vec<Response>, Error> {
        let mut body = serde_json::Map::new();
        body.insert(String::from("messages"), Value::from(self.messages.clone()));
        Ok(vec![Response::new_from_request(request, body)])
    }
}

impl BroadcastActor {
    pub(crate) fn handle_broadcast(&mut self, request: &Request) -> Result<Vec<Response>, Error> {
        let mut responses = vec![];

        let value = match request.body.get("message") {
            None => unreachable!(),
            Some(value) => value,
        };

        if !self.messages.contains(value) {
            self.messages.push(value.clone());

            // Gossip this message to neighbors
            for neighbor in &self.neighbors {
                let mut body = Map::new();
                body.insert(String::from("message"), value.clone());
                responses.push(Response {
                    destination: neighbor.to_owned(),
                    message_type: String::from("broadcast"),
                    message_id: None,
                    in_reply_to: None,
                    body,
                });
            }
        }


        // Inter-server messages don't have a msg_id, and don't need a response
        if request.message_id.is_some() {
            responses.push(Response::new_from_request(request, Default::default()));
        }


        Ok(responses)
    }
}

impl BroadcastActor {
    pub(crate) fn handle_topology(&mut self, request: &Request) -> Result<Vec<Response>, Error> {
        let topology = match request.body.get("topology") {
            Some(Value::Object(t)) => t,
            _ => unreachable!(),
        };

        self.neighbors = match topology.get(&*self.node_id.as_ref().unwrap()) {
            Some(Value::Array(n)) => n
                .iter()
                .map(|s| s.as_str())
                .flatten()
                .map(String::from)
                .collect(),
            _ => return Err(Error::CustomError((10_001, String::from("bad topology")))),
        };
        eprintln!("{:?} got {:?} as neighbors", self.node_id, self.neighbors);

        Ok(vec![Response::new_from_request(request, Default::default())])
    }
}

impl Actor for BroadcastActor {
    fn init(&mut self, node_id: &str, node_ids: Vec<String>) -> Result<(), Error> {
        self.node_id = Some(String::from(node_id));
        self.node_ids = node_ids;

        eprintln!("node {} initialized", node_id);
        Ok(())
    }

    fn receive(&mut self, message: &Request) -> Result<Vec<Response>, Error> {
        match message.message_type.as_str() {
            "topology" => self.handle_topology(message),
            "broadcast" => self.handle_broadcast(message),
            "read" => self.handle_read(message),
            _ => unimplemented!("unknown message {:?}", message),
        }
    }
}
