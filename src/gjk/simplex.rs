// using
use lmaths::*;
use smallvec::*;
use crate::util::*;
use super::*;
//

pub struct Simplex(SmallVec<[SupportPoint; 5]>);

#[allow(dead_code)]
impl Simplex {

    pub fn new (t:SmallVec<[SupportPoint; 5]>) -> Self {
        Self(t)
    }

    pub fn reduce_to_closest_feature (&mut self, d: &mut Vector2) -> bool {
        // 3 points
        if self.len() == 3 {
            let a = self[2].v;
            let b = self[1].v;
            let c = self[0].v;

            let ao = -a;
            let ab = b - a;
            let ac = c - a;

            let abp = triple_product(ac, ab, ab);
            if abp.dot(ao) > 0.0 {
                self.remove(0);
                *d = abp;
            } else {
                let acp = triple_product(ab, ac, ac);
                if acp.dot(ao) > 0.0 {
                    self.remove(1);
                     *d = acp;
                 } else {
                    return true;
                }
           }
        }
        // 2 points
        else if self.len() == 2 {
            let a = self[1].v;
            let b = self[0].v;

            let ao = -a;
            let ab = b - a;

            *d = triple_product(ab, ao, ab);
            // TODO ulpseq!
            if *d == Vector2::ZERO {
                *d = Vector2::new(-ab.y, ab.x);
            }
        }
        // 0-1 point means we can't really do anything
        false
    }

    /// Get the closest point on the simplex to the origin.
    ///
    /// Make simplex only retain the closest feature to the origin.
    pub fn get_closest_point_to_origin(&mut self) -> Vector2 {
        let mut d = Vector2::ZERO;

        // reduce simplex to the closest feature to the origin
        // if check_origin return true, the origin is inside the simplex, so return the zero vector
        // if not, the simplex will be the closest edge to the origin, and d the normal of the edge
        // in the direction of the origin
        if self.reduce_to_closest_feature(&mut d) {
            return d;
        }

        // compute closest point to origin on the simplex (which is now an edge)
        if self.len() == 1
        {
            self[0].v
        }
        else
        {
            get_closest_point_on_edge(&self[1].v, &self[0].v, &Vector2::ZERO)
        }
    }
}

impl std::ops::Deref for Simplex {
    type Target = SmallVec<[SupportPoint; 5]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Simplex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}