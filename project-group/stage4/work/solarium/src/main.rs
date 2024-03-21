use std::ffi::{CStr, CString};

use solarium::types::InteropData;

fn main() {
    let mut data = [0u8; std::mem::size_of::<InteropData>()];
    let data = data.as_mut_ptr() as *mut InteropData;
    solarium::types::prepare_interop_buf(data);
    solarium::prepare::initialize_particles(data, 20000, 1000.0, 0.05);
    solarium::manip::perform_timesteps(data, 1000, 0.1);
    solarium::prepare::save_to_file(
        data,
        CString::new("1.postcard")
            .unwrap()
            .as_bytes_with_nul()
            .as_ptr() as *const i8,
    );
    solarium::timeseries::save_data_to_timeseries(
        data,
        CString::new("1-series.json")
            .unwrap()
            .as_bytes_with_nul()
            .as_ptr() as *const i8,
    );
}
