use serde::de::Error;
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct Request {
    pub source: String,
    pub destination: String,
    pub message_type: String,
    pub message_id: Option<u64>,
    pub in_reply_to: Option<u64>,
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

pub struct Response {
    pub message_type: String,
    pub message_id: Option<u64>,
    pub in_reply_to: Option<u64>,
    pub body: Map<String, Value>,
}
