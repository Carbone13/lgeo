# L-Geo

[![Documentation](https://docs.rs/collision/badge.svg)](https://docs.rs/lgeo/latest/lgeo/)
[![Version](https://img.shields.io/crates/v/collision.svg)](https://crates.io/crates/lgeo)
[![License](https://img.shields.io/crates/l/collision.svg)](https://github.com/carbone13/lgeo/blob/master/LICENSE)

2D Geometry library, written in Rust and based on [lmaths](https://github.com/carbone13/lmaths).

Provides classic 2D shapes, such as :
- Circles
- Rectangles
- Convex Polygons

It implements the [GJK](https://dyn4j.org/2010/04/gjk-gilbert-johnson-keerthi/) algorithm to compute collision and [EPA](https://dyn4j.org/2010/05/epa-expanding-polytope-algorithm/) (yet to be developped) to extrude collision informations.

Examples :

First, always create a GJK struct, which hold your settings, like the contact tolerance :
```rust
let gjk = GJK::new();
```
Then you can start spawning shapes :
```rust
let c = Circle::new(Vector2::new(4.6, -3.2), 1.43);
let r = AABB::new(Vector2::new(-1.0, 0.5), Vector2::new(2.0, 1.0));
let p = Polygon::new(Vector2::new(0.0, 0.0),
                    vec![Vector2::new(0.0, 0.5),
                         Vector2::new(-1.5, -0.5),
                         Vector2::new(-3.0, 0.5),
                    ]);
```
Finally, you can check for collisions between them.
Converting the result to a boolean is done by calling `result.is_some()`, as the `intersect()` function return the final Simplex of GJK.
```rust
let c_vs_r = gjk.intersect(&c, &r).is_some(); // False
let c_vs_p = gjk.intersect(&c, &p).is_some(); // False
let r_vs_p = gjk.intersect(&r, &p).is_some(); // True
```











