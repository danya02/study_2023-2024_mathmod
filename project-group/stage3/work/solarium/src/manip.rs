use std::cmp::Ordering;

use rayon::prelude::*;

use crate::{Particle, Vec2};

const UNIVERSAL_GRAVITATION: f64 = 6.674e-11 * 2e11;

/// all_src_particles is a view of all the particles that exist
/// my_particles is a slice of the particles that I ought to edit
pub fn apply_particle_forces(
    all_src_particles: &[Particle],
    my_particles: &mut [Particle],
    dt: f64,
) {
    for src in my_particles.iter_mut() {
        if src.mass == 0.0 {
            continue;
        }
        let mut force = Vec2::new(0.0, 0.0);
        for dst in all_src_particles.iter() {
            if src == dst {
                continue;
            }
            let extra_force = (dst.position - src.position).normalize()
                * UNIVERSAL_GRAVITATION
                * ((src.mass * dst.mass) / (src.position - dst.position).magnitude());
            force += extra_force;
        }
        src.velocity += force * dt;
        src.position += src.velocity * dt;
    }
}

pub fn run_glue(dst_particles: &mut [Particle]) -> usize {
    let mut glued_particles = 0;
    for idx in 1..dst_particles.len() {
        let (first, rest) = dst_particles.split_at_mut(idx);
        let first = first.last_mut().unwrap();
        for other in rest {
            glued_particles += if other.maybe_glue_to_other(first) {
                1
            } else {
                0
            };
        }
    }
    glued_particles
}

pub fn sort_zeroed(particles: &mut [Particle]) {
    particles.par_sort_unstable_by(|a, b| {
        if a.radius == 0.0 {
            Ordering::Greater
        } else if b.radius == 0.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    })
}

#[no_mangle]
pub extern "C" fn perform_timesteps(
    data: *mut *mut Particle,
    particle_count: usize,
    step_count: usize,
) {
    // data points to an array of pointers, the len of the array is step_count.
    // each of the pointers points to an array of Particles, the len of each of them is particle_count.
    // The arrays are modified in place.
    let particle_ptr_list = unsafe { std::slice::from_raw_parts(data, step_count) };
    let mut particle_slice_list = Vec::with_capacity(step_count);
    for ptr in particle_ptr_list {
        let data_at_ptr = *ptr;
        let slice = unsafe { std::slice::from_raw_parts_mut(data_at_ptr, particle_count) };
        particle_slice_list.push(slice);
    }
    println!("!");

    // For debugging, fill all the rows except the first one with an identifiable value.
    for row in particle_slice_list.iter_mut().skip(1) {
        let m = 420.69;
        let mut i = 0;
        row.fill_with(|| {
            i += 1;
            Particle {
                id: i,
                position: Vec2::new(m, m),
                velocity: Vec2::new(m, m),
                mass: m,
                radius: m,
            }
        });
    }

    let dt = 0.001;
    let mut living_particles = particle_count;
    for src_count in 0..step_count - 1 {
        let [src, dst] = &mut particle_slice_list[src_count..=src_count + 1] else {
            unreachable!()
        };

        dst.copy_from_slice(src);
        if living_particles == 0 {
            continue;
        }

        dst[..living_particles].chunks_mut(10).for_each(|dst| {
            apply_particle_forces(&src[..living_particles], dst, dt);
        });

        let glued_particles = run_glue(dst);
        if glued_particles > 0 {
            sort_zeroed(dst);
            living_particles -= glued_particles;
            println!("step {src_count}: {living_particles} living");
        }
    }
}
