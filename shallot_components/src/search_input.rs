//! Search Input Component
//!
//! Search input with clear button and keyboard navigation.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

#[derive(Clone)]
pub struct SearchInput<'a> {
    name: &'a str,
    placeholder: Option<&'a str>,
    value: Option<&'a str>,
    size: ComponentSize,
    variant: SearchVariant,
    disabled: bool,
    autofocus: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchVariant {
    #[default]
    Default,
    Filled,
    Underline,
    Expandable,
}

impl<'a> SearchInput<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            placeholder: None,
            value: None,
            size: ComponentSize::Md,
            variant: SearchVariant::Default,
            disabled: false,
            autofocus: false,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SearchVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            SearchVariant::Default => "",
            SearchVariant::Filled => "sh-search--filled",
            SearchVariant::Underline => "sh-search--underline",
            SearchVariant::Expandable => "sh-search--expandable",
        }
    }
}

impl<'a> Render for SearchInput<'a> {
    fn render(&self) -> Markup {
        let input_id = format!("sh-search-{}", self.name);
        let size_class = format!("sh-search--{}", self.size.class_suffix());

        html! {
            div class={(format!("sh-search {} {}", self.variant_class(), size_class))} {
                span class="sh-search__icon" {
                    svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        circle cx="11" cy="11" r="8";
                        line x1="21" y1="21" x2="16.65" y2="16.65";
                    }
                }

                input
                    type="search"
                    class="sh-search__input"
                    name=(self.name)
                    id=(input_id)
                    placeholder=(self.placeholder.unwrap_or("Search..."))
                    value=[self.value]
                    disabled?[self.disabled]
                    autofocus?[self.autofocus]
                    autocomplete="off";

                @if !self.disabled {
                    label for=(input_id) class="sh-search__clear" {
                        svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                            line x1="18" y1="6" x2="6" y2="18";
                            line x1="6" y1="6" x2="18" y2="18";
                        }
                    }
                }
            }
        }
    }
}

pub struct SearchWithResults<'a> {
    input: SearchInput<'a>,
    results: Vec<SearchResult<'a>>,
    show_results: bool,
}

pub struct SearchResult<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub href: Option<&'a str>,
    pub icon: Option<&'a str>,
}

impl<'a> SearchWithResults<'a> {
    pub fn new(input: SearchInput<'a>) -> Self {
        Self {
            input,
            results: Vec::new(),
            show_results: false,
        }
    }

    pub fn results(mut self, results: Vec<SearchResult<'a>>) -> Self {
        self.results = results;
        self
    }

    pub fn show_results(mut self, show: bool) -> Self {
        self.show_results = show;
        self
    }
}

impl<'a> Render for SearchWithResults<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-search-wrapper" {
                (self.input.clone())

                @if self.show_results && !self.results.is_empty() {
                    ul class="sh-search-results" role="listbox" {
                        @for result in &self.results {
                            li class="sh-search-result" role="option" {
                                @if let Some(href) = result.href {
                                    a href=(href) class="sh-search-result__link" {
                                        @if let Some(icon) = result.icon {
                                            span class="sh-search-result__icon" {
                                                (maud::PreEscaped(icon))
                                            }
                                        }
                                        div class="sh-search-result__content" {
                                            span class="sh-search-result__title" { (result.title) }
                                            @if let Some(desc) = result.description {
                                                span class="sh-search-result__desc" { (desc) }
                                            }
                                        }
                                    }
                                } @else {
                                    div class="sh-search-result__link" {
                                        @if let Some(icon) = result.icon {
                                            span class="sh-search-result__icon" {
                                                (maud::PreEscaped(icon))
                                            }
                                        }
                                        div class="sh-search-result__content" {
                                            span class="sh-search-result__title" { (result.title) }
                                            @if let Some(desc) = result.description {
                                                span class="sh-search-result__desc" { (desc) }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn search_input_css() -> String {
    r#"
.sh-search-wrapper {
    position: relative;
    width: 100%;
}

.sh-search {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
}

.sh-search__icon {
    position: absolute;
    left: 0.875rem;
    color: var(--sh-text-muted, #6b7280);
    pointer-events: none;
}

.sh-search__input {
    width: 100%;
    padding: 0.625rem 2.5rem 0.625rem 2.5rem;
    font-size: 0.9375rem;
    color: var(--sh-text, #1f2937);
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
    transition: all 0.2s ease;
}

.sh-search__input:focus {
    outline: none;
    border-color: var(--sh-accent, #3b82f6);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--sh-accent, #3b82f6) 15%, transparent);
}

.sh-search__input::placeholder {
    color: var(--sh-text-muted, #6b7280);
}

.sh-search__input::-webkit-search-cancel-button {
    display: none;
}

.sh-search__clear {
    position: absolute;
    right: 0.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    color: var(--sh-text-muted, #6b7280);
    cursor: pointer;
    border-radius: var(--sh-radius-sm, 0.25rem);
    transition: all 0.15s ease;
}

.sh-search__clear:hover {
    background: var(--sh-surface-hover, #f3f4f6);
    color: var(--sh-text, #1f2937);
}

/* Variants */
.sh-search--filled .sh-search__input {
    background: var(--sh-surface-2, #f3f4f6);
    border-color: transparent;
}

.sh-search--underline .sh-search__input {
    border-width: 0 0 1px 0;
    border-radius: 0;
    padding-left: 0;
    background: transparent;
}

.sh-search--underline .sh-search__icon {
    left: 0;
}

.sh-search--expandable {
    width: auto;
}

.sh-search--expandable .sh-search__input {
    width: 2.5rem;
    padding-right: 2.5rem;
    padding-left: 2.5rem;
    border-radius: 9999px;
    transition: width 0.3s ease;
}

.sh-search--expandable:focus-within .sh-search__input {
    width: 250px;
}

/* Size variants */
.sh-search--sm .sh-search__input {
    padding: 0.5rem 2.25rem 0.5rem 2.25rem;
    font-size: 0.875rem;
}

.sh-search--lg .sh-search__input {
    padding: 0.75rem 2.75rem 0.75rem 2.75rem;
    font-size: 1rem;
}

/* Results dropdown */
.sh-search-results {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 0.5rem;
    padding: 0.5rem 0;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
    box-shadow: var(--sh-shadow-lg, 0 10px 15px -3px rgba(0, 0, 0, 0.1));
    list-style: none;
    max-height: 300px;
    overflow-y: auto;
    z-index: 100;
}

.sh-search-result__link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 1rem;
    text-decoration: none;
    color: inherit;
    transition: background 0.15s ease;
}

a.sh-search-result__link:hover {
    background: var(--sh-surface-hover, #f3f4f6);
}

.sh-search-result__icon {
    flex-shrink: 0;
    width: 1.25rem;
    height: 1.25rem;
    color: var(--sh-text-muted, #6b7280);
}

.sh-search-result__content {
    flex: 1;
    min-width: 0;
}

.sh-search-result__title {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
}

.sh-search-result__desc {
    display: block;
    font-size: 0.75rem;
    color: var(--sh-text-muted, #6b7280);
    margin-top: 0.125rem;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_input_creation() {
        let search = SearchInput::new("q")
            .placeholder("Search products...")
            .variant(SearchVariant::Filled);

        assert_eq!(search.name, "q");
        assert_eq!(search.placeholder, Some("Search products..."));
    }

    #[test]
    fn test_search_with_results() {
        let input = SearchInput::new("search");
        let results = vec![SearchResult {
            title: "Result 1",
            description: Some("Description"),
            href: Some("/1"),
            icon: None,
        }];

        let search = SearchWithResults::new(input)
            .results(results)
            .show_results(true);

        assert_eq!(search.results.len(), 1);
    }
}
