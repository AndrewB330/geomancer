
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/andrewb330/geomancer#license)
[![Crates.io](https://img.shields.io/crates/v/geomancer.svg)](https://crates.io/crates/geomancer)
[![Downloads](https://img.shields.io/crates/d/geomancer.svg)](https://crates.io/crates/geomancer)
[![Docs](https://docs.rs/geomancer/badge.svg)](https://docs.rs/geomancer/latest/geomancer/)

## Geomancer
Performance-oriented Rust library providing a suite of geometry primitives and algorithms for efficient computational geometry.

Supported types:
- `Vec2` from [`bevy`](https://github.com/bevyengine/bevy) crate (use `bevy_math` feature)
- `Point2<T>` from [`nalgebra`](https://github.com/dimforge/nalgebra) crate (use `nalgebra` feature)

## Default kernels
Geomancer provides a range of default kernels tailored to different libraries. Each kernel defines a set of operations for a specific vector or point type. By enabling the corresponding feature, the DefaultKernel trait is automatically implemented for that type.

Below is a summary of the supported kernels:
| Library    | Vector Type      | Default Kernel                | Feature       | Dimension | Exact |
|------------|------------------|-------------------------------|---------------|-----------|-------|
| bevy_math  | Vec2             | BevyVec2Kernel                | bevy_math     | 2D        | No    |
| nalgebra   | Vector2<T>       | NalgebraVector2Kernel<T>      | nalgebra      | 2D        | No    |
| nalgebra   | Point2<T>        | NalgebraPoint2Kernel<T>       | nalgebra      | 2D        | No    |
| geo        | Coord<T>         | GeoCoordKernel<T>             | geo           | 2D        | No    |
|            | (T, T)           | TupleKernel2D<T>              | tuple_kernels | 2D        | No    |
|            | [T; 2]           | ArrayKernel2D<T>              | array_kernels | 2D        | No    |
|            | Any V: Point2D   | GenericKernel2D<V>*           | -             | 2D        | No    |

*If the data type or library you are using is not listed, you can still use the generic kernel. The generic kernel works with any type that implements the Point2D trait. Simply add the following implementations to your code:
```rust
impl Point2D for MY_VECTOR_TYPE { 
    // Provide the implementation for the Point2D trait.
}

impl DefaultKernel for MY_VECTOR_TYPE { 
    type Kernel = GenericKernel2D<MY_VECTOR_TYPE>; 
}

```
Using a specialized kernel is preferable when available, as it may include optimizations tailored to that particular vector type.

## Examples
```rust
use geomancer::algorithms2d::convex_hull;

// ...

let points = [Vec2::new(0.1, 0.2), Vec2::new(10.0, -1.0), Vec2::new(5.0, 2.0), Vec2::new(7.0, 7.0)];

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

## License

Geomancer is free and open source library, all code in this repository is dual-licensed under:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
