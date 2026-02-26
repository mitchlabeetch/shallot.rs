use maud::{html, Markup};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertKind {
    Info,
    Success,
    Warning,
    Error,
}

pub struct Alert<'a> {
    pub kind: AlertKind,
    pub title: &'a str,
    pub message: &'a str,
}

impl<'a> Alert<'a> {
    pub fn new(kind: AlertKind, title: &'a str, message: &'a str) -> Self {
        Self {
            kind,
            title,
            message,
        }
    }

    pub fn render(self) -> Markup {
        let class = match self.kind {
            AlertKind::Info => "sh-alert sh-alert--info",
            AlertKind::Success => "sh-alert sh-alert--success",
            AlertKind::Warning => "sh-alert sh-alert--warning",
            AlertKind::Error => "sh-alert sh-alert--error",
        };

        html! {
            div class=(class) role="alert" aria-live="polite" {
                div class="sh-alert__title" { (self.title) }
                div class="sh-alert__msg" { (self.message) }
            }
        }
    }
}

/// Generate CSS for alert component
pub fn alert_css() -> String {
    r#"
.sh-alert {
    padding: 1rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    border-left: 4px solid;
}

.sh-alert--info {
    background: var(--sh-info-bg, #eff6ff);
    border-color: var(--sh-info, #3b82f6);
    color: var(--sh-info-text, #1e40af);
}

.sh-alert--success {
    background: var(--sh-success-bg, #f0fdf4);
    border-color: var(--sh-success, #22c55e);
    color: var(--sh-success-text, #166534);
}

.sh-alert--warning {
    background: var(--sh-warning-bg, #fffbeb);
    border-color: var(--sh-warning, #f59e0b);
    color: var(--sh-warning-text, #92400e);
}

.sh-alert--error {
    background: var(--sh-error-bg, #fef2f2);
    border-color: var(--sh-error, #ef4444);
    color: var(--sh-error-text, #991b1b);
}

.sh-alert__title {
    font-weight: 600;
    margin-bottom: 0.25rem;
}

.sh-alert__msg {
    font-size: 0.875rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_creation() {
        let alert = Alert::new(AlertKind::Info, "Title", "Message");
        assert_eq!(alert.kind, AlertKind::Info);
        assert_eq!(alert.title, "Title");
        assert_eq!(alert.message, "Message");
    }

    #[test]
    fn test_alert_kind_values() {
        assert_eq!(AlertKind::Info as u8, 0);
        assert_eq!(AlertKind::Success as u8, 1);
        assert_eq!(AlertKind::Warning as u8, 2);
        assert_eq!(AlertKind::Error as u8, 3);
    }

    #[test]
    fn test_alert_css() {
        let css = alert_css();
        assert!(css.contains(".sh-alert"));
        assert!(css.contains(".sh-alert--success"));
    }
}
