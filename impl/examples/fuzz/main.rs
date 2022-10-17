use connect::*;
use std::cell::RefCell;
use std::rc::Rc;

mod _gui;

#[derive(Debug)]
pub struct City<'n> {
    pub bank: i32,
    pub node: &'n Node,
    pub x: i32,
    pub y: i32,
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

#[derive(Debug)]
pub struct Country<'n> {
    pub bank: i32,
    pub node: &'n Node,
    pub x: i32,
    pub y: i32,
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

#[derive(Debug)]
pub struct Route<'con, A, B>
where
    A: RefNode,
    B: RefNode,
{
    pub con: &'con Con<A, B>,
    pub send: u32,
}

impl<'con, 'city> Transmit for Route<'con, City<'city>, City<'city>> {
    fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, 'city, 'country> Transmit for Route<'con, City<'city>, Country<'country>> {
    fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, 'city, 'country> Transmit for Route<'con, Country<'country>, City<'city>> {
    fn transmit(&self) {
        self.con.targets.0.borrow_mut().bank -= self.send as i32;
        self.con.targets.1.borrow_mut().bank += self.send as i32;
    }
}

impl<'con, A, B> RefCon<A, B> for Route<'con, A, B>
where
    A: RefNode,
    B: RefNode,
{
    fn con_ref(&self) -> &'con Con<A, B> {
        &self.con
    }
}

pub fn main() {
    let node_1 = Node { id: 13 };
    let node_2 = Node { id: 42 };
    let node_3 = Node { id: 17 };
    let node_4 = Node { id: 69 };

    let city_1 = Rc::new(RefCell::new(City {
        node: &node_1,
        bank: 0,
        x: 100,
        y: 100,
    }));

    let city_2 = Rc::new(RefCell::new(City {
        node: &node_2,
        bank: 0,
        x: 300,
        y: 100,
    }));

    let city_3 = Rc::new(RefCell::new(City {
        node: &node_3,
        bank: 0,
        x: 300,
        y: 400,
    }));

    let country_1 = Rc::new(RefCell::new(Country {
        node: &node_4,
        bank: 0,
        x: 500,
        y: 400,
    }));
    // print_node(&node_1);
    // print_node(&*(*city_3).borrow());

    let con_1 = Con {
        targets: (Rc::clone(&city_1), Rc::clone(&city_2)),
    };

    let con_2 = Con {
        targets: (Rc::clone(&city_1), Rc::clone(&city_3)),
    };

    let con_3 = Con {
        targets: (Rc::clone(&country_1), Rc::clone(&city_1)),
    };

    let route_1 = Route {
        con: &con_1,
        send: 10,
    };
    let route_2 = Route {
        con: &con_2,
        send: 20,
    };
    let route_3 = Route {
        con: &con_3,
        send: 50,
    };

    let _ = con_like_to_node_tuple(&route_1);

    let city_routes = [&route_1, &route_2];
    let _ = cons_to_edge_set(&city_routes);
    let _ = cons_to_edge_set(&[&con_2, &con_1]);

    let country_city_routes = [&route_3];

    loop {
        clear_console();
        print_city(&city_1);
        print_city(&city_2);
        print_city(&city_3);
        print_country(&country_1);
        city_routes.into_iter().for_each(|route| route.transmit());
        country_city_routes
            .into_iter()
            .for_each(|route| route.transmit());

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // gui::display();
}
