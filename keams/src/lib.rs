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
        let end = 
            if num < num_max - 1 {
                (start + range) as usize
            } else {
                len
            };
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

#[derive(Debug)]
pub struct Team {
    len: usize,
    data: Vec<Vec<usize>>,
}
impl Team {
    pub fn new(len: usize) -> Team {
        Team {
            len,
            data: vec![vec![]; len],
        }
    }
    pub fn new_set_data(data: Vec<Vec<usize>>) -> Team{
        Team {
            len: data.len(),
            data
        }
    }
    pub fn get(&self, x: usize, y: usize) -> usize {
        self.data[x][y]
    }
    pub fn data(&self) -> &Vec<Vec<usize>> {
        &self.data
    }
    pub fn push(&mut self, index:usize , data: usize) {
        self.data[index].push(data);
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&self, range: std::ops::Range<usize>) -> std::slice::Iter<Vec<usize>>{
        self.data[range].iter()
    }
    
    fn truncate(&mut self) {
        self.data.truncate(self.len());
    }
    pub fn merge(&mut self, others: &Vec<Self>) {
        for other in others {
            for (v1, v2) in self.data.iter_mut().zip(other.data()) {
                v1.extend(v2);
            }
        }
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.len == other.len
    }
}
impl Eq for Team {}
pub struct KeamsTask {
    user_id: usize,
    user_max: usize,
    step: usize,
    points: Vec<Point>,
    points_center: Vec<Option<Point>>,
    center_range: OpsRange,
    points_team: Team,
    team_range: OpsRange,
}
impl KeamsTask {
    pub fn new(
        user_id: usize,
        user_max: usize,
        points: Vec<Point>,
        points_center: Vec<Option<Point>>,
    ) -> Result<KeamsTask, Error> {
        if user_id >= user_max{
            let err_msg = format!("輸入長度錯誤(user_id: {} >= user_max: {})", user_id, user_max);
            return Err(Error::KeamsError(err_msg));
        }
        let center_len = points.len();
        let team_len  = points_center.len();
        let center_range = OpsRange::new(center_len, user_id, user_max)?;
        let team_range = OpsRange::new(team_len, user_id, user_max)?;
        let keams_task = KeamsTask {
            user_id,
            user_max,
            step: 0,
            points,
            center_range,
            points_center,
            team_range,
            points_team: Team::new(team_len),
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
    pub fn points_center(&self) -> &Vec<Option<Point>> {
        &self.points_center
    }
    pub fn points_team(&self) -> &Team {
        &self.points_team
    }
    pub fn step(&self) -> usize {
        self.step
    }
    pub fn center_range(&self) -> &OpsRange {
        &self.center_range
    }
    pub fn team_range(&self) -> &OpsRange {
        &self.team_range
    }

    /// 分群
    pub fn cluster(&mut self) -> Result<(), Error> {
        self.points_team.truncate();
        for (i,p) in self.points[self.center_range.range()].iter().enumerate() {
            let (index, _) = p.min_dis_point(self.points_center())?;
            let data = self.center_range.start() + i;
            self.points_team.push(index, data);
        }
        Ok(())
    }
    /// 計算中心點
    pub fn center(&mut self) -> Result<(), Error> {
        self.points_center.truncate(self.points_center.len());
        let start = self.team_range.start();
        let range = self.team_range.range();
        for (index, points_index) in self.points_team.iter(range).enumerate() {
            let mut center_point = Point::new(0.0, 0.0);
            let len = points_index.len();
            for i in points_index {
                let p = &self.points[*i];
                center_point = center_point + p;
            }
            let center_point = center_point / len as f64;
            self.points_center[start + index] = Some(center_point);
        }
        Ok(())
    }
}
