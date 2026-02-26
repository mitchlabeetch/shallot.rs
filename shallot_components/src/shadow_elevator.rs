//! ShadowElevator Component - Dynamic Depth Shadow System
//!
//! A component that creates dynamic elevation shadows using CSS.
//! Simulates material design elevation levels.

use maud::{html, Markup, Render};

/// Shadow elevation levels (0-5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShadowLevel {
    #[default]
    Level0, // No shadow
    Level1, // Subtle
    Level2, // Default
    Level3, // Elevated
    Level4, // High
    Level5, // Maximum
}

impl ShadowLevel {
    fn css_class(&self) -> &'static str {
        match self {
            ShadowLevel::Level0 => "sh-shadow--0",
            ShadowLevel::Level1 => "sh-shadow--1",
            ShadowLevel::Level2 => "sh-shadow--2",
            ShadowLevel::Level3 => "sh-shadow--3",
            ShadowLevel::Level4 => "sh-shadow--4",
            ShadowLevel::Level5 => "sh-shadow--5",
        }
    }
}

/// ShadowElevator component
pub struct ShadowElevator<'a> {
    children: Markup,
    level: ShadowLevel,
    interactive: bool,
    class: Option<&'a str>,
}

impl<'a> ShadowElevator<'a> {
    /// Create a new ShadowElevator
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            level: ShadowLevel::default(),
            interactive: false,
            class: None,
        }
    }

    /// Set shadow level
    pub fn level(mut self, level: ShadowLevel) -> Self {
        self.level = level;
        self
    }

    /// Enable interactive hover effect
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-shadow-elevator".to_string()];
        classes.push(self.level.css_class().to_string());
        if self.interactive {
            classes.push("sh-shadow-elevator--interactive".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for ShadowElevator<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div
                class=(classes)
                role="group"
                aria-label="Elevated content"
            {
                (self.children)
            }
        }
    }
}

/// Generate CSS for ShadowElevator component
pub fn shadow_elevator_css() -> String {
    r#"
.sh-shadow-elevator {
    background: var(--sh-surface, #fff);
    border-radius: var(--sh-radius-lg, 0.5rem);
    transition: box-shadow 0.3s cubic-bezier(0.4, 0, 0.2, 1),
                transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

/* Elevation levels */
.sh-shadow--0 {
    box-shadow: none;
}

.sh-shadow--1 {
    box-shadow:
        0 1px 2px rgba(0, 0, 0, 0.05),
        0 1px 3px rgba(0, 0, 0, 0.1);
}

.sh-shadow--2 {
    box-shadow:
        0 4px 6px rgba(0, 0, 0, 0.05),
        0 10px 15px rgba(0, 0, 0, 0.1);
}

.sh-shadow--3 {
    box-shadow:
        0 10px 15px rgba(0, 0, 0, 0.05),
        0 20px 25px rgba(0, 0, 0, 0.1);
}

.sh-shadow--4 {
    box-shadow:
        0 15px 20px rgba(0, 0, 0, 0.05),
        0 30px 40px rgba(0, 0, 0, 0.1);
}

.sh-shadow--5 {
    box-shadow:
        0 20px 25px rgba(0, 0, 0, 0.05),
        0 40px 60px rgba(0, 0, 0, 0.15);
}

/* Interactive hover - elevates on hover */
.sh-shadow-elevator--interactive:hover {
    transform: translateY(-4px);
}

.sh-shadow-elevator--interactive.sh-shadow--0:hover {
    box-shadow:
        0 4px 6px rgba(0, 0, 0, 0.05),
        0 10px 15px rgba(0, 0, 0, 0.1);
}

.sh-shadow-elevator--interactive.sh-shadow--1:hover {
    box-shadow:
        0 10px 15px rgba(0, 0, 0, 0.05),
        0 20px 25px rgba(0, 0, 0, 0.1);
}

.sh-shadow-elevator--interactive.sh-shadow--2:hover {
    box-shadow:
        0 15px 20px rgba(0, 0, 0, 0.05),
        0 30px 40px rgba(0, 0, 0, 0.1);
}

.sh-shadow-elevator--interactive.sh-shadow--3:hover {
    box-shadow:
        0 20px 25px rgba(0, 0, 0, 0.05),
        0 40px 60px rgba(0, 0, 0, 0.15);
}

.sh-shadow-elevator--interactive.sh-shadow--4:hover,
.sh-shadow-elevator--interactive.sh-shadow--5:hover {
    box-shadow:
        0 25px 30px rgba(0, 0, 0, 0.05),
        0 50px 80px rgba(0, 0, 0, 0.2);
}

/* Active/pressed state */
.sh-shadow-elevator--interactive:active {
    transform: translateY(-1px);
}

/* Focus state */
.sh-shadow-elevator--interactive:focus-visible {
    outline: 2px solid var(--sh-primary, #3b82f6);
    outline-offset: 2px;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
    .sh-shadow--1 {
        box-shadow:
            0 1px 2px rgba(0, 0, 0, 0.1),
            0 1px 3px rgba(0, 0, 0, 0.2);
    }

    .sh-shadow--2 {
        box-shadow:
            0 4px 6px rgba(0, 0, 0, 0.1),
            0 10px 15px rgba(0, 0, 0, 0.2);
    }

    .sh-shadow--3 {
        box-shadow:
            0 10px 15px rgba(0, 0, 0, 0.1),
            0 20px 25px rgba(0, 0, 0, 0.2);
    }

    .sh-shadow--4 {
        box-shadow:
            0 15px 20px rgba(0, 0, 0, 0.1),
            0 30px 40px rgba(0, 0, 0, 0.25);
    }

    .sh-shadow--5 {
        box-shadow:
            0 20px 25px rgba(0, 0, 0, 0.1),
            0 40px 60px rgba(0, 0, 0, 0.3);
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_creation() {
        let shadow = ShadowElevator::new(html! { "Content" });
        assert_eq!(shadow.level, ShadowLevel::Level0);
    }

    #[test]
    fn test_shadow_level() {
        let shadow = ShadowElevator::new(html! {}).level(ShadowLevel::Level5);
        assert_eq!(shadow.level, ShadowLevel::Level5);
    }

    #[test]
    fn test_shadow_interactive() {
        let shadow = ShadowElevator::new(html! {}).interactive(true);
        assert!(shadow.interactive);
    }

    #[test]
    fn test_shadow_css() {
        let css = shadow_elevator_css();
        assert!(css.contains(".sh-shadow-elevator"));
        assert!(css.contains(".sh-shadow--1"));
        assert!(css.contains(".sh-shadow--5"));
    }
}
