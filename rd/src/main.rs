use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

mod node;
use node::*;

mod city;
use city::*;

mod route;
use route::*;

mod con;
use con::*;

mod gui;

use std::thread;

fn clear_console() {
    print!("{}c", 27 as char);
}

fn main() {
    let node_1 = Node {
        id: 13,
        x: 100,
        y: 100,
    };
    let node_2 = Node {
        id: 42,
        x: 300,
        y: 100,
    };
    let node_3 = Node {
        id: 17,
        x: 300,
        y: 400,
    };

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

    // print_node(&node_1);
    // print_node(&*(*city_3).borrow());

    let con_1 = Con {
        targets: (Rc::clone(&city_1), Rc::clone(&city_2)),
    };

    let con_2 = Con {
        targets: (Rc::clone(&city_1), Rc::clone(&city_3)),
    };

    let route_1 = Route {
        con: &con_1,
        send: 10,
    };
    let route_2 = Route {
        con: &con_2,
        send: 20,
    };

    let routes = vec![route_1, route_2];


    loop {
        clear_console();
        print_city(&city_1);
        print_city(&city_2);
        print_city(&city_3);
        route::step_all(&routes);
        thread::sleep(Duration::from_millis(500));
    }

    // gui::display();
}
