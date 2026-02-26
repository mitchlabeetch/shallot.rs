//! Tooltip Component
//!
//! CSS-only tooltips using hover states.

use maud::{html, Markup, Render};

pub struct Tooltip<'a> {
    content: Markup,
    text: &'a str,
    position: TooltipPosition,
    variant: TooltipVariant,
    delay: TooltipDelay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPosition {
    fn class(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "sh-tooltip--top",
            TooltipPosition::Bottom => "sh-tooltip--bottom",
            TooltipPosition::Left => "sh-tooltip--left",
            TooltipPosition::Right => "sh-tooltip--right",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipVariant {
    #[default]
    Default,
    Light,
    Dark,
    Primary,
}

impl TooltipVariant {
    fn class(&self) -> &'static str {
        match self {
            TooltipVariant::Default => "",
            TooltipVariant::Light => "sh-tooltip--light",
            TooltipVariant::Dark => "sh-tooltip--dark",
            TooltipVariant::Primary => "sh-tooltip--primary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipDelay {
    #[default]
    None,
    Fast,
    Normal,
    Slow,
}

impl TooltipDelay {
    fn class(&self) -> &'static str {
        match self {
            TooltipDelay::None => "",
            TooltipDelay::Fast => "sh-tooltip--delay-fast",
            TooltipDelay::Normal => "sh-tooltip--delay-normal",
            TooltipDelay::Slow => "sh-tooltip--delay-slow",
        }
    }
}

impl<'a> Tooltip<'a> {
    pub fn new(content: Markup, text: &'a str) -> Self {
        Self {
            content,
            text,
            position: TooltipPosition::Top,
            variant: TooltipVariant::Default,
            delay: TooltipDelay::Fast,
        }
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    pub fn variant(mut self, variant: TooltipVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn delay(mut self, delay: TooltipDelay) -> Self {
        self.delay = delay;
        self
    }
}

impl<'a> Render for Tooltip<'a> {
    fn render(&self) -> Markup {
        let classes = format!(
            "sh-tooltip {} {} {}",
            self.position.class(),
            self.variant.class(),
            self.delay.class()
        );

        html! {
            span class=(classes) {
                span class="sh-tooltip__trigger" {
                    (self.content.clone())
                }
                span class="sh-tooltip__content" role="tooltip" {
                    (self.text)
                }
            }
        }
    }
}

pub struct IconTooltip<'a> {
    icon: &'a str,
    text: &'a str,
    position: TooltipPosition,
}

impl<'a> IconTooltip<'a> {
    pub fn new(icon: &'a str, text: &'a str) -> Self {
        Self {
            icon,
            text,
            position: TooltipPosition::Top,
        }
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }
}

impl<'a> Render for IconTooltip<'a> {
    fn render(&self) -> Markup {
        let icon_html = html! {
            span class="sh-tooltip__icon" {
                (maud::PreEscaped(self.icon))
            }
        };

        Tooltip::new(icon_html, self.text)
            .position(self.position)
            .render()
    }
}

pub struct RichTooltip<'a> {
    content: Markup,
    title: &'a str,
    description: &'a str,
    position: TooltipPosition,
}

impl<'a> RichTooltip<'a> {
    pub fn new(content: Markup, title: &'a str, description: &'a str) -> Self {
        Self {
            content,
            title,
            description,
            position: TooltipPosition::Top,
        }
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }
}

impl<'a> Render for RichTooltip<'a> {
    fn render(&self) -> Markup {
        let position_class = self.position.class();

        html! {
            span class={(format!("sh-tooltip sh-tooltip--rich {}", position_class))} {
                span class="sh-tooltip__trigger" {
                    (self.content.clone())
                }
                span class="sh-tooltip__content sh-tooltip__content--rich" role="tooltip" {
                    span class="sh-tooltip__title" { (self.title) }
                    span class="sh-tooltip__description" { (self.description) }
                }
            }
        }
    }
}

pub fn tooltip_css() -> String {
    r#"
.sh-tooltip {
    position: relative;
    display: inline-flex;
}

.sh-tooltip__trigger {
    display: inline-flex;
}

.sh-tooltip__content {
    position: absolute;
    z-index: 100;
    padding: 0.5rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 500;
    line-height: 1.4;
    color: #fff;
    background: #1f2937;
    border-radius: var(--sh-radius-sm, 0.25rem);
    white-space: nowrap;
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.15s ease, visibility 0.15s ease;
    pointer-events: none;
}

.sh-tooltip:hover .sh-tooltip__content {
    opacity: 1;
    visibility: visible;
}

/* Position variants */
.sh-tooltip--top .sh-tooltip__content {
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%) translateY(-0.5rem);
    margin-bottom: 0.25rem;
}

.sh-tooltip--bottom .sh-tooltip__content {
    top: 100%;
    left: 50%;
    transform: translateX(-50%) translateY(0.5rem);
    margin-top: 0.25rem;
}

.sh-tooltip--left .sh-tooltip__content {
    right: 100%;
    top: 50%;
    transform: translateY(-50%) translateX(-0.5rem);
    margin-right: 0.25rem;
}

.sh-tooltip--right .sh-tooltip__content {
    left: 100%;
    top: 50%;
    transform: translateY(-50%) translateX(0.5rem);
    margin-left: 0.25rem;
}

/* Arrow */
.sh-tooltip__content::after {
    content: "";
    position: absolute;
    border: 5px solid transparent;
}

.sh-tooltip--top .sh-tooltip__content::after {
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border-top-color: #1f2937;
}

.sh-tooltip--bottom .sh-tooltip__content::after {
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    border-bottom-color: #1f2937;
}

.sh-tooltip--left .sh-tooltip__content::after {
    left: 100%;
    top: 50%;
    transform: translateY(-50%);
    border-left-color: #1f2937;
}

.sh-tooltip--right .sh-tooltip__content::after {
    right: 100%;
    top: 50%;
    transform: translateY(-50%);
    border-right-color: #1f2937;
}

/* Variant: Light */
.sh-tooltip--light .sh-tooltip__content {
    color: var(--sh-text, #1f2937);
    background: #fff;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.sh-tooltip--light.sh-tooltip--top .sh-tooltip__content::after {
    border-top-color: #fff;
}

.sh-tooltip--light.sh-tooltip--bottom .sh-tooltip__content::after {
    border-bottom-color: #fff;
}

.sh-tooltip--light.sh-tooltip--left .sh-tooltip__content::after {
    border-left-color: #fff;
}

.sh-tooltip--light.sh-tooltip--right .sh-tooltip__content::after {
    border-right-color: #fff;
}

/* Variant: Primary */
.sh-tooltip--primary .sh-tooltip__content {
    background: var(--sh-accent, #3b82f6);
}

.sh-tooltip--primary.sh-tooltip--top .sh-tooltip__content::after {
    border-top-color: var(--sh-accent, #3b82f6);
}

.sh-tooltip--primary.sh-tooltip--bottom .sh-tooltip__content::after {
    border-bottom-color: var(--sh-accent, #3b82f6);
}

/* Delays */
.sh-tooltip--delay-fast .sh-tooltip__content {
    transition-delay: 0.1s;
}

.sh-tooltip--delay-normal .sh-tooltip__content {
    transition-delay: 0.3s;
}

.sh-tooltip--delay-slow .sh-tooltip__content {
    transition-delay: 0.5s;
}

/* Rich tooltip */
.sh-tooltip__content--rich {
    padding: 0.75rem 1rem;
    min-width: 180px;
    white-space: normal;
}

.sh-tooltip__title {
    display: block;
    font-size: 0.8125rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
}

.sh-tooltip__description {
    display: block;
    font-size: 0.75rem;
    font-weight: 400;
    opacity: 0.9;
}

/* Icon in trigger */
.sh-tooltip__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    color: var(--sh-text-muted, #6b7280);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_creation() {
        let content = html! { button { "Hover me" } };
        let tooltip = Tooltip::new(content, "Tooltip text")
            .position(TooltipPosition::Bottom)
            .variant(TooltipVariant::Dark);

        assert_eq!(tooltip.text, "Tooltip text");
        assert_eq!(tooltip.position, TooltipPosition::Bottom);
    }

    #[test]
    fn test_rich_tooltip() {
        let content = html! { span { "?" } };
        let tooltip = RichTooltip::new(content, "Title", "Description");

        assert_eq!(tooltip.title, "Title");
        assert_eq!(tooltip.description, "Description");
    }
}
