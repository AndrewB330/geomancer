## Geomancer
Performance-oriented Rust library providing a suite of geometry primitives and algorithms for efficient computational geometry, ideal for graphics, simulations, and scientific computing.

## Examples
```rust
use geomancer::algorithms2d::convex_hull;

// ...

let points = [(0.1, 0.2), (10.0, -1.0), (5.0, 2.0), (7.0, 7.0)];

// Regular convex hull algorithm, works well for all real world scenarios.
// Robustly handles all degenerate cases. If there are points
// that are very close together, or are close to being collinear it may
// skip some points to ensure numerical stability for the rest of
// computations. It is guaranteed that resulting convex hull is within
// narrow tolerance from an actual exact convex hull. If set of points
// does not contain near-collinear points, or points that are too close together
// it will return actual exact convex hull.
let result = convex_hull(&points);
println!("Points that are part of the convex hull: {:?}", result.hull_points());
println!("Points that are not part of the convex hull: {:?}", result.inside_points());
println!("Area of the convex hull: {}", result.area());

let result_exact = convex_hull_exact(&points, false /* include_collinear */);
// Regular convex hull is within a narrow tolerance of the exact convex hull.
assert!((result_exact.area() - result.area()) < 1e-5);
```

## Traits
```rust
/// A trait for 2D points with x and y coordinates
pub trait Point2D {
    type Field;
    fn x(&self) -> Self::Field;
    fn y(&self) -> Self::Field;
}

/// Base trait for 2 dimensional geometric kernel that defines number and point type.
pub trait Kernel2D {
    type Point: Point2D<Field = Self::Field>;
    type Field: FieldNumber;
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

