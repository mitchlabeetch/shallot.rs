//! Badge Component
//!
//! Small status indicators and labels inspired by DaisyUI.
//! Supports multiple colors, sizes, and styles.

use crate::component::{
    Component, ComponentColor, ComponentShape, ComponentSize,
};
use maud::{html, Markup, Render};
use shallot_foundation::Icon;

/// Badge component for status indicators and labels
pub struct Badge<'a> {
    /// Badge text content
    label: &'a str,
    /// Size variant
    size: ComponentSize,
    /// Color variant
    color: ComponentColor,
    /// Visual variant
    variant: BadgeVariant,
    /// Shape variant
    shape: ComponentShape,
    /// Optional icon
    icon: Option<Icon>,
    /// Icon position
    icon_position: IconPosition,
    /// Whether the badge is dismissible
    dismissible: bool,
    /// Custom CSS class
    custom_class: Option<&'a str>,
}

/// Badge visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Solid,
    Soft,
    Outline,
    Ghost,
}

/// Icon position for badge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
}

impl<'a> Badge<'a> {
    /// Create a new badge with the given label
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            size: ComponentSize::Md,
            color: ComponentColor::Primary,
            variant: BadgeVariant::Soft,
            shape: ComponentShape::Pill,
            icon: None,
            icon_position: IconPosition::Left,
            dismissible: false,
            custom_class: None,
        }
    }

    /// Set the badge size
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    /// Set the badge color
    pub fn color(mut self, color: ComponentColor) -> Self {
        self.color = color;
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the shape
    pub fn shape(mut self, shape: ComponentShape) -> Self {
        self.shape = shape;
        self
    }

    /// Add an icon
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set icon position
    pub fn icon_position(mut self, position: IconPosition) -> Self {
        self.icon_position = position;
        self
    }

    /// Make the badge dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Add custom CSS class
    pub fn custom_class(mut self, class: &'a str) -> Self {
        self.custom_class = Some(class);
        self
    }

    /// Build the CSS class string
    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-badge".to_string()];

        // Size
        classes.push(format!("sh-badge--{}", self.size.class_suffix()));

        // Color
        classes.push(format!("sh-badge--{}", self.color.class_suffix()));

        // Variant
        classes.push(format!(
            "sh-badge--{}",
            match self.variant {
                BadgeVariant::Solid => "solid",
                BadgeVariant::Soft => "soft",
                BadgeVariant::Outline => "outline",
                BadgeVariant::Ghost => "ghost",
            }
        ));

        // Shape
        classes.push(format!("sh-badge--{}", self.shape.class_suffix()));

        // Dismissible
        if self.dismissible {
            classes.push("sh-badge--dismissible".to_string());
        }

        // Icon position
        if self.icon.is_some() {
            classes.push(format!(
                "sh-badge--icon-{}",
                match self.icon_position {
                    IconPosition::Left => "left",
                    IconPosition::Right => "right",
                }
            ));
        }

        // Custom class
        if let Some(custom) = self.custom_class {
            classes.push(custom.to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for Badge<'a> {
    fn render(&self) -> Markup {
        let class = self.build_classes();

        html! {
            span class=(class) {
                @if let Some(icon) = &self.icon {
                    @if self.icon_position == IconPosition::Left {
                        span class="sh-badge__icon" {
                            (maud::PreEscaped(icon.to_svg_string()))
                        }
                    }
                }

                span class="sh-badge__label" { (self.label) }

                @if let Some(icon) = &self.icon {
                    @if self.icon_position == IconPosition::Right {
                        span class="sh-badge__icon" {
                            (maud::PreEscaped(icon.to_svg_string()))
                        }
                    }
                }

                @if self.dismissible {
                    button class="sh-badge__dismiss" type="button" aria-label="Dismiss" {
                        svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                            path d="M18 6 6 18M6 6l12 12" {}
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Component for Badge<'a> {
    fn classes(&self) -> String {
        self.build_classes()
    }
}

/// Status dot/badge for indicating status
pub struct StatusDot {
    color: ComponentColor,
    pulse: bool,
    size: ComponentSize,
}

impl StatusDot {
    pub fn new() -> Self {
        Self {
            color: ComponentColor::Success,
            pulse: false,
            size: ComponentSize::Md,
        }
    }

    pub fn color(mut self, color: ComponentColor) -> Self {
        self.color = color;
        self
    }

    pub fn pulse(mut self, pulse: bool) -> Self {
        self.pulse = pulse;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for StatusDot {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for StatusDot {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-status-dot sh-status-dot--{} sh-status-dot--{} {}",
            self.color.class_suffix(),
            self.size.class_suffix(),
            if self.pulse {
                "sh-status-dot--pulse"
            } else {
                ""
            }
        );

        html! {
            span class=(class) aria-hidden="true" {}
        }
    }
}

/// Count badge (notification count)
pub struct CountBadge {
    count: u32,
    max: Option<u32>,
    color: ComponentColor,
}

impl CountBadge {
    pub fn new(count: u32) -> Self {
        Self {
            count,
            max: Some(99),
            color: ComponentColor::Error,
        }
    }

    pub fn max(mut self, max: u32) -> Self {
        self.max = Some(max);
        self
    }

    pub fn color(mut self, color: ComponentColor) -> Self {
        self.color = color;
        self
    }

    fn format_count(&self) -> String {
        match self.max {
            Some(max) if self.count > max => format!("{}+", max),
            _ => self.count.to_string(),
        }
    }
}

impl Render for CountBadge {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-count-badge sh-count-badge--{}",
            self.color.class_suffix()
        );
        let display = self.format_count();

        html! {
            span class=(class) { (display) }
        }
    }
}

/// Generate CSS for badge components
pub fn badge_css() -> String {
    r#"
/* Badge Base */
.sh-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  font-weight: 500;
  line-height: 1;
  white-space: nowrap;
  transition: all 0.2s ease;
}

/* Size variants */
.sh-badge--xs {
  padding: 0.125rem 0.375rem;
  font-size: 0.625rem;
}

.sh-badge--sm {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
}

.sh-badge--md {
  padding: 0.375rem 0.75rem;
  font-size: 0.8125rem;
}

.sh-badge--lg {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
}

.sh-badge--xl {
  padding: 0.625rem 1.25rem;
  font-size: 1rem;
}

/* Shape variants */
.sh-badge--default {
  border-radius: var(--sh-radius-sm);
}

.sh-badge--pill {
  border-radius: 9999px;
}

.sh-badge--square {
  border-radius: 0;
}

.sh-badge--rounded {
  border-radius: var(--sh-radius-md);
}

/* Color + Variant combinations */

/* Primary */
.sh-badge--primary.sh-badge--solid {
  background: var(--sh-accent);
  color: white;
}

.sh-badge--primary.sh-badge--soft {
  background: color-mix(in srgb, var(--sh-accent) 15%, transparent);
  color: var(--sh-accent);
}

.sh-badge--primary.sh-badge--outline {
  background: transparent;
  border: 1px solid var(--sh-accent);
  color: var(--sh-accent);
}

.sh-badge--primary.sh-badge--ghost {
  background: transparent;
  color: var(--sh-accent);
}

.sh-badge--primary.sh-badge--ghost:hover {
  background: color-mix(in srgb, var(--sh-accent) 10%, transparent);
}

/* Secondary */
.sh-badge--secondary.sh-badge--solid {
  background: var(--sh-accent-2);
  color: white;
}

.sh-badge--secondary.sh-badge--soft {
  background: color-mix(in srgb, var(--sh-accent-2) 15%, transparent);
  color: var(--sh-accent-2);
}

.sh-badge--secondary.sh-badge--outline {
  background: transparent;
  border: 1px solid var(--sh-accent-2);
  color: var(--sh-accent-2);
}

/* Success */
.sh-badge--success.sh-badge--solid {
  background: var(--sh-success);
  color: white;
}

.sh-badge--success.sh-badge--soft {
  background: color-mix(in srgb, var(--sh-success) 15%, transparent);
  color: var(--sh-success);
}

.sh-badge--success.sh-badge--outline {
  background: transparent;
  border: 1px solid var(--sh-success);
  color: var(--sh-success);
}

/* Warning */
.sh-badge--warning.sh-badge--solid {
  background: var(--sh-warning);
  color: white;
}

.sh-badge--warning.sh-badge--soft {
  background: color-mix(in srgb, var(--sh-warning) 15%, transparent);
  color: var(--sh-warning);
}

.sh-badge--warning.sh-badge--outline {
  background: transparent;
  border: 1px solid var(--sh-warning);
  color: var(--sh-warning);
}

/* Error */
.sh-badge--error.sh-badge--solid {
  background: var(--sh-error);
  color: white;
}

.sh-badge--error.sh-badge--soft {
  background: color-mix(in srgb, var(--sh-error) 15%, transparent);
  color: var(--sh-error);
}

.sh-badge--error.sh-badge--outline {
  background: transparent;
  border: 1px solid var(--sh-error);
  color: var(--sh-error);
}

/* Neutral */
.sh-badge--neutral.sh-badge--solid {
  background: var(--sh-text-muted);
  color: white;
}

.sh-badge--neutral.sh-badge--soft {
  background: var(--sh-surface-2);
  color: var(--sh-text);
}

.sh-badge--neutral.sh-badge--outline {
  background: transparent;
  border: 1px solid var(--sh-border);
  color: var(--sh-text);
}

/* Icon */
.sh-badge__icon {
  display: inline-flex;
  flex-shrink: 0;
}

.sh-badge__icon svg {
  width: 1em;
  height: 1em;
}

/* Dismissible */
.sh-badge--dismissible {
  padding-right: 0.25rem;
}

.sh-badge__dismiss {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-left: 0.125rem;
  padding: 0.125rem;
  background: transparent;
  border: none;
  border-radius: var(--sh-radius-sm);
  cursor: pointer;
  opacity: 0.6;
  transition: opacity 0.2s;
}

.sh-badge__dismiss:hover {
  opacity: 1;
}

/* Status Dot */
.sh-status-dot {
  display: inline-block;
  border-radius: 50%;
}

.sh-status-dot--xs { width: 6px; height: 6px; }
.sh-status-dot--sm { width: 8px; height: 8px; }
.sh-status-dot--md { width: 10px; height: 10px; }
.sh-status-dot--lg { width: 12px; height: 12px; }
.sh-status-dot--xl { width: 14px; height: 14px; }

.sh-status-dot--primary { background: var(--sh-accent); }
.sh-status-dot--success { background: var(--sh-success); }
.sh-status-dot--warning { background: var(--sh-warning); }
.sh-status-dot--error { background: var(--sh-error); }
.sh-status-dot--neutral { background: var(--sh-text-muted); }

/* Pulse animation */
.sh-status-dot--pulse {
  position: relative;
}

.sh-status-dot--pulse::after {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 50%;
  background: inherit;
  opacity: 0.4;
  animation: status-pulse 2s ease-out infinite;
}

@keyframes status-pulse {
  0% { transform: scale(1); opacity: 0.4; }
  100% { transform: scale(3); opacity: 0; }
}

/* Count Badge */
.sh-count-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 1.25rem;
  height: 1.25rem;
  padding: 0 0.375rem;
  font-size: 0.6875rem;
  font-weight: 600;
  border-radius: 9999px;
  background: var(--sh-error);
  color: white;
}

.sh-count-badge--primary { background: var(--sh-accent); }
.sh-count-badge--success { background: var(--sh-success); }
.sh-count-badge--warning { background: var(--sh-warning); }
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_creation() {
        let badge = Badge::new("New")
            .color(ComponentColor::Success)
            .variant(BadgeVariant::Soft)
            .size(ComponentSize::Sm);

        assert_eq!(badge.label, "New");
        assert_eq!(badge.color, ComponentColor::Success);
        assert_eq!(badge.variant, BadgeVariant::Soft);
    }

    #[test]
    fn test_status_dot() {
        let dot = StatusDot::new().color(ComponentColor::Success).pulse(true);

        assert!(dot.pulse);
        assert_eq!(dot.color, ComponentColor::Success);
    }

    #[test]
    fn test_count_badge() {
        let badge = CountBadge::new(150).max(99);
        assert_eq!(badge.format_count(), "99+");

        let badge2 = CountBadge::new(50).max(99);
        assert_eq!(badge2.format_count(), "50");
    }
}
