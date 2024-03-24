use std::collections::HashMap;

use crate::{particle::Particle, Num};
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
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[repr(C)]
pub struct TimestepState {
    pub time: Num,
    pub particles: Vec<Particle>,
    pub living_particles: usize,
    pub particle_energies: Vec<ParticleEnergy>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[repr(C)]
pub struct ParticleEnergy {
    pub potential: Num,
    pub kinetic: Num,
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
