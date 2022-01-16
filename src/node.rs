pub trait Node {
    fn receive(&mut self, message: &str);
}
