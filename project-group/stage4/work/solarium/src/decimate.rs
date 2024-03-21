use crate::{types::TimeSeriesPoint, Num};

// Returns the distance from point p to the line between p1 and p2
fn perpendicular_distance(p: &TimeSeriesPoint, p1: &TimeSeriesPoint, p2: &TimeSeriesPoint) -> Num {
    let dx = p2.x - p1.x;
    let dx = dx as Num;
    let dy = p2.y - p1.y;
    (p.x as Num * dy - p.y * dx + p2.x as Num * p1.y - p2.y * p1.x as Num).abs() / dx.hypot(dy)
}

fn rdp(points: &[TimeSeriesPoint], epsilon: Num, result: &mut Vec<TimeSeriesPoint>) {
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

pub fn ramer_douglas_peucker(points: &[TimeSeriesPoint], epsilon: Num) -> Vec<TimeSeriesPoint> {
    let mut result = Vec::new();
    if points.len() > 0 && epsilon >= 0.0 {
        result.push(points[0]);
        rdp(points, epsilon, &mut result);
    }
    result
}
