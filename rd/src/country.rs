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

impl<'n> std::fmt::Display for Country<'n> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Country({}, bank={})", self.node.id, self.bank)
    }
}

pub fn print_country(country: &Rc<RefCell<Country>>) {
    println!("{}", &*(*country).borrow());
}
