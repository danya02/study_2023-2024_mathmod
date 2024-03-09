use rayon::prelude::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TimeSeriesPoint {
    x: u64,
    y: f64,
}

// Returns the distance from point p to the line between p1 and p2
fn perpendicular_distance(p: &TimeSeriesPoint, p1: &TimeSeriesPoint, p2: &TimeSeriesPoint) -> f64 {
    let dx = p2.x - p1.x;
    let dx = dx as f64;
    let dy = p2.y - p1.y;
    (p.x as f64 * dy - p.y * dx + p2.x as f64 * p1.y - p2.y * p1.x as f64).abs() / dx.hypot(dy)
}

fn rdp(points: &[TimeSeriesPoint], epsilon: f64, result: &mut Vec<TimeSeriesPoint>) {
    let n = points.len();
    if n < 2 {
        return;
    }
    let mut max_dist = 0.0;
    let mut index = 0;
    for i in 1..n - 1 {
        let dist = perpendicular_distance(&points[i], &points[0], &points[n - 1]);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }
    if max_dist > epsilon {
        rdp(&points[0..=index], epsilon, result);
        rdp(&points[index..n], epsilon, result);
    } else {
        result.push(points[n - 1]);
    }
}

fn ramer_douglas_peucker(points: &[TimeSeriesPoint], epsilon: f64) -> Vec<TimeSeriesPoint> {
    let mut result = Vec::new();
    if points.len() > 0 && epsilon >= 0.0 {
        result.push(points[0]);
        rdp(points, epsilon, &mut result);
    }
    result
}

#[no_mangle]
pub extern "C" fn decimate_timeseries(
    src: *const TimeSeriesPoint,
    len: usize,
    eps: f64,
    result_len: *mut usize,
) -> *mut TimeSeriesPoint {
    // Make a slice out of the source timeseries
    let src_slice = unsafe { std::slice::from_raw_parts(src, len) };
    let mut dst = ramer_douglas_peucker(src_slice, eps);
    dst.shrink_to_fit();
    assert_eq!(dst.len(), dst.capacity());
    let ptr = dst.as_mut_ptr();
    unsafe {
        *result_len = dst.len();
    }
    std::mem::forget(dst);
    ptr
}

#[repr(C)]
pub struct ParticleSeries {
    particle_id: usize,
    series_data: *mut TimeSeriesPoint,
}

#[repr(C)]
pub struct DecimationData {
    pub number_of_series: usize,
    pub lengths_of_series: *mut usize,
    pub source_series_list: *const ParticleSeries,
    pub out_series_list: *mut ParticleSeries,
}

#[no_mangle]
pub extern "C" fn decimate_many_timeseries(data: DecimationData, eps: f64) {}
