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

    fn switch_colors(&mut self, other: &mut Self) {
        (self.color, other.color) = (other.color, self.color);
    }

    fn collides_with_other_bouncer(&self, other: &Self) -> bool {
        self.pos.distance_to(other.pos) <= self.r + other.r
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
fn norm_random_velocity(rng: &mut ThreadRng) -> Vector2 {
    let angle: f32 = rng.gen_range(0.0..=1.0) * PI * 2.0;
    Vector2::new(angle.cos(), angle.sin())
}

fn main() {
    let (ww, wh) = (500, 500);
    let mut rng = rand::thread_rng();
    let fps = 60;
    #[allow(unused_variables)]
    let dt = 1f32 / fps as f32;

    #[allow(unused_mut)]
    let mut bouncer_alpha = Bouncer {
        node: &Node { id: 0 },
        pos: Vector2 {
            x: (ww / 2) as f32 - 100.0,
            y: (wh / 2) as f32,
        },
        vel: norm_random_velocity(&mut rng) * 700.0,
        acc: Vector2::zero(),
        r: 60.0,
        color: Color::BLUE,
    };

    #[allow(unused_mut)]
    let mut bouncer_beta = Bouncer {
        node: &Node { id: 0 },
        pos: Vector2 {
            x: (ww / 2) as f32 + 100.0,
            y: (wh / 2) as f32,
        },
        vel: norm_random_velocity(&mut rng) * 700.0,
        acc: Vector2::zero(),
        r: 60.0,
        color: Color::ORANGE,
    };

    let (mut rl, thread) = raylib::init().size(ww, wh).title("Bouncer").build();

    rl.set_target_fps(fps);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(ww - 90, 15);

        bouncer_alpha.draw(&mut d);
        bouncer_alpha.upd_vel(dt);
        bouncer_alpha.upd_pos(dt);
        handle_box_collision(&mut bouncer_alpha, ww, wh);

        bouncer_beta.draw(&mut d);
        bouncer_beta.upd_vel(dt);
        bouncer_beta.upd_pos(dt);
        handle_box_collision(&mut bouncer_beta, ww, wh);

        if bouncer_alpha.collides_with_other_bouncer(&bouncer_beta) {
            bouncer_alpha.vel = -bouncer_alpha.vel;
            bouncer_beta.vel = -bouncer_beta.vel;
            bouncer_alpha.switch_colors(&mut bouncer_beta);
        }
    }
}
