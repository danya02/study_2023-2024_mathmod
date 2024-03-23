use std::{collections::HashMap, ffi::CStr};

use rayon::prelude::*;

use crate::{
    decimate, timeit,
    types::{InteropData, ParticleTimeSeriesSet, TimeSeriesPoint, TimeSeriesSet},
};

// Use the raw particle data to build a timeseries sequence.
pub fn build_timeseries(data: &InteropData) -> TimeSeriesSet {
    let mut particles = HashMap::new();
    for (frameidx, framedata) in data.timestep_states.iter().enumerate() {
        for particle in framedata.iter() {
            let this_series: &mut ParticleTimeSeriesSet = particles.entry(particle.id).or_default();
            this_series.position_x.push(TimeSeriesPoint {
                x: frameidx,
                y: particle.position.x,
            });
            this_series.position_y.push(TimeSeriesPoint {
                x: frameidx,
                y: particle.position.y,
            });
            // this_series.velocity_x.push(TimeSeriesPoint {
            //     x: frameidx,
            //     y: particle.velocity.x,
            // });
            // this_series.velocity_y.push(TimeSeriesPoint {
            //     x: frameidx,
            //     y: particle.velocity.y,
            // });
            this_series.radius.push(TimeSeriesPoint {
                x: frameidx,
                y: particle.radius,
            });
            // this_series.mass.push(TimeSeriesPoint {
            //     x: frameidx,
            //     y: particle.mass,
            // });
        }
    }
    TimeSeriesSet(particles)
}

fn minify(mut src: TimeSeriesSet) -> TimeSeriesSet {
    let epsilon = 0.1;
    timeit("minify position X", || {
        src.0.par_iter_mut().for_each(|(_id, datas)| {
            datas.position_x = decimate::ramer_douglas_peucker(&datas.position_x, epsilon)
        })
    });
    timeit("minify position Y", || {
        src.0.par_iter_mut().for_each(|(_id, datas)| {
            datas.position_y = decimate::ramer_douglas_peucker(&datas.position_y, epsilon)
        })
    });

    // timeit("minify velocity X", || {
    //     src.0.par_iter_mut().for_each(|(_id, datas)| {
    //         datas.velocity_x = decimate::ramer_douglas_peucker(&datas.velocity_x, epsilon)
    //     })
    // });
    // timeit("minify velocity Y", || {
    //     src.0.par_iter_mut().for_each(|(_id, datas)| {
    //         datas.velocity_y = decimate::ramer_douglas_peucker(&datas.velocity_y, epsilon)
    //     })
    // });

    timeit("minify radius", || {
        src.0.par_iter_mut().for_each(|(_id, datas)| {
            datas.radius = decimate::ramer_douglas_peucker(&datas.radius, epsilon)
        })
    });

    // timeit("minify mass", || {
    //     src.0.par_iter_mut().for_each(|(_id, datas)| {
    //         datas.mass = decimate::ramer_douglas_peucker(&datas.mass, epsilon)
    //     })
    // });

    src
}

#[no_mangle]
pub extern "C" fn save_data_to_timeseries(data: *mut InteropData, dst_path: *const i8) {
    let data = unsafe { &mut *data };

    let dst_path = unsafe { CStr::from_ptr(dst_path) }
        .to_string_lossy()
        .to_string();
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dst_path)
        .unwrap();

    let minified_series = timeit("convert and minify timeseries", move || {
        let series = timeit("convert data to big timeseries", || build_timeseries(data));
        minify(series)
    });
    let mut file = std::io::BufWriter::with_capacity(128 * 1024 * 1024, file);

    timeit("write series to disk", move || {
        serde_json::to_writer(&mut file, &minified_series).unwrap()
    });
}
