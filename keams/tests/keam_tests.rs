#[cfg(test)]
mod tests {
    use keams::*;
    use point::Point;
    #[test]
    fn rand_one_point() {
        let start = 0.0;
        let end = 1024.0;
        let point = rand_point(start, end);
        assert!(point.x() >= start && point.x() <= end);
        assert!(point.y() >= start && point.y() <= end);
    }
    #[test]
    fn rand_points_different() {
        let start = 0.0;
        let end = 1024.0;
        let num = 10;
        let point = rand_points(start, end, num);
        for (i, p1) in point.iter().enumerate() {
            for (j, p2) in point.iter().enumerate() {
                if i != j {
                    assert_ne!(p1, p2);
                }
            }
        }
    }
    #[test]
    fn rand_center_different() {
        let centers_len = 4;
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
        let centers = rand_centers(centers_len, &points);
        for (i, c1) in centers.iter().enumerate() {
            for (j, c2) in centers.iter().enumerate() {
                if i != j {
                    assert_ne!(c1, c2);
                }
            }
        }
    }
    #[test]
    fn cluster_range_same_id_zero() {
        let len = 10;
        let user_id = 0;
        let user_max = 3;
        let ans = (0, 2);
        match cluster_range(len, user_id, user_max) {
            Ok(range) => assert_eq!(ans, range),
            Err(err) => panic!("不該錯誤: 錯誤資訊: {}", err),
        };
    }
    #[test]
    fn cluster_range_same_id_max() {
        let len = 10;
        let user_id = 2;
        let user_max = 3;
        let ans = (6, 10);
        match cluster_range(len, user_id, user_max) {
            Ok(range) => assert_eq!(ans, range),
            Err(err) => panic!("不該錯誤: 錯誤資訊: {}", err),
        };
    }
    #[test]
    fn cluster_range_error() {
        let len = 10;
        let user_id = 3;
        let user_max = 3;
        let ans = "Invalid input: 輸入長度錯誤(user_id: 3 >= user_max: 3)".to_string();
        match cluster_range(len, user_id, user_max) {
            Ok(_) => panic!("不該正確: 使用者範圍錯誤"),
            Err(err) => assert_eq!(ans, err.to_string()),
        };
    }
}