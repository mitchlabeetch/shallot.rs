//! Accordion Component
//!
//! Collapsible content panels for organizing information.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct AccordionItem<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub content: Markup,
    pub open: bool,
    pub disabled: bool,
}

pub struct Accordion<'a> {
    items: Vec<AccordionItem<'a>>,
    size: ComponentSize,
    variant: AccordionVariant,
    allow_multiple: bool,
    bordered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionVariant {
    #[default]
    Default,
    Filled,
    Bordered,
    Minimal,
}

impl<'a> Accordion<'a> {
    pub fn new(items: Vec<AccordionItem<'a>>) -> Self {
        Self {
            items,
            size: ComponentSize::Md,
            variant: AccordionVariant::Default,
            allow_multiple: false,
            bordered: false,
        }
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: AccordionVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn allow_multiple(mut self, allow: bool) -> Self {
        self.allow_multiple = allow;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-accordion".to_string()];

        classes.push(format!("sh-accordion--{}", self.size.class_suffix()));
        classes.push(format!("sh-accordion--{}", self.variant_class()));

        if self.bordered {
            classes.push("sh-accordion--bordered".to_string());
        }

        classes.join(" ")
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            AccordionVariant::Default => "default",
            AccordionVariant::Filled => "filled",
            AccordionVariant::Bordered => "bordered",
            AccordionVariant::Minimal => "minimal",
        }
    }
}

impl<'a> Render for Accordion<'a> {
    fn render(&self) -> Markup {
        let accordion_class = self.build_classes();

        html! {
            div class=(accordion_class) role="region" {
                @for item in &self.items {
                    @let item_id = format!("sh-accordion-{}", item.id);
                    @let content_id = format!("sh-accordion-content-{}", item.id);
                    @let item_classes = {
                        let mut cls = vec!["sh-accordion-item"];
                        if item.open { cls.push("sh-accordion-item--open"); }
                        if item.disabled { cls.push("sh-accordion-item--disabled"); }
                        cls.join(" ")
                    };

                    details
                        class=(item_classes)
                        id=(item_id)
                        open?[item.open]
                    {
                        summary
                            class="sh-accordion-header"
                            aria-expanded=(if item.open { "true" } else { "false" })
                            aria-controls=(content_id)
                        {
                            span class="sh-accordion-title" { (item.title) }
                            span class="sh-accordion-icon" aria-hidden="true" {
                                svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    polyline points="6 9 12 15 18 9";
                                }
                            }
                        }

                        div
                            class="sh-accordion-content"
                            id=(content_id)
                            role="region"
                            aria-labelledby=(item_id)
                        {
                            (item.content.clone())
                        }
                    }
                }
            }
        }
    }
}

pub fn accordion_css() -> String {
    r#"
.sh-accordion {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
}

.sh-accordion-item {
    background: var(--sh-surface, #fff);
    border-radius: var(--sh-radius-md, 0.5rem);
    overflow: hidden;
}

.sh-accordion-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    font-size: 0.9375rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    cursor: pointer;
    list-style: none;
    user-select: none;
    transition: background 0.2s ease;
}

.sh-accordion-header::-webkit-details-marker {
    display: none;
}

.sh-accordion-header:hover {
    background: var(--sh-surface-hover, #f9fafb);
}

.sh-accordion-item[open] .sh-accordion-icon {
    transform: rotate(180deg);
}

.sh-accordion-icon {
    flex-shrink: 0;
    transition: transform 0.2s ease;
    color: var(--sh-text-muted, #6b7280);
}

.sh-accordion-content {
    padding: 0 1.25rem 1rem;
    font-size: 0.875rem;
    line-height: 1.6;
    color: var(--sh-text-secondary, #4b5563);
    animation: sh-accordion-slide 0.2s ease;
}

@keyframes sh-accordion-slide {
    from {
        opacity: 0;
        transform: translateY(-8px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* Variants */
.sh-accordion--filled .sh-accordion-item {
    background: var(--sh-surface-2, #f9fafb);
}

.sh-accordion--filled .sh-accordion-header:hover {
    background: var(--sh-surface, #fff);
}

.sh-accordion--bordered .sh-accordion-item {
    border: 1px solid var(--sh-border, #e5e7eb);
}

.sh-accordion--bordered .sh-accordion-item[open] {
    border-color: var(--sh-accent, #3b82f6);
}

.sh-accordion--minimal .sh-accordion-item {
    background: transparent;
}

.sh-accordion--minimal .sh-accordion-header {
    padding-left: 0;
    padding-right: 0;
}

.sh-accordion--minimal .sh-accordion-content {
    padding-left: 0;
    padding-right: 0;
}

/* States */
.sh-accordion-item--disabled .sh-accordion-header {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

/* Size variants */
.sh-accordion--sm .sh-accordion-header {
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
}

.sh-accordion--sm .sh-accordion-content {
    padding: 0 1rem 0.75rem;
    font-size: 0.8125rem;
}

.sh-accordion--lg .sh-accordion-header {
    padding: 1.25rem 1.5rem;
    font-size: 1rem;
}

.sh-accordion--lg .sh-accordion-content {
    padding: 0 1.5rem 1.25rem;
    font-size: 0.9375rem;
}

/* Bordered container */
.sh-accordion--bordered {
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
}

.sh-accordion--bordered .sh-accordion-item {
    border-radius: 0;
    border: none;
    border-bottom: 1px solid var(--sh-border, #e5e7eb);
}

.sh-accordion--bordered .sh-accordion-item:last-child {
    border-bottom: none;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_creation() {
        let items = vec![AccordionItem {
            id: "item1",
            title: "First Section",
            content: html! { p { "Content here" } },
            open: true,
            disabled: false,
        }];

        let accordion = Accordion::new(items).variant(AccordionVariant::Bordered);

        assert_eq!(accordion.items.len(), 1);
        assert_eq!(accordion.variant, AccordionVariant::Bordered);
    }

    #[test]
    fn test_accordion_classes() {
        let items = vec![];
        let accordion = Accordion::new(items).bordered(true).size(ComponentSize::Lg);

        let classes = accordion.build_classes();
        assert!(classes.contains("sh-accordion--lg"));
        assert!(classes.contains("sh-accordion--bordered"));
    }
}
