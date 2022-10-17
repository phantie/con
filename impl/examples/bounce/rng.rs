pub use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use raylib::prelude::*;
use std::f32::consts::PI;

pub fn norm_random_velocity(rng: &mut ThreadRng) -> Vector2 {
    // pick random point on unit circle circumference
    let angle: f32 = rng.gen_range(0.0..=1.0) * PI * 2.0;
    Vector2::new(angle.cos(), angle.sin())
}

pub fn pick_random<T>(rng: &mut ThreadRng, value: &[T]) -> T
where
    T: Copy,
{
    *value.choose_multiple(rng, 1).next().unwrap()
}

pub fn shuffle<T>(rng: &mut ThreadRng, v: &mut [T]) {
    v.shuffle(rng);
}
