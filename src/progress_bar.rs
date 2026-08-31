use crate::gradient::Gradient;
use crate::style::BarStyle;
use indicatif::ProgressBar;

/// The default gradient used when creating a bar without specifying one.
pub const DEFAULT_GRADIENT: Gradient = Gradient::Ocean;

/// A self-contained wrapper around [`indicatif::ProgressBar`].
///
/// The underlying [`ProgressBar`] is `Clone` / `Send` / `Sync`, so an
/// [`GradientProgressBar`] is too : share it across worker threads, pass it by
/// reference, or keep several live at once.
#[derive(Debug, Clone)]
pub struct GradientProgressBar {
    pb: ProgressBar,
    style: BarStyle,
    gradient: Option<Gradient>,
    bar_width: usize,
}

impl GradientProgressBar {
    /// Create a bar with a known total, an explicit layout and an explicit
    /// style (gradient or not).
    pub fn new(len: u64, msg: String, style: BarStyle, gradient: Option<Gradient>) -> Self {
        let bar_width = BarStyle::DEFAULT_GRADIENT_WIDTH;
        let pb = ProgressBar::new(len)
            .with_message(msg)
            .with_style(style.to_progress_style(gradient, bar_width));
        Self {
            pb,
            style,
            gradient,
            bar_width,
        }
    }

    /// Create a byte-based bar (the usual entry point) with the default
    /// gradient ([`DEFAULT_GRADIENT`]).
    pub fn with_total_bytes(total: u64, msg: impl Into<String>) -> Self {
        Self::new(total, msg.into(), BarStyle::Main, Some(DEFAULT_GRADIENT))
    }

    /// Create a step/chunk based bar with the default gradient.
    pub fn with_total_steps(total: u64, msg: impl Into<String>) -> Self {
        Self::new(total, msg.into(), BarStyle::Steps, Some(DEFAULT_GRADIENT))
    }

    /// Create an indeterminate (unknown total) bar animated with a spinner.
    pub fn new_spinner(msg: impl Into<String>) -> Self {
        Self::new(u64::MAX, msg.into(), BarStyle::Spinner, None)
    }

    /// Create a byte-based bar with a specific [`Gradient`].
    pub fn with_gradient(total: u64, msg: impl Into<String>, gradient: Gradient) -> Self {
        Self::new(total, msg.into(), BarStyle::Main, Some(gradient))
    }

    /// Create a byte-based bar keeping the classic two-tone `wide_bar` (no
    /// true-color gradient).
    pub fn with_wide_bar(total: u64, msg: impl Into<String>) -> Self {
        Self::new(total, msg.into(), BarStyle::Main, None)
    }

    /// Update the progress to an absolute *position*.
    pub fn update(&self, position: u64) {
        self.pb.set_position(position);
    }

    /// Advance the progress by `delta`.
    pub fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }

    /// Set a new total length.
    pub fn set_length(&self, len: u64) {
        self.pb.set_length(len);
    }

    /// Grow the total length by `delta`.
    pub fn inc_length(&self, delta: u64) {
        self.pb.inc_length(delta);
    }

    /// Change the message shown next to the bar.
    pub fn set_message(&self, msg: impl Into<String>) {
        self.pb.set_message(msg.into());
    }

    /// Advance an indeterminate bar by one "tick" (re-draws the spinner).
    pub fn tick(&self) {
        self.pb.tick();
    }

    /// Mark the bar as finished without clearing it.
    pub fn finish(&self) {
        self.pb.finish_using_style();
    }

    /// Mark the bar as finished, attaching a final summary message.
    pub fn finish_with_message(&self, msg: impl Into<String>) {
        self.pb.finish_with_message(msg.into());
    }

    /// Finish and remove the bar from the terminal entirely (pristine output).
    pub fn finish_and_clear(&self) {
        self.pb.finish_and_clear();
    }

    /// Swap the look & feel of an already-created bar.
    pub fn set_style(&mut self, style: BarStyle) {
        self.style = style;
        self.pb
            .set_style(style.to_progress_style(self.gradient, self.bar_width));
    }

    /// Change the gradient (or set `None` for the classic two-tone bar).
    pub fn set_gradient(&mut self, gradient: Option<Gradient>) {
        self.gradient = gradient;
        self.pb
            .set_style(self.style.to_progress_style(gradient, self.bar_width));
    }

    /// Set the number of columns the gradient bar occupies.
    pub fn set_bar_width(&mut self, width: usize) {
        self.bar_width = width.max(1);
        self.pb
            .set_style(self.style.to_progress_style(self.gradient, self.bar_width));
    }

    /// The current layout of this bar.
    pub fn style(&self) -> BarStyle {
        self.style
    }

    /// The gradient currently attached (if any).
    pub fn gradient(&self) -> Option<Gradient> {
        self.gradient
    }

    /// Borrow the underlying [`indicatif::ProgressBar`] for advanced use.
    pub fn inner(&self) -> &ProgressBar {
        &self.pb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_total_bytes_defaults_to_main_and_ocean() {
        let bar = GradientProgressBar::with_total_bytes(1024, "test");
        assert_eq!(bar.style(), BarStyle::Main);
        assert_eq!(bar.gradient(), Some(Gradient::Ocean));
    }

    #[test]
    fn with_total_steps_defaults_to_steps_and_ocean() {
        let bar = GradientProgressBar::with_total_steps(100, "test");
        assert_eq!(bar.style(), BarStyle::Steps);
        assert_eq!(bar.gradient(), Some(Gradient::Ocean));
    }

    #[test]
    fn new_spinner_uses_spinner_style_no_gradient() {
        let bar = GradientProgressBar::new_spinner("test");
        assert_eq!(bar.style(), BarStyle::Spinner);
        assert_eq!(bar.gradient(), None);
        assert_eq!(bar.inner().length(), Some(u64::MAX));
    }

    #[test]
    fn with_gradient_uses_main_style() {
        let bar = GradientProgressBar::with_gradient(500, "test", Gradient::Ember);
        assert_eq!(bar.style(), BarStyle::Main);
        assert_eq!(bar.gradient(), Some(Gradient::Ember));
    }

    #[test]
    fn with_wide_bar_no_gradient() {
        let bar = GradientProgressBar::with_wide_bar(500, "test");
        assert_eq!(bar.style(), BarStyle::Main);
        assert_eq!(bar.gradient(), None);
    }

    #[test]
    fn new_sets_all_fields() {
        let bar = GradientProgressBar::new(
            200,
            "custom".into(),
            BarStyle::Minified,
            Some(Gradient::Ice),
        );
        assert_eq!(bar.style(), BarStyle::Minified);
        assert_eq!(bar.gradient(), Some(Gradient::Ice));
    }

    #[test]
    fn all_methods_run_without_panic() {
        let bar = GradientProgressBar::with_total_bytes(1000, "test");
        bar.inc(100);
        bar.update(500);
        bar.set_length(2000);
        bar.inc_length(500);
        bar.set_message("new msg");
        bar.tick();
        bar.finish();
    }

    #[test]
    fn spinner_methods_run_without_panic() {
        let bar = GradientProgressBar::new_spinner("spinner");
        bar.tick();
        bar.tick();
        bar.finish_with_message("done");
    }

    #[test]
    fn finish_and_clear_runs_without_panic() {
        let bar = GradientProgressBar::new(100, "clear".into(), BarStyle::ClearOnFinish, None);
        bar.finish_and_clear();
    }

    #[test]
    fn set_style_updates() {
        let mut bar = GradientProgressBar::with_total_bytes(100, "test");
        assert_eq!(bar.style(), BarStyle::Main);
        bar.set_style(BarStyle::Minified);
        assert_eq!(bar.style(), BarStyle::Minified);
    }

    #[test]
    fn set_gradient_updates() {
        let mut bar = GradientProgressBar::with_total_bytes(100, "test");
        assert_eq!(bar.gradient(), Some(Gradient::Ocean));
        bar.set_gradient(Some(Gradient::Violet));
        assert_eq!(bar.gradient(), Some(Gradient::Violet));
        bar.set_gradient(None);
        assert_eq!(bar.gradient(), None);
    }

    #[test]
    fn set_bar_width_clamps_to_minimum_one() {
        let mut bar = GradientProgressBar::with_total_bytes(100, "test");
        bar.set_bar_width(0);
        assert_eq!(bar.bar_width, 1);
        bar.set_bar_width(5);
        assert_eq!(bar.bar_width, 5);
    }

    #[test]
    fn clone_shares_state() {
        let bar = GradientProgressBar::with_total_bytes(100, "test");
        let bar2 = bar.clone();
        bar.inc(10);
        bar2.inc(10);
        bar.finish();
    }

    #[test]
    fn default_gradient_constant_is_ocean() {
        assert_eq!(DEFAULT_GRADIENT, Gradient::Ocean);
    }

    #[test]
    fn message_accepts_string_and_str() {
        let bar1 = GradientProgressBar::with_total_bytes(10, String::from("owned"));
        bar1.set_message("str ref");
        bar1.finish();
    }
}
