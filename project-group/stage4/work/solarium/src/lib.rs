#![feature(get_many_mut)]
#![allow(improper_ctypes_definitions)] // beware the man who speaks in void*s

use std::time::SystemTime;

pub mod decimate;
pub mod manip;
pub mod particle;
pub mod prepare;
pub mod timeseries;
pub mod types;
pub mod vec2;

pub type Num = f64;

#[no_mangle]
pub extern "C" fn hello_ffi(arg: f64) -> f64 {
    println!("Received from Julia: {arg}");
    420.69
}

pub fn timeit<F: FnMut() -> T, T>(name: &'static str, mut f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("{name} took {}s", duration.as_secs_f64());
    result
}

// #[no_mangle]
// pub extern "C" fn main() {
//     let mut particles = vec![];
//     for _ in 0..1000 {
//         particles.push(Particle::make_random(10.0, 10.0));
//     }

//     let dt = 0.1;
//     for _ in 0..100 {
//         apply_particle_forces(&mut particles, dt);
//         step_particles(&mut particles, dt);
//         println!("{:?}", particles[0]);
//     }
// }
