use std::cmp::Ordering;

use rayon::prelude::*;

use crate::{particle::Particle, timeit, types::InteropData, vec2::Vec2, Num};

const UNIVERSAL_GRAVITATION: Num = 6.674e-11 * 2e10;

/// all_src_particles is a view of all the particles that exist
/// my_particles is a slice of the particles that I ought to edit
pub fn apply_particle_forces(
    all_src_particles: &[Particle],
    my_particles: &mut [Particle],
    dt: Num,
) {
    for src in my_particles.iter_mut() {
        let old_src = *src;
        if src.mass == 0.0 {
            continue;
        }
        let mut force = Vec2::new(0.0, 0.0);
        for dst in all_src_particles.iter() {
            if src.id == dst.id {
                continue;
            }
            #[cfg(debug_assertions)]
            if (dst.position - src.position).magnitude() == 0.0 {
                panic!("Distance between {src:?} and {dst:?} is zero");
            }
            let extra_force = (dst.position - src.position).normalize()
                * UNIVERSAL_GRAVITATION
                * ((src.mass * dst.mass) / (src.position - dst.position).magnitude());
            let old_force = force;
            force += extra_force;

            #[cfg(debug_assertions)]
            if !force.is_finite() {
                panic!("Force {force:?} is not finite; was {old_force:?}; extra force is {extra_force:?}");
            }
        }
        src.velocity += force * dt;
        src.position += src.velocity * dt;

        #[cfg(debug_assertions)]
        if !src.is_finite() {
            panic!("{src:?} is not finite inside motion (was {old_src:?})")
        }
    }
}

pub fn run_glue(dst_particles: &mut [Particle]) -> usize {
    let mut glued_particles = 0;
    let mut skip_particle = vec![false; dst_particles.len()];
    for idx in 1..dst_particles.len() {
        if skip_particle[idx - 1] {
            continue;
        }
        let (first, rest) = dst_particles.split_at_mut(idx);
        let first = first.last_mut().unwrap();
        for (other_idx, other) in rest.into_iter().enumerate() {
            glued_particles += if other.maybe_glue_to_other(first) {
                skip_particle[idx + other_idx] = true;
                1
            } else {
                0
            };
        }
    }
    glued_particles
}

pub fn sort_zeroed(particles: &mut [Particle]) {
    // Parallelizable
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
pub extern "C" fn perform_timesteps(data: *mut InteropData, step_count: u64, dt: f64) -> u64 {
    let data = unsafe { &mut *data };

    // current_timestep is the index of the timestep array that's populated;
    // the next one needs to be edited.

    // For the first time only, we'll remove all dead particles from the next step.
    timeit("delete dead particles", || {
        data.timestep_states.push(vec![]);
        let [src, dst] =
            &mut data.timestep_states[data.current_timestep..=data.current_timestep + 1]
        else {
            unreachable!()
        };

        for item in src {
            // Assuming the dead particles are already sorted to the end
            if item.is_zeroed() {
                break;
            }
            dst.push(*item);
        }
        println!("Living particles at start: {}", dst.len());
    });

    for _ in 0..step_count - 1 {
        println!("Current step: {}", data.current_timestep);
        // src is the last completed step
        // Dst is already populated with a copy of src
        // next_dst doesn't exist: we push it here
        data.timestep_states.push(vec![]);
        let [src, dst, next_dst] =
            &mut data.timestep_states[data.current_timestep..=data.current_timestep + 2]
        else {
            unreachable!()
        };

        #[cfg(debug_assertions)]
        for part in dst.iter() {
            if !part.is_finite() {
                panic!("!! {part:?} is not finite at start");
            }
        }

        timeit("glue particles", || {
            let glued_particles = run_glue(&mut dst[0..data.current_living_particles]);
            if glued_particles > 0 {
                sort_zeroed(dst);
                data.current_living_particles -= glued_particles;
                println!(
                    "step {}: {} living",
                    data.current_timestep, data.current_living_particles
                );
            }
        });

        #[cfg(debug_assertions)]
        for part in dst.iter() {
            if !part.is_finite() {
                panic!("!! {part:?} is not finite after gluing");
            }
        }

        timeit("apply particle forces", || {
            // Parallelizable

            dst[0..data.current_living_particles]
                .par_chunks_mut(10)
                .for_each(|dst_chunk| {
                    apply_particle_forces(&src, dst_chunk, dt as Num);
                })
        });

        #[cfg(debug_assertions)]
        for part in dst.iter() {
            if !part.is_finite() {
                panic!("!! {part:?} is not finite after motion");
            }
        }

        // At the end of the operation, we copy the dst into a slot following it.
        data.current_timestep += 1;
        next_dst.clear();
        next_dst.extend_from_slice(&dst);
    }

    data.current_living_particles as u64
}
