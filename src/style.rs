use crate::gradient::Gradient;
use indicatif::{ProgressState, ProgressStyle};

/// The look & feel of a [`GradientProgressBar`].
///
/// This is the **layout**, independent of the gradient. Pick a layout here, and
/// pair it with any [`Gradient`] (or none, to keep the classic
/// two-tone `wide_bar`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BarStyle {
    /// The flagship layout: spinner + message + elapsed + wide bar with
    /// `bytes/total` + percent + ETA. Best suited to a byte-based workload.
    #[default]
    Main,
    /// Compact single-line layout, for tight output or secondary bars.
    Minified,
    /// For chunk / step based workloads (e.g. multithreaded splitting). Shows
    /// `pos/len` as *chunks*.
    Steps,
    /// Indeterminate progress (no known total): animates a spinner + message.
    Spinner,
    /// Like [`BarStyle::Main`], but cleared from the terminal
    /// once it finishes (pair with [`GradientProgressBar::finish_and_clear`]).
    ClearOnFinish,
}

impl BarStyle {
    /// Default bar width (in cells) used when rendering a gradient.
    pub const DEFAULT_GRADIENT_WIDTH: usize = 30;

    /// Build the [`indicatif::ProgressStyle`] for this layout.
    ///
    /// When `gradient` is `Some`, the bar is rendered by [`Gradient`]
    /// (true-color multi-stop). When `None`, the classic two-tone
    /// `{wide_bar:.cyan/blue}` is used. `width` controls the gradient bar's
    /// column count.
    pub fn to_progress_style(&self, gradient: Option<Gradient>, width: usize) -> ProgressStyle {
        let gname = gradient.map(Gradient::name).unwrap_or("");

        let (bar_key, spinner_color) = match gradient {
            Some(_) => ("{grad_bar}", "cyan"),
            None => ("{wide_bar:.cyan/blue}", "green"),
        };

        let template = match self {
            BarStyle::Main => format!(
                "{{spinner:.{spinner_color}}} {{msg}} [{{elapsed_precise}}] [{bar_key}] {{bytes}}/{{total_bytes}} ({{percent}}%) {{eta}}",
                spinner_color = spinner_color,
                bar_key = bar_key,
            ),
            BarStyle::Minified => format!(
                "{{spinner:.{spinner_color}}} {{msg}} {{bytes}}/{{total_bytes}} ({{percent}}%) [[{gname}]]",
                spinner_color = spinner_color,
                gname = gname,
            ),
            BarStyle::Steps => format!(
                "{{spinner:.cyan}} {{msg}} [{bar_key}] {{pos}}/{{len}} chunks ({{percent}}%) {{elapsed_precise}}",
                bar_key = bar_key,
            ),
            BarStyle::Spinner => "{{spinner:.cyan}} {{msg}}".to_string(),
            BarStyle::ClearOnFinish => format!(
                "{{spinner:.{spinner_color}}} {{msg}} [{{elapsed_precise}}] [{bar_key}] {{bytes}}/{{total_bytes}} ({{percent}}%) {{eta}}",
                spinner_color = spinner_color,
                bar_key = bar_key,
            ),
        };

        let chars = "█▉▊▋▌▍▎▏  ";
        let tick = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ";

        let mut style = ProgressStyle::with_template(&template)
            .expect("the template is valid")
            .progress_chars(chars)
            .tick_chars(tick);

        if let Some(gradient) = gradient {
            // Register a custom renderer for {grad_bar}: a fixed-width,
            // true-color gradient reflecting the current fraction.
            let grad = gradient;
            let w = width.max(1);
            style = style.with_key(
                "grad_bar",
                move |state: &ProgressState, wout: &mut dyn std::fmt::Write| {
                    let _ = write!(wout, "{}", grad.render(state.fraction(), w));
                },
            );
        }

        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_styles_build_without_panic() {
        let gradients = [None, Some(Gradient::Ocean), Some(Gradient::Violet)];
        for gradient in gradients {
            for style in [
                BarStyle::Main,
                BarStyle::Minified,
                BarStyle::Steps,
                BarStyle::Spinner,
                BarStyle::ClearOnFinish,
            ] {
                let _ = style.to_progress_style(gradient, 30);
            }
        }
    }

    #[test]
    fn default_gradient_width_is_30() {
        assert_eq!(BarStyle::DEFAULT_GRADIENT_WIDTH, 30);
    }

    #[test]
    fn zero_width_builds_without_panic() {
        for style in [
            BarStyle::Main,
            BarStyle::Minified,
            BarStyle::Steps,
            BarStyle::Spinner,
            BarStyle::ClearOnFinish,
        ] {
            let _ = style.to_progress_style(Some(Gradient::Ocean), 0);
            let _ = style.to_progress_style(None, 0);
        }
    }

    #[test]
    fn large_width_builds_without_panic() {
        let _ = BarStyle::Main.to_progress_style(Some(Gradient::Sunset), 500);
    }

    #[test]
    fn bar_style_is_copy_clone_debug() {
        let s = BarStyle::Main;
        let s2 = s;
        let s3 = s;
        let _ = format!("{:?}", s3);
        assert_eq!(s, s2);
    }

    #[test]
    fn all_variants_are_distinct() {
        let styles = [
            BarStyle::Main,
            BarStyle::Minified,
            BarStyle::Steps,
            BarStyle::Spinner,
            BarStyle::ClearOnFinish,
        ];
        for (i, a) in styles.iter().enumerate() {
            for (j, b) in styles.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }
}
