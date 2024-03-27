use std::cmp::Ordering;

use rayon::prelude::*;

use crate::{
    particle::Particle,
    timeit,
    types::{InteropData, ParticleEnergy, TimestepState},
    vec2::Vec2,
    Num,
};

fn calc_kinetic_energy(p: &Particle) -> Num {
    let velocity = p.velocity.magnitude() as Num;
    0.5 * p.mass * velocity.powi(2)
}
fn calc_potential_energy(p: &Particle, others: &[Particle], g_const: Num) -> Num {
    let gp = p.mass;
    others
        .par_iter()
        .map(|other| {
            if other.id == p.id || p.is_zeroed() || other.is_zeroed() {
                0.0
            } else {
                gp * g_const * other.mass / (p.position - other.position).magnitude()
            }
        })
        .sum()
}

pub fn calculate_energies(particles: &[Particle], g_const: Num) -> Vec<ParticleEnergy> {
    let mut dst = Vec::with_capacity(particles.len());
    particles
        .par_iter()
        .map(|particle| {
            let kinetic = calc_kinetic_energy(particle);
            let potential = calc_potential_energy(particle, particles, g_const);
            ParticleEnergy { potential, kinetic }
        })
        .collect_into_vec(&mut dst);
    dst
}

/// all_src_particles is a view of all the particles that exist
/// my_particles is a slice of the particles that I ought to edit
pub fn apply_particle_forces(
    all_src_particles: &TimestepState,
    my_particles: &mut [Particle],
    dt: Num,
    g_const: Num,
) {
    for src in my_particles.iter_mut() {
        let old_src = *src;
        if src.mass == 0.0 {
            continue;
        }
        let mut force = Vec2::new(0.0, 0.0);
        for dst in all_src_particles.particles.iter() {
            if src.id == dst.id {
                continue;
            }
            #[cfg(debug_assertions)]
            if (dst.position - src.position).magnitude() == 0.0 {
                panic!("Distance between {src:?} and {dst:?} is zero");
            }
            let force_factor =
                (dst.position - src.position).normalize() * g_const * (src.mass * dst.mass);
            let mag_sq = (src.position - dst.position).magnitude_squared();
            let extra_force = (force_factor / mag_sq) - (force_factor / (mag_sq.powi(4)));

            let old_force = force;
            force += extra_force;

            #[cfg(debug_assertions)]
            if !force.is_finite() {
                panic!("Force {force:?} is not finite; was {old_force:?}; extra force is {extra_force:?}");
            }
        }
        // println!("Force: {force:?}");
        // std::thread::sleep_ms(10);
        let acceleration = force / src.mass;
        src.velocity += acceleration * dt;
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
pub extern "C" fn perform_timesteps(data: *mut InteropData, step_count: u64) -> u64 {
    let data = unsafe { &mut *data };


       let dst = data.latest();
       let kin:f64 = dst.particle_energies.iter().map(|v| v.kinetic).sum();
       let pot:f64 = dst.particle_energies.iter().map(|v| v.potential).sum();

        println!("START: Potential energy: {:?}, kinetic energy: {:?}", pot, kin);
        let _ = dst;


    // current_timestep is the index of the timestep array that's populated;
    // the next one needs to be edited.

    for _ in 0..step_count - 1 {
        // data.current_state is the latest completed state
        // dst is the state we're mutating:
        // at the start it's equal to current_state.
        let src = data.latest();
        let mut dst: TimestepState = data.latest().clone();
        dst.time += data.dt;
        println!();
        println!("Current step: {}", dst.time);

        #[cfg(debug_assertions)]
        for part in dst.particles.iter() {
            if !part.is_finite() {
                panic!("!! {part:?} is not finite at start");
            }
        }

        // Delete all particles that became dead on the last step:
        // that way there'll be a single frame where they're witnessed dead.

        timeit("delete dead particles", || {
            let old = &dst.particles;
            let mut new = Vec::with_capacity(old.len());

            for item in old {
                // Assuming the dead particles are already sorted to the end
                if item.is_zeroed() {
                    break;
                }
                new.push(*item);
            }
            dst.particles = new;
            // Energies aren't updated here: we'll update them separately at the end.
        });

        timeit("apply particle forces", || {
            // Parallelizable

            dst.particles[0..dst.living_particles]
                .par_chunks_mut(10)
                .for_each(|dst_chunk| {
                    apply_particle_forces(
                        &src,
                        dst_chunk,
                        data.dt,
                        data.universal_gravitational_constant,
                    );
                })
        });

        #[cfg(debug_assertions)]
        for part in dst.particles.iter() {
            if !part.is_finite() {
                panic!("!! {part:?} is not finite after motion");
            }
        }

        timeit("glue particles", || {
            let glued_particles = run_glue(&mut dst.particles);
            if glued_particles > 0 {
                sort_zeroed(&mut dst.particles);
                dst.living_particles -= glued_particles;
                println!("\tstep {}: {} living", dst.time, dst.living_particles);
            }
        });

        #[cfg(debug_assertions)]
        for part in dst.particles.iter() {
            if !part.is_finite() {
                panic!("!! {part:?} is not finite after gluing");
            }
        }

        // At the end, update the energy vector.
        timeit("calculate particle energies", || {
            dst.particle_energies =
                calculate_energies(&dst.particles, data.universal_gravitational_constant);
        });

       let kin:f64 = dst.particle_energies.iter().map(|v| v.kinetic).sum();
       let pot:f64 = dst.particle_energies.iter().map(|v| v.potential).sum();

        println!("Potential energy: {:?}, kinetic energy: {:?}", pot, kin);

        data.timestep_states.push(dst);
    }

    data.timestep_states.last().unwrap().living_particles as u64
}
