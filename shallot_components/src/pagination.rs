//! Pagination Component
//!
//! Navigation for paginated content with CSS-only interactions.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct Pagination<'a> {
    current: u32,
    total: u32,
    base_url: &'a str,
    size: ComponentSize,
    variant: PaginationVariant,
    show_first_last: bool,
    show_prev_next: bool,
    max_visible: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaginationVariant {
    #[default]
    Default,
    Compact,
    Pills,
    Bordered,
}

impl<'a> Pagination<'a> {
    pub fn new(current: u32, total: u32, base_url: &'a str) -> Self {
        Self {
            current,
            total,
            base_url,
            size: ComponentSize::Md,
            variant: PaginationVariant::Default,
            show_first_last: true,
            show_prev_next: true,
            max_visible: 5,
        }
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: PaginationVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_first_last(mut self, show: bool) -> Self {
        self.show_first_last = show;
        self
    }

    pub fn show_prev_next(mut self, show: bool) -> Self {
        self.show_prev_next = show;
        self
    }

    pub fn max_visible(mut self, max: u32) -> Self {
        self.max_visible = max;
        self
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            PaginationVariant::Default => "sh-pagination--default",
            PaginationVariant::Compact => "sh-pagination--compact",
            PaginationVariant::Pills => "sh-pagination--pills",
            PaginationVariant::Bordered => "sh-pagination--bordered",
        }
    }

    fn build_page_range(&self) -> Vec<PageNumber> {
        let mut pages = Vec::new();
        let total = self.total;
        let current = self.current;
        let max = self.max_visible;

        if total <= max + 2 {
            for i in 1..=total {
                pages.push(PageNumber::Page(i));
            }
        } else {
            pages.push(PageNumber::Page(1));

            let half = max / 2;
            let start = if current <= half + 1 {
                2
            } else if current >= total - half {
                total - max
            } else {
                current - half
            };

            let end = (start + max - 1).min(total - 1);

            if start > 2 {
                pages.push(PageNumber::Ellipsis);
            }

            for i in start..=end {
                pages.push(PageNumber::Page(i));
            }

            if end < total - 1 {
                pages.push(PageNumber::Ellipsis);
            }

            pages.push(PageNumber::Page(total));
        }

        pages
    }

    fn url_for_page(&self, page: u32) -> String {
        if self.base_url.contains("{page}") {
            self.base_url.replace("{page}", &page.to_string())
        } else if self.base_url.contains('?') {
            format!("{}&page={}", self.base_url, page)
        } else {
            format!("{}?page={}", self.base_url, page)
        }
    }
}

enum PageNumber {
    Page(u32),
    Ellipsis,
}

impl<'a> Render for Pagination<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-pagination--{}", self.size.class_suffix());
        let pages = self.build_page_range();
        let prev_page = if self.current > 1 {
            Some(self.current - 1)
        } else {
            None
        };
        let next_page = if self.current < self.total {
            Some(self.current + 1)
        } else {
            None
        };

        html! {
            nav class={(format!("sh-pagination {} {}", self.variant_class(), size_class))} role="navigation" aria-label="Pagination" {
                ul class="sh-pagination__list" {
                    @if self.show_first_last && self.current > 1 {
                        li class="sh-pagination__item" {
                            a href=(self.url_for_page(1)) class="sh-pagination__link sh-pagination__link--first" aria-label="First page" {
                                svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    polyline points="11 17 6 12 11 7";
                                    polyline points="18 17 13 12 18 7";
                                }
                            }
                        }
                    }

                    @if self.show_prev_next {
                        li class="sh-pagination__item" {
                            @if let Some(prev) = prev_page {
                                a href=(self.url_for_page(prev)) class="sh-pagination__link sh-pagination__link--prev" aria-label="Previous page" {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                        polyline points="15 18 9 12 15 6";
                                    }
                                }
                            } @else {
                                span class="sh-pagination__link sh-pagination__link--prev sh-pagination__link--disabled" {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                        polyline points="15 18 9 12 15 6";
                                    }
                                }
                            }
                        }
                    }

                    @for page in &pages {
                        @match page {
                            PageNumber::Page(num) => {
                                li class="sh-pagination__item" {
                                    @if *num == self.current {
                                        span class="sh-pagination__link sh-pagination__link--active" aria-current="page" {
                                            (num)
                                        }
                                    } @else {
                                        a href=(self.url_for_page(*num)) class="sh-pagination__link" {
                                            (num)
                                        }
                                    }
                                }
                            }
                            PageNumber::Ellipsis => {
                                li class="sh-pagination__item" {
                                    span class="sh-pagination__ellipsis" { "..." }
                                }
                            }
                        }
                    }

                    @if self.show_prev_next {
                        li class="sh-pagination__item" {
                            @if let Some(next) = next_page {
                                a href=(self.url_for_page(next)) class="sh-pagination__link sh-pagination__link--next" aria-label="Next page" {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                        polyline points="9 18 15 12 9 6";
                                    }
                                }
                            } @else {
                                span class="sh-pagination__link sh-pagination__link--next sh-pagination__link--disabled" {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                        polyline points="9 18 15 12 9 6";
                                    }
                                }
                            }
                        }
                    }

                    @if self.show_first_last && self.current < self.total {
                        li class="sh-pagination__item" {
                            a href=(self.url_for_page(self.total)) class="sh-pagination__link sh-pagination__link--last" aria-label="Last page" {
                                svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                    polyline points="13 17 18 12 13 7";
                                    polyline points="6 17 11 12 6 7";
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct SimplePagination<'a> {
    current: u32,
    total: u32,
    base_url: &'a str,
}

impl<'a> SimplePagination<'a> {
    pub fn new(current: u32, total: u32, base_url: &'a str) -> Self {
        Self {
            current,
            total,
            base_url,
        }
    }

    fn url_for_page(&self, page: u32) -> String {
        if self.base_url.contains("{page}") {
            self.base_url.replace("{page}", &page.to_string())
        } else if self.base_url.contains('?') {
            format!("{}&page={}", self.base_url, page)
        } else {
            format!("{}?page={}", self.base_url, page)
        }
    }
}

impl<'a> Render for SimplePagination<'a> {
    fn render(&self) -> Markup {
        let prev_page = if self.current > 1 {
            Some(self.current - 1)
        } else {
            None
        };
        let next_page = if self.current < self.total {
            Some(self.current + 1)
        } else {
            None
        };

        html! {
            nav class="sh-pagination sh-pagination--simple" role="navigation" {
                div class="sh-pagination__info" {
                    span { "Page " (self.current) " of " (self.total) }
                }
                div class="sh-pagination__nav" {
                    @if let Some(prev) = prev_page {
                        a href=(self.url_for_page(prev)) class="sh-pagination__btn sh-pagination__btn--prev" {
                            "Previous"
                        }
                    }

                    @if let Some(next) = next_page {
                        a href=(self.url_for_page(next)) class="sh-pagination__btn sh-pagination__btn--next" {
                            "Next"
                        }
                    }
                }
            }
        }
    }
}

pub struct PaginationInfo {
    pub current: u32,
    pub total: u32,
    pub per_page: u32,
    pub total_items: u32,
}

impl Render for PaginationInfo {
    fn render(&self) -> Markup {
        let start = (self.current - 1) * self.per_page + 1;
        let end = (self.current * self.per_page).min(self.total_items);

        html! {
            div class="sh-pagination-info" {
                span { "Showing " (start) "-" (end) " of " (self.total_items) " results" }
            }
        }
    }
}

pub fn pagination_css() -> String {
    r#"
.sh-pagination {
    display: flex;
    align-items: center;
    justify-content: center;
}

.sh-pagination__list {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    list-style: none;
    margin: 0;
    padding: 0;
}

.sh-pagination__item {
    display: flex;
}

.sh-pagination__link {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 2.25rem;
    height: 2.25rem;
    padding: 0 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    text-decoration: none;
    border-radius: var(--sh-radius-md, 0.5rem);
    transition: all 0.15s ease;
}

.sh-pagination__link:hover:not(.sh-pagination__link--disabled):not(.sh-pagination__link--active) {
    background: var(--sh-surface-hover, #f3f4f6);
}

.sh-pagination__link--active {
    background: var(--sh-accent, #3b82f6);
    color: #fff;
}

.sh-pagination__link--disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

.sh-pagination__ellipsis {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 2.25rem;
    height: 2.25rem;
    color: var(--sh-text-muted, #6b7280);
}

/* Size variants */
.sh-pagination--sm .sh-pagination__link {
    min-width: 1.875rem;
    height: 1.875rem;
    font-size: 0.8125rem;
}

.sh-pagination--lg .sh-pagination__link {
    min-width: 2.75rem;
    height: 2.75rem;
    font-size: 1rem;
}

/* Variant: Pills */
.sh-pagination--pills .sh-pagination__link {
    border-radius: 9999px;
}

/* Variant: Bordered */
.sh-pagination--bordered .sh-pagination__link {
    border: 1px solid var(--sh-border, #e5e7eb);
    background: var(--sh-surface, #fff);
}

.sh-pagination--bordered .sh-pagination__link:hover:not(.sh-pagination__link--active) {
    border-color: var(--sh-accent, #3b82f6);
}

.sh-pagination--bordered .sh-pagination__link--active {
    border-color: var(--sh-accent, #3b82f6);
}

/* Simple pagination */
.sh-pagination--simple {
    flex-direction: column;
    gap: 0.75rem;
}

.sh-pagination__info {
    font-size: 0.875rem;
    color: var(--sh-text-muted, #6b7280);
}

.sh-pagination__nav {
    display: flex;
    gap: 0.75rem;
}

.sh-pagination__btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    text-decoration: none;
    background: var(--sh-surface, #fff);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
    transition: all 0.15s ease;
}

.sh-pagination__btn:hover {
    background: var(--sh-surface-hover, #f3f4f6);
    border-color: var(--sh-accent, #3b82f6);
}

/* Pagination info */
.sh-pagination-info {
    font-size: 0.875rem;
    color: var(--sh-text-muted, #6b7280);
}

/* Responsive */
@media (max-width: 640px) {
    .sh-pagination__link {
        min-width: 2rem;
        height: 2rem;
        font-size: 0.8125rem;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_creation() {
        let pagination = Pagination::new(3, 10, "/items").variant(PaginationVariant::Pills);

        assert_eq!(pagination.current, 3);
        assert_eq!(pagination.total, 10);
        assert_eq!(pagination.variant, PaginationVariant::Pills);
    }

    #[test]
    fn test_page_range() {
        let pagination = Pagination::new(5, 20, "/items").max_visible(5);
        let pages = pagination.build_page_range();

        assert!(pages.len() > 0);
    }

    #[test]
    fn test_url_generation() {
        let pagination = Pagination::new(1, 10, "/items");
        assert_eq!(pagination.url_for_page(2), "/items?page=2");

        let pagination2 = Pagination::new(1, 10, "/items?category=1");
        assert_eq!(pagination2.url_for_page(2), "/items?category=1&page=2");

        let pagination3 = Pagination::new(1, 10, "/items/{page}");
        assert_eq!(pagination3.url_for_page(2), "/items/2");
    }
}
