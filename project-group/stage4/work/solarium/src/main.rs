use std::ffi::CString;

use solarium::types::InteropData;

fn main() {
    let mut data = [0u8; std::mem::size_of::<InteropData>()];
    let data = data.as_mut_ptr() as *mut InteropData;
    let count: u64 = std::env::args().last().unwrap().parse().unwrap();
    solarium::types::prepare_interop_buf(data);
    if count == 0 {
        solarium::prepare::initialize_particles(data, 1_000, 1000.0, 0.05);
    } else {
        solarium::prepare::load_from_file(
            data,
            CString::new(format!("{}-snapshot.json", count))
                .unwrap()
                .as_bytes_with_nul()
                .as_ptr() as *const i8,
        );
    }
    solarium::manip::perform_timesteps(data, 100, 0.1);
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
}
