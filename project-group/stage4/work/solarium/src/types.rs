use std::collections::HashMap;

use crate::{particle::Particle, Num};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct InteropData {
    pub initial_state: Vec<Particle>,
    pub current_timestep: usize,
    pub current_living_particles: usize,
    pub timestep_states: Vec<Vec<Particle>>,
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
    pub velocity_x: Vec<TimeSeriesPoint>,
    pub velocity_y: Vec<TimeSeriesPoint>,
    pub mass: Vec<TimeSeriesPoint>,
    pub radius: Vec<TimeSeriesPoint>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TimeSeriesSet(pub HashMap<u64, ParticleTimeSeriesSet>);
