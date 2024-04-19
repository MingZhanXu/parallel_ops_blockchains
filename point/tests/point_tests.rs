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
}
