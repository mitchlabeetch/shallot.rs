//! CapDrop Component - Initial Letter Drop Cap
//!
//! A decorative drop cap for the first letter of text blocks.
//! Creates an elegant typographic effect using CSS only.

use maud::{html, Markup, Render};

/// CapDrop size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CapDropSize {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

impl CapDropSize {
    fn css_class(&self) -> &'static str {
        match self {
            CapDropSize::Small => "sh-capdrop--sm",
            CapDropSize::Medium => "sh-capdrop--md",
            CapDropSize::Large => "sh-capdrop--lg",
            CapDropSize::XLarge => "sh-capdrop--xl",
        }
    }
}

/// CapDrop style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CapDropStyle {
    #[default]
    Raised, // Sits on baseline
    Sunk,       // Extends below baseline
    Inline,     // Stays in line
    Decorative, // With border/background
}

impl CapDropStyle {
    fn css_class(&self) -> &'static str {
        match self {
            CapDropStyle::Raised => "sh-capdrop--raised",
            CapDropStyle::Sunk => "sh-capdrop--sunk",
            CapDropStyle::Inline => "sh-capdrop--inline",
            CapDropStyle::Decorative => "sh-capdrop--decorative",
        }
    }
}

/// CapDrop component for decorative initial letters
pub struct CapDrop<'a> {
    letter: &'a str,
    content: &'a str,
    size: CapDropSize,
    style: CapDropStyle,
    lines: u8,
    class: Option<&'a str>,
}

impl<'a> CapDrop<'a> {
    /// Create a new CapDrop with letter and content
    pub fn new(letter: &'a str, content: &'a str) -> Self {
        Self {
            letter,
            content,
            size: CapDropSize::default(),
            style: CapDropStyle::default(),
            lines: 3,
            class: None,
        }
    }

    /// Set the size variant
    pub fn size(mut self, size: CapDropSize) -> Self {
        self.size = size;
        self
    }

    /// Set the style variant
    pub fn style(mut self, style: CapDropStyle) -> Self {
        self.style = style;
        self
    }

    /// Set number of lines the cap spans
    pub fn lines(mut self, lines: u8) -> Self {
        self.lines = lines.max(2).min(6);
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-capdrop".to_string()];
        classes.push(self.size.css_class().to_string());
        classes.push(self.style.css_class().to_string());
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for CapDrop<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let first_char = self.letter.chars().next().unwrap_or('A');
        let rest = &self.letter[1..];

        html! {
            p
                class=(classes)
                style=(format!("--sh-capdrop-lines: {}", self.lines))
                role="text"
                aria-label=(self.letter)
            {
                span
                    class="sh-capdrop__letter"
                    aria-hidden="true"
                {
                    (first_char)
                }
                @if !rest.is_empty() {
                    (rest)
                }
                (self.content)
            }
        }
    }
}

/// Generate CSS for CapDrop component
pub fn capdrop_css() -> String {
    r#"
.sh-capdrop {
    font-size: 1.125rem;
    line-height: 1.6;
    color: var(--sh-text, #1f2937);
}

.sh-capdrop__letter {
    float: left;
    font-weight: 700;
    line-height: 1;
    margin-right: 0.5rem;
    color: var(--sh-primary, #3b82f6);
}

/* Size variants */
.sh-capdrop--sm .sh-capdrop__letter {
    font-size: 2.5rem;
    margin-top: 0.25rem;
}

.sh-capdrop--md .sh-capdrop__letter {
    font-size: 3.5rem;
    margin-top: 0.5rem;
}

.sh-capdrop--lg .sh-capdrop__letter {
    font-size: 4.5rem;
    margin-top: 0.75rem;
}

.sh-capdrop--xl .sh-capdrop__letter {
    font-size: 6rem;
    margin-top: 1rem;
}

/* Style variants */
.sh-capdrop--raised .sh-capdrop__letter {
    margin-top: 0;
}

.sh-capdrop--sunk .sh-capdrop__letter {
    margin-top: 0.5rem;
}

.sh-capdrop--inline .sh-capdrop__letter {
    float: none;
    display: inline-block;
    vertical-align: baseline;
    margin-right: 0.25rem;
}

.sh-capdrop--decorative .sh-capdrop__letter {
    background: var(--sh-primary, #3b82f6);
    color: white;
    padding: 0.5rem 0.75rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

/* Line span control */
.sh-capdrop {
    --sh-capdrop-lines: 3;
}

.sh-capdrop--sunk .sh-capdrop__letter {
    height: calc(var(--sh-capdrop-lines) * 1.6em);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capdrop_creation() {
        let cap = CapDrop::new("A", "ncient text continues here...");
        assert_eq!(cap.letter, "A");
        assert_eq!(cap.size, CapDropSize::Medium);
    }

    #[test]
    fn test_capdrop_size() {
        let cap = CapDrop::new("W", "ord").size(CapDropSize::Large);
        assert_eq!(cap.size, CapDropSize::Large);
    }

    #[test]
    fn test_capdrop_style() {
        let cap = CapDrop::new("T", "ext").style(CapDropStyle::Decorative);
        assert_eq!(cap.style, CapDropStyle::Decorative);
    }

    #[test]
    fn test_capdrop_lines() {
        let cap = CapDrop::new("L", "orem").lines(4);
        assert_eq!(cap.lines, 4);
    }

    #[test]
    fn test_capdrop_css() {
        let css = capdrop_css();
        assert!(css.contains(".sh-capdrop"));
        assert!(css.contains(".sh-capdrop__letter"));
    }
}
