#[cfg(feature = "bevy_math")]
mod bevy;
#[cfg(feature = "bevy_math")]
pub use bevy::BevyVec2Kernel;

#[cfg(feature = "nalgebra")]
mod nalgebra;
#[cfg(feature = "nalgebra")]
pub use nalgebra::NalgebraVector2Kernel;
