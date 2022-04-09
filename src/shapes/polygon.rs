use std::fmt::{Display, Formatter};
use lmaths::*;
use crate::Shape;
use crate::util::*;

#[allow(dead_code)]
/// A Convex polygon, made by at least three vertices.
pub struct Polygon {
    pub position: Vector2,
    pub vertices: Vec<Vector2>
}

#[allow(dead_code)]
impl Polygon {
    /// Create a new polygon with the provided set of vertices
    pub fn new(position:Vector2, vertices: Vec<Vector2>) -> Self {
        Self { position, vertices }
    }
}

impl Shape for Polygon {
    fn position(&self) -> Vector2 {
        self.position
    }

    fn support_point(&self, direction: Vector2) -> Vector2 {
        get_max_point(self.vertices.iter(), direction, self.position())
    }
}

impl Display for Polygon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Polygon pos({0}), vertices_count: {1}", self.position, self.vertices.len())
    }
}