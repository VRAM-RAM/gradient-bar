use std::sync::{Arc, Mutex};

use crate::{BarStyle, Gradient};

#[derive(Debug, Clone)]
pub struct Config {
    pub style: BarStyle,
    pub gradient: Option<Gradient>,
    pub bar_width: usize,
    pub unit: Option<String>,
}

impl Config {
    pub fn new(
        style: BarStyle,
        gradient: Option<Gradient>,
        bar_width: usize,
        unit: Option<String>,
    ) -> Self {
        Self {
            style,
            gradient,
            bar_width,
            unit,
        }
    }

    pub fn arc_mutex(
        style: BarStyle,
        gradient: Option<Gradient>,
        bar_width: usize,
        unit: Option<String>,
    ) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new(style, gradient, bar_width, unit)))
    }
}
