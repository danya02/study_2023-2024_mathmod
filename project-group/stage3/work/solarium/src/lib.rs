#![feature(get_many_mut)]
pub mod manip;
pub mod types;
pub mod decimate;
use types::*;

#[no_mangle]
pub extern "C" fn hello_ffi(arg: f64) -> f64 {
    println!("Received from Julia: {arg}");
    420.69
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
