use std::{
    fmt,
    collections::HashSet,
};

use rand::Rng;
use point::Point;
use point::Error as PointError;

#[derive(Debug)]
pub enum Error {
    KeamsError(String),
}

impl From<PointError> for Error {
    fn from(error: PointError) -> Self {
        Error::KeamsError(error.to_string())
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeamsError(msg) => write!(f, "KeamsError: {}", msg),
        }
    }
}

/// 產生隨機符合範圍的Point
pub fn rand_point(start: f64, end: f64) -> Point {
    let x = rand::thread_rng().gen_range(start..end as f64);
    let y = rand::thread_rng().gen_range(start..end as f64);
    Point::new(x, y)
}
/// 產生多個隨機不重複符合範圍的Point群
pub fn rand_points(
    start: f64,
    end: f64,
    num: usize
) -> Vec<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    while points.len() < num {
        let p = rand_point(start, end);
        points.insert(p);
    }
    points.into_iter().collect()
}
/// 產生隨機不重複的中心點
pub fn rand_centers(
    centers_len: usize,
    points: &Vec<Point>
) -> Vec<Point> {
    let points_len = points.len();
    let mut centers: HashSet<Point> = HashSet::new();
    while centers.len() < centers_len {
        let n = rand::thread_rng().gen_range(0..points_len as usize);
        centers.insert(points[n].clone());
    }
    centers.into_iter().collect()
}
/// 產生符合user_id的運算範圍
pub fn cluster_range(
    len: usize,
    user_id: usize,
    user_max: usize
) -> Result<(usize, usize), Error> {
    let range = len / user_max;
    let start = range * user_id;
    let end;
    if user_id == user_max - 1 {
        end = len;
    } else if user_id < user_max {
        end = start + range - 1;
    } else {
        let err_msg = format!("輸入長度錯誤(user_id: {} >= user_max: {})", user_id, user_max);
        return Err(Error::KeamsError(err_msg));
    }
    Ok((start, end))
}
/// 分群
pub fn cluster<'a>(
    points: &Vec<Point>,
    center_points: &'a Vec<Point>,
    user_id: usize,
    user_max: usize
) -> Result<Vec<Vec<&'a Point>>, Error> {
    let points_len = points.len();
    let centers_len = center_points.len();

    let (start, end) = cluster_range(points_len, user_id, user_max)?;
    let mut teams = vec![vec![]; centers_len];

    for p in points[start..end].iter() {
        let (index, p) = p.min_dis_point(center_points)?;
        teams[index].push(p);
    }
    Ok(teams)
}