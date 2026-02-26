//! Description List Component - Semantic dl/dt/dd lists
//! CSS-only styling for definition lists

use maud::{html, Markup, Render};

/// Description list item
#[derive(Debug, Clone)]
pub struct DescriptionItem<'a> {
    pub term: &'a str,
    pub description: &'a str,
}

impl<'a> DescriptionItem<'a> {
    pub fn new(term: &'a str, description: &'a str) -> Self {
        Self { term, description }
    }

    pub fn render(&self) -> Markup {
        html! {
            div class="sh-desc-list__item" {
                dt class="sh-desc-list__term" {
                    (self.term)
                }
                dd class="sh-desc-list__desc" {
                    (self.description)
                }
            }
        }
    }
}

/// Description list orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DescListOrientation {
    #[default]
    Vertical,
    Horizontal,
    Inline,
}

/// Description list component
#[derive(Debug, Clone)]
pub struct DescriptionList<'a> {
    pub items: Vec<DescriptionItem<'a>>,
    pub orientation: DescListOrientation,
    pub divided: bool,
    pub title: Option<&'a str>,
}

impl<'a> DescriptionList<'a> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            orientation: DescListOrientation::default(),
            divided: false,
            title: None,
        }
    }

    pub fn add(mut self, item: DescriptionItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    pub fn orientation(mut self, orientation: DescListOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn divided(mut self, divided: bool) -> Self {
        self.divided = divided;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-desc-list"];

        match self.orientation {
            DescListOrientation::Vertical => classes.push("sh-desc-list--vertical"),
            DescListOrientation::Horizontal => classes.push("sh-desc-list--horizontal"),
            DescListOrientation::Inline => classes.push("sh-desc-list--inline"),
        }

        if self.divided {
            classes.push("sh-desc-list--divided");
        }

        classes.join(" ")
    }
}

impl<'a> Default for DescriptionList<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for DescriptionList<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div
                class=(classes)
                role="region"
                aria-label=(self.title.unwrap_or("Description list"))
            {
                @if let Some(title) = self.title {
                    h4 class="sh-desc-list__title" id="sh-desc-list-title" {
                        (title)
                    }
                }

                dl
                    class="sh-desc-list__content"
                    aria-labelledby=[self.title.map(|_| "sh-desc-list-title")]
                {
                    @for item in &self.items {
                        (item.render())
                    }
                }
            }
        }
    }
}

pub fn description_list_css() -> String {
    r#"
.sh-desc-list {
    font-size: var(--sh-font-size-md, 1rem);
}

.sh-desc-list__title {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-semibold, 600);
    color: var(--sh-color-text, #1a1a1a);
    margin: 0 0 var(--sh-spacing-sm, 0.5rem) 0;
}

.sh-desc-list__content {
    margin: 0;
}

.sh-desc-list__item {
    display: flex;
    gap: var(--sh-spacing-md, 1rem);
}

.sh-desc-list--vertical .sh-desc-list__item {
    flex-direction: column;
    gap: var(--sh-spacing-xs, 0.25rem);
}

.sh-desc-list--horizontal .sh-desc-list__item {
    flex-direction: row;
}

.sh-desc-list--inline .sh-desc-list__item {
    flex-direction: row;
    display: inline-flex;
    margin-right: var(--sh-spacing-lg, 1.5rem);
}

.sh-desc-list--divided .sh-desc-list__item {
    padding: var(--sh-spacing-sm, 0.5rem) 0;
    border-bottom: 1px solid var(--sh-color-border, #e5e5e5);
}

.sh-desc-list--divided .sh-desc-list__item:last-child {
    border-bottom: none;
}

.sh-desc-list__term {
    font-weight: var(--sh-font-weight-semibold, 600);
    color: var(--sh-color-text, #1a1a1a);
    min-width: 120px;
    margin: 0;
}

.sh-desc-list--inline .sh-desc-list__term::after {
    content: ":";
    margin-right: var(--sh-spacing-xs, 0.25rem);
}

.sh-desc-list__desc {
    color: var(--sh-color-text-muted, #666);
    margin: 0;
    flex: 1;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description_item_creation() {
        let item = DescriptionItem::new("Name", "John Doe");
        assert_eq!(item.term, "Name");
        assert_eq!(item.description, "John Doe");
    }

    #[test]
    fn test_description_list_creation() {
        let list = DescriptionList::new()
            .add(DescriptionItem::new("Key", "Value"))
            .add(DescriptionItem::new("Status", "Active"));

        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_description_list_render() {
        let list = DescriptionList::new()
            .title("Details")
            .add(DescriptionItem::new("Type", "Article"));

        let html = list.render().into_string();
        assert!(html.contains("sh-desc-list"));
        assert!(html.contains("Details"));
    }

    #[test]
    fn test_description_list_orientation() {
        let horizontal = DescriptionList::new().orientation(DescListOrientation::Horizontal);
        assert!(horizontal
            .build_classes()
            .contains("sh-desc-list--horizontal"));
    }

    #[test]
    fn test_css_generation() {
        let css = description_list_css();
        assert!(css.contains(".sh-desc-list"));
    }
}
