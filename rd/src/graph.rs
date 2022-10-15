use crate::con::*;
use crate::node::*;
use std::collections::HashSet;


pub fn con_like_to_node_tuple<C, A, B>(con: &C) -> (Node, Node)
where
    C: RefCon<A, B>,
    A: RefNode,
    B: RefNode,
{
    let (t0, t1) = &con.con_ref().targets;
    let t0 = &*(*t0).borrow();
    let t1 = &*(*t1).borrow();
    return (t0.node_ref().clone(), t1.node_ref().clone());
}

pub fn cons_to_edge_set<C, A, B>(cons: &[&C]) -> HashSet<(Node, Node)>
where
    C: RefCon<A, B>,
    A: RefNode,
    B: RefNode,
{
    cons.iter()
        .map(|con| con_like_to_node_tuple(*con))
        .collect::<HashSet<_>>()
}
