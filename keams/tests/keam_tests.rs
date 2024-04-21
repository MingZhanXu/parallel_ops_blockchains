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
    // #[test]
    // fn rand_center_different() {
    //     let centers_len = 4;
    //     let points_len = 50;
    //     let centers = rand_centers(centers_len, points_len);
    //     for (i, c1) in centers.iter().enumerate() {
    //         for (j, c2) in centers.iter().enumerate() {
    //             if i != j {
    //                 assert_ne!(c1, c2);
    //             }
    //         }
    //     }
    // }
    // #[test]
    // fn cluster_range_same_id_zero() {
    //     let len = 10;
    //     let user_id = 0;
    //     let user_max = 3;
    //     let range = cluster_range(len, user_id, user_max)?;
    //     let ans = (0, 2);
    //     assert_eq!(ans, range);
    // }
    // #[test]
    // fn cluster_range_same_id_max() {
    //     let len = 10;
    //     let user_id = 2;
    //     let user_max = 3;
    //     let range = cluster_range(len, user_id, user_max)?;
    //     let ans = (6, 10);
    //     assert_eq!(ans, range);
    // }
    // #[test]
    // #[should_panic(expected = "設定範圍錯誤")]
    // fn cluster_range_panic() {
    //     let len = 10;
    //     let user_id = 3;
    //     let user_max = 3;
    //     cluster_range(len, user_id, user_max);
    // }
}