use std::collections::HashMap;

use crate::{manip::calculate_energies, particle::Particle, vec2::Vec2, Num};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[repr(C)]
pub struct InteropData {
    pub universal_gravitational_constant: Num,
    pub dt: Num,
    pub timestep_states: Vec<TimestepState>,
}

impl InteropData {
    pub fn latest(&self) -> &TimestepState {
        &self.timestep_states.last().unwrap()
    }
    pub fn latest_mut(&mut self) -> &mut TimestepState {
        self.timestep_states.last_mut().unwrap()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[repr(C)]
pub struct TimestepState {
    pub time: Num,
    pub particles: Vec<Particle>,
    pub living_particles: usize,
    pub particle_energies: Vec<ParticleEnergy>,
}

impl TimestepState {
    pub fn recalculate_energies(&mut self, g_const: f64) {
        self.particle_energies = calculate_energies(&self.particles, g_const);
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[repr(C)]
pub struct ParticleEnergy {
    pub potential: Num,
    pub kinetic: Num,
    pub momentum: Vec2,
}

#[no_mangle]
pub extern "C" fn get_interop_size() -> usize {
    std::mem::size_of::<InteropData>()
}

#[no_mangle]
pub extern "C" fn prepare_interop_buf(dest: *mut InteropData) {
    unsafe { dest.write(InteropData::default()) }
}

#[no_mangle]
pub extern "C" fn debug_interop_buf(data: *mut InteropData) {
    let data = unsafe { &mut *data };
    dbg!(data);
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct TimeSeriesPoint {
    pub x: usize,
    pub y: Num,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParticleTimeSeriesSet {
    pub position_x: Vec<TimeSeriesPoint>,
    pub position_y: Vec<TimeSeriesPoint>,
    pub radius: Vec<TimeSeriesPoint>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TimeSeriesSet(pub HashMap<u64, ParticleTimeSeriesSet>);
