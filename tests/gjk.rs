#[cfg(test)]
mod tests {
    use lmaths::*;
    use lgeo::*;

    #[test]
    pub fn circles ()
    {
        let gjk = GJK::new();
        let circle_a = Circle::new(Vector2::ZERO, 1.0);
        let circle_b = Circle::new(Vector2::ONE * 1.0, 1.2);

        let r = gjk.intersect(&circle_a, &circle_b);

        match r {
            None => { println!("no collision !")}
            Some(_) => { println!("collision !")}
        }
    }

    #[test]
    fn test_support() {
        println!("Testing Support...");

        let left = AABB::new(Vector2::new(15.0, 0.0), Vector2::new(10.0, 10.0));
        let right = AABB::new(Vector2::new(-15.0, 0.0), Vector2::new(10.0, 10.0));

        let direction = Vector2::new(1., 0.);
        assert_eq!(
            Vector2::new(40., 0.),
            SupportPoint::from_minkowski(
                &left,
                &right,
                direction,
            )
                .v
        );

        println!("Support test passed.")
    }
}