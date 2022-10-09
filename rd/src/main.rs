// use raylib::prelude::*;
mod models;

use models::{print_node, step, City, Con, Node, Route};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let node_1 = Node { id: 13 };
    let node_2 = Node { id: 42 };
    let node_3 = Node { id: 17 };

    let city_1 = Rc::new(RefCell::new(City {
        node: &node_1,
        bank: 0,
    }));

    let city_2 = Rc::new(RefCell::new(City {
        node: &node_2,
        bank: 0,
    }));

    let city_3 = Rc::new(RefCell::new(City {
        node: &node_3,
        bank: 0,
    }));

    print_node(&node_1);
    print_node(&*city_3.borrow());

    let con_1 = Con {
        targets: (Rc::clone(&city_1), Rc::clone(&city_2)),
    };

    let con_2 = Con {
        targets: (Rc::clone(&city_1), Rc::clone(&city_3)),
    };

    let route_1 = Route {
        con: con_1,
        send: 10,
    };
    let route_2 = Route {
        con: con_2,
        send: 10,
    };

    let mut routes = vec![route_1];

    dbg!(&routes);
    step(&mut routes);
    dbg!(&routes);
}
