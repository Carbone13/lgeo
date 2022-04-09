use std::fmt::{Display, Formatter};
use lmaths::*;
use crate::Shape;
use crate::util::*;

/// Represent a Rectangle shape
pub struct AABB {
    /// Top Left Corner
    pub position:Vector2,
    pub size:Vector2,

    /// The four corners of the rect
    corners: [Vector2; 4],
}

impl AABB {
    /// Create a new rectangle at the specified position, with the specified size
    pub fn new (position:Vector2, size:Vector2) -> Self {
        Self {
            position,
            size,
            corners: Self::generate_corners(size / 2.0)
        }
    }

    /// Generate the position of the four corners, to cache them
    fn generate_corners(half_dim:Vector2) -> [Vector2; 4] {
        [
            Vector2::new(half_dim.x, half_dim.y),
            Vector2::new(-half_dim.x, half_dim.y),
            Vector2::new(-half_dim.x, -half_dim.y),
            Vector2::new(half_dim.x, -half_dim.y),
        ]
    }
}

#[allow(dead_code)]
impl Shape for AABB {

    fn position(&self) -> Vector2 {
        self.position
    }

    fn support_point(&self, direction: Vector2) -> Vector2 {
        get_max_point(self.corners.iter(), direction, self.position())
    }
}

impl Display for AABB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AABB pos({0}), size({1})", self.position, self.size)
    }
}