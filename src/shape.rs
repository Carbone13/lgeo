use lmaths::*;

pub trait Shape {
    fn position(&self) -> Vector2;
    fn support_point(&self, direction:Vector2) -> Vector2;
}
