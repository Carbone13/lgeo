mod gjk;
mod shapes;
mod util;
mod shape;
mod results;

// gjk folder re-exports
pub use gjk::GJK;
pub use gjk::Simplex;
pub use gjk::SupportPoint;

// shapes re-exports
pub use shapes::AABB;
pub use shapes::Circle;
pub use shapes::Polygon;

// results re-exports
pub use results::Contact;

// shape trait
pub use shape::Shape;
