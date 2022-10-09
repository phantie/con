// use raylib::prelude::*;
mod models;

use models::{print_node, step, City, Con, Node, Route};

fn main() {
    let node_1 = Node { id: 13 };
    let node_2 = Node { id: 42 };
    let node_3 = Node { id: 17 };

    let city_1 = City {
        node: &node_1,
        bank: 0,
    };

    let city_2 = City {
        node: &node_2,
        bank: 0,
    };

    let city_3 = City {
        node: &node_3,
        bank: 0,
    };

    print_node(&node_1);
    print_node(&city_1);

    let con_1 = Con {
        targets: (city_1, city_2),
    };

    // let con_2 = Con {
    //     targets: (city_1, city_3),
    // };

    let route_1 = Route { con: con_1, send: 10 };
    // let route_2 = Route { con: con_2, send: 10 };

    let mut routes = vec![route_1];

    dbg!(&routes);
    step(&mut routes);
    dbg!(&routes);
}
