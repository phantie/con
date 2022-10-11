
use crate::*;

#[derive(Debug)]
pub struct Country<'n> {
    pub bank: i32,
    pub node: &'n Node,
}

impl<'n> RefNode for Country<'n> {
    fn node_ref(&self) -> &Node {
        &self.node
    }
}
