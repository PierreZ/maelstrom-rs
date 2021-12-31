use crate::node::{Node};


use std::io::stdin;

pub struct Runtime {
    node: Box<dyn Node>,
}

impl Runtime {
    pub fn new(node: Box<dyn Node>) -> Runtime {
        Runtime { node }
    }

    pub fn start(&mut self) {
        let mut buffer = String::new();
        loop {
            stdin()
                .read_line(&mut buffer)
                .expect("could not read stdin");

            self.node.receive(&buffer);
            buffer.clear();
        }
    }
}
