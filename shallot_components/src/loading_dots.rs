//! Loading Dots Component - Animated loading indicator dots
//! CSS-only animation using keyframes

use maud::{html, Markup, Render};

/// Loading dots size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadingDotsSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Loading dots variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadingDotsVariant {
    #[default]
    Primary,
    Secondary,
    Muted,
}

/// Loading dots component
#[derive(Debug, Clone)]
pub struct LoadingDots {
    pub size: LoadingDotsSize,
    pub variant: LoadingDotsVariant,
    pub dot_count: u8,
    pub label: Option<&'static str>,
}

impl LoadingDots {
    /// Create new loading dots
    pub fn new() -> Self {
        Self {
            size: LoadingDotsSize::default(),
            variant: LoadingDotsVariant::default(),
            dot_count: 3,
            label: Some("Loading"),
        }
    }

    /// Set the size
    pub fn size(mut self, size: LoadingDotsSize) -> Self {
        self.size = size;
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: LoadingDotsVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the number of dots
    pub fn dot_count(mut self, count: u8) -> Self {
        self.dot_count = count.clamp(2, 5);
        self
    }

    /// Set the accessibility label
    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-loading-dots".to_string()];

        let size_class = match self.size {
            LoadingDotsSize::Sm => "sh-loading-dots--sm",
            LoadingDotsSize::Md => "sh-loading-dots--md",
            LoadingDotsSize::Lg => "sh-loading-dots--lg",
        };
        classes.push(size_class.to_string());

        let variant_class = match self.variant {
            LoadingDotsVariant::Primary => "sh-loading-dots--primary",
            LoadingDotsVariant::Secondary => "sh-loading-dots--secondary",
            LoadingDotsVariant::Muted => "sh-loading-dots--muted",
        };
        classes.push(variant_class.to_string());

        classes.join(" ")
    }
}

impl Default for LoadingDots {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for LoadingDots {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let label = self.label.unwrap_or("Loading");

        html! {
            div
                class=(classes)
                role="status"
                aria-label=(label) {
                @for i in 0..self.dot_count {
                    span class="sh-loading-dots__dot" style={"animation-delay: " (i * 150) "ms;"} {}
                }
                span class="sh-visually-hidden" {
                    (label)
                }
            }
        }
    }
}

/// Generate CSS for loading dots component
pub fn loading_dots_css() -> String {
    r#"
.sh-loading-dots {
    display: inline-flex;
    align-items: center;
    gap: var(--sh-spacing-1, 0.25rem);
}

.sh-loading-dots__dot {
    width: 0.5em;
    height: 0.5em;
    border-radius: 50%;
    background-color: currentColor;
    animation: sh-loading-dots-bounce 1.4s infinite ease-in-out both;
}

.sh-loading-dots--sm {
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-loading-dots--md {
    font-size: var(--sh-font-size-base, 1rem);
}

.sh-loading-dots--lg {
    font-size: var(--sh-font-size-xl, 1.25rem);
}

.sh-loading-dots--primary {
    color: var(--sh-color-primary, #3b82f6);
}

.sh-loading-dots--secondary {
    color: var(--sh-color-secondary, #6b7280);
}

.sh-loading-dots--muted {
    color: var(--sh-color-muted-foreground, #9ca3af);
}

@keyframes sh-loading-dots-bounce {
    0%, 80%, 100% {
        transform: scale(0);
        opacity: 0.4;
    }
    40% {
        transform: scale(1);
        opacity: 1;
    }
}

.sh-visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_dots_creation() {
        let dots = LoadingDots::new();

        assert_eq!(dots.size, LoadingDotsSize::Md);
        assert_eq!(dots.variant, LoadingDotsVariant::Primary);
        assert_eq!(dots.dot_count, 3);
    }

    #[test]
    fn test_loading_dots_render() {
        let dots = LoadingDots::new();
        let html = dots.render().into_string();

        assert!(html.contains("sh-loading-dots"));
        assert!(html.contains("sh-loading-dots__dot"));
        assert!(html.contains("role=\"status\""));
    }

    #[test]
    fn test_loading_dots_sizes() {
        let sm = LoadingDots::new().size(LoadingDotsSize::Sm);
        let md = LoadingDots::new().size(LoadingDotsSize::Md);
        let lg = LoadingDots::new().size(LoadingDotsSize::Lg);

        assert!(sm.render().into_string().contains("sh-loading-dots--sm"));
        assert!(md.render().into_string().contains("sh-loading-dots--md"));
        assert!(lg.render().into_string().contains("sh-loading-dots--lg"));
    }

    #[test]
    fn test_loading_dots_variants() {
        let primary = LoadingDots::new().variant(LoadingDotsVariant::Primary);
        let secondary = LoadingDots::new().variant(LoadingDotsVariant::Secondary);
        let muted = LoadingDots::new().variant(LoadingDotsVariant::Muted);

        assert!(primary
            .render()
            .into_string()
            .contains("sh-loading-dots--primary"));
        assert!(secondary
            .render()
            .into_string()
            .contains("sh-loading-dots--secondary"));
        assert!(muted
            .render()
            .into_string()
            .contains("sh-loading-dots--muted"));
    }

    #[test]
    fn test_loading_dots_dot_count() {
        let dots = LoadingDots::new().dot_count(4);
        let html = dots.render().into_string();

        let dot_count = html.matches("sh-loading-dots__dot").count();
        assert_eq!(dot_count, 4);
    }

    #[test]
    fn test_loading_dots_css() {
        let css = loading_dots_css();
        assert!(css.contains(".sh-loading-dots"));
        assert!(css.contains("@keyframes sh-loading-dots-bounce"));
    }
}
