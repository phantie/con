#![allow(unused_imports)]

use connect::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use raylib::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use std::vec;
mod colors;
use colors::COLORS;

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

    fn swap_colors(&mut self, other: &mut Self) {
        (self.color, other.color) = (other.color, self.color);
    }

    fn collides_with_other_bouncer(&self, other: &Self) -> bool {
        self.pos.distance_to(other.pos) <= self.r + other.r
    }

    fn handle_collided_bouncers(&mut self, other: &mut Self, c: f32) {
        let tangent_vector = Vector2 {
            x: other.pos.y - self.pos.y,
            y: -(other.pos.x - self.pos.x),
        }
        .normalized();

        let relative_velocity = Vector2 {
            x: other.pos.x - self.pos.x,
            y: other.pos.y - self.pos.y,
        };

        let length = tangent_vector.dot(relative_velocity);

        let velocity_component_on_tangent = tangent_vector.scale_by(length);

        let velocity_component_pependicular_to_tangent =
            relative_velocity - velocity_component_on_tangent;

        self.vel.x -= velocity_component_pependicular_to_tangent.x;
        self.vel.y -= velocity_component_pependicular_to_tangent.y;
        other.vel.x += velocity_component_pependicular_to_tangent.x;
        other.vel.y += velocity_component_pependicular_to_tangent.y;
        // solves problem with slow speed up with time up to infinity
        self.vel.normalize();
        self.vel *= c;
        other.vel.normalize();
        other.vel *= c;
    }

    fn handle_box_collision(&mut self, ww: i32, wh: i32) {
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

        if box_x_collision(self, ww) {
            handle_box_collision_x(self);
        }

        if box_y_collision(self, wh) {
            handle_box_collision_y(self);
        }
    }
}

// random point on circle circumference
fn norm_random_velocity(rng: &mut ThreadRng) -> Vector2 {
    let angle: f32 = rng.gen_range(0.0..=1.0) * PI * 2.0;
    Vector2::new(angle.cos(), angle.sin())
}

fn gen_available_positions(ww: i32, wh: i32, max_r: f32) -> Vec<Vector2> {
    let mut all: Vec<Vector2> = vec![];

    let ww = ww as f32;
    let wh = wh as f32;

    let mut x = max_r;
    let mut y = max_r;

    loop {
        if x + max_r <= ww {
            all.push(Vector2::new(x, y));
            x += 2.0 * max_r;
        } else {
            if y + max_r + 2.0 * max_r <= wh {
                x = max_r;
                y += 2.0 * max_r;
            } else {
                break;
            }
        }
    }

    all
}

fn pick_random<T>(rng: &mut ThreadRng, value: &[T]) -> T
where
    T: Copy,
{
    *value.choose_multiple(rng, 1).next().unwrap()
}

fn main() {
    let (ww, wh) = (700, 700);
    const VEL: f32 = 1.0;
    let fps = 60;
    let bouncer_number: usize = 40;
    let radiuses: Vec<_> = (15..=25).step_by(5).map(|n| n as f32).collect();

    let mut rng = rand::thread_rng();
    let dt = 1f32 / fps as f32;
    let max_r = *radiuses
        .iter()
        .max_by(|f, s| f.partial_cmp(s).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    let mut available_positions = {
        let mut available_positions = gen_available_positions(ww, wh, max_r);
        available_positions.shuffle(&mut rng);
        available_positions.into_iter()
    };

    let mut bouncers = vec![];

    let nodes = (0..bouncer_number)
        .into_iter()
        .map(|id| Node { id: id as u32 })
        .collect::<Vec<_>>();

    for id in 0..bouncer_number {
        #[allow(unused_mut)]
        let bouncer = Bouncer {
            node: &nodes[id],
            pos: available_positions
                .next()
                .expect("Not enough available positions on a plot"),
            vel: norm_random_velocity(&mut rng) * VEL,
            acc: Vector2::zero(),
            r: pick_random(&mut rng, &radiuses),
            color: pick_random(&mut rng, &COLORS),
        };

        bouncers.push(bouncer);
    }

    let (mut rl, thread) = raylib::init().size(ww, wh).title("Bouncer").build();

    rl.set_target_fps(fps);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for bouncer in &bouncers {
            bouncer.draw(&mut d);
        }

        if bouncers.len() == 1 {
            let bouncer = &mut bouncers[0];
            bouncer.handle_box_collision(ww, wh);
            bouncer.upd_pos(dt);
            continue;
        }

        for i in 0..bouncers.len() {
            for j in 0..bouncers.len() {
                if i == j || i > j {
                    continue;
                }

                let (a, b) = bouncers.split_at_mut(i + 1);

                let b1 = &mut a[i];
                let b2 = &mut b[j - i - 1];

                if b1.collides_with_other_bouncer(&b2) {
                    b1.handle_collided_bouncers(b2, VEL);
                    b1.swap_colors(b2);
                }

                b1.handle_box_collision(ww, wh);
                b2.handle_box_collision(ww, wh);
                // // b1.upd_vel(dt);
                // // b1.upd_vel(dt);
                b1.upd_pos(dt);
                b2.upd_pos(dt);
            }
        }
        d.draw_fps(ww - 90, 15);
    }
}
