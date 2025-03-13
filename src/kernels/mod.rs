mod generic;

pub use generic::GenericKernel2D;

#[cfg(feature = "bevy_math")]
mod bevy;
#[cfg(feature = "bevy_math")]
pub use bevy::BevyVec2Kernel;

#[cfg(feature = "nalgebra")]
mod nalgebra;
#[cfg(feature = "nalgebra")]
pub use nalgebra::{NalgebraPoint2Kernel, NalgebraVector2Kernel};

#[cfg(feature = "geo")]
mod geo;
#[cfg(feature = "geo")]
pub use geo::GeoCoordKernel;

#[cfg(feature = "tuple_kernels")]
mod tuple_kernels;

#[cfg(feature = "tuple_kernels")]
pub use tuple_kernels::TupleKernel2D;

#[cfg(feature = "array_kernels")]
mod array_kernels;

#[cfg(feature = "array_kernels")]
pub use array_kernels::ArrayKernel2D;
