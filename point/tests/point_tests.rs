#[cfg(test)]
mod tests {
    use std::vec;

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
    fn center_point_points_is_empty() {
        let point = Point::new(0.0, 0.0);
        let points: Vec<Point> = vec![];
        let ans = "PointError: Points vector is empty".to_string();
        match point.min_dis_point(&points) {
            Ok(_) => panic!("不該成功，points裡面沒有任何元素"),
            Err(err) => assert_eq!(ans, err.to_string())
        }
    }
    #[test]
    fn center_point_ok() {
        let points = vec![
            Point::new(0.0, -0.0),
            Point::new(1.0, -1.0),
            Point::new(2.0, -2.0),
            Point::new(3.0, -3.0),
            Point::new(4.0, -4.0),
            Point::new(5.0, -5.0),
            Point::new(6.0, -6.0),
            Point::new(7.0, -7.0),
            Point::new(8.0, -8.0),
            Point::new(9.0, -9.0),
        ];
        let ans = Point::new(45.0/10.0, -45.0/10.0);
        match Point::center_point(&points) {
            Ok(point) => assert_eq!(ans, point),
            Err(err) => panic!("不該發生錯誤: 錯誤訊息:{}", err),
        }
    }
    #[test]
    fn min_dis_point_points_is_empty() {
        let points: Vec<Point> = vec![];
        let point = Point::new(0.0, 0.0);
        let ans = "PointError: Points vector is empty".to_string();
        match point.min_dis_point(&points) {
            Ok(_) => panic!("不該正確，輸入points為空"),
            Err(err) => assert_eq!(ans, err.to_string()),
        }
    }
    #[test]
    fn min_dis_point_ok() {
        let point = Point::new(4.6, -4.6);
        let points = vec![
            Point::new(0.0, -0.0),
            Point::new(1.0, -1.0),
            Point::new(2.0, -2.0),
            Point::new(3.0, -3.0),
            Point::new(4.0, -4.0),
            Point::new(5.0, -5.0),
            Point::new(6.0, -6.0),
            Point::new(7.0, -7.0),
            Point::new(8.0, -8.0),
            Point::new(9.0, -9.0),
        ];
        let ans = (5, &points[5]);
        match point.min_dis_point(&points) {
            Ok(nearest_point) => assert_eq!(ans, nearest_point),
            Err(err) => panic!("不該發生錯誤: 錯誤訊息:{}", err),
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
    #[test]
    fn eq_rhs_point() {
        let point1 = Point::new(1.0, -1.0);
        let point2 = &point1;
        let poiint_eq = point1 == point2;
        let ans = true;
        assert_eq!(ans, poiint_eq);
    }
}
