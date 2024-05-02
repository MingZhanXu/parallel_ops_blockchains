#[cfg(test)]
mod tests {
    use keams::*;
    use point::Point;
    /// 回傳一個 points ， reset 用
    fn points() -> Vec<Point> {
        vec![
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
        ]
    }
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
        let points = points();
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
    /// len < num_max 是否會回傳錯誤    
    fn ops_range_len_smaller_num_max() {
        let len = 3;
        let num = 0;
        let num_max = 4;
        let ans = "KeamsError: 輸入長度錯誤(len: 3 < num_max: 4)".to_string();
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
        let len = 3;
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
        let len = 3;
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
        let num_max = 2;
        let num = [0, 1];
        let ans_start = [0, 5];
        let ans_end = [5, 10];
        for n in num {
            match OpsRange::new(len, n, num_max) {
                Ok(range) => {
                    assert_eq!(ans_start[n], range.start());
                    assert_eq!(ans_end[n], range.end());
                },
                Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
            }
        }
    }
    #[test]
    /// 測試不整除範圍回傳是否正確
    fn ops_range_three_max_two() {
        let len = 10;
        let num_max = 3;
        let num = [0, 1, 2];
        let ans_start = [0, 3, 6];
        let ans_end = [3, 6, 10];
        for n in num {
            match OpsRange::new(len, n, num_max) {
                Ok(range) => {
                    assert_eq!(ans_start[n], range.start());
                    assert_eq!(ans_end[n], range.end());
                },
                Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
            }
        }
    }

    #[test]
    /// 測試 team 合併
    fn merge_team() {
        let data1 = vec![
            vec![0, 1, 2],
            vec![9, 8, 7],
        ];
        let data2 = vec![
            vec![3, 4, 5],
            vec![6, 5, 4],
        ];
        let data3 = vec![
            vec![2, 1, 0],
            vec![7, 8, 9],
        ];
        let mut team1 = Team::new_set_data(data1);
        let team2 = Team::new_set_data(data2);
        let team3 = Team::new_set_data(data3);
        let teams = vec![team2, team3];
        team1.merge(&teams);
        let ans1 = vec![
            vec![0, 1, 2, 3, 4, 5, 2, 1, 0],
            vec![9, 8, 7, 6, 5, 4, 7, 8, 9],
        ];
        assert_eq!(ans1, *team1.data());
    }
    
    #[test]
    /// 測試 center 合併
    fn merge_cneter() {
        let ans = vec![
            Some(Point::new(0.0, -0.0)),
            Some(Point::new(1.0, -1.0)),
            Some(Point::new(2.0, -2.0)),
            Some(Point::new(3.0, -3.0)),
            Some(Point::new(4.0, -4.0)),
            Some(Point::new(5.0, -5.0)),
            Some(Point::new(6.0, -6.0)),
            Some(Point::new(7.0, -7.0)),
            Some(Point::new(8.0, -8.0)),
        ];
        let mut center1 = Center::new(vec![
            ans[0].clone(),
            ans[1].clone(),
            ans[2].clone(),
            None, None, None,
            None, None, None,
        ]);
        let center2 = Center::new(vec![
            None, None, None,
            ans[3].clone(),
            ans[4].clone(),
            ans[5].clone(),
            None, None, None,
        ]);
        let center3 = Center::new(vec![
            None, None, None,
            None, None, None,
            ans[6].clone(),
            ans[7].clone(),
            ans[8].clone(),
        ]);
        match center1.merge(vec![center2, center3]) {
            Ok(()) => {
                assert_eq!(ans, *center1.centers());
            }
            Err(err) => panic!("不該錯誤，錯誤資訊: {}", err),
        }
    }

    #[test]
    /// 測試新建是否成功
    fn new_keams_task() {
        let points = vec![Point::new(0.0, -0.0), Point::new(1.0, -1.0)];
        let points_center = Center::new(vec![Some(Point::new(0.0, -1.0))]);
        let keams_task_result = KeamsTask::new(0, 1, points, points_center);
        match keams_task_result {
            Ok(keams_task) => {
                let ans_user_id = 0;
                let ans_user_max = 1;
                let ans_step = 0;
                let ans_points = vec![Point::new(0.0, -0.0), Point::new(1.0, -1.0)];
                let ans_points_center = Center::new(vec![Some(Point::new(0.0, -1.0))]);
                let ans_points_team = Team::new(1);
                assert_eq!(ans_user_id, keams_task.user_id());
                assert_eq!(ans_user_max, keams_task.user_max());
                assert_eq!(ans_step, keams_task.step());
                assert_eq!(ans_points, *keams_task.points());
                assert_eq!(ans_points_center, *keams_task.center_point());
                assert_eq!(ans_points_team, *keams_task.points_team());
            },
            Err(err) => panic!("不該發生錯誤，錯誤訊息: {}", err),
        }
        
    }
    
    #[test]
    /// 測試分群，最max_user為1時是否回傳正確的資料
    /// user_id = 0 point 0~9
    fn cluster_one_user_ok() {
        let points = points();
        let points_center = Center::new(vec![
            Some(Point::new(2.0, -2.0)),
            Some(Point::new(7.0, -7.0)),
        ]);
        let data = vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
        ];
        let ans = Team::new_set_data(data);
        let user_max = 1;
        let user_id = 0;
        let mut task = 
            KeamsTask::new(user_id, user_max, points, points_center).
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        task.cluster().
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        let team = task.points_team();
        assert_eq!(ans, *team);
    }
    #[test]
    /// 測試分群，最max_user為2時是否回傳正確的資料
    /// 整除測試
    /// user_id = 0~1 point 0~9
    fn cluster_two_user_ok() {
        let user_max = 2;
        let user_id = [0, 1];
        let points = points();
        let points_center = Center::new(vec![
            Some(Point::new(2.0, -2.0)),
            Some(Point::new(7.0, -7.0)),
        ]);
        let data = vec![
            vec![
                vec![0, 1, 2, 3, 4],
                vec![],
            ],
            vec![
                vec![],
                vec![5, 6, 7, 8, 9],
            ]
        ];
        let ans = vec![
            Team::new_set_data(data[0].clone()),
            Team::new_set_data(data[1].clone())
        ];
        for id in user_id {
            let mut task = 
                KeamsTask::new(id, user_max, points.clone(), points_center.clone()).
                unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
            task.cluster().
                unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
            let team = task.points_team();
            assert_eq!(ans[id], *team);
        }
    }
    #[test]
    /// 測試分群，最max_user為3時是否回傳正確的資料
    /// 不整除測試
    /// user_id = 0~2 point 0~9
    fn cluster_three_user_ok() {
        let user_max = 3;
        let user_id = [0, 1, 2];
        let points = points();
        let points_center = Center::new(vec![
            Some(Point::new(2.0, -2.0)),
            Some(Point::new(5.0, -5.0)),
            Some(Point::new(8.0, -8.0)),
        ]);
        let data = vec![
            vec![
                vec![0, 1, 2],
                vec![],
                vec![],
            ],
            vec![
                vec![3],
                vec![4, 5],
                vec![],
            ],
            vec![
                vec![],
                vec![6],
                vec![7, 8, 9],
            ]
        ];
        let ans = vec![
            keams::Team::new_set_data(data[0].clone()),
            keams::Team::new_set_data(data[1].clone()),
            keams::Team::new_set_data(data[2].clone()),
        ];
        for id in user_id {
            let mut task = 
                KeamsTask::new(id, user_max, points.clone(), points_center.clone()).
                unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
            task.cluster().
                unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
            let team = task.points_team();
            assert_eq!(ans[id], *team);
        }
    }
    #[test]
    /// 測試尋找中心點是否正確
    fn cneter_one_user_correct() {
        let user_id = 0;
        let user_max = 1;
        let points = points();
        let points_center = Center::new(vec![Some(Point::new(0.0, 0.0))]);
        let ans = Center::new(vec![Some(Point::new(4.5, -4.5))]);
        let mut task = 
            KeamsTask::new(user_id, user_max, points, points_center).
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        task.cluster().
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        task.center().
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        assert_eq!(ans, *task.center_point());

    }
    #[test]
    fn cneter_one_user_two_center_correct() {
        let user_id = 0;
        let user_max = 1;
        let points = points();
        let points_center = Center::new(vec![
            Some(Point::new(1.0, -1.0)),
            Some(Point::new(4.0, -4.0))
            ]);
        let ans = Center::new(vec![
            Some(Point::new(1.0, -1.0)),
            Some(Point::new(6.0, -6.0))
            ]);
        let mut task = 
        KeamsTask::new(user_id, user_max, points, points_center).
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        task.cluster().
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        task.center().
            unwrap_or_else(|err| panic!("不該發生錯誤，錯誤訊息: {}", err));
        assert_eq!(ans, *task.center_point());
    }

 }