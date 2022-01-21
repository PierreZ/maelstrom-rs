//! The actor runtime
use crate::actor::Actor;
use crate::message::{Request, Response};
use serde::de::Error;
use serde_json::{Map, Value};
use std::io::stdin;

/// A Runtime to run an Actor
pub struct Runtime {
    node: Box<dyn Actor>,
}

impl Runtime {
    /// Create a new Runtime.
    pub fn new(node: Box<dyn Actor>) -> Runtime {
        Runtime { node }
    }

    /// Start the runtime.
    pub fn start(&mut self) {
        let mut buffer = String::new();
        loop {
            stdin()
                .read_line(&mut buffer)
                .expect("could not read stdin");

            let mut valid_json: Map<String, Value> = match serde_json::from_slice(buffer.as_bytes())
            {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("could not deserialize stdin as json: {}", e);
                    eprintln!("stdin's content is {}", buffer);
                    continue;
                }
            };

            let request = match Request::try_from_json(&mut valid_json) {
                Ok(m) => m,
                Err(error) => {
                    eprintln!("could not deserialize stdin as a Maelstrom json: {}", error);
                    continue;
                }
            };
            eprintln!("received {:?}", &request);

            if request.message_type.as_str().eq("init") {
                match self.handle_init(&request) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!(
                            "could not deserialize stdin as a Maelstrom init json: {}",
                            error
                        );
                        continue;
                    }
                }
            } else {
                match self.node.receive(&request) {
                    Ok(vec) => vec.iter().map(|response |self.create_response(&request, response)).collect(),
                    Err(_) => unimplemented!(),
                }
            }

            buffer.clear();
        }
    }
    fn handle_init(&mut self, message: &Request) -> Result<(), serde_json::Error> {
        let node_id = match message.body.get("node_id") {
            Some(Value::String(s)) => s,
            _ => {
                return Err(serde_json::Error::custom(
                    "could not find node_id as a string in init message",
                ))
            }
        };
        let node_ids: Vec<String> = match message.body.get("node_ids") {
            Some(Value::Array(ids)) => ids
                .iter()
                .map(|id| id.as_str())
                .filter(|maybe_str| maybe_str.is_some())
                .map(|id| String::from(id.unwrap()))
                .collect(),
            _ => {
                return Err(serde_json::Error::custom(
                    "could not find node_ids as a vec of string in init message",
                ))
            }
        };

        match self.node.init(node_id.as_str(), node_ids) {
            Ok(_) => self.create_init_response(message),
            Err(error) => unimplemented!("handle error on init {:?}", error),
        };
        Ok(())
    }

    fn create_response(&self, request: &Request, response: &Response) {
        let mut body = Map::new();
        body.insert(
            String::from("type"),
            Value::from(response.message_type.to_owned()),
        );
        if let Some(message_id) = response.message_id {
            body.insert(String::from("msg_id"), Value::from(message_id));
        }
        if let Some(in_reply_to) = response.in_reply_to {
            body.insert(String::from("in_reply_to"), Value::from(in_reply_to));
        }

        for (k, v) in response.body.iter() {
            body.insert(k.to_owned(), v.to_owned());
        }
        self.send_response(request, &response.destination, body);
    }

    fn create_init_response(&self, request: &Request) {
        self.create_response(request, &Response::new_from_request(request, Default::default()));
    }

    fn send_response(&self, request: &Request, destination: &str, body: Map<String, Value>) {
        let mut reply = Map::new();
        reply.insert(
            String::from("src"),
            Value::from(request.destination.clone()),
        );
        reply.insert(String::from("dest"), Value::from(String::from(destination)));
        reply.insert(String::from("body"), Value::from(body));
        eprintln!("reply: {:?}", &reply);

        let response = serde_json::to_string(&reply).expect("could not serialize");
        println!("{}", response)
    }
}
