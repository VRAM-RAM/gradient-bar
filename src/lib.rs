pub mod gradient;
pub mod progress_bar;
pub mod style;

pub use {gradient::Gradient, progress_bar::{DEFAULT_GRADIENT, GradientProgressBar}, style::BarStyle, indicatif::MultiProgress};
