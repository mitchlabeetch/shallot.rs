//! List Component
//!
//! Lists for displaying collections of items.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

/// List - Display a collection of items
pub struct List<'a> {
    items: Vec<ListItem<'a>>,
    variant: ListVariant,
    size: ComponentSize,
    interactive: bool,
}

#[derive(Clone)]
pub struct ListItem<'a> {
    pub content: &'a str,
    pub icon: Option<&'a str>,
    pub avatar: Option<&'a str>,
    pub subtitle: Option<&'a str>,
    pub badge: Option<&'a str>,
    pub action: Option<&'a str>,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListVariant {
    #[default]
    Default,
    Borderless,
    Striped,
    Ordered,
}

impl ListVariant {
    fn class_suffix(&self) -> &'static str {
        match self {
            ListVariant::Default => "default",
            ListVariant::Borderless => "borderless",
            ListVariant::Striped => "striped",
            ListVariant::Ordered => "ordered",
        }
    }
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            variant: ListVariant::Default,
            size: ComponentSize::Md,
            interactive: false,
        }
    }

    pub fn items(mut self, items: Vec<ListItem<'a>>) -> Self {
        self.items = items;
        self
    }

    pub fn variant(mut self, variant: ListVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }
}

impl Default for List<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for List<'a> {
    fn render(&self) -> Markup {
        let variant_class = format!("sh-list--{}", self.variant.class_suffix());
        let size_class = match self.size {
            ComponentSize::Xs => "sh-list--xs",
            ComponentSize::Sm => "sh-list--sm",
            ComponentSize::Md => "sh-list--md",
            ComponentSize::Lg => "sh-list--lg",
            ComponentSize::Xl => "sh-list--xl",
        };
        let interactive_class = if self.interactive {
            "sh-list--interactive"
        } else {
            ""
        };

        html! {
            ul
                class=(format!("sh-list {} {} {}", variant_class, size_class, interactive_class))
                role="list"
                aria-label="List"
            {
                @for (i, item) in self.items.iter().enumerate() {
                    li
                        class="sh-list-item"
                        data-index=(i)
                        role="listitem"
                    {
                        @if let Some(icon) = item.icon {
                            div class="sh-list-item-icon" aria-hidden="true" { (icon) }
                        }

                        @if let Some(avatar) = item.avatar {
                            div class="sh-list-item-avatar" {
                                img src=(avatar) alt="Avatar";
                            }
                        }

                        div class="sh-list-item-content" {
                            span class="sh-list-item-main" { (item.content) }
                            @if let Some(subtitle) = item.subtitle {
                                span class="sh-list-item-subtitle" { (subtitle) }
                            }
                        }

                        @if let Some(badge) = item.badge {
                            span class="sh-list-item-badge" { (badge) }
                        }

                        @if let Some(action) = item.action {
                            div class="sh-list-item-action" { (action) }
                        }
                    }
                }
            }
        }
    }
}

/// ListGroup - Grouped lists with headers
pub struct ListGroup<'a> {
    groups: Vec<ListGroupSection<'a>>,
    variant: ListVariant,
    size: ComponentSize,
}

pub struct ListGroupSection<'a> {
    pub title: Option<&'a str>,
    pub items: Vec<ListItem<'a>>,
    pub footer: Option<&'a str>,
}

impl<'a> ListGroup<'a> {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            variant: ListVariant::Default,
            size: ComponentSize::Md,
        }
    }

    pub fn groups(mut self, groups: Vec<ListGroupSection<'a>>) -> Self {
        self.groups = groups;
        self
    }

    pub fn variant(mut self, variant: ListVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for ListGroup<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for ListGroup<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-list-group" {
                @for group in &self.groups {
                    @if let Some(title) = group.title {
                        div class="sh-list-group-header" { (title) }
                    }

                    (List {
                        items: group.items.clone(),
                        variant: self.variant,
                        size: self.size,
                        interactive: false,
                    })

                    @if let Some(footer) = group.footer {
                        div class="sh-list-group-footer" { (footer) }
                    }
                }
            }
        }
    }
}

/// DefinitionList - Key-value pairs
pub struct DefinitionList<'a> {
    items: Vec<DefinitionItem<'a>>,
    variant: DefinitionListVariant,
    size: ComponentSize,
}

pub struct DefinitionItem<'a> {
    pub term: &'a str,
    pub description: &'a str,
    pub icon: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DefinitionListVariant {
    #[default]
    Default,
    Horizontal,
    Bordered,
}

impl<'a> DefinitionList<'a> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            variant: DefinitionListVariant::Default,
            size: ComponentSize::Md,
        }
    }

    pub fn items(mut self, items: Vec<DefinitionItem<'a>>) -> Self {
        self.items = items;
        self
    }

    pub fn variant(mut self, variant: DefinitionListVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for DefinitionList<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for DefinitionList<'a> {
    fn render(&self) -> Markup {
        let variant_class = match self.variant {
            DefinitionListVariant::Default => "sh-deflist--default",
            DefinitionListVariant::Horizontal => "sh-deflist--horizontal",
            DefinitionListVariant::Bordered => "sh-deflist--bordered",
        };

        let size_class = match self.size {
            ComponentSize::Xs => "sh-deflist--xs",
            ComponentSize::Sm => "sh-deflist--sm",
            ComponentSize::Md => "sh-deflist--md",
            ComponentSize::Lg => "sh-deflist--lg",
            ComponentSize::Xl => "sh-deflist--xl",
        };

        html! {
            dl class=(format!("sh-deflist {} {}", variant_class, size_class)) {
                @for item in &self.items {
                    div class="sh-deflist-item" {
                        @if let Some(icon) = item.icon {
                            dt class="sh-deflist-icon" { (icon) }
                        }
                        dt class="sh-deflist-term" { (item.term) }
                        dd class="sh-deflist-desc" { (item.description) }
                    }
                }
            }
        }
    }
}

/// Generate list CSS
pub fn list_css() -> String {
    r#"
    .sh-list {
        display: flex;
        flex-direction: column;
        list-style: none;
        padding: 0;
        margin: 0;
        width: 100%;
    }

    .sh-list-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1rem;
        border-bottom: 1px solid var(--sh-border, #e5e7eb);
    }

    .sh-list--borderless .sh-list-item {
        border-bottom: none;
    }

    .sh-list--striped .sh-list-item:nth-child(even) {
        background: var(--sh-surface, #f9fafb);
    }

    .sh-list--ordered .sh-list-item {
        list-style-type: decimal;
        padding-left: 1.5rem;
    }

    .sh-list--interactive .sh-list-item--clickable {
        cursor: pointer;
    }

    .sh-list--interactive .sh-list-item--clickable:hover {
        background: var(--sh-surface, #f9fafb);
    }

    .sh-list-item--disabled {
        opacity: 0.5;
    }

    .sh-list-item-icon {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        height: 24px;
        flex-shrink: 0;
        color: var(--sh-text-secondary, #6b7280);
    }

    .sh-list-item-avatar {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        overflow: hidden;
        flex-shrink: 0;
    }

    .sh-list-item-avatar img {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    .sh-list-item-content {
        display: flex;
        flex-direction: column;
        flex: 1;
        min-width: 0;
    }

    .sh-list-item-main {
        font-size: 0.9375rem;
        color: var(--sh-text, #1f2937);
    }

    .sh-list-item-subtitle {
        font-size: 0.8125rem;
        color: var(--sh-text-secondary, #6b7280);
    }

    .sh-list-item-badge {
        padding: 0.25rem 0.5rem;
        font-size: 0.75rem;
        font-weight: 500;
        background: var(--sh-primary-light, #dbeafe);
        color: var(--sh-primary, #3b82f6);
        border-radius: var(--sh-radius-sm, 0.25rem);
    }

    .sh-list-item-action {
        flex-shrink: 0;
    }

    .sh-list-group {
        display: flex;
        flex-direction: column;
        width: 100%;
    }

    .sh-list-group-header {
        padding: 0.75rem 1rem;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: var(--sh-text-secondary, #6b7280);
        background: var(--sh-surface, #f9fafb);
    }

    .sh-list-group-footer {
        padding: 0.5rem 1rem;
        font-size: 0.8125rem;
        color: var(--sh-text-secondary, #6b7280);
        background: var(--sh-surface, #f9fafb);
    }

    .sh-deflist {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .sh-deflist-item {
        display: flex;
        align-items: flex-start;
        gap: 1rem;
    }

    .sh-deflist--horizontal .sh-deflist-item {
        flex-direction: column;
    }

    .sh-deflist--bordered .sh-deflist-item {
        padding: 0.75rem;
        border: 1px solid var(--sh-border, #e5e7eb);
        border-radius: var(--sh-radius-md, 0.5rem);
    }

    .sh-deflist-term {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--sh-text, #1f2937);
        min-width: 120px;
    }

    .sh-deflist-desc {
        font-size: 0.9375rem;
        color: var(--sh-text-secondary, #4b5563);
        margin: 0;
    }

    .sh-list--xs .sh-list-item { padding: 0.5rem 0.75rem; }
    .sh-list--xs .sh-list-item-main { font-size: 0.75rem; }
    .sh-list--xs .sh-list-item-subtitle { font-size: 0.6875rem; }

    .sh-list--sm .sh-list-item { padding: 0.625rem 0.875rem; }
    .sh-list--sm .sh-list-item-main { font-size: 0.8125rem; }
    .sh-list--sm .sh-list-item-subtitle { font-size: 0.75rem; }

    .sh-list--lg .sh-list-item { padding: 1rem 1.25rem; }
    .sh-list--lg .sh-list-item-main { font-size: 1rem; }
    .sh-list--lg .sh-list-item-subtitle { font-size: 0.875rem; }

    .sh-list--xl .sh-list-item { padding: 1.25rem 1.5rem; }
    .sh-list--xl .sh-list-item-main { font-size: 1.0625rem; }
    .sh-list--xl .sh-list-item-subtitle { font-size: 0.9375rem; }

    .sh-deflist--xs .sh-deflist-term { font-size: 0.75rem; }
    .sh-deflist--xs .sh-deflist-desc { font-size: 0.8125rem; }

    .sh-deflist--sm .sh-deflist-term { font-size: 0.8125rem; }
    .sh-deflist--sm .sh-deflist-desc { font-size: 0.875rem; }

    .sh-deflist--lg .sh-deflist-term { font-size: 0.9375rem; }
    .sh-deflist--lg .sh-deflist-desc { font-size: 1rem; }

    .sh-deflist--xl .sh-deflist-term { font-size: 1rem; }
    .sh-deflist--xl .sh-deflist-desc { font-size: 1.0625rem; }
    "#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let list = List::new();
        assert_eq!(list.variant, ListVariant::Default);
        assert_eq!(list.size, ComponentSize::Md);
    }

    #[test]
    fn test_list_variant() {
        let list = List::new().variant(ListVariant::Ordered);
        assert_eq!(list.variant, ListVariant::Ordered);
    }

    #[test]
    fn test_list_size() {
        let list = List::new().size(ComponentSize::Lg);
        assert_eq!(list.size, ComponentSize::Lg);
    }

    #[test]
    fn test_list_css() {
        let css = list_css();
        assert!(css.contains(".sh-list"));
        assert!(css.contains(".sh-list-item"));
    }
}
