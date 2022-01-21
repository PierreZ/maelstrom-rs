# maelstrom-rs
A crate that is providing an Actor model to develop toy distributed systems using [Maelstrom](https://github.com/jepsen-io/maelstrom).

## What is Maelstrom?

[Maelstrom](https://github.com/jepsen-io/maelstrom) is a workbench for learning distributed systems by writing your own. It uses the Jepsen testing library to test toy implementations of distributed systems.

## Getting started

Examples following the official documentations can be found in the [examples folder](https://github.com/PierreZ/maelstrom-rs/tree/main/examples).

The crate exposes:

* an [`Actor`](crate::actor::Actor) trait that you can implement:
* a [`Runtime`](crate::runtime::Runtime), that will run it.

## Examples

 ```rust
 use maelstrom_rs::actor::Actor;
 use maelstrom_rs::message::{Request, Response};
 use maelstrom_rs::error::Error;
 use maelstrom_rs::runtime::Runtime;

 fn main() {
    let node = EchoActor { node_id: None };
    let mut runtime = Runtime::new(Box::new(node));
    // runtime.start();
 }

 struct EchoActor {
     node_id: Option<String>,
 }

 impl Actor for EchoActor {
   fn init(&mut self, node_id: &str, _node_ids: Vec<String>) -> Result<(), Error> {
        self.node_id = Some(String::from(node_id));
        eprintln!("node {} initiated", node_id);
        Ok(())
    }

    fn receive(&mut self, message: &Request) -> Result<Vec<Response>, Error> {
        match message.message_type.as_str() {
            "echo" => unimplemented!(),
            _ => unimplemented!(),
         }
    }
 }
 ```
