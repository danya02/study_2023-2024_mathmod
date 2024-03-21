use std::ffi::CStr;

use rayon::prelude::*;

use crate::{particle::Particle, types::InteropData, Num};

#[no_mangle]
pub extern "C" fn initialize_particles(
    data: *mut InteropData,
    count: u64,
    radius_max: f64,
    angular_speed: f64,
) {
    let data = unsafe { &mut *data };
    // data.current_state = Vec::with_capacity(count as usize);
    // for id in 0..count {
    //     data.current_state
    //         .push(Particle::make_random(radius_max, angular_speed, id))
    // }

    data.initial_state = (0..count)
        .into_par_iter()
        .map(|id| Particle::make_random(radius_max as Num, angular_speed as Num, id))
        .collect();
    data.current_living_particles = count as usize;
    data.timestep_states.push(data.initial_state.clone());
}

// #[no_mangle]
// pub extern "C" fn alloc_for_raw_timesteps(data: *mut InteropData, count: u64) {
//     let data = unsafe { &mut *data };
//     data.timestep_states = Vec::with_capacity(count as usize);
//     for _ in 0..count {
//         data.timestep_states
//             .push(Vec::with_capacity(data.initial_state.len()))
//     }
//     data.timestep_states[0].extend_from_slice(&data.initial_state);
// }

#[no_mangle]
pub extern "C" fn save_to_file(data: *mut InteropData, dest: *const i8) {
    let data = unsafe { &mut *data };
    let dest = unsafe { CStr::from_ptr(dest) };

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest.to_string_lossy().to_string())
        .unwrap();
    let file = std::io::BufWriter::with_capacity(128 * 1024, file);
    postcard::to_io(&data, file).unwrap();
}

#[no_mangle]
pub extern "C" fn load_from_file(data: *mut InteropData, dest: *const i8) {
    let data = unsafe { &mut *data };
    let dest = unsafe { CStr::from_ptr(dest) };

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(dest.to_string_lossy().to_string())
        .unwrap();
    let file = std::io::BufReader::with_capacity(128 * 1024, file);
    let mut mem = [0; 16386];
    *data = postcard::from_io((file, &mut mem)).unwrap().0;
}

#[no_mangle]
pub extern "C" fn save_as_json(data: *mut InteropData, dest: *const i8) {
    let data = unsafe { &mut *data };
    let dest = unsafe { CStr::from_ptr(dest) };

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest.to_string_lossy().to_string())
        .unwrap();
    let file = std::io::BufWriter::with_capacity(128 * 1024, file);
    serde_json::to_writer(file, data).unwrap();
}
