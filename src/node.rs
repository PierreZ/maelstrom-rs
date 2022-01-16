use crate::message::{Request, Response};

use std::fmt::Error;

pub trait Node {
    /// Initiate node with a name and a topology
    fn init(&mut self, node_id: &str, node_ids: Vec<String>) -> Result<(), Error>;
    /// React to a Request. Will answer with a Vec of Responses.
    fn receive(&mut self, message: &Request) -> Result<Vec<Response>, Error>;
}
