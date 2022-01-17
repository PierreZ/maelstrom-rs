#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![allow(clippy::needless_doctest_main)]

//! A crate that is providing an Actor model to develop toy distributed systems using [Maelstrom](https://github.com/jepsen-io/maelstrom).
//! Examples can be found in the [Examples folder](https://github.com/PierreZ/maelstrom-rs/tree/main/examples).
//!
//! ## Example
//! ```rust
//! use maelstrom_rs::actor::Actor;
//! use maelstrom_rs::message::{Request, Response};
//! use maelstrom_rs::error::Error;
//! use maelstrom_rs::runtime::Runtime;
//!
//! fn main() {
//!    let node = EchoActor { node_id: None };
//!    let mut runtime = Runtime::new(Box::new(node));
//!    // runtime.start();
//! }
//!
//! struct EchoActor {
//!     node_id: Option<String>,
//! }
//!
//! impl Actor for EchoActor {
//!   fn init(&mut self, node_id: &str, _node_ids: Vec<String>) -> Result<(), Error> {
//!        self.node_id = Some(String::from(node_id));
//!        eprintln!("node {} initiated", node_id);
//!        Ok(())
//!    }
//!
//!    fn receive(&mut self, message: &Request) -> Result<Option<Response>, Error> {
//!        match message.message_type.as_str() {
//!            "echo" => unimplemented!(),
//!            _ => unimplemented!(),
//!         }
//!    }
//! }
//! ```

pub mod actor;
pub mod error;
pub mod message;
pub mod runtime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
