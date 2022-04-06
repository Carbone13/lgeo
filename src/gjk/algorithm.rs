// using
use lmaths::*;
use smallvec::*;
use super::*;
//

#[allow(dead_code)]
const MAX_ITERATIONS: u32 = 100;
#[allow(dead_code)]
const GJK_DISTANCE_TOLERANCE: f64 = 0.000_001;
#[allow(dead_code)]
const GJK_CONTINUOUS_TOLERANCE: f64 = 0.000_001;

#[allow(dead_code)]
pub struct GJK {
    distance_tolerance: f64,
    continuous_tolerance: f64,
    max_iterations: u32,
}

#[allow(dead_code)]
impl GJK {
    pub fn new() -> Self {
        Self {
            distance_tolerance: GJK_DISTANCE_TOLERANCE,
            continuous_tolerance: GJK_CONTINUOUS_TOLERANCE,
            max_iterations: MAX_ITERATIONS,
        }
    }

    pub fn intersect(&self, left: &dyn crate::Shape, right: &dyn crate::Shape) -> Option<Simplex>
    {
        let left_pos = left.position();
        let right_pos = right.position();
        let mut d = right_pos - left_pos;

        if d == Vector2::ZERO {
            d = Vector2::ONE;
        }

        let a = SupportPoint::from_minkowski(left, right, d);
        if a.v.dot(d) <= 0.0 {
            return None;
        }

        let mut simplex = Simplex::new(smallvec![]);
        simplex.push(a);
        d = -d;
        for _ in 0..self.max_iterations {
            let a = SupportPoint::from_minkowski(left, right, d);
            if a.v.dot(d) <= 0.0 {
                return None;
            } else {
                simplex.push(a);
                if simplex.reduce_to_closest_feature(&mut d)
                {
                    return Some(simplex);
                }
            }
        }

        None
    }
}