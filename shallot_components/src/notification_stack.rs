//! Notification Stack Component - Stack of toast notifications
//! CSS-only positioning with support for multiple positions

use maud::{html, Markup, Render};

/// Position of the notification stack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationPosition {
    TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
}

/// A single notification item
#[derive(Debug, Clone)]
pub struct NotificationItem<'a> {
    /// Unique ID
    pub id: &'a str,
    /// Notification title
    pub title: &'a str,
    /// Notification message
    pub message: Option<&'a str>,
    /// Notification type
    pub kind: NotificationKind,
    /// Dismissible
    pub dismissible: bool,
}

/// Type of notification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// A stack of notifications
#[derive(Debug, Clone)]
pub struct NotificationStack<'a> {
    /// Position on screen
    pub position: NotificationPosition,
    /// List of notifications
    pub notifications: Vec<NotificationItem<'a>>,
    /// Maximum visible notifications
    pub max_visible: usize,
}

impl<'a> NotificationItem<'a> {
    /// Create a new notification item
    pub fn new(id: &'a str, title: &'a str) -> Self {
        Self {
            id,
            title,
            message: None,
            kind: NotificationKind::default(),
            dismissible: true,
        }
    }

    /// Set the message
    pub fn message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }

    /// Set the kind/type
    pub fn kind(mut self, kind: NotificationKind) -> Self {
        self.kind = kind;
        self
    }

    /// Set dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Build CSS classes for the item
    fn build_classes(&self) -> String {
        let kind_class = match self.kind {
            NotificationKind::Info => "sh-notification--info",
            NotificationKind::Success => "sh-notification--success",
            NotificationKind::Warning => "sh-notification--warning",
            NotificationKind::Error => "sh-notification--error",
        };

        let mut classes = vec!["sh-notification", kind_class];

        if self.dismissible {
            classes.push("sh-notification--dismissible");
        }

        classes.join(" ")
    }

    /// Render the notification item
    pub fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div
                id=(self.id)
                class=(classes)
                role="alert"
                aria-live="polite"
            {
                @if self.dismissible {
                    details class="sh-notification__wrapper" {
                        summary class="sh-notification__dismiss" aria-label="Dismiss notification" {
                            span class="sh-notification__icon sh-notification__icon--dismiss" {
                                "×"
                            }
                        }
                        div class="sh-notification__content" {
                            span class="sh-notification__title" {
                                (self.title)
                            }
                            @if let Some(message) = self.message {
                                p class="sh-notification__message" {
                                    (message)
                                }
                            }
                        }
                    }
                } @else {
                    div class="sh-notification__wrapper" {
                        div class="sh-notification__content" {
                            span class="sh-notification__title" {
                                (self.title)
                            }
                            @if let Some(message) = self.message {
                                p class="sh-notification__message" {
                                    (message)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<'a> NotificationStack<'a> {
    /// Create a new notification stack
    pub fn new() -> Self {
        Self {
            position: NotificationPosition::default(),
            notifications: Vec::new(),
            max_visible: 5,
        }
    }

    /// Set the position
    pub fn position(mut self, position: NotificationPosition) -> Self {
        self.position = position;
        self
    }

    /// Add a notification
    pub fn add(mut self, notification: NotificationItem<'a>) -> Self {
        self.notifications.push(notification);
        self
    }

    /// Set max visible
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Build CSS classes
    fn build_classes(&self) -> String {
        let position_class = match self.position {
            NotificationPosition::TopLeft => "sh-notification-stack--top-left",
            NotificationPosition::TopRight => "sh-notification-stack--top-right",
            NotificationPosition::BottomLeft => "sh-notification-stack--bottom-left",
            NotificationPosition::BottomRight => "sh-notification-stack--bottom-right",
            NotificationPosition::TopCenter => "sh-notification-stack--top-center",
            NotificationPosition::BottomCenter => "sh-notification-stack--bottom-center",
        };

        format!("sh-notification-stack {}", position_class)
    }
}

impl<'a> Default for NotificationStack<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for NotificationStack<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let visible_notifications: Vec<_> =
            self.notifications.iter().take(self.max_visible).collect();

        html! {
            div
                class=(classes)
                role="region"
                aria-label="Notifications"
            {
                @for notification in visible_notifications {
                    (notification.render())
                }

                @if self.notifications.len() > self.max_visible {
                    div class="sh-notification-stack__overflow" {
                        span {
                            "+" (self.notifications.len() - self.max_visible) " more"
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for the notification stack component
pub fn notification_stack_css() -> String {
    r#"
.sh-notification-stack {
    position: fixed;
    z-index: var(--sh-z-notification, 1000);
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-sm, 0.5rem);
    max-width: 400px;
    width: calc(100% - 2rem);
}

.sh-notification-stack--top-left {
    top: var(--sh-spacing-md, 1rem);
    left: var(--sh-spacing-md, 1rem);
}

.sh-notification-stack--top-right {
    top: var(--sh-spacing-md, 1rem);
    right: var(--sh-spacing-md, 1rem);
}

.sh-notification-stack--bottom-left {
    bottom: var(--sh-spacing-md, 1rem);
    left: var(--sh-spacing-md, 1rem);
    flex-direction: column-reverse;
}

.sh-notification-stack--bottom-right {
    bottom: var(--sh-spacing-md, 1rem);
    right: var(--sh-spacing-md, 1rem);
    flex-direction: column-reverse;
}

.sh-notification-stack--top-center {
    top: var(--sh-spacing-md, 1rem);
    left: 50%;
    transform: translateX(-50%);
}

.sh-notification-stack--bottom-center {
    bottom: var(--sh-spacing-md, 1rem);
    left: 50%;
    transform: translateX(-50%);
    flex-direction: column-reverse;
}

.sh-notification-stack__overflow {
    padding: var(--sh-spacing-xs, 0.25rem) var(--sh-spacing-sm, 0.5rem);
    background: var(--sh-color-surface, #f5f5f5);
    border-radius: var(--sh-radius-md, 0.375rem);
    font-size: var(--sh-font-size-xs, 0.75rem);
    color: var(--sh-color-text-muted, #666);
    text-align: center;
}

/* Individual notification styles */
.sh-notification {
    background: white;
    border-radius: var(--sh-radius-lg, 0.5rem);
    box-shadow: var(--sh-shadow-lg, 0 10px 15px -3px rgba(0, 0, 0, 0.1));
    overflow: hidden;
    animation: sh-slide-in 0.3s ease-out;
}

@keyframes sh-slide-in {
    from {
        opacity: 0;
        transform: translateX(100%);
    }
    to {
        opacity: 1;
        transform: translateX(0);
    }
}

.sh-notification--info {
    border-left: 4px solid var(--sh-color-info, #3b82f6);
}

.sh-notification--success {
    border-left: 4px solid var(--sh-color-success, #22c55e);
}

.sh-notification--warning {
    border-left: 4px solid var(--sh-color-warning, #f59e0b);
}

.sh-notification--error {
    border-left: 4px solid var(--sh-color-danger, #ef4444);
}

.sh-notification__wrapper {
    display: flex;
    align-items: flex-start;
    padding: var(--sh-spacing-md, 1rem);
}

.sh-notification__dismiss {
    list-style: none;
    cursor: pointer;
    margin-right: var(--sh-spacing-sm, 0.5rem);
    color: var(--sh-color-text-muted, #666);
    font-size: 1.25rem;
    line-height: 1;
}

.sh-notification__dismiss::-webkit-details-marker {
    display: none;
}

.sh-notification__content {
    flex: 1;
    min-width: 0;
}

.sh-notification__title {
    font-weight: var(--sh-font-weight-semibold, 600);
    color: var(--sh-color-text, #1a1a1a);
    display: block;
}

.sh-notification__message {
    font-size: var(--sh-font-size-sm, 0.875rem);
    color: var(--sh-color-text-muted, #666);
    margin: var(--sh-spacing-xs, 0.25rem) 0 0 0;
}

/* Dismiss animation */
details.sh-notification__wrapper[open] .sh-notification__content,
details.sh-notification__wrapper:not([open]) .sh-notification__content {
    display: block;
}

details.sh-notification__wrapper:not([open]) + .sh-notification__content,
details.sh-notification__wrapper[open] .sh-notification__dismiss {
    visibility: visible;
}

/* Hide notification when dismissed */
details.sh-notification__wrapper:not([open]) {
    animation: sh-fade-out 0.2s ease-out forwards;
}

@keyframes sh-fade-out {
    to {
        opacity: 0;
        height: 0;
        padding: 0;
        margin: 0;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_item_creation() {
        let item = NotificationItem::new("notif-1", "Hello")
            .message("This is a message")
            .kind(NotificationKind::Success);

        assert_eq!(item.id, "notif-1");
        assert_eq!(item.title, "Hello");
        assert_eq!(item.message, Some("This is a message"));
        assert_eq!(item.kind, NotificationKind::Success);
    }

    #[test]
    fn test_notification_item_render() {
        let item = NotificationItem::new("test", "Test notification");
        let rendered = item.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-notification"));
        assert!(html.contains("Test notification"));
    }

    #[test]
    fn test_notification_stack_creation() {
        let stack = NotificationStack::new()
            .position(NotificationPosition::BottomRight)
            .max_visible(3);

        assert_eq!(stack.position, NotificationPosition::BottomRight);
        assert_eq!(stack.max_visible, 3);
    }

    #[test]
    fn test_notification_stack_render() {
        let stack = NotificationStack::new()
            .add(NotificationItem::new("1", "First"))
            .add(NotificationItem::new("2", "Second"));

        let rendered = stack.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-notification-stack"));
        assert!(html.contains("First"));
        assert!(html.contains("Second"));
    }

    #[test]
    fn test_notification_kinds() {
        let info = NotificationItem::new("info", "Info").kind(NotificationKind::Info);
        let error = NotificationItem::new("error", "Error").kind(NotificationKind::Error);

        assert!(info.build_classes().contains("sh-notification--info"));
        assert!(error.build_classes().contains("sh-notification--error"));
    }

    #[test]
    fn test_css_generation() {
        let css = notification_stack_css();
        assert!(css.contains(".sh-notification-stack"));
        assert!(css.contains(".sh-notification"));
    }
}
