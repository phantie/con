#![allow(unused_imports)]

use connect::*;
use rand::rngs::ThreadRng;
use rand::Rng;
use raylib::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

#[derive(Debug)]
pub struct Bouncer<'n> {
    pub node: &'n Node,
    pub pos: Vector2,
    pub vel: Vector2,
    pub acc: Vector2,
    pub r: f32,
    pub color: Color,
}

impl<'n> RefNode for Bouncer<'n> {
    fn node_ref(&self) -> &Node {
        &self.node
    }
}

impl<'n> Bouncer<'n> {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.pos.x as i32, self.pos.y as i32, self.r, self.color);
    }

    #[allow(dead_code)]
    fn upd_vel(&mut self, dt: f32) {
        self.vel = self.vel + self.acc * dt;
    }

    fn upd_pos(&mut self, dt: f32) {
        self.pos = self.pos + self.vel * dt;
    }
}

fn handle_box_collision(b: &mut Bouncer, ww: i32, wh: i32) {
    fn box_x_collision(b: &Bouncer, ww: i32) -> bool {
        b.pos.x - b.r <= 0f32 || b.pos.x + b.r >= ww as f32
    }

    fn box_y_collision(b: &Bouncer, wh: i32) -> bool {
        b.pos.y - b.r <= 0f32 || b.pos.y + b.r >= wh as f32
    }

    fn handle_box_collision_x(b: &mut Bouncer) {
        b.vel.x = -b.vel.x;
    }

    fn handle_box_collision_y(b: &mut Bouncer) {
        b.vel.y = -b.vel.y;
    }

    if box_x_collision(b, ww) {
        handle_box_collision_x(b);
    }

    if box_y_collision(b, wh) {
        handle_box_collision_y(b);
    }
}

// random point on circle circumference
// for smooth velocity regarding both axes
fn norm_random_velocity(rng: &mut ThreadRng) -> Vector2 {
    let angle: f32 = rng.gen_range(0.0..=1.0) * PI * 2.0;
    Vector2::new(angle.cos(), angle.sin())
}

fn main() {
    let (ww, wh) = (900, 600);
    let mut rng = rand::thread_rng();
    let fps = 60;
    #[allow(unused_variables)]
    let dt = 1f32 / fps as f32;

    #[allow(unused_mut)]
    let mut bouncer = Bouncer {
        node: &Node { id: 0 },
        pos: Vector2 {
            x: (ww / 2) as f32,
            y: (wh / 2) as f32,
        },
        vel: norm_random_velocity(&mut rng) * 700.0,
        acc: Vector2::zero(),
        r: 20.0,
        color: Color::WHITE,
    };

    let (mut rl, thread) = raylib::init().size(ww, wh).title("Bouncer").build();

    dbg!(&bouncer.vel);

    rl.set_target_fps(fps);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(ww - 100, 30);

        bouncer.draw(&mut d);
        bouncer.upd_vel(dt);
        bouncer.upd_pos(dt);
        handle_box_collision(&mut bouncer, ww, wh);
    }
}
