use maud::{html, Markup};

pub struct ProgressBar {
    pub value_0_100: u8,
}

impl ProgressBar {
    pub fn new(value: u8) -> Self {
        Self {
            value_0_100: value.min(100),
        }
    }

    pub fn render(self) -> Markup {
        let v = self.value_0_100.min(100);
        html! {
            div class="sh-progress" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow=(v) {
                div class="sh-progress__bar" style={(format!("width: {}%", v))} {}
            }
        }
    }
}

/// Generate CSS for progress component
pub fn progress_css() -> String {
    r#"
.sh-progress {
    width: 100%;
    height: 0.5rem;
    background: var(--sh-surface-2, #f3f4f6);
    border-radius: 9999px;
    overflow: hidden;
}

.sh-progress__bar {
    height: 100%;
    background: var(--sh-primary, #3b82f6);
    border-radius: 9999px;
    transition: width 0.3s ease;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_creation() {
        let progress = ProgressBar::new(50);
        assert_eq!(progress.value_0_100, 50);
    }

    #[test]
    fn test_progress_max_value() {
        let progress = ProgressBar::new(150);
        assert_eq!(progress.value_0_100, 100);
    }

    #[test]
    fn test_progress_css() {
        let css = progress_css();
        assert!(css.contains(".sh-progress"));
        assert!(css.contains(".sh-progress__bar"));
    }
}
