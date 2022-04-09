use std::fmt::*;
use lmaths::*;

/// Trait implemented by every shape
pub trait Shape : Display {
    /// binding to the position field
    fn position(&self) -> Vector2;
    /// Return the furthest point/vertex of this shape along a direction
    fn support_point(&self, direction:Vector2) -> Vector2;
}