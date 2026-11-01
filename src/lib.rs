pub mod config;
pub mod gradient;
pub mod progress_bar;
pub mod style;

pub use {
    gradient::Gradient,
    indicatif::MultiProgress,
    progress_bar::{DEFAULT_GRADIENT, GradientProgressBar},
    style::BarStyle,
};
