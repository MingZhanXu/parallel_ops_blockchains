#[cfg(test)]
mod tests {
    use keams::*;
    use point::Point;
    #[test]
    /// 測試是否能產生符合範圍的點
    fn rand_one_point() {
        let simulation = Simulation;
        let start = 0.0;
        let end = 1024.0;
        let point = simulation.rand_point(start, end);
        assert!(point.x() >= start && point.x() <= end);
        assert!(point.y() >= start && point.y() <= end);
    }
    #[test]
    /// 測試隨機點群是否會重複
    fn rand_points_different() {
        let simulation = Simulation;
        let start = 0.0;
        let end = 1024.0;
        let num = 10;
        let point = simulation.rand_points(start, end, num);
        for (i, p1) in point.iter().enumerate() {
            for (j, p2) in point.iter().enumerate() {
                if i != j {
                    assert_ne!(p1, p2);
                }
            }
        }
    }
    #[test]
    /// 測試隨機中心是否會重複
    fn rand_center_different() {
        let simulation = Simulation;
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
        let centers = simulation.rand_centers(centers_len, &points);
        for (i, c1) in centers.iter().enumerate() {
            for (j, c2) in centers.iter().enumerate() {
                if i != j {
                    assert_ne!(c1, c2);
                }
            }
        }
    }

    #[test]
    /// len 為 0 是否會回傳錯誤    
    fn ops_range_len_zero() {
        let len = 0;
        let num = 0;
        let num_max = 1;
        let ans = "KeamsError: 輸入長度錯誤(len == 0)".to_string();
        match OpsRange::new(len, num, num_max) {
            Ok(_) => panic!("不該正確"),
            Err(err) => assert_eq!(ans, err.to_string()),
        }
    }
    #[test]
    /// num_max 為 0 是否會回傳錯誤
    fn ops_range_num_max_zero() {
        let len = 1;
        let num = 0;
        let num_max = 0;
        let ans = "KeamsError: 輸入長度錯誤(num_max == 0)".to_string();
        match OpsRange::new(len, num, num_max) {
            Ok(_) => panic!("不該正確"),
            Err(err) => assert_eq!(ans, err.to_string()),
        }
    }
    #[test]
    /// num == num_max 是否會回傳錯誤
    fn ops_range_num_equal_num_max() {
        let len = 1;
        let num = 3;
        let num_max = 3;
        let ans = "KeamsError: 輸入長度錯誤(num: 3 >= num_max: 3)".to_string();
        match OpsRange::new(len, num, num_max) {
            Ok(_) => panic!("不該正確"),
            Err(err) => assert_eq!(ans, err.to_string()),
        }
    }
    #[test]
    /// num > num_max 是否會回傳錯誤
    fn ops_range_num_greater_num_max() {
        let len = 1;
        let num = 4;
        let num_max = 3;
        let ans = "KeamsError: 輸入長度錯誤(num: 4 >= num_max: 3)".to_string();
        match OpsRange::new(len, num, num_max) {
            Ok(_) => panic!("不該正確"),
            Err(err) => assert_eq!(ans, err.to_string()),
        }
    }
    #[test]
    /// 測試整除範圍回傳是否正確
    fn ops_range_num_max_two() {
        let len = 10;
        let num_0 = 0;
        let num_1 = 1;
        let num_max = 2;
        let ans_start_0 = 0;
        let ans_end_0 = 4;
        let ans_start_1 = 5;
        let ans_end_1 = 9;
        match OpsRange::new(len, num_0, num_max) {
            Ok(range) => {
                assert_eq!(ans_start_0, range.start());
                assert_eq!(ans_end_0, range.end());
            },
            Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
        }
        match OpsRange::new(len, num_1, num_max) {
            Ok(range) => {
                assert_eq!(ans_start_1, range.start());
                assert_eq!(ans_end_1, range.end());
            },
            Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
        }
    }
    #[test]
    /// 測試不整除範圍回傳是否正確
    fn ops_range_three_max_two() {
        let len = 10;
        let num_0 = 0;
        let num_1 = 1;
        let num_2 = 2;
        let num_max = 3;
        let ans_start_0 = 0;
        let ans_end_0 = 2;
        let ans_start_1 = 3;
        let ans_end_1 = 5;
        let ans_start_2 = 6;
        let ans_end_2 = 9;

        match OpsRange::new(len, num_0, num_max) {
            Ok(range) => {
                assert_eq!(ans_start_0, range.start());
                assert_eq!(ans_end_0, range.end());
            },
            Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
        }
        match OpsRange::new(len, num_1, num_max) {
            Ok(range) => {
                assert_eq!(ans_start_1, range.start());
                assert_eq!(ans_end_1, range.end());
            },
            Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
        }
        match OpsRange::new(len, num_2, num_max) {
            Ok(range) => {
                assert_eq!(ans_start_2, range.start());
                assert_eq!(ans_end_2, range.end());
            },
            Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
        }
    }

    #[test]
    /// 測試新建是否成功
    fn new_keams_task() {
        let points = vec![Point::new(0.0, -0.0), Point::new(1.0, -1.0)];
        let points_center = vec![Point::new(0.0, -1.0)];
        match KeamsTask::new(0, 1, points, points_center) {
            Ok(keams_task) => {
                let ans_user_id = 0;
                let ans_user_max = 1;
                let ans_step = 0;
                let ans_points = vec![Point::new(0.0, -0.0), Point::new(1.0, -1.0)];
                let ans_points_center = vec![Point::new(0.0, -1.0)];
                let ans_points_team: Vec<Vec<usize>> = vec![vec![]];
                assert_eq!(ans_user_id, keams_task.user_id());
                assert_eq!(ans_user_max, keams_task.user_max());
                assert_eq!(ans_step, keams_task.step());
                assert_eq!(ans_points, *keams_task.points());
                assert_eq!(ans_points_center, *keams_task.points_center());
                assert_eq!(ans_points_team, *keams_task.points_team());
            },
            Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
        }
        
    }
    
    // #[test]
    // /// 測試分群，最max_user為2時是否回傳正確的資料
    // /// user_id = 0 point 0~9
    // fn cluster_one_user_ok() {
    //     let points = vec![
    //         Point::new(0.0, -0.0),
    //         Point::new(1.0, -1.0),
    //         Point::new(2.0, -2.0),
    //         Point::new(3.0, -3.0),
    //         Point::new(4.0, -4.0),
    //         Point::new(5.0, -5.0),
    //         Point::new(6.0, -6.0),
    //         Point::new(7.0, -7.0),
    //         Point::new(8.0, -8.0),
    //         Point::new(9.0, -9.0),
    //     ];
    //     let center = vec![
    //         Point::new(2.0, -2.0),
    //         Point::new(7.0, -7.0),
    //     ];
    //     let ans = vec![
    //         vec![
    //             &points[0],
    //             &points[1],
    //             &points[2],
    //             &points[3],
    //             &points[4],
    //         ],
    //         vec![
    //             &points[5],
    //             &points[6],
    //             &points[7],
    //             &points[8],
    //             &points[9],
    //             ],
    //     ];
    //     match cluster(&points, &center, 0, 1) {
    //         Ok(team) => assert_eq!(ans, team),
    //         Err(err) => panic!("不該錯誤: {}", err),
    //     }
    // }
    // #[test]
    // /// 測試分群，最max_user為2時是否回傳正確的資料
    // /// user_id = 0 point 0~4
    // /// user_id = 1 point 5~9
    // fn cluster_two_user_ok() {
    //     let points = vec![
    //         Point::new(0.0, -0.0),
    //         Point::new(1.0, -1.0),
    //         Point::new(2.0, -2.0),
    //         Point::new(3.0, -3.0),
    //         Point::new(4.0, -4.0),
    //         Point::new(5.0, -5.0),
    //         Point::new(6.0, -6.0),
    //         Point::new(7.0, -7.0),
    //         Point::new(8.0, -8.0),
    //         Point::new(9.0, -9.0),
    //     ];
    //     let center = vec![
    //         Point::new(2.0, -2.0),
    //         Point::new(7.0, -7.0),
    //     ];
    //     let ans_1 = vec![
    //         vec![
    //             &points[0],
    //             &points[1],
    //             &points[2],
    //             &points[3],
    //             &points[4],
    //         ],
    //         vec![]
    //     ];
    //     let ans_2 = vec![
    //         vec![],
    //         vec![
    //             &points[5],
    //             &points[6],
    //             &points[7],
    //             &points[8],
    //             &points[9],
    //         ],
    //     ];
    //     // user1
    //     match cluster(&points, &center, 0, 2) {
    //         Ok(team) => assert_eq!(ans_1, team),
    //         Err(err) => panic!("不該錯誤: {}", err),
    //     }
    //     // user2
    //     match cluster(&points, &center, 1, 2) {
    //         Ok(team) => assert_eq!(ans_2, team),
    //         Err(err) => panic!("不該錯誤: {}", err),
    //     }
    // }
    // #[test]
    // /// 測試分群，最max_user為3時是否回傳正確的資料
    // /// user_id = 0 point 0~2
    // /// user_id = 1 point 3~5
    // /// user_id = 2 point 6~9
    // fn cluster_three_user_ok() {
    //     let points = vec![
    //         Point::new(0.0, -0.0),
    //         Point::new(1.0, -1.0),
    //         Point::new(2.0, -2.0),
    //         Point::new(3.0, -3.0),
    //         Point::new(4.0, -4.0),
    //         Point::new(5.0, -5.0),
    //         Point::new(6.0, -6.0),
    //         Point::new(7.0, -7.0),
    //         Point::new(8.0, -8.0),
    //         Point::new(9.0, -9.0),
    //     ];
    //     let center = vec![
    //         Point::new(2.0, -2.0),
    //         Point::new(7.0, -7.0),
    //     ];
    //     let ans_1 = vec![
    //         vec![
    //             &points[0],
    //             &points[1],
    //             &points[2],
    //         ],
    //         vec![]
    //     ];
    //     let ans_2 = vec![
    //         vec![
    //             &points[3],
    //             &points[4],
    //         ],
    //         vec![
    //             &points[5],
    //         ],
    //     ];
    //     let ans_3 = vec![
    //         vec![],
    //         vec![
    //             &points[6],
    //             &points[7],
    //             &points[8],
    //             &points[9],
    //         ],
    //     ];
    //     // user1
    //     match cluster(&points, &center, 0, 3) {
    //         Ok(team) => assert_eq!(ans_1, team),
    //         Err(err) => panic!("不該錯誤: {}", err),
    //     }
    //     // user2
    //     match cluster(&points, &center, 1, 3) {
    //         Ok(team) => assert_eq!(ans_2, team),
    //         Err(err) => panic!("不該錯誤: {}", err),
    //     }
    //     // user2
    //     match cluster(&points, &center, 2, 3) {
    //         Ok(team) => assert_eq!(ans_3, team),
    //         Err(err) => panic!("不該錯誤: {}", err),
    //     }
    // }

}