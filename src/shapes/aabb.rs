use lmaths::*;
use crate::Shape;
use crate::util::*;

#[allow(dead_code)]
pub struct AABB {
    /// Top Left Corner
    pub position:Vector2,
    pub size:Vector2,
    pub corners: [Vector2; 4],
}

#[allow(dead_code)]
impl AABB {

    pub fn new (position:Vector2, size:Vector2) -> Self {
        Self {
            position,
            size,
            corners: Self::generate_corners(size / 2.0)
        }
    }

    fn generate_corners(half_dim:Vector2) -> [Vector2; 4] {
        [
            Vector2::new(half_dim.x, half_dim.y),
            Vector2::new(-half_dim.x, half_dim.y),
            Vector2::new(-half_dim.x, -half_dim.y),
            Vector2::new(half_dim.x, -half_dim.y),
        ]
    }
}

impl Shape for AABB {

    fn position(&self) -> Vector2 {
        self.position
    }

    fn support_point(&self, direction: Vector2) -> Vector2 {
        get_max_point(self.corners.iter(), direction, self.position())
    }
}