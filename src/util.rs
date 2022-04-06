use std::slice::Iter;
use lmaths::*;

#[inline]
#[allow(dead_code)]
pub fn get_closest_point_on_edge (start: &Vector2, end: &Vector2, point: &Vector2) -> Vector2 {
    let line = *end - *start;
    let line_dir = line.normalized();
    let v = *point - *start;
    let d = v.dot(line_dir);

    if d < 0.0
    {
        *start
    }
    else if (d * d) > line.sqr_length()
    {
        *end
    }
    else
    {
        *start + line_dir * d
    }
}

pub fn get_max_point(vertices:Iter<Vector2>, direction:Vector2, transform:Vector2) -> Vector2
{
    let (p, _) = vertices.map(|&v| (v, v.dot(direction))).fold(
        (Vector2::ZERO, f64::NEG_INFINITY),
        |(max_p, max_dot), (v, dot)| {
            if dot > max_dot {
                (v, dot)
            } else {
                (max_p, max_dot)
            }
        },
    );

    p + transform
}