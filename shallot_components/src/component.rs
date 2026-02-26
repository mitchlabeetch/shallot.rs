//! Component Base Traits and Types
//!
//! This module provides unified interfaces for all Shallot components:
//! - Component trait for consistent API
//! - Style variants (sizes, variants, colors)
//! - Common builder patterns
//! - ARIA attribute helpers

use maud::Render;
use shallot_foundation::Icon;

/// The core Component trait that all UI components should implement
pub trait Component: Render {
    /// Get the component's CSS classes
    fn classes(&self) -> String;

    /// Get the component's inline styles (if any)
    fn styles(&self) -> Option<String> {
        None
    }

    /// Check if the component is interactive
    fn is_interactive(&self) -> bool {
        false
    }

    /// Check if the component is disabled
    fn is_disabled(&self) -> bool {
        false
    }
}

/// Component size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentSize {
    #[default]
    Md,
    Xs,
    Sm,
    Lg,
    Xl,
}

impl ComponentSize {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            ComponentSize::Xs => "xs",
            ComponentSize::Sm => "sm",
            ComponentSize::Md => "md",
            ComponentSize::Lg => "lg",
            ComponentSize::Xl => "xl",
        }
    }

    /// Get the pixel size for fixed-size components
    pub fn px(&self) -> u8 {
        match self {
            ComponentSize::Xs => 24,
            ComponentSize::Sm => 32,
            ComponentSize::Md => 40,
            ComponentSize::Lg => 48,
            ComponentSize::Xl => 56,
        }
    }
}

/// Component color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentColor {
    #[default]
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Info,
    Neutral,
}

impl ComponentColor {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            ComponentColor::Primary => "primary",
            ComponentColor::Secondary => "secondary",
            ComponentColor::Accent => "accent",
            ComponentColor::Success => "success",
            ComponentColor::Warning => "warning",
            ComponentColor::Error => "error",
            ComponentColor::Info => "info",
            ComponentColor::Neutral => "neutral",
        }
    }

    /// Get the CSS variable name for this color
    pub fn css_var(&self) -> &'static str {
        match self {
            ComponentColor::Primary => "var(--sh-accent)",
            ComponentColor::Secondary => "var(--sh-accent-2)",
            ComponentColor::Accent => "var(--sh-accent)",
            ComponentColor::Success => "var(--sh-success)",
            ComponentColor::Warning => "var(--sh-warning)",
            ComponentColor::Error => "var(--sh-error)",
            ComponentColor::Info => "var(--sh-info, var(--sh-accent))",
            ComponentColor::Neutral => "var(--sh-text-muted)",
        }
    }
}

/// Component visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentVariant {
    #[default]
    Solid,
    Outline,
    Ghost,
    Soft,
    Link,
}

impl ComponentVariant {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            ComponentVariant::Solid => "solid",
            ComponentVariant::Outline => "outline",
            ComponentVariant::Ghost => "ghost",
            ComponentVariant::Soft => "soft",
            ComponentVariant::Link => "link",
        }
    }
}

/// Component shape variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentShape {
    #[default]
    Default,
    Square,
    Rounded,
    Pill,
    Circle,
}

impl ComponentShape {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            ComponentShape::Default => "default",
            ComponentShape::Square => "square",
            ComponentShape::Rounded => "rounded",
            ComponentShape::Pill => "pill",
            ComponentShape::Circle => "circle",
        }
    }
}

/// ARIA attributes configuration for accessibility
#[derive(Debug, Clone, Default)]
pub struct AriaAttrs {
    pub label: Option<String>,
    pub labelled_by: Option<String>,
    pub described_by: Option<String>,
    pub expanded: Option<bool>,
    pub pressed: Option<bool>,
    pub selected: Option<bool>,
    pub hidden: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub invalid: Option<bool>,
    pub busy: Option<bool>,
    pub live: Option<AriaLive>,
    pub atomic: Option<bool>,
    pub relevance: Option<AriaRelevance>,
    pub controls: Option<String>,
    pub owns: Option<String>,
    pub has_popup: Option<AriaPopup>,
    pub role: Option<String>,
}

impl AriaAttrs {
    /// Create a new empty ARIA configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set aria-label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set aria-labelledby
    pub fn labelled_by(mut self, id: impl Into<String>) -> Self {
        self.labelled_by = Some(id.into());
        self
    }

    /// Set aria-describedby
    pub fn described_by(mut self, id: impl Into<String>) -> Self {
        self.described_by = Some(id.into());
        self
    }

    /// Set aria-expanded
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    /// Set aria-pressed
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    /// Set aria-selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Set aria-hidden
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }

    /// Set aria-disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set aria-readonly
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = Some(readonly);
        self
    }

    /// Set aria-required
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Set aria-invalid
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = Some(invalid);
        self
    }

    /// Set aria-busy
    pub fn busy(mut self, busy: bool) -> Self {
        self.busy = Some(busy);
        self
    }

    /// Set aria-live
    pub fn live(mut self, live: AriaLive) -> Self {
        self.live = Some(live);
        self
    }

    /// Set aria-atomic
    pub fn atomic(mut self, atomic: bool) -> Self {
        self.atomic = Some(atomic);
        self
    }

    /// Set aria-controls
    pub fn controls(mut self, id: impl Into<String>) -> Self {
        self.controls = Some(id.into());
        self
    }

    /// Set aria-owns
    pub fn owns(mut self, id: impl Into<String>) -> Self {
        self.owns = Some(id.into());
        self
    }

    /// Set aria-haspopup
    pub fn has_popup(mut self, popup: AriaPopup) -> Self {
        self.has_popup = Some(popup);
        self
    }

    /// Set role attribute
    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }
}

/// ARIA live region types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaLive {
    Off,
    Polite,
    Assertive,
}

impl AriaLive {
    /// Convert to string value
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaLive::Off => "off",
            AriaLive::Polite => "polite",
            AriaLive::Assertive => "assertive",
        }
    }
}

/// ARIA relevance types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRelevance {
    Additions,
    Removals,
    Text,
    All,
}

impl AriaRelevance {
    /// Convert to string value
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaRelevance::Additions => "additions",
            AriaRelevance::Removals => "removals",
            AriaRelevance::Text => "text",
            AriaRelevance::All => "all",
        }
    }
}

/// ARIA popup types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaPopup {
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
    True,
}

impl AriaPopup {
    /// Convert to string value
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaPopup::Menu => "menu",
            AriaPopup::Listbox => "listbox",
            AriaPopup::Tree => "tree",
            AriaPopup::Grid => "grid",
            AriaPopup::Dialog => "dialog",
            AriaPopup::True => "true",
        }
    }
}

/// Common component state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentState {
    #[default]
    Default,
    Hover,
    Focus,
    Active,
    Disabled,
    Loading,
    Selected,
    Checked,
    Indeterminate,
}

impl ComponentState {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            ComponentState::Default => "default",
            ComponentState::Hover => "hover",
            ComponentState::Focus => "focus",
            ComponentState::Active => "active",
            ComponentState::Disabled => "disabled",
            ComponentState::Loading => "loading",
            ComponentState::Selected => "selected",
            ComponentState::Checked => "checked",
            ComponentState::Indeterminate => "indeterminate",
        }
    }
}

/// Component with icon configuration
#[derive(Debug, Clone)]
pub struct IconConfig {
    pub icon: Icon,
    pub position: IconPosition,
}

impl IconConfig {
    /// Create a new icon configuration
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            position: IconPosition::Left,
        }
    }

    /// Set the icon position
    pub fn position(mut self, position: IconPosition) -> Self {
        self.position = position;
        self
    }
}

/// Icon position relative to content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
    Above,
    Below,
}

impl IconPosition {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            IconPosition::Left => "left",
            IconPosition::Right => "right",
            IconPosition::Above => "above",
            IconPosition::Below => "below",
        }
    }
}

/// Spacing scale for component gaps/padding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Spacing {
    /// Get the CSS variable value
    pub fn css_var(&self) -> &'static str {
        match self {
            Spacing::None => "0",
            Spacing::Xs => "var(--sh-spacing-1, 0.25rem)",
            Spacing::Sm => "var(--sh-spacing-2, 0.5rem)",
            Spacing::Md => "var(--sh-spacing-4, 1rem)",
            Spacing::Lg => "var(--sh-spacing-6, 1.5rem)",
            Spacing::Xl => "var(--sh-spacing-8, 2rem)",
        }
    }

    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Spacing::None => "none",
            Spacing::Xs => "xs",
            Spacing::Sm => "sm",
            Spacing::Md => "md",
            Spacing::Lg => "lg",
            Spacing::Xl => "xl",
        }
    }
}

/// Generate base component CSS
pub fn component_base_css() -> String {
    r#"
/* Component Base Styles */
.sh-component {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: var(--sh-font-sans);
  box-sizing: border-box;
}

.sh-component:focus-visible {
  outline: 2px solid var(--sh-accent);
  outline-offset: 2px;
}

/* Size variants */
.sh-component--xs { font-size: 0.75rem; }
.sh-component--sm { font-size: 0.875rem; }
.sh-component--md { font-size: 1rem; }
.sh-component--lg { font-size: 1.125rem; }
.sh-component--xl { font-size: 1.25rem; }

/* Color variants */
.sh-component--primary { color: var(--sh-accent); }
.sh-component--secondary { color: var(--sh-accent-2); }
.sh-component--success { color: var(--sh-success); }
.sh-component--warning { color: var(--sh-warning); }
.sh-component--error { color: var(--sh-error); }

/* State variants */
.sh-component--disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.sh-component--loading {
  cursor: wait;
  pointer-events: none;
}

/* Shape variants */
.sh-component--square { border-radius: var(--sh-radius-sm); }
.sh-component--rounded { border-radius: var(--sh-radius-md); }
.sh-component--pill { border-radius: 9999px; }
.sh-component--circle { border-radius: 50%; }

/* Icon positioning */
.sh-component__icon--left { margin-right: 0.5rem; }
.sh-component__icon--right { margin-left: 0.5rem; }
.sh-component__icon--above { margin-bottom: 0.25rem; }
.sh-component__icon--below { margin-top: 0.25rem; }
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_size() {
        assert_eq!(ComponentSize::Xs.class_suffix(), "xs");
        assert_eq!(ComponentSize::Md.px(), 40);
    }

    #[test]
    fn test_component_color() {
        assert_eq!(ComponentColor::Primary.class_suffix(), "primary");
        assert!(ComponentColor::Error.css_var().contains("error"));
    }

    #[test]
    fn test_component_variant() {
        assert_eq!(ComponentVariant::Outline.class_suffix(), "outline");
        assert_eq!(ComponentVariant::Ghost.class_suffix(), "ghost");
    }

    #[test]
    fn test_component_shape() {
        assert_eq!(ComponentShape::Pill.class_suffix(), "pill");
        assert_eq!(ComponentShape::Circle.class_suffix(), "circle");
    }

    #[test]
    fn test_aria_attrs_builder() {
        let attrs = AriaAttrs::new()
            .label("Test button")
            .expanded(false)
            .disabled(true)
            .controls("menu-id");

        assert_eq!(attrs.label, Some("Test button".to_string()));
        assert_eq!(attrs.expanded, Some(false));
        assert_eq!(attrs.disabled, Some(true));
        assert_eq!(attrs.controls, Some("menu-id".to_string()));
    }

    #[test]
    fn test_aria_live() {
        assert_eq!(AriaLive::Polite.as_str(), "polite");
        assert_eq!(AriaLive::Assertive.as_str(), "assertive");
    }

    #[test]
    fn test_icon_position() {
        assert_eq!(IconPosition::Left.class_suffix(), "left");
        assert_eq!(IconPosition::Right.class_suffix(), "right");
    }

    #[test]
    fn test_spacing() {
        assert_eq!(Spacing::Md.class_suffix(), "md");
        assert!(Spacing::Lg.css_var().contains("spacing"));
    }

    #[test]
    fn test_component_state() {
        assert_eq!(ComponentState::Loading.class_suffix(), "loading");
        assert_eq!(ComponentState::Disabled.class_suffix(), "disabled");
    }
}
