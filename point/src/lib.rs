// use std::process::Output;
use std::{
    ops::Add,
    hash::{Hash, Hasher},
};
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
    pub fn dis(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dis_squared = dx.powf(2.0) + dy.powf(2.0);
        dis_squared.sqrt()
    }
    pub fn average(points: &Vec<Point>) -> Option<Point> {
        if points.is_empty() {
            return None;
        }
        let mut sum = Point::new(0.0, 0.0);
        for p in points {
            sum = sum + p;
        }
        let len = points.len() as f64;
        Some(Point::new(sum.x / len, sum.y / len))
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}
impl Eq for Point {}