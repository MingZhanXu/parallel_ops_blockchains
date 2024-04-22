#[cfg(test)]
mod tests {
    use point::*;
    #[test]
    fn new_point() {
        let p = Point::new(0.0, 0.1);
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.1);
    }
    #[test]
    fn add_point() {
        let point1 = Point::new(5.0, 6.0);
        let point2 = Point::new(8.0, 3.0);
        let add_point = point1 + point2;
        let ans = Point::new(5.0 + 8.0, 6.0 + 3.0);
        assert_eq!(ans, add_point);
    }
    #[test]
    fn add_point_ref() {
        let point1 = Point::new(5.0, 6.0);
        let point2 = Point::new(8.0, 3.0);
        let add_point = point1 + &point2;
        let ans = Point::new(5.0 + 8.0, 6.0 + 3.0);
        assert_eq!(ans, add_point);
    }
    #[test]
    fn dis_point() {
        let point1 = Point::new(0.0, 0.0);
        let point2 = Point::new(3.0, 4.0);
        let point3 = Point::new(4.0, -3.0);
        let dis_point_1 = point1.dis(&point2);
        let dis_point_2 = point1.dis(&point3);
        let ans = 5.0;
        assert_eq!(ans, dis_point_1);
        assert_eq!(ans, dis_point_2);
    }
    #[test]
    fn average_point_none() {
        let points: Vec<Point> = vec![];
        let ans = "PointError: Points vector is empty".to_string();
        match Point::center_point(&points) {
            Ok(_) => panic!("不該正確: 輸入點群為空"),
            Err(err) => assert_eq!(ans, err.to_string()),
        }
    }
    #[test]
    fn average_point() {
        let point1 = Point::new(1.0, -1.0);
        let point2 = Point::new(3.3, -6.0);
        let point3 = Point::new(5.0, -9.2);
        let total_x = 1.0 + 3.3 + 5.0;
        let total_y = -1.0 + -6.0 + -9.2;
        let ans = Point::new(total_x / 3.0, total_y / 3.0);
        let points: Vec<Point> = vec![point1, point2, point3];
        match Point::center_point(&points) {
            Ok(point) => assert_eq!(ans, point),
            Err(err) => panic!("不該錯誤，錯誤資訊: {}", err),
        }
    }
    #[test]
    fn eq_point() {
        let point1 = Point::new(1.0, -1.0);
        let point2 = Point::new(1.0, -1.0);
        let point_eq = point1 == point2;
        let ans = true;
        assert_eq!(ans, point_eq);
    }
}
