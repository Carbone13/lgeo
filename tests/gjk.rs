#[cfg(test)]
mod tests {
    use lmaths::*;
    use lgeo::*;

    #[test]
    fn circle_circle () {
        let gjk = GJK::new();

        let a = Circle::new(Vector2::new(4.6, -3.2), 1.43);
        let b = Circle::new(Vector2::new(0.0, 0.0), 1.0);
        let c = Circle::new(Vector2::new(-1.0, 1.0), 7.4567);

        assert!(gjk.intersect(&a, &b).is_none());
        assert!(gjk.intersect(&a, &c).is_some());
        assert!(gjk.intersect(&b, &c).is_some());

        let c = Circle::new(Vector2::new(4.6, -3.2), 1.43);
        let r = AABB::new(Vector2::new(-1.0, 0.5), Vector2::new(2.0, 1.0));
        let p = Polygon::new(Vector2::new(0.0, 0.0),
                             vec![Vector2::new(0.0, 0.5),
                                  Vector2::new(-1.5, -0.5),
                                  Vector2::new(-3.0, 0.5),
                             ]);
    }

    #[test]
    fn circle_polygon () {
    }

    #[test]
    fn polygon_polygon () {
        let gjk = GJK::new();

        let p_a = Polygon::new(Vector2::new(0.0, 0.0),
                               vec![Vector2::new(0.0, 0.5),
                                    Vector2::new(-1.5, -0.5),
                                    Vector2::new(-3.0, 0.5),
                               ]);
        let p_b = Polygon::new(Vector2::new(-1.0154, -0.85),
                               vec![Vector2::new(0.0, 0.0),
                                    Vector2::new(2.5, -0.5),
                                    Vector2::new(1.5, 3.5),
                                    Vector2::new(0.5, 3.0),
                                    Vector2::new(0.0, 2.5)
                               ]);

        let p_c = Polygon::new(Vector2::new(1.6, 2.1),
                               vec![Vector2::new(-0.5, 0.0),
                                    Vector2::new(1.0, 0.0),
                                    Vector2::new(2.0, -1.0),
                                    Vector2::new(2.5, -2.0),
                               ]);

        assert!(gjk.intersect(&p_a, &p_b).is_some());
        assert!(gjk.intersect(&p_a, &p_c).is_none());
        assert!(gjk.intersect(&p_b, &p_c).is_none());
    }

    #[test]
    // only one test with AABBs, because they are just special polygon, so no real point on testing them
    fn circle_aabb () {
        let gjk = GJK::new();

        let c_a = Circle::new(Vector2::new(-2.0, -1.0), 1.2);
        let c_b = Circle::new(Vector2::new(2.0, 1.0), 1.0);

        let r_a = AABB::new(Vector2::new(-1.0, 0.5), Vector2::new(2.0, 1.0));

        assert!(gjk.intersect(&c_a, &r_a).is_some());
        assert!(gjk.intersect(&c_b, &r_a).is_none());
    }
}