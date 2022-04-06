// uses
use lmaths::*;
use smallvec::*;
use super::*;
use crate::Contact;

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
    /// Initialize new GJK settings, using the default one
    pub fn new() -> Self {
        Self {
            distance_tolerance: GJK_DISTANCE_TOLERANCE,
            continuous_tolerance: GJK_CONTINUOUS_TOLERANCE,
            max_iterations: MAX_ITERATIONS,
        }
    }

    /// Check if the shapes intersect and return the final Simplex if they do, or 'None' if they don't
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

    pub fn intersection_time_of_impact(&self, left: &dyn crate::Shape, left_velocity:Vector2, right: &dyn crate::Shape, right_velocity:Vector2, )
        -> Option<Contact> {

        let ray = right_velocity - left_velocity;

        // initialize time of impact
        let mut lambda = 0.0;
        let mut normal = Vector2::ZERO;
        let mut ray_origin = Vector2::ZERO;

        // build simplex and get an initial support point to bootstrap the algorithm
        let mut simplex = Simplex::new(smallvec![]);
        let p = SupportPoint::from_minkowski(left, right, -ray);
        // we only need the actual support point for this
        let mut v = p.v;

        // if the squared magnitude is small enough, we have a hit and can stop
        while v.sqr_length() > self.continuous_tolerance {
            // get a new support point
            let p = SupportPoint::from_minkowski(left, right, -v);

            let vp = v.dot(p.v);
            let vr = v.dot(ray);
            // check if we have a hit point along the ray further than the current clipped ray
            if vp > lambda * vr {
                // if the hit point is in the positive ray direction, we clip the ray, clear
                // the simplex and start over from a new ray origin
                if vr > 0.0 {
                    lambda = vp / vr;
                    // if the clipped hit point is beyond the end of the ray,
                    // we can never have a hit
                    if lambda > 1.0 {
                        return None;
                    }
                    ray_origin = ray * lambda;
                    simplex.clear();
                    normal = -v;
                } else {
                    // if the hitpoint is behind the ray origin, we can never have a hit
                    return None;
                }
            }
            // we construct the simplex around the current ray origin (if we can)
            simplex.push(p - ray_origin);
            v = simplex.get_closest_point_to_origin();
        }
        if v.sqr_length() <= self.continuous_tolerance {
            let transform = right.position() + right_velocity * lambda;
            let mut contact = Contact::new_populate(
                -normal.normalized(), // our convention is normal points from B towards A
                v.length(),       // will always be very close to zero
                transform + ray_origin,
            );
            contact.time_of_impact = lambda;
            Some(contact)
        } else {
            None
        }
    }
}