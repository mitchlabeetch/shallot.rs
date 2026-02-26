//! Tag Input Component
//!
//! Input for managing multiple tags/chips.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct TagInput<'a> {
    name: &'a str,
    tags: Vec<&'a str>,
    placeholder: Option<&'a str>,
    size: ComponentSize,
    disabled: bool,
    max_tags: Option<usize>,
}

impl<'a> TagInput<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            tags: Vec::new(),
            placeholder: None,
            size: ComponentSize::Md,
            disabled: false,
            max_tags: None,
        }
    }

    pub fn tags(mut self, tags: Vec<&'a str>) -> Self {
        self.tags = tags;
        self
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn max_tags(mut self, max: usize) -> Self {
        self.max_tags = Some(max);
        self
    }
}

impl<'a> Render for TagInput<'a> {
    fn render(&self) -> Markup {
        let input_id = format!("sh-tag-input-{}", self.name);
        let size_class = format!("sh-tag-input--{}", self.size.class_suffix());
        let can_add_more = self.max_tags.map_or(true, |max| self.tags.len() < max);

        html! {
            div class={(format!("sh-tag-input {} {}", size_class, if self.disabled { "sh-tag-input--disabled" } else { "" }))} {
                @for (i, tag) in self.tags.iter().enumerate() {
                    @let _tag_id = format!("{}-tag-{}", self.name, i);

                    span class="sh-tag" {
                        input
                            type="hidden"
                            name={(format!("{}[]", self.name))}
                            value=(tag);
                        span class="sh-tag__text" { (tag) }
                        @if !self.disabled {
                            button
                                type="button"
                                class="sh-tag__remove"
                                aria-label=(format!("Remove {}", tag))
                            {
                                svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    line x1="18" y1="6" x2="6" y2="18";
                                    line x1="6" y1="6" x2="18" y2="18";
                                }
                            }
                        }
                    }
                }

                @if can_add_more && !self.disabled {
                    input
                        type="text"
                        class="sh-tag-input__field"
                        id=(input_id)
                        placeholder=(self.placeholder.unwrap_or("Add tag..."));
                }
            }
        }
    }
}

pub struct Tag<'a> {
    text: &'a str,
    variant: TagVariant,
    size: ComponentSize,
    removable: bool,
    href: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Error,
    Info,
}

impl<'a> Tag<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            variant: TagVariant::Default,
            size: ComponentSize::Md,
            removable: false,
            href: None,
        }
    }

    pub fn variant(mut self, variant: TagVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            TagVariant::Default => "",
            TagVariant::Primary => "sh-tag--primary",
            TagVariant::Success => "sh-tag--success",
            TagVariant::Warning => "sh-tag--warning",
            TagVariant::Error => "sh-tag--error",
            TagVariant::Info => "sh-tag--info",
        }
    }
}

impl<'a> Render for Tag<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-tag--{}", self.size.class_suffix());

        html! {
            @if let Some(href) = self.href {
                a href=(href) class={(format!("sh-tag {} {} sh-tag--link", self.variant_class(), size_class))} {
                    span class="sh-tag__text" { (self.text) }
                }
            } @else {
                span class={(format!("sh-tag {} {}", self.variant_class(), size_class))} {
                    span class="sh-tag__text" { (self.text) }
                    @if self.removable {
                        button type="button" class="sh-tag__remove" aria-label="Remove" {
                            svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                line x1="18" y1="6" x2="6" y2="18";
                                line x1="6" y1="6" x2="18" y2="18";
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct TagList<'a> {
    tags: Vec<Tag<'a>>,
}

impl<'a> TagList<'a> {
    pub fn new(tags: Vec<Tag<'a>>) -> Self {
        Self { tags }
    }
}

impl<'a> Render for TagList<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-tag-list" {
                @for tag in &self.tags {
                    (tag)
                }
            }
        }
    }
}

pub fn tag_input_css() -> String {
    r#"
.sh-tag-input {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
    min-height: 2.5rem;
}

.sh-tag-input:focus-within {
    border-color: var(--sh-accent, #3b82f6);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent, #3b82f6) 15%, transparent);
}

.sh-tag-input--disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background: var(--sh-surface-2, #f3f4f6);
}

.sh-tag-input__field {
    flex: 1;
    min-width: 100px;
    padding: 0.25rem 0;
    font-size: 0.875rem;
    border: none;
    background: transparent;
    color: var(--sh-text, #1f2937);
    outline: none;
}

.sh-tag-input__field::placeholder {
    color: var(--sh-text-muted, #6b7280);
}

/* Individual tag */
.sh-tag {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    background: var(--sh-surface-2, #f3f4f6);
    border-radius: var(--sh-radius-sm, 0.25rem);
    white-space: nowrap;
}

.sh-tag--link {
    text-decoration: none;
    cursor: pointer;
}

.sh-tag--link:hover {
    background: var(--sh-surface-hover, #e5e7eb);
}

.sh-tag__remove {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    padding: 0;
    margin-left: 0.125rem;
    background: none;
    border: none;
    color: var(--sh-text-muted, #6b7280);
    cursor: pointer;
    border-radius: 50%;
    transition: all 0.15s ease;
}

.sh-tag__remove:hover {
    background: rgba(0, 0, 0, 0.1);
    color: var(--sh-text, #1f2937);
}

/* Tag variants */
.sh-tag--primary {
    background: color-mix(in srgb, var(--sh-accent, #3b82f6) 15%, transparent);
    color: var(--sh-accent, #3b82f6);
}

.sh-tag--success {
    background: color-mix(in srgb, var(--sh-success, #10b981) 15%, transparent);
    color: var(--sh-success, #10b981);
}

.sh-tag--warning {
    background: color-mix(in srgb, var(--sh-warning, #f59e0b) 15%, transparent);
    color: var(--sh-warning, #f59e0b);
}

.sh-tag--error {
    background: color-mix(in srgb, var(--sh-error, #ef4444) 15%, transparent);
    color: var(--sh-error, #ef4444);
}

.sh-tag--info {
    background: color-mix(in srgb, var(--sh-info, #3b82f6) 15%, transparent);
    color: var(--sh-info, #3b82f6);
}

/* Size variants */
.sh-tag--sm {
    padding: 0.125rem 0.375rem;
    font-size: 0.6875rem;
}

.sh-tag--lg {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
}

/* Tag list */
.sh-tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
}

/* Size variants for input */
.sh-tag-input--sm {
    padding: 0.375rem;
    min-height: 2rem;
}

.sh-tag-input--lg {
    padding: 0.625rem;
    min-height: 3rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_input_creation() {
        let input = TagInput::new("tags")
            .tags(vec!["rust", "web"])
            .placeholder("Add tag...");

        assert_eq!(input.name, "tags");
        assert_eq!(input.tags.len(), 2);
    }

    #[test]
    fn test_tag_creation() {
        let tag = Tag::new("Featured")
            .variant(TagVariant::Primary)
            .removable(true);

        assert_eq!(tag.text, "Featured");
        assert!(tag.removable);
    }
}
