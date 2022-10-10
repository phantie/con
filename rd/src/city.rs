
use crate::*;

#[derive(Debug)]
pub struct City<'n> {
    pub bank: i32,
    pub node: &'n Node,
}

impl<'n> RefNode for City<'n> {
    fn node_ref(&self) -> &Node {
        &self.node
    }
}

impl<'n> std::fmt::Display for City<'n> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "City({}, bank={})", self.node.id, self.bank)
    }
}


pub fn print_city(city: &Rc<RefCell<City>>) {
    println!("{}", &*(*city).borrow());
}