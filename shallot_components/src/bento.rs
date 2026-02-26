use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BentoVariant {
    #[default]
    Default,
    Grid,
    Masonry,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BentoGap {
    #[default]
    Md,
    None,
    Sm,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BentoCardSize {
    Default,
    Wide,
    Tall,
    Large,
}

impl Default for BentoCardSize {
    fn default() -> Self {
        Self::Default
    }
}

pub struct BentoGrid {
    pub children: Vec<Markup>,
    pub variant: BentoVariant,
    pub gap: BentoGap,
    pub columns: Option<u32>,
}

impl Default for BentoGrid {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            variant: BentoVariant::Default,
            gap: BentoGap::Md,
            columns: None,
        }
    }
}

impl BentoGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: BentoVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn gap(mut self, gap: BentoGap) -> Self {
        self.gap = gap;
        self
    }

    pub fn columns(mut self, columns: u32) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn child(mut self, child: Markup) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<Markup>) -> Self {
        self.children = children;
        self
    }
}

impl Render for BentoGrid {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            BentoVariant::Default => "sh-bento",
            BentoVariant::Grid => "sh-bento sh-bento--grid",
            BentoVariant::Masonry => "sh-bento sh-bento--masonry",
            BentoVariant::Auto => "sh-bento sh-bento--auto",
        };

        let gap_class = match self.gap {
            BentoGap::None => "sh-bento--gap-none",
            BentoGap::Sm => "sh-bento--gap-sm",
            BentoGap::Md => "sh-bento--gap-md",
            BentoGap::Lg => "sh-bento--gap-lg",
        };

        let columns_style = self
            .columns
            .map(|c| format!("--sh-bento-cols: {};", c))
            .unwrap_or_default();

        html! {
            div class=(format!("{} {}", variant_class, gap_class)) style=(columns_style) role="grid" aria-label="Bento grid" {
                @for child in &self.children {
                    (child)
                }
            }
        }
    }
}

pub struct BentoCard<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub href: Option<&'a str>,
    pub badge: Option<&'a str>,
    pub footer: Option<Markup>,
    pub size: BentoCardSize,
    pub children: Option<Markup>,
}

impl<'a> Default for BentoCard<'a> {
    fn default() -> Self {
        Self {
            title: "",
            description: "",
            href: None,
            badge: None,
            footer: None,
            size: BentoCardSize::Default,
            children: None,
        }
    }
}

impl<'a> BentoCard<'a> {
    pub fn new(title: &'a str, description: &'a str) -> Self {
        Self::default().title(title).description(description)
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub fn badge(mut self, badge: &'a str) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn size(mut self, size: BentoCardSize) -> Self {
        self.size = size;
        self
    }

    pub fn children(mut self, children: Markup) -> Self {
        self.children = Some(children);
        self
    }
}

impl<'a> Render for BentoCard<'a> {
    fn render(&self) -> Markup {
        let size_class = match self.size {
            BentoCardSize::Default => "",
            BentoCardSize::Wide => "sh-bento-card--wide",
            BentoCardSize::Tall => "sh-bento-card--tall",
            BentoCardSize::Large => "sh-bento-card--large",
        };

        let inner = html! {
            @if let Some(badge) = self.badge {
                div class="sh-bento-card__badge" { (badge) }
            }
            h3 class="sh-bento-card__title" { (self.title) }
            @if !self.description.is_empty() {
                p class="sh-bento-card__desc" { (self.description) }
            }
            @if let Some(children) = &self.children {
                div class="sh-bento-card__content" { (children.clone()) }
            }
            @if let Some(footer) = &self.footer {
                div class="sh-bento-card__footer" { (footer.clone()) }
            }
        };

        html! {
            @if let Some(href) = self.href {
                a class=(format!("sh-bento-card {}", size_class)) href=(href) role="gridcell" {
                    (inner)
                }
            } @else {
                div class=(format!("sh-bento-card {}", size_class)) role="gridcell" {
                    (inner)
                }
            }
        }
    }
}

pub fn bento_css() -> String {
    r#"
.sh-bento {
    --sh-bento-cols: 3;
    display: grid;
    grid-template-columns: repeat(var(--sh-bento-cols), 1fr);
    width: 100%;
}

.sh-bento--grid {
    grid-auto-rows: minmax(120px, auto);
}

.sh-bento--masonry {
    grid-auto-rows: masonry;
}

.sh-bento--auto {
    grid-auto-flow: dense;
}

.sh-bento--gap-none {
    gap: 0;
}

.sh-bento--gap-sm {
    gap: 0.5rem;
}

.sh-bento--gap-md {
    gap: 1rem;
}

.sh-bento--gap-lg {
    gap: 1.5rem;
}

.sh-bento-card {
    background: var(--sh-bg-primary, #ffffff);
    border-radius: 12px;
    padding: 1.5rem;
    border: 1px solid var(--sh-border-color, #e5e7eb);
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    text-decoration: none;
    color: inherit;
    display: flex;
    flex-direction: column;
    min-height: 120px;
}

.sh-bento-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
}

.sh-bento-card--wide {
    grid-column: span 2;
}

.sh-bento-card--tall {
    grid-row: span 2;
}

.sh-bento-card--large {
    grid-column: span 2;
    grid-row: span 2;
}

.sh-bento-card__badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.75rem;
    background: var(--sh-accent, #6366f1);
    color: white;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 500;
    margin-bottom: 0.75rem;
    width: fit-content;
}

.sh-bento-card__title {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0 0 0.5rem 0;
    color: var(--sh-text-primary, #111827);
}

.sh-bento-card__desc {
    font-size: 0.875rem;
    color: var(--sh-text-secondary, #6b7280);
    margin: 0;
    line-height: 1.5;
}

.sh-bento-card__content {
    margin-top: 1rem;
    flex: 1;
}

.sh-bento-card__footer {
    margin-top: auto;
    padding-top: 1rem;
    border-top: 1px solid var(--sh-border-color, #e5e7eb);
}

@media (max-width: 768px) {
    .sh-bento {
        grid-template-columns: 1fr;
    }

    .sh-bento-card--wide,
    .sh-bento-card--large {
        grid-column: span 1;
    }

    .sh-bento-card--tall,
    .sh-bento-card--large {
        grid-row: span 1;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bento_grid_default() {
        let grid = BentoGrid::new();
        let rendered = grid.render();
        assert!(rendered.0.as_str().contains("sh-bento"));
    }

    #[test]
    fn test_bento_grid_with_children() {
        let grid = BentoGrid::new()
            .child(html! { "Card 1" })
            .child(html! { "Card 2" });
        let rendered = grid.render();
        assert!(rendered.0.as_str().contains("Card 1"));
        assert!(rendered.0.as_str().contains("Card 2"));
    }

    #[test]
    fn test_bento_grid_variant_masonry() {
        let grid = BentoGrid::new().variant(BentoVariant::Masonry);
        let rendered = grid.render();
        assert!(rendered.0.as_str().contains("sh-bento--masonry"));
    }

    #[test]
    fn test_bento_grid_gap_lg() {
        let grid = BentoGrid::new().gap(BentoGap::Lg);
        let rendered = grid.render();
        assert!(rendered.0.as_str().contains("sh-bento--gap-lg"));
    }

    #[test]
    fn test_bento_grid_columns() {
        let grid = BentoGrid::new().columns(4);
        let rendered = grid.render();
        assert!(rendered.0.as_str().contains("--sh-bento-cols: 4"));
    }

    #[test]
    fn test_bento_card_default() {
        let card = BentoCard::new("Title", "Description");
        let rendered = card.render();
        assert!(rendered.0.as_str().contains("sh-bento-card"));
        assert!(rendered.0.as_str().contains("Title"));
        assert!(rendered.0.as_str().contains("Description"));
    }

    #[test]
    fn test_bento_card_with_href() {
        let card = BentoCard::new("Title", "Desc").href("https://example.com");
        let rendered = card.render();
        assert!(rendered.0.as_str().contains("href=\"https://example.com\""));
    }

    #[test]
    fn test_bento_card_with_badge() {
        let card = BentoCard::new("Title", "Desc").badge("New");
        let rendered = card.render();
        assert!(rendered.0.as_str().contains("sh-bento-card__badge"));
        assert!(rendered.0.as_str().contains("New"));
    }

    #[test]
    fn test_bento_card_size_wide() {
        let card = BentoCard::new("Title", "Desc").size(BentoCardSize::Wide);
        let rendered = card.render();
        assert!(rendered.0.as_str().contains("sh-bento-card--wide"));
    }

    #[test]
    fn test_bento_card_size_tall() {
        let card = BentoCard::new("Title", "Desc").size(BentoCardSize::Tall);
        let rendered = card.render();
        assert!(rendered.0.as_str().contains("sh-bento-card--tall"));
    }

    #[test]
    fn test_bento_card_size_large() {
        let card = BentoCard::new("Title", "Desc").size(BentoCardSize::Large);
        let rendered = card.render();
        assert!(rendered.0.as_str().contains("sh-bento-card--large"));
    }

    #[test]
    fn test_bento_a11y() {
        let grid = BentoGrid::new();
        let rendered = grid.render();
        assert!(rendered.0.as_str().contains("role=\"grid\""));
        assert!(rendered.0.as_str().contains("aria-label"));
    }

    #[test]
    fn test_bento_css() {
        let css = bento_css();
        assert!(css.contains(".sh-bento"));
        assert!(css.contains(".sh-bento-card"));
    }
}
