// using
use lmaths::*;
use crate::Shape;
//

#[derive(Clone, Debug, Copy)]
pub struct SupportPoint {
    pub v: Vector2,
    pub sup_a: Vector2,
    pub sup_b: Vector2,
}

#[allow(dead_code)]
impl SupportPoint {

    pub fn new() -> Self {
        Self {
            v: Vector2::ZERO,
            sup_a: Vector2::ZERO,
            sup_b: Vector2::ZERO,
        }
    }

    pub fn from_minkowski (left:&dyn Shape, right:&dyn Shape, direction:Vector2) -> Self {
        let l = left.support_point(direction);
        let r = right.support_point(-direction);

        Self {
            v: (l - r),
            sup_a: l,
            sup_b: r,
        }
    }
}

impl std::ops::Sub<Vector2> for SupportPoint {
    type Output = Self;

    fn sub(self, rhs:Vector2) -> Self {
        SupportPoint {
            v: self.v - rhs,
            sup_a: self.sup_a,
            sup_b: self.sup_b,
        }
    }
}
