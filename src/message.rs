//! Message used by the Actor model

use serde::de::Error;
use serde_json::{Map, Value};

/// A request from the Maelstrom system
#[derive(Debug, Clone)]
pub struct Request {
    /// Source of the request
    pub source: String,
    /// Destination of the request
    pub destination: String,
    /// Type of the message
    pub message_type: String,
    /// ID of the message
    pub message_id: Option<u64>,
    /// ID of the originated message
    pub in_reply_to: Option<u64>,
    /// Body, composed of JSON values
    pub body: Map<String, Value>,
}

impl Request {
    pub(crate) fn try_from_json(
        json: &mut Map<String, Value>,
    ) -> Result<Request, serde_json::Error> {
        let source = match json.remove("src") {
            None => {
                return Err(serde_json::error::Error::custom("missing field src"));
            }
            Some(Value::String(s)) => s,
            _ => return Err(serde_json::error::Error::custom("bad type for src")),
        };
        let destination = match json.remove("dest") {
            None => {
                return Err(serde_json::error::Error::custom("missing field dest"));
            }
            Some(Value::String(s)) => s,
            _ => return Err(serde_json::error::Error::custom("bad type for dest")),
        };

        let mut body = match json.remove("body") {
            None => {
                return Err(serde_json::error::Error::custom("missing field body"));
            }
            Some(Value::Object(s)) => s,
            _ => return Err(serde_json::error::Error::custom("bad type for body")),
        };

        let message_type = match body.remove("type") {
            None => {
                return Err(serde_json::error::Error::custom("missing field body"));
            }
            Some(Value::String(s)) => s,
            _ => return Err(serde_json::error::Error::custom("bad type for type")),
        };

        let message_id = match body.remove("msg_id") {
            None => None,
            Some(Value::Number(n)) => Some(n.as_u64().unwrap()),
            _ => return Err(serde_json::error::Error::custom("bad type for msg_id")),
        };
        let in_reply_to = match body.remove("in_reply_to") {
            None => None,
            Some(Value::Number(n)) => Some(n.as_u64().unwrap()),
            _ => return Err(serde_json::error::Error::custom("bad type for in_reply_to")),
        };

        Ok(Request {
            source,
            destination,
            message_type,
            message_id,
            in_reply_to,
            body,
        })
    }
}

/// A response to broadcast to Maelstrom
#[derive(Debug, Clone)]
pub struct Response {
    /// Destination
    pub destination: String,
    /// Type of the response
    pub message_type: String,
    /// the response ID
    pub message_id: Option<u64>,
    /// ID of the request
    pub in_reply_to: Option<u64>,
    /// Body of the response, composed of JSON values
    pub body: Map<String, Value>,
}

impl Response {
    /// Create a Response that will be a reply from a Request
    pub fn new_from_request(request: &Request, body: Map<String, Value>) -> Self {
        Response {
            destination: request.source.to_owned(),
            message_type: (request.message_type.to_owned() + "_ok"),
            message_id: request.message_id.map(|u64| u64 + 1),
            in_reply_to: request.message_id,
            body
        }
    }
}
