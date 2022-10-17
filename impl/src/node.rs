pub trait RefNode {
    fn node_ref(&self) -> &Node;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Node {
    pub id: u32,
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

#[cfg(test)]
mod tests {
    use super::Node;
    use std::collections::HashSet;

    #[test]
    fn test_node_eq_ne() {
        let node_0 = Node { id: 0 };
        let node_1 = Node { id: 1 };
        assert_eq!(&node_0, &node_0);
        assert_ne!(&node_0, &node_1);
    }

    #[test]
    fn test_node_hash() {
        let node_0 = Node { id: 0 };
        let node_1 = Node { id: 1 };

        let mut h = HashSet::new();

        assert_eq!(h.len(), 0);
        h.insert(&node_0);
        assert_eq!(h.len(), 1);
        h.insert(&node_0);
        assert_eq!(h.len(), 1);
        h.insert(&node_1);
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn test_node_tuple_hash() {
        let node_0 = Node { id: 0 };
        let node_1 = Node { id: 1 };
        let node_2 = Node { id: 2 };
        let mut h = HashSet::new();
        assert_eq!(h.len(), 0);
        h.insert((&node_0, &node_1));
        assert_eq!(h.len(), 1);
        h.insert((&node_0, &node_1));
        assert_eq!(h.len(), 1);
        h.insert((&node_1, &node_2));
        assert_eq!(h.len(), 2);
    }
}
