use lmaths::*;

pub struct Contact {
    pub normal:Vector2,
    pub penetration_depth:f64,
    pub contact_point:Vector2,
    pub time_of_impact:f64
}

impl Contact {
    pub fn new () -> Self {
        Self {
            normal: Vector2::ZERO,
            penetration_depth: 0.0,
            contact_point: Vector2::ZERO,
            time_of_impact: 0.0
        }
    }

    pub fn new_populate (normal:Vector2, penetration_depth:f64, contact_point:Vector2) -> Self {
        Self {
            normal,
            penetration_depth,
            contact_point,
            time_of_impact: 0.0
        }
    }
}