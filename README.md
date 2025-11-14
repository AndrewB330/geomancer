
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/andrewb330/geomancer#license)
[![Crates.io](https://img.shields.io/crates/v/geomancer.svg)](https://crates.io/crates/geomancer)
[![Downloads](https://img.shields.io/crates/d/geomancer.svg)](https://crates.io/crates/geomancer)
[![Docs](https://docs.rs/geomancer/badge.svg)](https://docs.rs/geomancer/latest/geomancer/)

## Geomancer
Performance-oriented Rust library providing a suite of geometry primitives and algorithms for efficient computational geometry.

#### Supported types:
- `(f32, f32)`, `(f64, f64)` - rust tuples
- `[f32;2]`, `[f64;2]` - rust arrays
- `Vec2` from [`bevy`](https://github.com/bevyengine/bevy) crate
- `Point2<T>` from [`nalgebra`](https://github.com/dimforge/nalgebra) crate
- `Coord<T>` from [`geo`]() crate

#### Supported algortihms:
- 2D
  - convex hull
  - farthest points


## Examples
#### Convex hull
```rust
use geomancer::algorithms::convex_hull;

// ...

let points = [(0.1, 0.2), (10.0, -1.0), (5.0, 2.0), (7.0, 7.0)];

let result = convex_hull(&points).unwrap();

println!("Points that are part of the convex hull: {:?}", result.hull_points());
println!("Points that are not part of the convex hull: {:?}", result.inside_points());
println!("Area: {} Perimeter: {}", result.area(), result.perimeter());

// Find convex hull using exact math, without rounding and floating point errors.
let result_exact = convex_hull_exact(&points, false /* include_collinear */).unwrap();
```
#### Farthest points
```rust
use geomancer::algorithms::farthest_points;

// ...

let points = [(0.0, 0.0), (10.0, 1.0), (-5.0, -1.0)];

let (point_a_idx, point_b_idx) = farthest_points(&points).unwrap();
```

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

## Traits
```rust
/// A trait for 2D points with x and y coordinates
pub trait Point2D {
    type Scalar;
    fn x(&self) -> Self::Scalar;
    fn y(&self) -> Self::Scalar;
}

/// Base trait for 2 dimensional geometric kernel that defines number and point type.
pub trait Kernel2D {
    type Point;
    type Scalar;
}
```

## License

Geomancer is free and open source library, all code in this repository is dual-licensed under:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
