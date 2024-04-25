// use std::process::Output;
use std::{
    fmt, 
    hash::{
        Hash,
        Hasher
    }, 
    ops::{
        Add,
        Div
    }
};
#[derive(Debug)]
pub enum Error {
    PointError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PointError(msg) => write!(f, "PointError: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}
impl Point {
    pub fn new(x:f64, y:f64) -> Point {
        Point { x, y }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    /// 兩點差距
    pub fn dis(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dis_squared = dx.powf(2.0) + dy.powf(2.0);
        dis_squared.sqrt()
    }
    /// 計算中心點
    pub fn center_point(points: &Vec<Point>) -> Result<Point, Error> {
        if points.is_empty() {
            let error_msg = "Points vector is empty".to_string();
            return Err(Error::PointError(error_msg));
        }
        let mut sum = Point::new(0.0, 0.0);
        for p in points {
            sum = sum + p;
        }
        let len = points.len() as f64;
        Ok(Point::new(sum.x / len, sum.y / len))
    }
    /// 回傳最近點
    pub fn min_dis_point<'a>(
        &self,
        points: &'a [Point]
    ) -> Result<(usize, &'a Point), Error> {
        if points.is_empty() {
            let error_msg = "Points vector is empty".to_string();
            return Err(Error::PointError(error_msg));
        }
        let mut min_dis = std::f64::MAX;
        let mut nearest_point = (0, &points[0]);
        for (i, c) in points.iter().enumerate() {
            let dis = self.dis(c);
            if min_dis >= dis {
                min_dis = dis;
                nearest_point = (i, c);
            }
        }
        Ok(nearest_point)
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x() + other.x(), self.y() + other.y())
    }
}
impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, other: &Self) -> Self::Output {
        Point::new(self.x() + other.x(), self.y() + other.y())
    }
}
impl Div for Point {
    type Output = Point;
    fn div(self, rhs: Self) -> Self::Output {
        Point::new(self.x() / rhs.x(), self.y() / rhs.y())
    }
}
impl Div<&Point>  for Point {
    type Output = Point;
    fn div(self, rhs: &Self) -> Self::Output {
        Point::new(self.x() / rhs.x(), self.y() / rhs.y())
    }
}
impl Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x() / rhs, self.y() / rhs)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
impl PartialEq<&Point> for Point {
    fn eq(&self, other: &&Self) -> bool {
        self == *other
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}