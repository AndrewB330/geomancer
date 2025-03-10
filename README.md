## Geomancer
Performance-oriented Rust library providing a suite of geometry primitives and algorithms for efficient computational geometry, ideal for graphics, simulations, and scientific computing.

## Examples
```rust
use geomancer::algorithms::d2::convex_hull;

let points = vec![(0, 0), (7, 1), (3, 8), (4, 2)];
let convex_hull = ConvexHull::new(points);
```

## Notable traits
```rust
/// A trait for 2D points with x and y coordinates
pub trait Point2D {
    type Field: Number;
    fn x(&self) -> Field;
    fn y(&self) -> Field;
}

/// Defines basic 2D geometric operations using an abstract number and point type.
pub trait Kernel2D {
    type Field: Number;
    type Point: Point2D<Field=Self::Field>;

    /* ... vector operations and predicates like dot, cross, orient, in circle ... */
}
```


## Algorithm ideas (TODO)
- Convex hull (dynamic, static)
- Farthest pair of points
- Closest pair of points
- EMST
- Proximity queries (Kd Tree)
- Voronoi diagram
- Delaunay triangulation
- Other triangulations
- Polygon area
- Point in polygon check

