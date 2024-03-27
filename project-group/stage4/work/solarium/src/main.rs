use std::{collections::HashMap, ffi::CString};

use solarium::types::InteropData;

macro_rules! str {
    ($data:tt) => {
        CString::new($data).unwrap().as_bytes_with_nul().as_ptr() as *const i8
    };
}

fn main() {
    let mut data = [0u8; std::mem::size_of::<InteropData>()];
    let data = data.as_mut_ptr() as *mut InteropData;
    let count: u64 = std::env::args().last().unwrap().parse().unwrap();
    solarium::types::prepare_interop_buf(data);

    if count == 0 {
        solarium::prepare::initialize_particles(data, 1_000, 1000.0, 0.0000000025);
    } else {
        solarium::prepare::load_from_file(
            data,
            CString::new(format!("{}-snapshot.json", count))
                .unwrap()
                .as_bytes_with_nul()
                .as_ptr() as *const i8,
        );
    }
    solarium::manip::perform_timesteps(data, 1000);
    solarium::prepare::save_as_json(
        data,
        CString::new(format!("{}-snapshot.json", count + 1))
            .unwrap()
            .as_bytes_with_nul()
            .as_ptr() as *const i8,
    );
    solarium::timeseries::save_data_to_timeseries(
        data,
        CString::new(format!("{}-series.json", count + 1))
            .unwrap()
            .as_bytes_with_nul()
            .as_ptr() as *const i8,
    );

    let data = unsafe { &mut *data };

    let mut energies: HashMap<_, Vec<_>> = HashMap::new();

    for frame in data.timestep_states.iter() {
        for (particle, energy) in frame.particles.iter().zip(frame.particle_energies.iter()) {
            energies.entry(particle.id).or_default().push(energy)
        }
    }
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}-partenergies.json", count + 1))
        .unwrap();
    let file = std::io::BufWriter::with_capacity(128 * 1024 * 1024, file);
    serde_json::to_writer(file, &energies).unwrap();

    #[derive(serde::Serialize)]
    struct TotalEnergies {
        potential: Vec<f64>,
        kinetic: Vec<f64>,
        active_particles: Vec<usize>,
    }

    let mut total_energies = TotalEnergies {
        potential: vec![],
        kinetic: vec![],
        active_particles: vec![],
    };

    for frame in data.timestep_states.iter() {
        let mut full_pot = 0.0;
        let mut full_kin = 0.0;
        for (_particle, energy) in frame.particles.iter().zip(frame.particle_energies.iter()) {
            full_pot += energy.potential;
            full_kin += energy.kinetic;
        }
        total_energies.potential.push(full_pot);
        total_energies.kinetic.push(full_kin);
        total_energies.active_particles.push(frame.particles.len());
    }

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}-fullenergies.json", count + 1))
        .unwrap();
    let file = std::io::BufWriter::with_capacity(128 * 1024 * 1024, file);
    serde_json::to_writer(file, &total_energies).unwrap();
}
