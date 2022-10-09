pub trait RefNode {
    fn node_ref(&self) -> &Node;
}

#[derive(Debug)]
pub struct Node {
    pub id: u32,
}

impl RefNode for Node {
    fn node_ref(&self) -> &Node {
        &self
    }
}

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
        write!(f, "City({}, bank={}", self.node.id, self.bank)
    }
}

pub fn print_node(o: &impl RefNode) {
    let node = o.node_ref();
    dbg!(node);
}

pub trait RefCon<T>
where
    T: RefNode,
{
    fn con_ref(&self) -> &Con<T>;
}

#[derive(Debug)]
pub struct Con<T>
where
    T: RefNode,
{
    pub targets: (T, T),
}

impl<T> RefCon<T> for Con<T>
where
    T: RefNode,
{
    fn con_ref(&self) -> &Con<T> {
        &self
    }
}

#[derive(Debug)]
pub struct Route<T>
where
    T: RefNode,
{
    pub con: Con<T>,
    pub send: i32,
}

impl<T> RefCon<T> for Route<T>
where
    T: RefNode,
{
    fn con_ref(&self) -> &Con<T> {
        &self.con
    }
}

pub fn step(routes: &mut Vec<Route<City>>) {
    for route in routes.iter_mut() {
        let send = route.send;

        route.con.targets.0.bank -= send;
        route.con.targets.1.bank += send;
    }
}
