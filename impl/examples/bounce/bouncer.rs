use connect::*;
use raylib::prelude::*;

#[derive(Debug)]
pub struct Bouncer {
    pub node: Node,
    pub pos: Vector2,
    pub vel: Vector2,
    pub acc: Vector2,
    pub r: f32,
    pub color: Color,
}

impl RefNode for Bouncer {
    fn node_ref(&self) -> &Node {
        &self.node
    }
}

impl Bouncer {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.pos.x as i32, self.pos.y as i32, self.r, self.color);
    }

    #[allow(dead_code)]
    pub fn upd_vel(&mut self, dt: f32) {
        self.vel = self.vel + self.acc * dt;
    }

    pub fn upd_pos(&mut self, dt: f32) {
        self.pos = self.pos + self.vel * dt;
    }

    pub fn swap_colors(&mut self, other: &mut Self) {
        (self.color, other.color) = (other.color, self.color);
    }

    pub fn collides_with_other_bouncer(&self, other: &Self) -> bool {
        self.pos.distance_to(other.pos) <= self.r + other.r
    }

    pub fn handle_collided_bouncers(&mut self, other: &mut Self, c: f32) {
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

    pub fn handle_box_collision(&mut self, ww: i32, wh: i32) {
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

    pub fn gen_available_positions(ww: i32, wh: i32, max_r: f32) -> Vec<Vector2> {
        // reserve positions row by row until out of space

        let mut all: Vec<Vector2> = vec![];

        let mut x = max_r;
        let mut y = max_r;

        loop {
            if x + max_r <= ww as f32 {
                all.push(Vector2::new(x, y));
                x += 2.0 * max_r;
            } else {
                if y + max_r + 2.0 * max_r <= wh as f32 {
                    x = max_r;
                    y += 2.0 * max_r;
                } else {
                    break;
                }
            }
        }

        all
    }
}
