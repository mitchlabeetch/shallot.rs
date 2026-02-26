//! Menu Component
//!
//! Navigation menus and dropdown menus with CSS-only interactions.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct MenuItem<'a> {
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub badge: Option<&'a str>,
    pub disabled: bool,
    pub active: bool,
    pub children: Vec<MenuItem<'a>>,
}

pub struct Menu<'a> {
    items: Vec<MenuItem<'a>>,
    size: ComponentSize,
    variant: MenuVariant,
    label: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuVariant {
    #[default]
    Default,
    Compact,
    Pills,
    Bordered,
}

impl<'a> Menu<'a> {
    pub fn new(items: Vec<MenuItem<'a>>) -> Self {
        Self {
            items,
            size: ComponentSize::Md,
            variant: MenuVariant::Default,
            label: None,
        }
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: MenuVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            MenuVariant::Default => "sh-menu--default",
            MenuVariant::Compact => "sh-menu--compact",
            MenuVariant::Pills => "sh-menu--pills",
            MenuVariant::Bordered => "sh-menu--bordered",
        }
    }

    fn render_item(&self, item: &MenuItem<'a>) -> Markup {
        if !item.children.is_empty() {
            return self.render_submenu(item);
        }

        let item_classes = {
            let mut cls = vec!["sh-menu-item"];
            if item.disabled {
                cls.push("sh-menu-item--disabled");
            }
            if item.active {
                cls.push("sh-menu-item--active");
            }
            cls.join(" ")
        };

        html! {
            li class=(item_classes) {
                @if let Some(href) = item.href {
                    a href=(href) class="sh-menu-item__link" {
                        @if let Some(icon) = item.icon {
                            span class="sh-menu-item__icon" {
                                (maud::PreEscaped(icon))
                            }
                        }
                        span class="sh-menu-item__label" { (item.label) }
                        @if let Some(badge) = item.badge {
                            span class="sh-menu-item__badge" { (badge) }
                        }
                    }
                } @else {
                    span class="sh-menu-item__link" {
                        @if let Some(icon) = item.icon {
                            span class="sh-menu-item__icon" {
                                (maud::PreEscaped(icon))
                            }
                        }
                        span class="sh-menu-item__label" { (item.label) }
                        @if let Some(badge) = item.badge {
                            span class="sh-menu-item__badge" { (badge) }
                        }
                    }
                }
            }
        }
    }

    fn render_submenu(&self, item: &MenuItem<'a>) -> Markup {
        let item_classes = {
            let mut cls = vec!["sh-menu-item sh-menu-item--has-submenu"];
            if item.disabled {
                cls.push("sh-menu-item--disabled");
            }
            if item.active {
                cls.push("sh-menu-item--active");
            }
            cls.join(" ")
        };

        html! {
            li class=(item_classes) {
                details class="sh-submenu-wrapper" {
                    summary class="sh-menu-item__link" {
                        @if let Some(icon) = item.icon {
                            span class="sh-menu-item__icon" {
                                (maud::PreEscaped(icon))
                            }
                        }
                        span class="sh-menu-item__label" { (item.label) }
                        span class="sh-menu-item__arrow" {
                            svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                polyline points="6 9 12 15 18 9";
                            }
                        }
                    }
                    ul class="sh-submenu" {
                        @for child in &item.children {
                            (self.render_item(child))
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Render for Menu<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-menu--{}", self.size.class_suffix());

        html! {
            nav class={(format!("sh-menu {} {} {}", self.variant_class(), size_class, ""))} {
                @if let Some(label) = self.label {
                    div class="sh-menu__label" { (label) }
                }
                ul class="sh-menu__list" role="menu" {
                    @for item in &self.items {
                        (self.render_item(item))
                    }
                }
            }
        }
    }
}

pub struct DropdownMenu<'a> {
    id: &'a str,
    trigger: Markup,
    items: Vec<MenuItem<'a>>,
    position: DropdownPosition,
    size: ComponentSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownPosition {
    #[default]
    BottomStart,
    BottomEnd,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
}

impl DropdownPosition {
    fn class(&self) -> &'static str {
        match self {
            DropdownPosition::BottomStart => "sh-dropdown--bottom-start",
            DropdownPosition::BottomEnd => "sh-dropdown--bottom-end",
            DropdownPosition::TopStart => "sh-dropdown--top-start",
            DropdownPosition::TopEnd => "sh-dropdown--top-end",
            DropdownPosition::LeftStart => "sh-dropdown--left-start",
            DropdownPosition::LeftEnd => "sh-dropdown--left-end",
            DropdownPosition::RightStart => "sh-dropdown--right-start",
            DropdownPosition::RightEnd => "sh-dropdown--right-end",
        }
    }
}

impl<'a> DropdownMenu<'a> {
    pub fn new(id: &'a str, trigger: Markup, items: Vec<MenuItem<'a>>) -> Self {
        Self {
            id,
            trigger,
            items,
            position: DropdownPosition::BottomStart,
            size: ComponentSize::Md,
        }
    }

    pub fn position(mut self, position: DropdownPosition) -> Self {
        self.position = position;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Render for DropdownMenu<'a> {
    fn render(&self) -> Markup {
        let dropdown_id = format!("sh-dropdown-{}", self.id);
        let size_class = format!("sh-dropdown--{}", self.size.class_suffix());

        html! {
            div class={(format!("sh-dropdown {} {}", self.position.class(), size_class))} id=(dropdown_id) {
                details class="sh-dropdown__details" {
                    summary class="sh-dropdown__trigger" {
                        (self.trigger.clone())
                    }
                    div class="sh-dropdown__content" {
                        ul class="sh-dropdown__menu" role="menu" {
                            @for item in &self.items {
                                (self.render_dropdown_item(item))
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<'a> DropdownMenu<'a> {
    fn render_dropdown_item(&self, item: &MenuItem<'a>) -> Markup {
        let item_classes = {
            let mut cls = vec!["sh-dropdown-item"];
            if item.disabled {
                cls.push("sh-dropdown-item--disabled");
            }
            if item.active {
                cls.push("sh-dropdown-item--active");
            }
            cls.join(" ")
        };

        html! {
            li class=(item_classes) role="menuitem" {
                @if let Some(href) = item.href {
                    a href=(href) class="sh-dropdown-item__link" {
                        @if let Some(icon) = item.icon {
                            span class="sh-dropdown-item__icon" {
                                (maud::PreEscaped(icon))
                            }
                        }
                        (item.label)
                        @if let Some(badge) = item.badge {
                            span class="sh-dropdown-item__badge" { (badge) }
                        }
                    }
                } @else {
                    span class="sh-dropdown-item__link" {
                        @if let Some(icon) = item.icon {
                            span class="sh-dropdown-item__icon" {
                                (maud::PreEscaped(icon))
                            }
                        }
                        (item.label)
                    }
                }
            }
        }
    }
}

pub struct MenuDivider;

impl Render for MenuDivider {
    fn render(&self) -> Markup {
        html! {
            li class="sh-menu-divider" role="separator" {}
        }
    }
}

pub struct MenuGroup<'a> {
    pub label: &'a str,
    pub items: Vec<MenuItem<'a>>,
}

pub fn menu_css() -> String {
    r#"
.sh-menu {
    display: flex;
    flex-direction: column;
    width: 100%;
}

.sh-menu__label {
    padding: 0.5rem 1rem;
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--sh-text-muted, #6b7280);
}

.sh-menu__list {
    list-style: none;
    margin: 0;
    padding: 0;
}

.sh-menu-item {
    position: relative;
}

.sh-menu-item__link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 1rem;
    font-size: 0.875rem;
    color: var(--sh-text, #1f2937);
    text-decoration: none;
    cursor: pointer;
    transition: all 0.15s ease;
}

.sh-menu-item__link:hover {
    background: var(--sh-surface-hover, #f3f4f6);
    color: var(--sh-text, #1f2937);
}

.sh-menu-item--active .sh-menu-item__link {
    background: color-mix(in srgb, var(--sh-accent, #3b82f6) 10%, transparent);
    color: var(--sh-accent, #3b82f6);
}

.sh-menu-item--disabled .sh-menu-item__link {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

.sh-menu-item__icon {
    flex-shrink: 0;
    width: 1rem;
    height: 1rem;
    color: var(--sh-text-muted, #6b7280);
}

.sh-menu-item--active .sh-menu-item__icon {
    color: var(--sh-accent, #3b82f6);
}

.sh-menu-item__label {
    flex: 1;
}

.sh-menu-item__badge {
    padding: 0.125rem 0.5rem;
    font-size: 0.6875rem;
    font-weight: 500;
    background: var(--sh-accent, #3b82f6);
    color: #fff;
    border-radius: 9999px;
}

.sh-menu-item__arrow {
    flex-shrink: 0;
    width: 1rem;
    height: 1rem;
    color: var(--sh-text-muted, #6b7280);
    transition: transform 0.2s ease;
}

.sh-submenu-wrapper[open] .sh-menu-item__arrow {
    transform: rotate(180deg);
}

.sh-submenu {
    list-style: none;
    margin: 0;
    padding: 0;
    padding-left: 1.5rem;
}

/* Menu variants */
.sh-menu--compact .sh-menu-item__link {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
}

.sh-menu--pills .sh-menu-item__link {
    border-radius: var(--sh-radius-md, 0.5rem);
    margin: 0.125rem 0.5rem;
}

.sh-menu--bordered .sh-menu-item__link {
    border-left: 2px solid transparent;
}

.sh-menu--bordered .sh-menu-item--active .sh-menu-item__link {
    border-left-color: var(--sh-accent, #3b82f6);
    background: transparent;
}

/* Divider */
.sh-menu-divider {
    height: 1px;
    background: var(--sh-border, #e5e7eb);
    margin: 0.5rem 0;
}

/* Dropdown */
.sh-dropdown {
    position: relative;
    display: inline-block;
}

.sh-dropdown__details {
    position: relative;
}

.sh-dropdown__trigger {
    cursor: pointer;
    list-style: none;
}

.sh-dropdown__trigger::-webkit-details-marker {
    display: none;
}

.sh-dropdown__content {
    display: none;
    position: absolute;
    z-index: 100;
    min-width: 180px;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
    box-shadow: var(--sh-shadow-lg, 0 10px 15px -3px rgba(0, 0, 0, 0.1));
    animation: sh-dropdown-enter 0.15s ease;
}

.sh-dropdown__details[open] .sh-dropdown__content {
    display: block;
}

@keyframes sh-dropdown-enter {
    from {
        opacity: 0;
        transform: translateY(-4px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* Dropdown positions */
.sh-dropdown--bottom-start .sh-dropdown__content {
    top: 100%;
    left: 0;
    margin-top: 0.25rem;
}

.sh-dropdown--bottom-end .sh-dropdown__content {
    top: 100%;
    right: 0;
    margin-top: 0.25rem;
}

.sh-dropdown--top-start .sh-dropdown__content {
    bottom: 100%;
    left: 0;
    margin-bottom: 0.25rem;
}

.sh-dropdown--top-end .sh-dropdown__content {
    bottom: 100%;
    right: 0;
    margin-bottom: 0.25rem;
}

.sh-dropdown__menu {
    list-style: none;
    margin: 0;
    padding: 0.25rem 0;
}

.sh-dropdown-item__link {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    color: var(--sh-text, #1f2937);
    text-decoration: none;
    transition: background 0.15s ease;
}

.sh-dropdown-item__link:hover {
    background: var(--sh-surface-hover, #f3f4f6);
}

.sh-dropdown-item--disabled .sh-dropdown-item__link {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-dropdown-item__icon {
    width: 1rem;
    height: 1rem;
    color: var(--sh-text-muted, #6b7280);
}

.sh-dropdown-item__badge {
    margin-left: auto;
    padding: 0.125rem 0.375rem;
    font-size: 0.6875rem;
    background: var(--sh-accent, #3b82f6);
    color: #fff;
    border-radius: 9999px;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_creation() {
        let items = vec![MenuItem {
            label: "Home",
            href: Some("/"),
            icon: None,
            badge: None,
            disabled: false,
            active: true,
            children: vec![],
        }];

        let menu = Menu::new(items).variant(MenuVariant::Pills);

        assert_eq!(menu.items.len(), 1);
        assert_eq!(menu.variant, MenuVariant::Pills);
    }

    #[test]
    fn test_dropdown_creation() {
        let trigger = html! { button { "Menu" } };
        let items = vec![];

        let dropdown =
            DropdownMenu::new("test", trigger, items).position(DropdownPosition::BottomEnd);

        assert_eq!(dropdown.id, "test");
        assert_eq!(dropdown.position, DropdownPosition::BottomEnd);
    }
}
