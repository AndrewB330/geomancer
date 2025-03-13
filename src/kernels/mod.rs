#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "bevy")]
pub use bevy::BevyVec2Kernel;

#[cfg(feature = "nalgebra")]
mod nalgebra;
#[cfg(feature = "nalgebra")]
pub use nalgebra::NalgebraVector2Kernel;
