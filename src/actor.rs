//! The actor trait
use crate::error::Error;
use crate::message::{Request, Response};

/// The Actor trait that you need to implement
pub trait Actor {
    /// Initiate node with a name and a topology
    fn init(&mut self, node_id: &str, node_ids: Vec<String>) -> Result<(), Error>;
    /// Receive a request. Will answer with a Vec of Responses.
    fn receive(&mut self, request: &Request) -> Result<Vec<Response>, Error>;
}
