#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

pub mod actor;
pub mod error;
pub mod message;
pub mod runtime;

#[cfg(test)]
mod tests {
    use crate::actor::Actor;
    use crate::error::Error;
    use crate::message::{Request, Response};
    use crate::runtime::Runtime;

    pub struct DummyActor;

    impl Actor for DummyActor {
        fn init(&mut self, _node_id: &str, _node_ids: Vec<String>) -> Result<(), Error> {
            Ok(())
        }

        fn receive(&mut self, _request: &Request) -> Result<Vec<Response>, Error> {
            Ok(vec![])
        }
    }

    #[test]
    fn dummy_actor_works() {
        let mut actor = DummyActor;
        assert!(actor.init("n1", vec![String::from("n1")]).is_ok());
        assert!(actor.receive(&Request{
            source: "".to_string(),
            destination: "".to_string(),
            message_type: "".to_string(),
            message_id: None,
            in_reply_to: None,
            body: Default::default()
        }).is_ok());
    }
    #[test]
    fn runtime_accept_actor() {
        let actor = DummyActor;
        let mut _runtime = Runtime::new(Box::new(actor));
    }
}
