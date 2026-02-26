use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SectionVariant {
    #[default]
    Default,
    Bordered,
    Elevated,
    Glass,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SectionSize {
    #[default]
    Md,
    Sm,
    Lg,
    Full,
}

pub struct Section<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub body: Markup,
    pub variant: SectionVariant,
    pub size: SectionSize,
    pub centered: bool,
}

impl<'a> Default for Section<'a> {
    fn default() -> Self {
        Self {
            title: "",
            subtitle: "",
            body: html! {},
            variant: SectionVariant::Default,
            size: SectionSize::Md,
            centered: false,
        }
    }
}

impl<'a> Section<'a> {
    pub fn new(title: &'a str, subtitle: &'a str, body: Markup) -> Self {
        Self::default().title(title).subtitle(subtitle).body(body)
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = subtitle;
        self
    }

    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }

    pub fn variant(mut self, variant: SectionVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: SectionSize) -> Self {
        self.size = size;
        self
    }

    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }
}

impl<'a> Render for Section<'a> {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            SectionVariant::Default => "sh-section",
            SectionVariant::Bordered => "sh-section sh-section--bordered",
            SectionVariant::Elevated => "sh-section sh-section--elevated",
            SectionVariant::Glass => "sh-section sh-section--glass",
            SectionVariant::Minimal => "sh-section sh-section--minimal",
        };

        let size_class = match self.size {
            SectionSize::Sm => "sh-section--sm",
            SectionSize::Md => "sh-section--md",
            SectionSize::Lg => "sh-section--lg",
            SectionSize::Full => "sh-section--full",
        };

        let align_class = if self.centered {
            "sh-section--centered"
        } else {
            ""
        };

        html! {
            section class=(format!("{} {} {}", variant_class, size_class, align_class)) {
                @if !self.title.is_empty() {
                    div class="sh-section__header" {
                        h2 class="sh-section__title" { (self.title) }
                        @if !self.subtitle.is_empty() {
                            p class="sh-section__subtitle" { (self.subtitle) }
                        }
                    }
                }
                div class="sh-section__content" {
                    (self.body)
                }
            }
        }
    }
}

pub fn section_css() -> String {
    r#"
.sh-section {
    width: 100%;
    padding: 2rem;
}

.sh-section--sm {
    padding: 1rem;
}

.sh-section--lg {
    padding: 3rem;
}

.sh-section--full {
    padding: 0;
    min-height: 100vh;
}

.sh-section--bordered {
    border: 1px solid var(--sh-border-color, #e5e7eb);
    border-radius: 12px;
}

.sh-section--elevated {
    border-radius: 12px;
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.08);
}

.sh-section--glass {
    background: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(12px);
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.3);
}

.sh-section--minimal {
    padding: 0;
}

.sh-section--centered {
    text-align: center;
}

.sh-section__header {
    margin-bottom: 1.5rem;
}

.sh-section__title {
    font-size: 1.875rem;
    font-weight: 700;
    line-height: 1.2;
    color: var(--sh-text-primary, #111827);
    margin: 0;
}

.sh-section__subtitle {
    font-size: 1.125rem;
    color: var(--sh-text-secondary, #6b7280);
    margin: 0.5rem 0 0 0;
}

.sh-section__content {
    color: var(--sh-text-primary, #111827);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_default() {
        let section = Section::new("Title", "Subtitle", html! { "Content" });
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("sh-section"));
        assert!(rendered.0.as_str().contains("Title"));
        assert!(rendered.0.as_str().contains("Subtitle"));
        assert!(rendered.0.as_str().contains("Content"));
    }

    #[test]
    fn test_section_variant_bordered() {
        let section = Section::new("Title", "", html! {}).variant(SectionVariant::Bordered);
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("sh-section--bordered"));
    }

    #[test]
    fn test_section_variant_elevated() {
        let section = Section::new("Title", "", html! {}).variant(SectionVariant::Elevated);
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("sh-section--elevated"));
    }

    #[test]
    fn test_section_variant_glass() {
        let section = Section::new("Title", "", html! {}).variant(SectionVariant::Glass);
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("sh-section--glass"));
    }

    #[test]
    fn test_section_size_sm() {
        let section = Section::new("Title", "", html! {}).size(SectionSize::Sm);
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("sh-section--sm"));
    }

    #[test]
    fn test_section_size_lg() {
        let section = Section::new("Title", "", html! {}).size(SectionSize::Lg);
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("sh-section--lg"));
    }

    #[test]
    fn test_section_centered() {
        let section = Section::new("Title", "", html! {}).centered(true);
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("sh-section--centered"));
    }

    #[test]
    fn test_section_without_title() {
        let section = Section::new("", "", html! { "Content" });
        let rendered = section.render();
        assert!(rendered.0.as_str().contains("Content"));
        assert!(!rendered.0.as_str().contains("sh-section__header"));
    }

    #[test]
    fn test_section_css() {
        let css = section_css();
        assert!(css.contains(".sh-section"));
        assert!(css.contains(".sh-section__title"));
    }
}
