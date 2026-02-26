use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JoinVariant {
    #[default]
    Default,
    Vertical,
    Responsive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JoinGap {
    #[default]
    Md,
    None,
    Sm,
    Lg,
}

pub struct Join {
    pub children: Vec<Markup>,
    pub variant: JoinVariant,
    pub gap: JoinGap,
}

impl Default for Join {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            variant: JoinVariant::Default,
            gap: JoinGap::Md,
        }
    }
}

impl Join {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: JoinVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn gap(mut self, gap: JoinGap) -> Self {
        self.gap = gap;
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

impl Render for Join {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            JoinVariant::Default => "sh-join",
            JoinVariant::Vertical => "sh-join sh-join--vertical",
            JoinVariant::Responsive => "sh-join sh-join--responsive",
        };

        let gap_class = match self.gap {
            JoinGap::None => "sh-join--gap-none",
            JoinGap::Sm => "sh-join--gap-sm",
            JoinGap::Md => "sh-join--gap-md",
            JoinGap::Lg => "sh-join--gap-lg",
        };

        html! {
            div class=(format!("{} {}", variant_class, gap_class)) role="group" {
                @for child in &self.children {
                    div class="sh-join__item" { (child) }
                }
            }
        }
    }
}

pub struct JoinItem {
    pub children: Markup,
}

impl JoinItem {
    pub fn new(children: Markup) -> Self {
        Self { children }
    }
}

impl Render for JoinItem {
    fn render(&self) -> Markup {
        html! {
            div class="sh-join__item" { (self.children) }
        }
    }
}

pub fn join_css() -> String {
    r#"
.sh-join {
    display: flex;
    align-items: center;
    width: 100%;
}

.sh-join--vertical {
    flex-direction: column;
    align-items: stretch;
}

.sh-join--responsive {
    flex-direction: column;
    align-items: stretch;
}

@media (min-width: 640px) {
    .sh-join--responsive {
        flex-direction: row;
        align-items: center;
    }
}

.sh-join--gap-none {
    gap: 0;
}

.sh-join--gap-none .sh-join__item {
    border-radius: 0;
}

.sh-join--gap-none .sh-join__item:first-child {
    border-radius: 8px 0 0 8px;
}

.sh-join--gap-none .sh-join__item:last-child {
    border-radius: 0 8px 8px 0;
}

.sh-join--gap-none .sh-join__item:only-child {
    border-radius: 8px;
}

.sh-join--gap-none.sh-join--vertical .sh-join__item:first-child {
    border-radius: 8px 8px 0 0;
}

.sh-join--gap-none.sh-join--vertical .sh-join__item:last-child {
    border-radius: 0 0 8px 8px;
}

.sh-join--gap-sm {
    gap: 0.25rem;
}

.sh-join--gap-md {
    gap: 0.5rem;
}

.sh-join--gap-lg {
    gap: 1rem;
}

.sh-join__item {
    flex: 1;
    min-width: 0;
}

.sh-join__item > * {
    width: 100%;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_default() {
        let join = Join::new();
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("sh-join"));
    }

    #[test]
    fn test_join_with_children() {
        let join = Join::new()
            .child(html! { "Item 1" })
            .child(html! { "Item 2" });
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("Item 1"));
        assert!(rendered.0.as_str().contains("Item 2"));
    }

    #[test]
    fn test_join_variant_vertical() {
        let join = Join::new().variant(JoinVariant::Vertical);
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("sh-join--vertical"));
    }

    #[test]
    fn test_join_variant_responsive() {
        let join = Join::new().variant(JoinVariant::Responsive);
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("sh-join--responsive"));
    }

    #[test]
    fn test_join_gap_none() {
        let join = Join::new().gap(JoinGap::None);
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("sh-join--gap-none"));
    }

    #[test]
    fn test_join_gap_sm() {
        let join = Join::new().gap(JoinGap::Sm);
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("sh-join--gap-sm"));
    }

    #[test]
    fn test_join_gap_lg() {
        let join = Join::new().gap(JoinGap::Lg);
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("sh-join--gap-lg"));
    }

    #[test]
    fn test_join_item() {
        let item = JoinItem::new(html! { "Content" });
        let rendered = item.render();
        assert!(rendered.0.as_str().contains("sh-join__item"));
        assert!(rendered.0.as_str().contains("Content"));
    }

    #[test]
    fn test_join_a11y() {
        let join = Join::new();
        let rendered = join.render();
        assert!(rendered.0.as_str().contains("role=\"group\""));
    }

    #[test]
    fn test_join_css() {
        let css = join_css();
        assert!(css.contains(".sh-join"));
        assert!(css.contains(".sh-join__item"));
    }
}
