//! Key Value List Component - Display key-value pairs in a list format

use maud::{html, Markup, Render};

/// A single key-value pair
#[derive(Debug, Clone)]
pub struct KeyValueItem<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub label: Option<&'a str>,
    pub copyable: bool,
}

impl<'a> KeyValueItem<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self {
            key,
            value,
            label: None,
            copyable: false,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn copyable(mut self, copyable: bool) -> Self {
        self.copyable = copyable;
        self
    }

    pub fn render(&self) -> Markup {
        html! {
            div class="sh-kv-item" {
                dt class="sh-kv-item__key" {
                    @if let Some(label) = self.label {
                        (label)
                    } @else {
                        (self.key)
                    }
                }
                dd class="sh-kv-item__value" {
                    span class="sh-kv-item__value-text" {
                        (self.value)
                    }
                }
            }
        }
    }
}

/// Key Value List orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyValueOrientation {
    #[default]
    Horizontal,
    Vertical,
    Stacked,
}

/// Key Value List size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyValueSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Key Value List component
#[derive(Debug, Clone)]
pub struct KeyValueList<'a> {
    pub items: Vec<KeyValueItem<'a>>,
    pub orientation: KeyValueOrientation,
    pub size: KeyValueSize,
    pub divided: bool,
    pub title: Option<&'a str>,
}

impl<'a> KeyValueList<'a> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            orientation: KeyValueOrientation::default(),
            size: KeyValueSize::default(),
            divided: false,
            title: None,
        }
    }

    pub fn items(mut self, items: Vec<KeyValueItem<'a>>) -> Self {
        self.items = items;
        self
    }

    pub fn add(mut self, item: KeyValueItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    pub fn orientation(mut self, orientation: KeyValueOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: KeyValueSize) -> Self {
        self.size = size;
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
        let mut classes = vec!["sh-kv-list"];

        match self.orientation {
            KeyValueOrientation::Horizontal => classes.push("sh-kv-list--horizontal"),
            KeyValueOrientation::Vertical => classes.push("sh-kv-list--vertical"),
            KeyValueOrientation::Stacked => classes.push("sh-kv-list--stacked"),
        }

        match self.size {
            KeyValueSize::Sm => classes.push("sh-kv-list--sm"),
            KeyValueSize::Md => classes.push("sh-kv-list--md"),
            KeyValueSize::Lg => classes.push("sh-kv-list--lg"),
        }

        if self.divided {
            classes.push("sh-kv-list--divided");
        }

        classes.join(" ")
    }
}

impl<'a> Default for KeyValueList<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for KeyValueList<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            div
                class=(classes)
                role="region"
                aria-label=(self.title.unwrap_or("Key value list"))
            {
                @if let Some(title) = self.title {
                    h4 class="sh-kv-list__title" id="sh-kv-list-title" {
                        (title)
                    }
                }
                dl
                    class="sh-kv-list__content"
                    aria-labelledby=[self.title.map(|_| "sh-kv-list-title")]
                {
                    @for item in &self.items {
                        (item.render())
                    }
                }
            }
        }
    }
}

pub fn key_value_list_css() -> String {
    r#"
.sh-kv-list {
    font-size: var(--sh-font-size-md, 1rem);
}

.sh-kv-list--sm {
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-kv-list--lg {
    font-size: var(--sh-font-size-lg, 1.125rem);
}

.sh-kv-list__title {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-semibold, 600);
    color: var(--sh-color-text-muted, #666);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin: 0 0 var(--sh-spacing-sm, 0.5rem) 0;
}

.sh-kv-list__content {
    margin: 0;
}

.sh-kv-list--horizontal .sh-kv-list__content {
    display: flex;
    flex-wrap: wrap;
    gap: var(--sh-spacing-md, 1rem);
}

.sh-kv-list--vertical .sh-kv-list__content {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-sm, 0.5rem);
}

.sh-kv-list--stacked .sh-kv-list__content {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-md, 1rem);
}

.sh-kv-list--divided .sh-kv-item {
    padding: var(--sh-spacing-sm, 0.5rem) 0;
    border-bottom: 1px solid var(--sh-color-border, #e5e5e5);
}

.sh-kv-list--divided .sh-kv-item:last-child {
    border-bottom: none;
}

.sh-kv-item {
    margin: 0;
}

.sh-kv-list--horizontal .sh-kv-item {
    display: flex;
    gap: var(--sh-spacing-sm, 0.5rem);
}

.sh-kv-list--vertical .sh-kv-item {
    display: flex;
    gap: var(--sh-spacing-md, 1rem);
}

.sh-kv-list--stacked .sh-kv-item {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-xs, 0.25rem);
}

.sh-kv-item__key {
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-text-muted, #666);
    min-width: 100px;
}

.sh-kv-list--stacked .sh-kv-item__key {
    font-size: var(--sh-font-size-xs, 0.75rem);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.sh-kv-item__value {
    color: var(--sh-color-text, #1a1a1a);
    flex: 1;
}

.sh-kv-item__value-text {
    word-break: break-word;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_item_creation() {
        let item = KeyValueItem::new("name", "John Doe").label("Full Name");

        assert_eq!(item.key, "name");
        assert_eq!(item.value, "John Doe");
        assert_eq!(item.label, Some("Full Name"));
    }

    #[test]
    fn test_kv_list_creation() {
        let list = KeyValueList::new()
            .orientation(KeyValueOrientation::Vertical)
            .divided(true);

        assert_eq!(list.orientation, KeyValueOrientation::Vertical);
        assert!(list.divided);
    }

    #[test]
    fn test_kv_list_render() {
        let list = KeyValueList::new()
            .title("User Info")
            .add(KeyValueItem::new("name", "John"))
            .add(KeyValueItem::new("email", "john@example.com"));

        let rendered = list.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-kv-list"));
        assert!(html.contains("User Info"));
        assert!(html.contains("John"));
    }

    #[test]
    fn test_kv_list_orientations() {
        let horizontal = KeyValueList::new().orientation(KeyValueOrientation::Horizontal);
        let stacked = KeyValueList::new().orientation(KeyValueOrientation::Stacked);

        assert!(horizontal
            .build_classes()
            .contains("sh-kv-list--horizontal"));
        assert!(stacked.build_classes().contains("sh-kv-list--stacked"));
    }

    #[test]
    fn test_css_generation() {
        let css = key_value_list_css();
        assert!(css.contains(".sh-kv-list"));
        assert!(css.contains(".sh-kv-item"));
    }
}
