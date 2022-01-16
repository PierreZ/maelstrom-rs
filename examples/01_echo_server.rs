use maelstrom_rs::node::Node;
use maelstrom_rs::runtime::Runtime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaelstromMessage {
    src: String,
    dest: String,
    body: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "init")]
    Init {
        // must be included in every message
        msg_id: Option<u64>,
        in_reply_to: Option<u64>,

        // specific to Init
        node_id: String,
        node_ids: Vec<String>,
    },
    #[serde(rename = "init_ok")]
    InitOk {
        msg_id: Option<u64>,
        in_reply_to: Option<u64>,
    },
    #[serde(rename = "echo")]
    Echo {
        msg_id: Option<u64>,
        in_reply_to: Option<u64>,

        echo: String,
    },
    #[serde(rename = "echo_ok")]
    EchoOk {
        msg_id: Option<u64>,
        in_reply_to: Option<u64>,

        echo: String,
    },
}

struct EchoNode {
    node_id: Option<String>,
}

impl EchoNode {
    pub(crate) fn echo(&self, src: String, dst: String, msg_id: Option<u64>, echo: String) {
        eprintln!("echoing {}", &echo);
        let reply = MaelstromMessage {
            dest: src,
            src: dst,
            body: Message::EchoOk {
                msg_id: msg_id.map(|id| id + 1),
                in_reply_to: msg_id,
                echo,
            },
        };
        let response = serde_json::to_string(&reply).expect("could not serialize");
        eprintln!("responding {:?}", response);
        println!("{}", response)
    }
}

impl EchoNode {
    pub(crate) fn init(&mut self, src: String, dst: String, msg_id: Option<u64>, node_id: String) {
        eprintln!("initialized node {}", &node_id);
        self.node_id = Some(node_id);
        let reply = MaelstromMessage {
            dest: src,
            src: dst,
            body: Message::InitOk {
                msg_id: msg_id.map(|id| id + 1),
                in_reply_to: msg_id,
            },
        };

        let response = serde_json::to_string(&reply).expect("could not serialize");
        eprintln!("responding {:?}", response);
        println!("{}", response)
    }
}

impl Node for EchoNode {
    fn receive(&mut self, message: &str) {
        eprintln!("received an message {}", message);
        let msg: MaelstromMessage = serde_json::from_str(message).expect("could not deserialize");
        match msg.body {
            Message::Init {
                msg_id,
                in_reply_to: _,
                node_id,
                node_ids: _,
            } => self.init(msg.src, msg.dest, msg_id, node_id),
            Message::Echo {
                msg_id,
                in_reply_to: _,
                echo,
            } => self.echo(msg.src, msg.dest, msg_id, echo),
            _ => {}
        }
    }
}

fn main() {
    let node = EchoNode { node_id: None };
    let mut runtime = Runtime::new(Box::new(node));

    runtime.start();
}
