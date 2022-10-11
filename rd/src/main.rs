mod city;
use city::*;

mod route;
use route::*;

mod country;
use country::*;

use connect::*;

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
    let node_4 = Node {
        id: 69,
        x: 500,
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

    let country_1 = Rc::new(RefCell::new(Country {
        node: &node_4,
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

    let city_routes = vec![route_1, route_2];

    let country_city_routes = vec![route_3];

    loop {
        clear_console();
        print_city(&city_1);
        print_city(&city_2);
        print_city(&city_3);
        print_country(&country_1);
        city_routes.iter().for_each(|route| route.transmit());
        country_city_routes
            .iter()
            .for_each(|route| route.transmit());

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // gui::display();
}
