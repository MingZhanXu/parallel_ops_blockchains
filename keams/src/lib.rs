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

pub struct Simulation;
impl Simulation {
    /// 產生隨機符合範圍的Point
    pub fn rand_point(
        &self,
        start: f64,
        end: f64
    ) -> Point {
        let x = rand::thread_rng().gen_range(start..end as f64);
        let y = rand::thread_rng().gen_range(start..end as f64);
        Point::new(x, y)
    }
    /// 產生多個隨機不重複符合範圍的Point群
    pub fn rand_points(
        &self,
        start: f64,
        end: f64,
        num: usize
    ) -> Vec<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        while points.len() < num {
            let p = self.rand_point(start, end);
            points.insert(p);
        }
        points.into_iter().collect()
    }
    /// 產生隨機不重複的中心點
    pub fn rand_centers(
        &self,
        centers_len: usize,
        points: &[Point]
    ) -> Vec<Point> {
        let points_len = points.len();
        let mut centers: HashSet<Point> = HashSet::new();
        while centers.len() < centers_len {
            let n = rand::thread_rng().gen_range(0..points_len as usize);
            centers.insert(points[n].clone());
        }
        centers.into_iter().collect()
    }
}

pub struct OpsRange {
    start: usize,
    end: usize,
}
impl OpsRange {
    pub fn new(
        len: usize,
        num: usize,
        num_max: usize
    ) -> Result<OpsRange, Error> {
        // 資料為空
        if len == 0 {
            let err_msg = format!("輸入長度錯誤(len == 0)");
            return Err(Error::KeamsError(err_msg));
        }
        // 不可分配
        else if len < num_max {
            let err_msg = format!("輸入長度錯誤(len: {} < num_max: {})", len, num_max);
            return Err(Error::KeamsError(err_msg));
        }
        // 除以零
        else if num_max == 0 {
            let err_msg = format!("輸入長度錯誤(num_max == 0)");
            return Err(Error::KeamsError(err_msg));
        }
        // index 比總量多
        else if num >=  num_max {
            let err_msg = format!("輸入長度錯誤(num: {} >= num_max: {})", num, num_max);
            return Err(Error::KeamsError(err_msg));
        }
        let range = len as f64 / num_max as f64;
        let start = range * num as f64;
        let end = (start + range) as usize;
        let start = start as usize;
        Ok(OpsRange{start, end})
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

pub struct KeamsTask {
    user_id: usize,
    user_max: usize,
    step: usize,
    points: Vec<Point>,
    points_center: Vec<Point>,
    center_ops_range: OpsRange,
    points_team: Vec<Vec<usize>>,
    team_ops_range: OpsRange,
}
impl KeamsTask {
    pub fn new(
        user_id: usize,
        user_max: usize,
        points: Vec<Point>,
        points_center: Vec<Point>,
    ) -> Result<KeamsTask, Error> {
        if user_id >= user_max{
            let err_msg = format!("輸入長度錯誤(user_id: {} >= user_max: {})", user_id, user_max);
            return Err(Error::KeamsError(err_msg));
        }
        let center_len = points.len();
        let team_len  = points_center.len();
        let center_ops_range = OpsRange::new(center_len, user_id, user_max)?;
        let team_ops_range = OpsRange::new(team_len, user_id, user_max)?;
        let keams_task = KeamsTask {
            user_id,
            user_max,
            step: 0,
            points,
            center_ops_range,
            points_center,
            team_ops_range,
            points_team: vec![vec![]; team_len],
        };
        Ok(keams_task)
    }
    pub fn user_id(&self) -> usize {
        self.user_id
    }
    pub fn user_max(&self) -> usize {
        self.user_max
    }
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
    pub fn points_center(&self) -> &Vec<Point> {
        &self.points_center
    }
    pub fn points_team(&self) -> &Vec<Vec<usize>> {
        &self.points_team
    }
    pub fn step(&self) -> usize {
        self.step
    }
    pub fn center_ops_range(&self) -> &OpsRange {
        &self.center_ops_range
    }
    pub fn team_ops_range(&self) -> &OpsRange {
        &self.team_ops_range
    }

    /// 分群
    pub fn cluster(&mut self) -> Result<(), Error> {
        self.points_team = vec![vec![]; self.points_center.len()];
        for (i,p) in self.points[self.center_ops_range.range()].iter().enumerate() {
            let (index, _) = p.min_dis_point(self.points_center())?;
            self.points_team[index].push(self.center_ops_range.start() + i);
        }
        Ok(())
    }
    /// 計算中心點
    pub fn center(&mut self) -> Result<(), Error> {
        self.points_center = vec![];
        for (index, points_index) in self.points_team[self.team_ops_range.range()].iter().enumerate() {
            let mut center_point = Point::new(0.0, 0.0);
            let len = points_index.len();
            for i in points_index {
                let p = &self.points[*i];
                center_point = center_point + p;
            }
            let center_point = center_point / len as f64;
            self.points_center[index] = center_point;
        }
        Ok(())
    }
}

// /// 計算新中心點群
// pub fn center_points(
//     team_points: &[Vec<Point>],
//     user_id: usize,
//     user_max: usize
// ) -> Result<Vec<Point>, Error> {
//     let centers_len = team_points.len();
//     let (start, end) = user_range(centers_len, user_id, user_max)?;
//     let mut center_points = Vec::with_capacity(centers_len);
//     for i in start..=end {
//         center_points[i] = Point::center_point(&team_points[i])?;
//     }
//     Ok(center_points)
// }