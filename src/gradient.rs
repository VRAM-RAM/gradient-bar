use std::fmt::{Display, Write as _};

/// A curated set of smooth, true-color progress gradients.
///
/// Each variant interpolates a *linear* ramp between a start and an end RGB
/// color across the bar, giving a gradual multi-stop feel rather than the flat
/// two-tone `wide_bar`. Works on any terminal with 24-bit color support (the
/// default on modern terminals); degraded gracefully elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Gradient {
    /// Deep blue -> ice cyan. The Enkryptit signature look.
    #[default]
    Ocean,

    /// Violet -> magenta.
    Violet,

    /// Warm orange -> gold.
    Sunset,

    /// Aqua green -> mint.
    Mint,

    /// Amber -> Light Red (danger / critical ops).
    Ember,

    /// Cool white -> pale blue.
    Ice,

    /// Blue / light cyan green -> light purple
    Aurora,

    /// Cyan -> light blue / purple
    Neon,

    /// Pink -> light Coral
    Rose,

    /// Green -> light green
    Forest,

    /// Dark blue -> light purple
    Midnight,

    /// A gray-blue -> soft lavender.
    Twilight,

    /// Deep coral -> soft peach.
    Coral,
}

impl Gradient {
    /// The `(start, end)` RGB stops for this gradient.
    pub fn stops(self) -> ((u8, u8, u8), (u8, u8, u8)) {
        match self {
            Gradient::Ocean => ((8, 150, 255), (150, 235, 255)),
            Gradient::Violet => ((140, 60, 255), (255, 90, 220)),
            Gradient::Sunset => ((255, 120, 60), (255, 220, 90)),
            Gradient::Mint => ((40, 200, 150), (150, 245, 190)),
            Gradient::Ember => ((255, 120, 30), (220, 40, 60)),
            Gradient::Ice => ((150, 200, 255), (245, 250, 255)),
            Gradient::Aurora => ((80, 255, 210), (170, 80, 255)),
            Gradient::Neon => ((0, 255, 220), (100, 120, 255)),
            Gradient::Rose => ((255, 120, 180), (255, 170, 130)),
            Gradient::Forest => ((30, 150, 90), (180, 220, 80)),
            Gradient::Twilight => ((25, 65, 130), (155, 135, 210)),
            Gradient::Midnight => ((20, 30, 80), (100, 70, 180)),
            Gradient::Coral => ((220, 80, 85), (255, 175, 135)),
        }
    }

    /// Render a `width`-column true-color gradient bar reflecting `fraction`
    /// (0.0 → 1.0) of completed work.
    pub fn render(self, fraction: f32, width: usize) -> String {
        let ((r0, g0, b0), (r1, g1, b1)) = self.stops();
        let filled = (fraction.clamp(0.0, 1.0) * width as f32).round() as usize;

        let mut out = String::with_capacity(width * 16 + 8);

        for i in 0..filled {
            let t = if width > 1 {
                i as f32 / (width - 1) as f32
            } else {
                0.0
            };
            let r = r0 as f32 + (r1 as f32 - r0 as f32) * t;
            let g = g0 as f32 + (g1 as f32 - g0 as f32) * t;
            let b = b0 as f32 + (b1 as f32 - b0 as f32) * t;
            let _ = write!(out, "\x1b[38;2;{};{};{}m█", r as u8, g as u8, b as u8);
        }

        if filled < width {
            let _ = write!(out, "\x1b[38;2;58;66;82m"); // dim slate for the remainder
            for _ in filled..width {
                out.push('·');
            }
        }

        out.push_str("\x1b[0m");
        out
    }

    /// name for the gradient (used in `Debug`/messages).
    pub fn name(self) -> &'static str {
        match self {
            Gradient::Ocean => "Ocean",
            Gradient::Violet => "Violet",
            Gradient::Sunset => "Sunset",
            Gradient::Mint => "Mint",
            Gradient::Ember => "Ember",
            Gradient::Ice => "Ice",
            Gradient::Aurora => "Aurora",
            Gradient::Twilight => "Twilight",
            Gradient::Forest => "Forest",
            Gradient::Midnight => "Midnight",
            Gradient::Neon => "Neon",
            Gradient::Rose => "Rose",
            Gradient::Coral => "Coral",
        }
    }
}

impl Display for Gradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_returns_correct_strings() {
        assert_eq!(Gradient::Ocean.name(), "Ocean");
        assert_eq!(Gradient::Violet.name(), "Violet");
        assert_eq!(Gradient::Sunset.name(), "Sunset");
        assert_eq!(Gradient::Mint.name(), "Mint");
        assert_eq!(Gradient::Ember.name(), "Ember");
        assert_eq!(Gradient::Ice.name(), "Ice");
        assert_eq!(Gradient::Aurora.name(), "Aurora");
        assert_eq!(Gradient::Neon.name(), "Neon");
        assert_eq!(Gradient::Rose.name(), "Rose");
        assert_eq!(Gradient::Forest.name(), "Forest");
        assert_eq!(Gradient::Midnight.name(), "Midnight");
        assert_eq!(Gradient::Twilight.name(), "Twilight");
        assert_eq!(Gradient::Coral.name(), "Coral");
    }

    #[test]
    fn stops_are_distinct_per_gradient() {
        let seen: Vec<_> = [
            Gradient::Ocean,
            Gradient::Violet,
            Gradient::Sunset,
            Gradient::Mint,
            Gradient::Ember,
            Gradient::Ice,
            Gradient::Aurora,
            Gradient::Neon,
            Gradient::Rose,
            Gradient::Forest,
            Gradient::Midnight,
            Gradient::Twilight,
            Gradient::Coral,
        ]
        .iter()
        .map(|g| g.stops())
        .collect();
        for (i, a) in seen.iter().enumerate() {
            for (j, b) in seen.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b, "gradients {i} and {j} should have different stops");
                }
            }
        }
    }

    #[test]
    fn render_at_zero_is_all_dots() {
        let out = Gradient::Ocean.render(0.0, 20);
        assert!(out.contains("·"), "expected dots at 0%: {out}");
        assert!(
            !out.contains("█"),
            "should have no filled blocks at 0%: {out}"
        );
    }

    #[test]
    fn render_at_one_is_all_filled() {
        let out = Gradient::Ocean.render(1.0, 20);
        assert!(out.contains("█"), "expected filled blocks at 100%: {out}");
        let dot_count = out.matches('·').count();
        assert_eq!(dot_count, 0, "should have no dots at 100%: {out}");
    }

    #[test]
    fn render_halfway_approximately_half() {
        let width = 20;
        let out = Gradient::Sunset.render(0.5, width);
        let filled = out.matches('█').count();
        let dots = out.matches('·').count();
        assert_eq!(filled + dots, width, "total chars should equal width");
        assert!(
            (filled as i32 - dots as i32).unsigned_abs() <= 2,
            "filled ({filled}) and dots ({dots}) should be roughly equal"
        );
    }

    #[test]
    fn render_width_one() {
        let out_zero = Gradient::Violet.render(0.0, 1);
        assert!(out_zero.contains('·'));

        let out_one = Gradient::Violet.render(1.0, 1);
        assert!(out_one.contains('█'));
    }

    #[test]
    fn render_clamps_above_one() {
        let out = Gradient::Mint.render(1.5, 10);
        let filled = out.matches('█').count();
        assert_eq!(filled, 10, "fraction > 1.0 should clamp to full bar");
    }

    #[test]
    fn render_clamps_below_zero() {
        let out = Gradient::Ember.render(-0.5, 10);
        let filled = out.matches('█').count();
        assert_eq!(filled, 0, "fraction < 0.0 should clamp to empty bar");
    }

    #[test]
    fn render_contains_ansi_reset() {
        let out = Gradient::Ice.render(0.5, 10);
        assert!(
            out.ends_with("\x1b[0m"),
            "output should end with ANSI reset"
        );
    }

    #[test]
    fn default_is_ocean() {
        assert_eq!(Gradient::default(), Gradient::Ocean);
    }

    #[test]
    fn display_formats_with_backticks() {
        for grad in [
            Gradient::Ocean,
            Gradient::Violet,
            Gradient::Sunset,
            Gradient::Mint,
            Gradient::Ember,
            Gradient::Ice,
            Gradient::Aurora,
            Gradient::Neon,
            Gradient::Rose,
            Gradient::Forest,
            Gradient::Midnight,
            Gradient::Twilight,
            Gradient::Coral,
        ] {
            let expected = grad.name().to_string();
            assert_eq!(format!("{grad}"), expected, "Display for {grad:?}");
        }
    }

    #[test]
    fn render_each_variant_does_not_panic() {
        for grad in [
            Gradient::Ocean,
            Gradient::Violet,
            Gradient::Sunset,
            Gradient::Mint,
            Gradient::Ember,
            Gradient::Ice,
            Gradient::Aurora,
            Gradient::Neon,
            Gradient::Rose,
            Gradient::Forest,
            Gradient::Midnight,
            Gradient::Twilight,
            Gradient::Coral,
        ] {
            grad.render(0.0, 0);
            grad.render(0.33, 30);
            grad.render(1.0, 1);
        }
    }
}
