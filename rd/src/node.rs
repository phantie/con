pub trait RefNode {
    fn node_ref(&self) -> &Node;
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub x: i32,
    pub y: i32,
}

impl RefNode for Node {
    fn node_ref(&self) -> &Node {
        &self
    }
}

pub fn print_node(o: &impl RefNode) {
    let node = o.node_ref();
    dbg!(node);
}
