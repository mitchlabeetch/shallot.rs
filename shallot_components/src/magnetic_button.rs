//! Magnetic Button Component - Mouse-following magnetic effect
//!
//! A button that creates a magnetic attraction effect when the mouse hovers nearby.
//! Pure CSS implementation using hover states and transforms.

use maud::{html, Markup, Render};

/// Magnetic button variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MagneticVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Accent,
}

/// Magnetic button size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MagneticSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Magnetic strength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MagneticStrength {
    Weak,
    #[default]
    Medium,
    Strong,
}

/// Magnetic Button Component
#[derive(Debug, Clone)]
pub struct MagneticButton<'a> {
    pub label: &'a str,
    pub variant: MagneticVariant,
    pub size: MagneticSize,
    pub strength: MagneticStrength,
    pub href: Option<&'a str>,
    pub disabled: bool,
    pub aria_label: Option<&'a str>,
}

impl<'a> MagneticButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: MagneticVariant::default(),
            size: MagneticSize::default(),
            strength: MagneticStrength::default(),
            href: None,
            disabled: false,
            aria_label: None,
        }
    }

    pub fn variant(mut self, variant: MagneticVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: MagneticSize) -> Self {
        self.size = size;
        self
    }

    pub fn strength(mut self, strength: MagneticStrength) -> Self {
        self.strength = strength;
        self
    }

    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn aria_label(mut self, label: &'a str) -> Self {
        self.aria_label = Some(label);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-magnetic-btn"];

        match self.variant {
            MagneticVariant::Primary => classes.push("sh-magnetic-btn--primary"),
            MagneticVariant::Secondary => classes.push("sh-magnetic-btn--secondary"),
            MagneticVariant::Ghost => classes.push("sh-magnetic-btn--ghost"),
            MagneticVariant::Accent => classes.push("sh-magnetic-btn--accent"),
        }

        match self.size {
            MagneticSize::Sm => classes.push("sh-magnetic-btn--sm"),
            MagneticSize::Md => classes.push("sh-magnetic-btn--md"),
            MagneticSize::Lg => classes.push("sh-magnetic-btn--lg"),
        }

        match self.strength {
            MagneticStrength::Weak => classes.push("sh-magnetic-btn--weak"),
            MagneticStrength::Medium => classes.push("sh-magnetic-btn--medium"),
            MagneticStrength::Strong => classes.push("sh-magnetic-btn--strong"),
        }

        if self.disabled {
            classes.push("sh-magnetic-btn--disabled");
        }

        classes.join(" ")
    }
}

impl<'a> Render for MagneticButton<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div class="sh-magnetic-wrapper" {
                @if let Some(href) = self.href {
                    a
                        class=(classes)
                        href=(href)
                        aria-label=[self.aria_label]
                        role="button"
                    {
                        span class="sh-magnetic-btn__content" {
                            (self.label)
                        }
                        span class="sh-magnetic-btn__magnet" {}
                    }
                } @else {
                    button
                        class=(classes)
                        type="button"
                        disabled?[self.disabled]
                        aria-label=[self.aria_label]
                    {
                        span class="sh-magnetic-btn__content" {
                            (self.label)
                        }
                        span class="sh-magnetic-btn__magnet" {}
                    }
                }
            }
        }
    }
}

/// Generate CSS for magnetic button components
pub fn magnetic_button_css() -> String {
    r#"
/* Magnetic Button Styles */
.sh-magnetic-wrapper {
    display: inline-block;
    position: relative;
}

.sh-magnetic-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.75rem 1.5rem;
    font-weight: 500;
    font-size: 1rem;
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
    overflow: hidden;
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    outline: none;
}

.sh-magnetic-btn__content {
    position: relative;
    z-index: 1;
}

.sh-magnetic-btn__magnet {
    position: absolute;
    inset: -20%;
    background: radial-gradient(circle, rgba(255,255,255,0.1) 0%, transparent 70%);
    opacity: 0;
    transition: opacity 0.3s ease;
    pointer-events: none;
}

/* Variants */
.sh-magnetic-btn--primary {
    background: var(--sh-primary);
    color: white;
}

.sh-magnetic-btn--secondary {
    background: var(--sh-secondary);
    color: white;
}

.sh-magnetic-btn--ghost {
    background: transparent;
    color: var(--sh-text);
    border: 1px solid var(--sh-border);
}

.sh-magnetic-btn--accent {
    background: var(--sh-accent);
    color: white;
}

/* Sizes */
.sh-magnetic-btn--sm {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
}

.sh-magnetic-btn--lg {
    padding: 1rem 2rem;
    font-size: 1.125rem;
}

/* Magnetic strength - transform on hover */
.sh-magnetic-btn--weak:hover {
    transform: scale(1.02);
}

.sh-magnetic-btn--medium:hover {
    transform: scale(1.05);
}

.sh-magnetic-btn--strong:hover {
    transform: scale(1.08);
}

.sh-magnetic-btn:hover .sh-magnetic-btn__magnet {
    opacity: 1;
}

.sh-magnetic-btn:active {
    transform: scale(0.98);
}

.sh-magnetic-btn--disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

/* Focus states */
.sh-magnetic-btn:focus-visible {
    box-shadow: 0 0 0 3px var(--sh-accent-muted);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-magnetic-btn {
        transition: none;
    }
    
    .sh-magnetic-btn:hover {
        transform: none;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetic_button_creation() {
        let btn = MagneticButton::new("Click Me")
            .variant(MagneticVariant::Primary)
            .size(MagneticSize::Lg);

        assert_eq!(btn.label, "Click Me");
        assert_eq!(btn.variant, MagneticVariant::Primary);
        assert_eq!(btn.size, MagneticSize::Lg);
    }

    #[test]
    fn test_magnetic_button_with_href() {
        let btn = MagneticButton::new("Link").href("/path");

        assert!(btn.href.is_some());
    }

    #[test]
    fn test_magnetic_strength() {
        let btn = MagneticButton::new("Strong").strength(MagneticStrength::Strong);

        assert_eq!(btn.strength, MagneticStrength::Strong);
    }
}
