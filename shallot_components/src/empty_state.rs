//! Empty State Component - Placeholder for empty content areas
//! Provides helpful guidance when no data is available

use maud::{html, Markup, Render};

/// Size variants for the empty state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmptyStateSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// An empty state placeholder component
#[derive(Debug, Clone)]
pub struct EmptyState<'a> {
    /// Title text
    pub title: &'a str,
    /// Description text
    pub description: Option<&'a str>,
    /// Icon name (CSS class or inline SVG)
    pub icon: Option<&'a str>,
    /// Size variant
    pub size: EmptyStateSize,
    /// Action button markup (passed in)
    pub action: Option<Markup>,
    /// Custom illustration/image URL
    pub image: Option<&'a str>,
}

impl<'a> EmptyState<'a> {
    /// Create a new empty state with a title
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            icon: None,
            size: EmptyStateSize::default(),
            action: None,
            image: None,
        }
    }

    /// Set the description
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the icon
    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set the size
    pub fn size(mut self, size: EmptyStateSize) -> Self {
        self.size = size;
        self
    }

    /// Set an action button
    pub fn action(mut self, action: Markup) -> Self {
        self.action = Some(action);
        self
    }

    /// Set a custom image
    pub fn image(mut self, image: &'a str) -> Self {
        self.image = Some(image);
        self
    }

    /// Build CSS class string
    fn build_classes(&self) -> String {
        let size_class = match self.size {
            EmptyStateSize::Sm => "sh-empty-state--sm",
            EmptyStateSize::Md => "sh-empty-state--md",
            EmptyStateSize::Lg => "sh-empty-state--lg",
        };

        format!("sh-empty-state {}", size_class)
    }
}

impl<'a> Render for EmptyState<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div class=(classes) role="status" aria-label="Empty state" {
                @if let Some(image) = self.image {
                    div class="sh-empty-state__image" {
                        img src=(image) alt="" loading="lazy";
                    }
                } @else if let Some(icon) = self.icon {
                    div class="sh-empty-state__icon" {
                        span class=(format!("sh-icon sh-icon--{}", icon)) {}
                    }
                } @else {
                    div class="sh-empty-state__icon sh-empty-state__icon--default" {
                        svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="48"
                            height="48"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        {
                            rect x="3" y="3" width="18" height="18" rx="2" ry="2";
                            line x1="9" y1="9" x2="15" y2="15";
                            line x1="15" y1="9" x2="9" y2="15";
                        }
                    }
                }

                h3 class="sh-empty-state__title" {
                    (self.title)
                }

                @if let Some(description) = self.description {
                    p class="sh-empty-state__description" {
                        (description)
                    }
                }

                @if let Some(ref action) = self.action {
                    div class="sh-empty-state__action" {
                        (action)
                    }
                }
            }
        }
    }
}

/// Generate CSS for the empty state component
pub fn empty_state_css() -> String {
    r#"
.sh-empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: var(--sh-spacing-xl, 2rem);
    color: var(--sh-color-text-muted, #666);
}

.sh-empty-state--sm {
    padding: var(--sh-spacing-md, 1rem);
}

.sh-empty-state--sm .sh-empty-state__icon {
    width: 32px;
    height: 32px;
}

.sh-empty-state--sm .sh-empty-state__title {
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-empty-state--lg {
    padding: var(--sh-spacing-2xl, 3rem);
}

.sh-empty-state--lg .sh-empty-state__icon {
    width: 64px;
    height: 64px;
}

.sh-empty-state--lg .sh-empty-state__title {
    font-size: var(--sh-font-size-xl, 1.25rem);
}

.sh-empty-state__image {
    max-width: 200px;
    margin-bottom: var(--sh-spacing-md, 1rem);
}

.sh-empty-state__image img {
    width: 100%;
    height: auto;
    opacity: 0.8;
}

.sh-empty-state__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    margin-bottom: var(--sh-spacing-md, 1rem);
    color: var(--sh-color-text-muted, #666);
    opacity: 0.5;
}

.sh-empty-state__icon--default svg {
    width: 100%;
    height: 100%;
}

.sh-empty-state__title {
    font-size: var(--sh-font-size-md, 1rem);
    font-weight: var(--sh-font-weight-semibold, 600);
    color: var(--sh-color-text, #1a1a1a);
    margin: 0 0 var(--sh-spacing-xs, 0.25rem) 0;
}

.sh-empty-state__description {
    font-size: var(--sh-font-size-sm, 0.875rem);
    color: var(--sh-color-text-muted, #666);
    margin: 0 0 var(--sh-spacing-md, 1rem) 0;
    max-width: 400px;
}

.sh-empty-state__action {
    margin-top: var(--sh-spacing-sm, 0.5rem);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_creation() {
        let state =
            EmptyState::new("No results found").description("Try adjusting your search criteria");

        assert_eq!(state.title, "No results found");
        assert_eq!(
            state.description,
            Some("Try adjusting your search criteria")
        );
    }

    #[test]
    fn test_empty_state_render() {
        let state = EmptyState::new("Empty inbox");
        let rendered = state.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-empty-state"));
        assert!(html.contains("Empty inbox"));
    }

    #[test]
    fn test_empty_state_with_icon() {
        let state = EmptyState::new("No data").icon("inbox");

        let rendered = state.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-empty-state__icon"));
    }

    #[test]
    fn test_empty_state_sizes() {
        let sm = EmptyState::new("Small").size(EmptyStateSize::Sm);
        let lg = EmptyState::new("Large").size(EmptyStateSize::Lg);

        assert!(sm.build_classes().contains("sh-empty-state--sm"));
        assert!(lg.build_classes().contains("sh-empty-state--lg"));
    }

    #[test]
    fn test_css_generation() {
        let css = empty_state_css();
        assert!(css.contains(".sh-empty-state"));
        assert!(css.contains(".sh-empty-state__title"));
    }
}
