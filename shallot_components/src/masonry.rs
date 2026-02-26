//! Masonry Layout - Pinterest-style masonry grid
//! CSS-only using CSS columns

use maud::{html, Markup, Render};

/// Masonry gap size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MasonryGap {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Masonry columns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MasonryColumns {
    #[default]
    Auto,
    Fixed(u8),
}

/// Masonry layout component
#[derive(Debug, Clone)]
pub struct Masonry<'a> {
    pub columns: MasonryColumns,
    pub gap: MasonryGap,
    pub items: Vec<Markup>,
    pub id: Option<&'a str>,
}

impl<'a> Masonry<'a> {
    pub fn new() -> Self {
        Self {
            columns: MasonryColumns::default(),
            gap: MasonryGap::default(),
            items: Vec::new(),
            id: None,
        }
    }

    pub fn columns(mut self, columns: MasonryColumns) -> Self {
        self.columns = columns;
        self
    }

    pub fn gap(mut self, gap: MasonryGap) -> Self {
        self.gap = gap;
        self
    }

    pub fn add(mut self, item: Markup) -> Self {
        self.items.push(item);
        self
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes: Vec<String> = vec!["sh-masonry".to_string()];

        match self.gap {
            MasonryGap::None => classes.push("sh-masonry--gap-none".to_string()),
            MasonryGap::Sm => classes.push("sh-masonry--gap-sm".to_string()),
            MasonryGap::Md => classes.push("sh-masonry--gap-md".to_string()),
            MasonryGap::Lg => classes.push("sh-masonry--gap-lg".to_string()),
            MasonryGap::Xl => classes.push("sh-masonry--gap-xl".to_string()),
        }

        if let MasonryColumns::Fixed(n) = self.columns {
            classes.push(format!("sh-masonry--cols-{}", n.min(6)));
        }

        classes.join(" ")
    }
}
impl<'a> Default for Masonry<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for Masonry<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div
                class=(classes)
                id=[self.id]
                role="region"
                aria-label="Masonry layout"
            {
                @for item in &self.items {
                    div class="sh-masonry__item" role="article" {
                        (item)
                    }
                }
            }
        }
    }
}

pub fn masonry_css() -> String {
    r#"
.sh-masonry {
    column-width: 280px;
    column-gap: var(--sh-spacing-md, 1rem);
}

.sh-masonry--gap-none {
    column-gap: 0;
}

.sh-masonry--gap-sm {
    column-gap: var(--sh-spacing-sm, 0.5rem);
}

.sh-masonry--gap-md {
    column-gap: var(--sh-spacing-md, 1rem);
}

.sh-masonry--gap-lg {
    column-gap: var(--sh-spacing-lg, 1.5rem);
}

.sh-masonry--gap-xl {
    column-gap: var(--sh-spacing-xl, 2rem);
}

.sh-masonry--cols-1 { column-count: 1; }
.sh-masonry--cols-2 { column-count: 2; }
.sh-masonry--cols-3 { column-count: 3; }
.sh-masonry--cols-4 { column-count: 4; }
.sh-masonry--cols-5 { column-count: 5; }
.sh-masonry--cols-6 { column-count: 6; }

.sh-masonry__item {
    break-inside: avoid;
    margin-bottom: var(--sh-spacing-md, 1rem);
}

.sh-masonry--gap-none .sh-masonry__item {
    margin-bottom: 0;
}

.sh-masonry--gap-sm .sh-masonry__item {
    margin-bottom: var(--sh-spacing-sm, 0.5rem);
}

.sh-masonry--gap-lg .sh-masonry__item {
    margin-bottom: var(--sh-spacing-lg, 1.5rem);
}

.sh-masonry--gap-xl .sh-masonry__item {
    margin-bottom: var(--sh-spacing-xl, 2rem);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masonry_creation() {
        let masonry = Masonry::new()
            .columns(MasonryColumns::Fixed(3))
            .gap(MasonryGap::Lg);

        assert_eq!(masonry.columns, MasonryColumns::Fixed(3));
        assert_eq!(masonry.gap, MasonryGap::Lg);
    }

    #[test]
    fn test_masonry_render() {
        let masonry = Masonry::new()
            .add(html! { div { "Item 1" } })
            .add(html! { div { "Item 2" } });

        let html = masonry.render().into_string();
        assert!(html.contains("sh-masonry"));
        assert!(html.contains("Item 1"));
    }

    #[test]
    fn test_masonry_gap_classes() {
        let none = Masonry::new().gap(MasonryGap::None);
        let xl = Masonry::new().gap(MasonryGap::Xl);

        assert!(none.build_classes().contains("sh-masonry--gap-none"));
        assert!(xl.build_classes().contains("sh-masonry--gap-xl"));
    }

    #[test]
    fn test_masonry_columns_classes() {
        let fixed = Masonry::new().columns(MasonryColumns::Fixed(4));
        assert!(fixed.build_classes().contains("sh-masonry--cols-4"));
    }

    #[test]
    fn test_css_generation() {
        let css = masonry_css();
        assert!(css.contains(".sh-masonry"));
        assert!(css.contains("column-count"));
    }
}
