mod models;

use models::*;
use raylib::prelude::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;


fn main() {
    let node_1 = Node { id: 13, x: 100, y: 100 };
    let node_2 = Node { id: 42, x: 200, y: 200 };
    let node_3 = Node { id: 17, x: 300, y: 300 };

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
    print_node(&*(*city_3).borrow());

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

    let mut routes = vec![route_1, route_2];

    dbg!(&routes);
    step(&mut routes);
    dbg!(&routes);

    
    
    let (mut rl, thread) = raylib::init()
    .size(900, 600)
    .title("Hello, World")
    .build();

    rl.set_target_fps(10);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);

        for route in &routes {
            let targets: &(Rc<RefCell<City>>, Rc<RefCell<City>>) = &route.con.targets;

            let (target_0, target_1) = targets;
            let city_0 = (**target_0).borrow();
            let node_0 = city_0.node_ref();
            d.draw_circle(node_1.x, node_0.y, 15.0, Color::WHITE);

            let city_1 = (**target_1).borrow();
            let node_1 = city_1.node_ref();
            d.draw_circle(node_1.x, node_1.y, 15.0, Color::WHITE);

        }   
        
    }
}
