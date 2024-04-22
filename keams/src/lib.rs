use std::{collections::HashSet};

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

pub fn rand_centers(centers_len: usize, points_len: usize) -> Vec<usize> {
    let mut centers: HashSet<usize> = HashSet::new();
    while centers.len() < centers_len {
        let n = rand::thread_rng().gen_range(0..points_len as usize);
        centers.insert(n);
    }
    centers.into_iter().collect()
}

pub fn cluster_range(len: usize, user_id: usize, user_max: usize) -> (usize, usize) {
    let range = len / user_max;
    let start = range * user_id;
    let end;
    if user_id == user_max - 1 {
        end = len;
    } else if user_id < user_max {
        end = start + range - 1;
    } else {
        panic!("設定範圍錯誤");
    }
    (start, end)
}
fn min_dis(point: &Point, points: &Vec<Point>, centers: &Vec<usize>) -> usize {
    let mut min_dis = std::f64::MAX;
    let mut index = 0;
    for (i, c) in centers.iter().enumerate() {
        let dis = point.dis(&points[*c]);
        if min_dis > dis {
            min_dis = dis;
            index = i;
        }
    }
    index
}
pub fn cluster(points: &Vec<Point>, centers: &Vec<usize>, user_id: usize, user_max: usize) -> Vec<Vec<usize>> {
    let points_len = points.len();
    let centers_len = centers.len();
    let (start, end) = cluster_range(points_len, user_id, user_max);
    let mut teams: Vec<Vec<usize>> = vec![vec![]; centers_len];
    for (i, p) in points[start..end].iter().enumerate() {
        let team_index = min_dis(p, points, centers);
        teams[team_index].push(i + start);
    }
    teams
}