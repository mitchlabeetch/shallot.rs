use crate::component::{
    AriaAttrs, Component, ComponentSize,
};
use maud::{html, Markup, Render};

/// Enhanced Card component with comprehensive styling options
pub struct Card {
    /// Card content body
    body: Markup,
    /// Optional header content
    header: Option<Markup>,
    /// Optional footer content
    footer: Option<Markup>,
    /// Card title (displayed in header if no custom header)
    title: Option<String>,
    /// Card subtitle
    subtitle: Option<String>,
    /// Visual variant
    variant: CardVariant,
    /// Size variant
    size: ComponentSize,
    /// Border/shadow intensity
    elevation: Elevation,
    /// Whether the card is clickable
    clickable: bool,
    /// Click handler href
    href: Option<String>,
    /// Custom CSS classes
    custom_class: Option<String>,
    /// ARIA attributes
    aria: AriaAttrs,
    /// Whether to show dividers between sections
    divided: bool,
    /// Card cover image URL
    cover_image: Option<String>,
    /// Card cover image alt text
    cover_alt: Option<String>,
}

/// Card visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Outlined,
    Filled,
    Elevated,
}

impl CardVariant {
    fn class_suffix(&self) -> &'static str {
        match self {
            CardVariant::Default => "default",
            CardVariant::Outlined => "outlined",
            CardVariant::Filled => "filled",
            CardVariant::Elevated => "elevated",
        }
    }
}

/// Card elevation levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Elevation {
    None,
    Low,
    #[default]
    Default,
    High,
    Higher,
}

impl Elevation {
    fn class_suffix(&self) -> &'static str {
        match self {
            Elevation::None => "none",
            Elevation::Low => "low",
            Elevation::Default => "default",
            Elevation::High => "high",
            Elevation::Higher => "higher",
        }
    }
}

impl Card {
    /// Create a new card with the given body content
    pub fn new(body: Markup) -> Self {
        Self {
            body,
            header: None,
            footer: None,
            title: None,
            subtitle: None,
            variant: CardVariant::default(),
            size: ComponentSize::Md,
            elevation: Elevation::Default,
            clickable: false,
            href: None,
            custom_class: None,
            aria: AriaAttrs::new(),
            divided: false,
            cover_image: None,
            cover_alt: None,
        }
    }

    /// Set the card header content
    pub fn header(mut self, header: Markup) -> Self {
        self.header = Some(header);
        self
    }

    /// Set the card footer content
    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the card title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the card subtitle
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size variant
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    /// Set the elevation level
    pub fn elevation(mut self, elevation: Elevation) -> Self {
        self.elevation = elevation;
        self
    }

    /// Make the card clickable with a link
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self.clickable = true;
        self
    }

    /// Make the card clickable without a specific link
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Add custom CSS classes
    pub fn custom_class(mut self, class: impl Into<String>) -> Self {
        self.custom_class = Some(class.into());
        self
    }

    /// Set ARIA attributes
    pub fn aria(mut self, aria: AriaAttrs) -> Self {
        self.aria = aria;
        self
    }

    /// Show dividers between sections
    pub fn divided(mut self, divided: bool) -> Self {
        self.divided = divided;
        self
    }

    /// Set a cover image
    pub fn cover_image(mut self, url: impl Into<String>, alt: impl Into<String>) -> Self {
        self.cover_image = Some(url.into());
        self.cover_alt = Some(alt.into());
        self
    }

    /// Build the CSS class string
    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-card".to_string()];

        classes.push(format!("sh-card--{}", self.variant.class_suffix()));
        classes.push(format!("sh-card--{}", self.size.class_suffix()));
        classes.push(format!(
            "sh-card--elevation-{}",
            self.elevation.class_suffix()
        ));

        if self.clickable {
            classes.push("sh-card--clickable".to_string());
        }

        if self.divided {
            classes.push("sh-card--divided".to_string());
        }

        if let Some(custom) = &self.custom_class {
            classes.push(custom.clone());
        }

        classes.join(" ")
    }

    /// Render the header section
    fn render_header(&self) -> Option<Markup> {
        if let Some(header) = &self.header {
            return Some(html! {
                div class="sh-card__header" {
                    (header)
                }
            });
        }

        if self.title.is_some() || self.subtitle.is_some() {
            return Some(html! {
                div class="sh-card__header" {
                    @if let Some(title) = &self.title {
                        h3 class="sh-card__title" { (title) }
                    }
                    @if let Some(subtitle) = &self.subtitle {
                        p class="sh-card__subtitle" { (subtitle) }
                    }
                }
            });
        }

        None
    }

    /// Render the cover image
    fn render_cover(&self) -> Option<Markup> {
        self.cover_image.as_ref().map(|url| {
            let alt = self.cover_alt.as_deref().unwrap_or("");
            html! {
                div class="sh-card__cover" {
                    img src=(url) alt=(alt) class="sh-card__cover-image";
                }
            }
        })
    }
}

impl Render for Card {
    fn render(&self) -> Markup {
        let class = self.build_classes();
        let header = self.render_header();
        let cover = self.render_cover();

        let content = html! {
            @if let Some(ref cover) = cover {
                (cover)
            }
            @if let Some(ref header) = header {
                (header)
            }
            @if self.divided && (header.is_some() || cover.is_some()) {
                div class="sh-card__divider" {}
            }
            div class="sh-card__body" {
                (self.body.clone())
            }
            @if self.divided && self.footer.is_some() {
                div class="sh-card__divider" {}
            }
            @if let Some(footer) = &self.footer {
                div class="sh-card__footer" {
                    (footer)
                }
            }
        };

        if let Some(href) = &self.href {
            html! {
                a class=(class) href=(href) role="article" {
                    (content)
                }
            }
        } else if self.clickable {
            html! {
                div class=(class) role="button" tabindex="0" {
                    (content)
                }
            }
        } else {
            html! {
                div class=(class) role="article" {
                    (content)
                }
            }
        }
    }
}

impl Component for Card {
    fn classes(&self) -> String {
        self.build_classes()
    }

    fn is_interactive(&self) -> bool {
        self.clickable || self.href.is_some()
    }
}

/// Card section component for organizing card content
pub struct CardSection {
    title: Option<String>,
    content: Markup,
    class: Option<String>,
}

impl CardSection {
    pub fn new(content: Markup) -> Self {
        Self {
            title: None,
            content,
            class: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Render for CardSection {
    fn render(&self) -> Markup {
        let class = self.class.as_deref().unwrap_or("sh-card__section");
        html! {
            div class=(class) {
                @if let Some(title) = &self.title {
                    h4 class="sh-card__section-title" { (title) }
                }
                (self.content.clone())
            }
        }
    }
}

/// Generate CSS for cards
pub fn card_css() -> String {
    r#"
/* Card Base */
.sh-card {
  display: flex;
  flex-direction: column;
  background: var(--sh-surface);
  border-radius: var(--sh-radius-lg);
  overflow: hidden;
  transition: all var(--sh-dur-med) var(--sh-ease-out);
}

/* Size variants */
.sh-card--xs { max-width: 240px; }
.sh-card--sm { max-width: 320px; }
.sh-card--md { max-width: 400px; }
.sh-card--lg { max-width: 520px; }
.sh-card--xl { max-width: 640px; }

/* Variant styles */
.sh-card--default {
  border: 1px solid var(--sh-border);
  box-shadow: var(--sh-shadow-sm);
}

.sh-card--outlined {
  border: 2px solid var(--sh-border);
  box-shadow: none;
}

.sh-card--filled {
  border: none;
  background: var(--sh-surface-2);
}

.sh-card--elevated {
  border: none;
  background: var(--sh-surface);
  box-shadow: var(--sh-shadow-md);
}

/* Elevation levels */
.sh-card--elevation-none { box-shadow: none; }
.sh-card--elevation-low { box-shadow: var(--sh-shadow-sm); }
.sh-card--elevation-default { box-shadow: var(--sh-shadow-md); }
.sh-card--elevation-high { box-shadow: var(--sh-shadow-xl); }
.sh-card--elevation-higher {
  box-shadow: 
    0 20px 25px -5px rgba(0, 0, 0, 0.1),
    0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

/* Clickable card */
.sh-card--clickable {
  cursor: pointer;
  text-decoration: none;
  color: inherit;
}

.sh-card--clickable:hover {
  transform: translateY(-2px);
  box-shadow: var(--sh-shadow-lg);
}

.sh-card--clickable:active {
  transform: translateY(0);
}

/* Card sections */
.sh-card__cover {
  width: 100%;
  aspect-ratio: 16 / 9;
  overflow: hidden;
}

.sh-card__cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--sh-dur-slow) var(--sh-ease-out);
}

.sh-card--clickable:hover .sh-card__cover-image {
  transform: scale(1.05);
}

.sh-card__header {
  padding: 1.25rem 1.25rem 0;
}

.sh-card__body {
  padding: 1.25rem;
  flex: 1;
}

.sh-card__footer {
  padding: 0 1.25rem 1.25rem;
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.sh-card__title {
  margin: 0 0 0.25rem;
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--sh-text);
}

.sh-card__subtitle {
  margin: 0;
  font-size: 0.875rem;
  color: var(--sh-text-muted);
}

.sh-card__section {
  padding: 1rem 0;
}

.sh-card__section-title {
  margin: 0 0 0.75rem;
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--sh-text-muted);
}

/* Dividers */
.sh-card--divided .sh-card__divider {
  height: 1px;
  background: var(--sh-border);
  margin: 0 1.25rem;
}

.sh-card--divided .sh-card__header + .sh-card__divider {
  margin-top: 1rem;
}

/* Horizontal layout */
.sh-card--horizontal {
  flex-direction: row;
}

.sh-card--horizontal .sh-card__cover {
  width: 40%;
  aspect-ratio: auto;
}

/* Compact size */
.sh-card--compact .sh-card__header,
.sh-card--compact .sh-card__body,
.sh-card--compact .sh-card__footer {
  padding: 0.75rem;
}

/* Full width */
.sh-card--full {
  max-width: none;
  width: 100%;
}

/* Responsive */
@media (max-width: 640px) {
  .sh-card--horizontal {
    flex-direction: column;
  }
  
  .sh-card--horizontal .sh-card__cover {
    width: 100%;
    aspect-ratio: 16 / 9;
  }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let body = html! { p { "Test content" } };
        let card = Card::new(body)
            .title("Test Card")
            .subtitle("A test card")
            .variant(CardVariant::Elevated);

        assert!(card.title.is_some());
        assert_eq!(card.variant, CardVariant::Elevated);
    }

    #[test]
    fn test_card_clickable() {
        let body = html! { p { "Content" } };
        let card = Card::new(body).href("/test");

        assert!(card.clickable);
        assert_eq!(card.href, Some("/test".to_string()));
    }

    #[test]
    fn test_card_classes() {
        let body = html! { p { "Content" } };
        let card = Card::new(body)
            .variant(CardVariant::Outlined)
            .size(ComponentSize::Lg)
            .elevation(Elevation::High);

        let classes = card.build_classes();
        assert!(classes.contains("sh-card--outlined"));
        assert!(classes.contains("sh-card--lg"));
        assert!(classes.contains("sh-card--elevation-high"));
    }

    #[test]
    fn test_card_css_generation() {
        let css = card_css();
        assert!(css.contains(".sh-card"));
        assert!(css.contains(".sh-card--clickable:hover"));
    }
}
