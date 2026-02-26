//! Progress Circle Component - Circular progress indicator
//! CSS-only using conic-gradient

use maud::{html, Markup, Render};

/// Progress circle size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressCircleSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Progress circle variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressCircleVariant {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

/// Progress circle component
#[derive(Debug, Clone)]
pub struct ProgressCircle<'a> {
    pub value: u8,
    pub max: u8,
    pub size: ProgressCircleSize,
    pub variant: ProgressCircleVariant,
    pub label: Option<&'a str>,
    pub show_value: bool,
}

impl<'a> ProgressCircle<'a> {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            max: 100,
            size: ProgressCircleSize::default(),
            variant: ProgressCircleVariant::default(),
            label: None,
            show_value: true,
        }
    }

    pub fn max(mut self, max: u8) -> Self {
        self.max = max.max(1);
        self
    }

    pub fn size(mut self, size: ProgressCircleSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ProgressCircleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-progress-circle"];

        match self.size {
            ProgressCircleSize::Sm => classes.push("sh-progress-circle--sm"),
            ProgressCircleSize::Md => classes.push("sh-progress-circle--md"),
            ProgressCircleSize::Lg => classes.push("sh-progress-circle--lg"),
            ProgressCircleSize::Xl => classes.push("sh-progress-circle--xl"),
        }

        match self.variant {
            ProgressCircleVariant::Primary => classes.push("sh-progress-circle--primary"),
            ProgressCircleVariant::Secondary => classes.push("sh-progress-circle--secondary"),
            ProgressCircleVariant::Success => classes.push("sh-progress-circle--success"),
            ProgressCircleVariant::Warning => classes.push("sh-progress-circle--warning"),
            ProgressCircleVariant::Danger => classes.push("sh-progress-circle--danger"),
        }

        classes.join(" ")
    }

    fn percentage(&self) -> f32 {
        (self.value as f32 / self.max as f32 * 100.0).min(100.0)
    }
}

impl<'a> Render for ProgressCircle<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let percentage = self.percentage();

        html! {
            div
                class=(classes)
                role="progressbar"
                aria-valuenow=(self.value)
                aria-valuemin="0"
                aria-valuemax=(self.max)
                aria-label=[self.label]
                style=(format!("--sh-progress: {}%", percentage))
            {
                svg class="sh-progress-circle__svg" viewBox="0 0 36 36" {
                    circle
                        class="sh-progress-circle__track"
                        cx="18"
                        cy="18"
                        r="16"
                        fill="none"
                        stroke-width="3";
                    circle
                        class="sh-progress-circle__fill"
                        cx="18"
                        cy="18"
                        r="16"
                        fill="none"
                        stroke-width="3"
                        stroke-dasharray=(format!("{} {}", percentage, 100.0 - percentage));
                }
                @if self.show_value {
                    span class="sh-progress-circle__value" {
                        (self.value)
                    }
                }
            }
        }
    }
}

pub fn progress_circle_css() -> String {
    r#"
.sh-progress-circle {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.sh-progress-circle__svg {
    transform: rotate(-90deg);
}

.sh-progress-circle--sm {
    width: 32px;
    height: 32px;
}

.sh-progress-circle--md {
    width: 48px;
    height: 48px;
}

.sh-progress-circle--lg {
    width: 64px;
    height: 64px;
}

.sh-progress-circle--xl {
    width: 96px;
    height: 96px;
}

.sh-progress-circle__track {
    stroke: var(--sh-color-surface-muted, #e5e5e5);
}

.sh-progress-circle__fill {
    stroke-linecap: round;
    transition: stroke-dasharray 0.3s ease;
    stroke-dasharray: var(--sh-progress, 0) 100;
}

.sh-progress-circle--primary .sh-progress-circle__fill {
    stroke: var(--sh-color-primary, #3b82f6);
}

.sh-progress-circle--secondary .sh-progress-circle__fill {
    stroke: var(--sh-color-secondary, #6b7280);
}

.sh-progress-circle--success .sh-progress-circle__fill {
    stroke: var(--sh-color-success, #22c55e);
}

.sh-progress-circle--warning .sh-progress-circle__fill {
    stroke: var(--sh-color-warning, #f59e0b);
}

.sh-progress-circle--danger .sh-progress-circle__fill {
    stroke: var(--sh-color-danger, #ef4444);
}

.sh-progress-circle__value {
    position: absolute;
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-semibold, 600);
    color: var(--sh-color-text, #1a1a1a);
}

.sh-progress-circle--sm .sh-progress-circle__value {
    font-size: var(--sh-font-size-xs, 0.75rem);
}

.sh-progress-circle--lg .sh-progress-circle__value {
    font-size: var(--sh-font-size-md, 1rem);
}

.sh-progress-circle--xl .sh-progress-circle__value {
    font-size: var(--sh-font-size-lg, 1.125rem);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_circle_creation() {
        let progress = ProgressCircle::new(75).max(100);

        assert_eq!(progress.value, 75);
        assert_eq!(progress.max, 100);
    }

    #[test]
    fn test_progress_circle_percentage() {
        let progress = ProgressCircle::new(50).max(200);
        assert!((progress.percentage() - 25.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_circle_max_clamp() {
        let progress = ProgressCircle::new(50).max(0);
        assert_eq!(progress.max, 1);
    }

    #[test]
    fn test_progress_circle_render() {
        let progress = ProgressCircle::new(80)
            .label("Progress")
            .variant(ProgressCircleVariant::Success);

        let html = progress.render().into_string();
        assert!(html.contains("sh-progress-circle"));
        assert!(html.contains("sh-progress-circle--success"));
    }

    #[test]
    fn test_css_generation() {
        let css = progress_circle_css();
        assert!(css.contains(".sh-progress-circle"));
    }
}
