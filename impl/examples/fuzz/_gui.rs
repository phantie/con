use crate::*;
use raylib::prelude::*;

#[allow(dead_code)]
pub fn display() {
    let node_1 = Node { id: 13 };
    let node_2 = Node { id: 42 };
    let node_3 = Node { id: 17 };

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

    print_node(&node_1);
    print_node(&*(*city_3).borrow());

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

    let (wv, wh) = (900, 600);
    let (mut rl, thread) = raylib::init().size(wv, wh).title("Hello, World").build();

    rl.set_target_fps(1);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);

        for route in &routes {
            let targets: &(Rc<RefCell<City>>, Rc<RefCell<City>>) = &route.con.targets;

            let (target_0, target_1) = targets;
            let city_0 = (**target_0).borrow();
            let city_1 = (**target_1).borrow();

            let node_h = 50;
            let node_w = 50;

            let node_r = node_h / 2;

            d.draw_circle(
                city_0.x + node_w / 2,
                city_0.y + node_h / 2,
                node_r as f32,
                Color::WHITE,
            );
            d.draw_circle(
                city_1.x + node_w / 2,
                city_1.y + node_h / 2,
                node_r as f32,
                Color::WHITE,
            );

            d.draw_line(
                city_0.x + (node_w / 2),
                city_0.y + (node_h / 2),
                city_1.x + (node_w / 2),
                city_1.y + (node_h / 2),
                Color::WHITE,
            );

            d.draw_circle(
                (city_0.x + city_1.x + node_w) / 2,
                (city_0.y + city_1.y + node_h) / 2,
                20.0,
                Color::WHITE,
            );

            d.draw_text(
                route.send.to_string().as_str(),
                (city_0.x + city_1.x + node_w - measure_text(route.send.to_string().as_str(), 15))
                    / 2,
                (city_0.y + city_1.y + node_h - measure_text(route.send.to_string().as_str(), 15))
                    / 2,
                15,
                Color::BLACK,
            );

            d.draw_text(
                city_0.bank.to_string().as_str(),
                city_0.x + node_r - (measure_text(city_0.bank.to_string().as_str(), 15) / 2),
                city_0.y + node_r - (15 / 2),
                15,
                Color::BLACK,
            );

            d.draw_text(
                city_1.bank.to_string().as_str(),
                city_1.x + node_r - (measure_text(city_1.bank.to_string().as_str(), 15) / 2),
                city_1.y + node_r - (15 / 2),
                15,
                Color::BLACK,
            );

            d.draw_fps(wv - 100, 30);
        }

        for route in &routes {
            route.transmit();
        }
    }
}
