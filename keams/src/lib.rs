use std::collections::HashSet;

use rand::Rng;
use point::*;
pub fn rand_point(start: f64, end: f64) -> Point {
    let x = rand::thread_rng().gen_range(start..end as f64);
    let y = rand::thread_rng().gen_range(start..end as f64);
    Point::new(x, y)
}
pub fn rand_points(start: f64, end: f64, num: usize) -> Vec<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    while points.len() < num {
        let p = rand_point(start, end);
        points.insert(p);
    }
    points.into_iter().collect()
}