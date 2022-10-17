use connect::*;
use raylib::prelude::*;
mod bouncer;
use bouncer::Bouncer;
mod opt;
use opt::*;
mod rng;
mod colors;
use colors::COLORS;

fn main() {
    let opt = opt::Opt::from_args();

    let ww: i32 = opt.ww.into();
    let wh: i32 = opt.wh.into();
    let vel: f32 = opt.velocity;
    let fps: u32 = opt.fps.into();
    let bouncer_number: usize = opt.node_number.into();

    let radiuses: Vec<f32> = (15..=25).step_by(5).map(|n| n as f32).collect();

    let mut rng = rand::thread_rng();
    let dt: f32 = 1f32 / fps as f32;
    let max_r = *radiuses
        .iter()
        .max_by(|f, s| f.partial_cmp(s).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    let mut available_positions = {
        let mut available_positions = Bouncer::gen_available_positions(ww, wh, max_r);
        rng::shuffle(&mut rng, &mut available_positions);
        available_positions.into_iter()
    };

    let mut bouncers = vec![];

    let nodes = (0..bouncer_number)
        .into_iter()
        .map(|id| Node { id: id as u32 })
        .collect::<Vec<_>>();

    for id in 0..bouncer_number {
        let bouncer = Bouncer {
            node: &nodes[id],
            pos: available_positions
                .next()
                .expect("Not enough available positions on a plot"),
            vel: rng::norm_random_velocity(&mut rng) * vel,
            acc: Vector2::zero(),
            r: rng::pick_random(&mut rng, &radiuses),
            color: rng::pick_random(&mut rng, &COLORS),
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

        d.draw_fps(ww - 90, 15);

        if bouncers.len() == 1 {
            let bouncer = &mut bouncers[0];
            bouncer.handle_box_collision(ww, wh);
            bouncer.upd_pos(dt);
        } else {
            for i in 0..bouncers.len() {
                for j in 0..bouncers.len() {
                    if i == j || i > j {
                        continue;
                    }

                    let (a, b) = bouncers.split_at_mut(i + 1);

                    let b1 = &mut a[i];
                    let b2 = &mut b[j - i - 1];

                    if b1.collides_with_other_bouncer(&b2) {
                        b1.handle_collided_bouncers(b2, vel);
                        b1.swap_colors(b2);
                    }

                    b1.handle_box_collision(ww, wh);
                    b2.handle_box_collision(ww, wh);

                    b1.upd_pos(dt);
                    b2.upd_pos(dt);
                }
            }
        }
    }
}
