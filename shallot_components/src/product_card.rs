//! Product Card Component - E-Commerce Product Display
//!
//! A comprehensive product card component for e-commerce interfaces.
//! Implements research section 5.7.1 - E-Commerce patterns.
//!
//! # Example
//! ```
//! use shallot_components::product_card::{ProductCard, ProductPrice, Rating};
//! use maud::html;
//!
//! let card = ProductCard::new("Premium Widget")
//!     .image("/widget.jpg")
//!     .price(ProductPrice::sale(99.99, 149.99))
//!     .rating(Rating::new(4.5, 128))
//!     .badge("New")
//!     .quick_action(html! { "Add to Cart" });
//! ```

use maud::{html, Markup};

/// Product price with optional sale
#[derive(Debug, Clone)]
pub enum ProductPrice {
    Regular(f64),
    Sale { current: f64, original: f64 },
    Range { min: f64, max: f64 },
    From(f64),
}

impl ProductPrice {
    pub fn regular(price: f64) -> Self {
        Self::Regular(price)
    }

    pub fn sale(current: f64, original: f64) -> Self {
        Self::Sale { current, original }
    }

    pub fn range(min: f64, max: f64) -> Self {
        Self::Range { min, max }
    }

    pub fn from(price: f64) -> Self {
        Self::From(price)
    }

    fn format_price(price: f64) -> String {
        format!("${:.2}", price)
    }

    pub fn render(&self) -> Markup {
        match self {
            Self::Regular(price) => html! {
                span class="sh-product-price" {
                    (Self::format_price(*price))
                }
            },
            Self::Sale { current, original } => html! {
                span class="sh-product-price sh-product-price--sale" {
                    span class="sh-product-price-current" {
                        (Self::format_price(*current))
                    }
                    span class="sh-product-price-original" {
                        (Self::format_price(*original))
                    }
                }
            },
            Self::Range { min, max } => html! {
                span class="sh-product-price sh-product-price--range" {
                    (Self::format_price(*min))
                    " - "
                    (Self::format_price(*max))
                }
            },
            Self::From(price) => html! {
                span class="sh-product-price sh-product-price--from" {
                    "From "
                    (Self::format_price(*price))
                }
            },
        }
    }

    /// Get the primary price value for sorting
    pub fn value(&self) -> f64 {
        match self {
            Self::Regular(p) => *p,
            Self::Sale { current, .. } => *current,
            Self::Range { min, .. } => *min,
            Self::From(p) => *p,
        }
    }
}

/// Star rating display
#[derive(Debug, Clone)]
pub struct Rating {
    value: f32, // 0.0 to 5.0
    count: Option<u32>,
    size: RatingSize,
    show_empty: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingSize {
    Sm,
    Md,
    Lg,
}

impl Rating {
    pub fn new(value: f32, count: u32) -> Self {
        Self {
            value: value.clamp(0.0, 5.0),
            count: Some(count),
            size: RatingSize::Md,
            show_empty: true,
        }
    }

    pub fn without_count(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 5.0),
            count: None,
            size: RatingSize::Md,
            show_empty: true,
        }
    }

    pub fn size(mut self, size: RatingSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_empty(mut self, show: bool) -> Self {
        self.show_empty = show;
        self
    }

    fn size_class(&self) -> &'static str {
        match self.size {
            RatingSize::Sm => "sh-rating--sm",
            RatingSize::Md => "sh-rating--md",
            RatingSize::Lg => "sh-rating--lg",
        }
    }

    fn star_icon(filled: bool, half: bool) -> Markup {
        html! {
            span class=(if half { "sh-star sh-star--half" } else if filled { "sh-star sh-star--filled" } else { "sh-star sh-star--empty" }) {
                svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                    @if half {
                        // Half-filled star
                        defs {
                            linearGradient id="half" x1="0" x2="1" y1="0" y2="0" {
                                stop offset="50%" stop-color="currentColor" {}
                                stop offset="50%" stop-color="transparent" {}
                            }
                        }
                        path fill="url(#half)" d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" {}
                    } @else if filled {
                        path fill="currentColor" d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" {}
                    } @else {
                        path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" {}
                    }
                }
            }
        }
    }

    pub fn render(&self) -> Markup {
        let full_stars = self.value.floor() as u8;
        let has_half = self.value - self.value.floor() >= 0.5;
        let empty_stars = if self.show_empty {
            5 - full_stars - if has_half { 1 } else { 0 }
        } else {
            0
        };

        let size_class = self.size_class();

        html! {
            div class=(format!("sh-rating {}", size_class)) {
                div class="sh-stars" aria-label=(format!("Rated {} out of 5 stars", self.value)) {
                    @for _ in 0..full_stars {
                        (Self::star_icon(true, false))
                    }
                    @if has_half {
                        (Self::star_icon(false, true))
                    }
                    @for _ in 0..empty_stars {
                        (Self::star_icon(false, false))
                    }
                }
                @if let Some(count) = self.count {
                    span class="sh-rating-count" {
                        "("
                        (count)
                        ")"
                    }
                }
            }
        }
    }
}

/// Product card variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductCardVariant {
    Default,
    Compact,
    Horizontal,
    Feature,
}

/// Product card component
#[derive(Debug, Clone)]
pub struct ProductCard {
    title: String,
    image: Option<String>,
    image_alt: Option<String>,
    price: Option<ProductPrice>,
    rating: Option<Rating>,
    description: Option<String>,
    badge: Option<(String, BadgeStyle)>,
    quick_action: Option<Markup>,
    variant: ProductCardVariant,
    href: Option<String>,
    class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeStyle {
    New,
    Sale,
    Popular,
    Limited,
    Custom(&'static str),
}

impl BadgeStyle {
    fn css_class(&self) -> &'static str {
        match self {
            Self::New => "sh-badge--new",
            Self::Sale => "sh-badge--sale",
            Self::Popular => "sh-badge--popular",
            Self::Limited => "sh-badge--limited",
            Self::Custom(_) => "sh-badge--custom",
        }
    }

    #[allow(dead_code)] // Reserved for future badge text generation
    fn default_text(&self) -> &'static str {
        match self {
            Self::New => "New",
            Self::Sale => "Sale",
            Self::Popular => "Popular",
            Self::Limited => "Limited",
            Self::Custom(s) => s,
        }
    }
}

impl ProductCard {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            image: None,
            image_alt: None,
            price: None,
            rating: None,
            description: None,
            badge: None,
            quick_action: None,
            variant: ProductCardVariant::Default,
            href: None,
            class: None,
        }
    }

    pub fn image(mut self, src: impl Into<String>) -> Self {
        self.image = Some(src.into());
        self
    }

    pub fn image_alt(mut self, alt: impl Into<String>) -> Self {
        self.image_alt = Some(alt.into());
        self
    }

    pub fn price(mut self, price: ProductPrice) -> Self {
        self.price = Some(price);
        self
    }

    pub fn rating(mut self, rating: Rating) -> Self {
        self.rating = Some(rating);
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn badge(mut self, text: impl Into<String>) -> Self {
        self.badge = Some((text.into(), BadgeStyle::New));
        self
    }

    pub fn styled_badge(mut self, text: impl Into<String>, style: BadgeStyle) -> Self {
        self.badge = Some((text.into(), style));
        self
    }

    pub fn quick_action(mut self, action: Markup) -> Self {
        self.quick_action = Some(action);
        self
    }

    pub fn variant(mut self, variant: ProductCardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    fn container_class(&self) -> String {
        let mut classes = vec!["sh-product-card".to_string()];

        match self.variant {
            ProductCardVariant::Compact => classes.push("sh-product-card--compact".to_string()),
            ProductCardVariant::Horizontal => {
                classes.push("sh-product-card--horizontal".to_string())
            }
            ProductCardVariant::Feature => classes.push("sh-product-card--feature".to_string()),
            _ => {}
        }

        if self.href.is_some() {
            classes.push("sh-product-card--link".to_string());
        }

        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }

        classes.join(" ")
    }

    fn image_placeholder(&self) -> Markup {
        html! {
            div class="sh-product-image-placeholder" {
                svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" {
                    rect x="3" y="3" width="18" height="18" rx="2" ry="2" {}
                    circle cx="8.5" cy="8.5" r="1.5" {}
                    polyline points="21 15 16 10 5 21" {}
                }
            }
        }
    }

    fn render_image(&self) -> Markup {
        html! {
            div class="sh-product-image-wrapper" {
                @if let Some(ref src) = self.image {
                    img
                        class="sh-product-image"
                        src=(src)
                        alt=[self.image_alt.as_deref().or(Some(&self.title))]
                        loading="lazy"
                    {}                } @else {
                    (self.image_placeholder())
                }

                @if let Some((ref text, ref style)) = self.badge {
                    span class=(format!("sh-product-badge {}", style.css_class())) {
                        (text)
                    }
                }
            }
        }
    }

    fn render_content(&self) -> Markup {
        html! {
            div class="sh-product-content" {
                h3 class="sh-product-title" {
                    @if let Some(ref href) = self.href {
                        a href=(href) { (&self.title) }
                    } @else {
                        (&self.title)
                    }
                }

                @if let Some(ref rating) = self.rating {
                    div class="sh-product-rating" {
                        (rating.render())
                    }
                }

                @if let Some(ref desc) = self.description {
                    p class="sh-product-description" {
                        (desc)
                    }
                }

                div class="sh-product-footer" {
                    @if let Some(ref price) = self.price {
                        div class="sh-product-price-wrapper" {
                            (price.render())
                        }
                    }

                    @if let Some(ref action) = self.quick_action {
                        div class="sh-product-action" {
                            (action)
                        }
                    }
                }
            }
        }
    }

    pub fn render(self) -> Markup {
        let class = self.container_class();

        html! {
            article class=(class) {
                (self.render_image())
                (self.render_content())
            }
        }
    }
}

/// Cart summary component
#[derive(Debug, Clone)]
pub struct CartSummary {
    items: Vec<CartItem>,
    subtotal: f64,
    shipping: Option<f64>,
    tax: Option<f64>,
    currency: String,
}

#[derive(Debug, Clone)]
pub struct CartItem {
    name: String,
    quantity: u32,
    price: f64,
    image: Option<String>,
}

impl CartItem {
    pub fn new(name: impl Into<String>, quantity: u32, price: f64) -> Self {
        Self {
            name: name.into(),
            quantity,
            price,
            image: None,
        }
    }

    pub fn image(mut self, src: impl Into<String>) -> Self {
        self.image = Some(src.into());
        self
    }

    fn total(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

impl CartSummary {
    pub fn new(items: Vec<CartItem>) -> Self {
        let subtotal = items.iter().map(|i| i.total()).sum();
        Self {
            items,
            subtotal,
            shipping: None,
            tax: None,
            currency: "$".to_string(),
        }
    }

    pub fn shipping(mut self, amount: f64) -> Self {
        self.shipping = Some(amount);
        self
    }

    pub fn tax(mut self, amount: f64) -> Self {
        self.tax = Some(amount);
        self
    }

    pub fn currency(mut self, symbol: impl Into<String>) -> Self {
        self.currency = symbol.into();
        self
    }

    fn total(&self) -> f64 {
        self.subtotal + self.shipping.unwrap_or(0.0) + self.tax.unwrap_or(0.0)
    }

    fn format_price(&self, amount: f64) -> String {
        format!("{}{:.2}", self.currency, amount)
    }

    pub fn render(&self) -> Markup {
        html! {
            div class="sh-cart-summary" {
                h3 class="sh-cart-title" { "Order Summary" }

                ul class="sh-cart-items" {
                    @for item in &self.items {
                        li class="sh-cart-item" {
                            @if let Some(ref img) = item.image {
                                img class="sh-cart-item-image" src=(img) alt="" {}
                            }
                            div class="sh-cart-item-details" {
                                span class="sh-cart-item-name" { (&item.name) }
                                span class="sh-cart-item-qty" { "Qty: " (item.quantity) }
                            }
                            span class="sh-cart-item-price" {
                                (self.format_price(item.total()))
                            }
                        }
                    }
                }

                div class="sh-cart-totals" {
                    div class="sh-cart-row" {
                        span { "Subtotal" }
                        span { (self.format_price(self.subtotal)) }
                    }
                    @if let Some(shipping) = self.shipping {
                        div class="sh-cart-row" {
                            span { "Shipping" }
                            span { (self.format_price(shipping)) }
                        }
                    }
                    @if let Some(tax) = self.tax {
                        div class="sh-cart-row" {
                            span { "Tax" }
                            span { (self.format_price(tax)) }
                        }
                    }
                    div class="sh-cart-row sh-cart-row--total" {
                        span { "Total" }
                        span { (self.format_price(self.total())) }
                    }
                }
            }
        }
    }
}

/// Quantity stepper component
#[derive(Debug, Clone)]
pub struct QuantityStepper {
    value: u32,
    min: u32,
    max: Option<u32>,
    name: String,
}

impl QuantityStepper {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            value: 1,
            min: 1,
            max: None,
            name: name.into(),
        }
    }

    pub fn value(mut self, value: u32) -> Self {
        self.value = value.max(self.min);
        self
    }

    pub fn min(mut self, min: u32) -> Self {
        self.min = min;
        self.value = self.value.max(min);
        self
    }

    pub fn max(mut self, max: u32) -> Self {
        self.max = Some(max);
        self.value = self.value.min(max);
        self
    }

    pub fn render(self) -> Markup {
        let can_decrement = self.value > self.min;
        let can_increment = self.max.map(|m| self.value < m).unwrap_or(true);

        html! {
            div class="sh-quantity-stepper" {
                button
                    class="sh-quantity-btn sh-quantity-btn--decrement"
                    type="button"
                    disabled=[if !can_decrement { Some("") } else { None }]
                    aria-label="Decrease quantity"
                {
                    "−"
                }
                input
                    class="sh-quantity-input"
                    type="number"
                    name=(self.name)
                    value=(self.value)
                    min=(self.min)
                    max=[self.max.map(|m| m.to_string())]
                    aria-label="Quantity"
                {}
                button
                    class="sh-quantity-btn sh-quantity-btn--increment"
                    type="button"
                    disabled=[if !can_increment { Some("") } else { None }]
                    aria-label="Increase quantity"
                {
                    "+"
                }
            }
        }
    }
}

/// Generate CSS for product card components
pub fn product_card_css() -> String {
    r#"
/* Product Card Styles */

.sh-product-card {
    display: flex;
    flex-direction: column;
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: 0.75rem;
    overflow: hidden;
    transition: transform 0.2s, box-shadow 0.2s;
}

.sh-product-card--link:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 24px -8px rgba(0, 0, 0, 0.15);
}

.sh-product-card--horizontal {
    flex-direction: row;
}

.sh-product-card--compact .sh-product-description {
    display: none;
}

.sh-product-image-wrapper {
    position: relative;
    aspect-ratio: 1 / 1;
    background: var(--sh-surface-alt);
    overflow: hidden;
}

.sh-product-card--horizontal .sh-product-image-wrapper {
    width: 40%;
    aspect-ratio: auto;
}

.sh-product-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s;
}

.sh-product-card:hover .sh-product-image {
    transform: scale(1.05);
}

.sh-product-image-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    color: var(--sh-text-muted);
}

.sh-product-image-placeholder svg {
    width: 48px;
    height: 48px;
}

.sh-product-badge {
    position: absolute;
    top: 0.75rem;
    left: 0.75rem;
    padding: 0.25rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-radius: 9999px;
}

.sh-badge--new {
    background: var(--sh-success);
    color: white;
}

.sh-badge--sale {
    background: var(--sh-error);
    color: white;
}

.sh-badge--popular {
    background: var(--sh-warning);
    color: black;
}

.sh-badge--limited {
    background: var(--sh-accent);
    color: white;
}

.sh-product-content {
    display: flex;
    flex-direction: column;
    flex: 1;
    padding: 1rem;
    gap: 0.5rem;
}

.sh-product-title {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    line-height: 1.4;
}

.sh-product-title a {
    color: inherit;
    text-decoration: none;
}

.sh-product-title a:hover {
    color: var(--sh-accent);
}

.sh-product-description {
    margin: 0;
    font-size: 0.875rem;
    color: var(--sh-text-muted);
    line-height: 1.5;
}

.sh-product-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: auto;
    padding-top: 0.75rem;
}

/* Rating */
.sh-rating {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.sh-stars {
    display: flex;
    gap: 0.125rem;
    color: var(--sh-warning);
}

.sh-star {
    display: flex;
}

.sh-star svg {
    width: 1em;
    height: 1em;
}

.sh-rating--sm .sh-star svg {
    width: 12px;
    height: 12px;
}

.sh-rating--lg .sh-star svg {
    width: 20px;
    height: 20px;
}

.sh-rating-count {
    font-size: 0.875rem;
    color: var(--sh-text-muted);
}

/* Price */
.sh-product-price {
    font-weight: 600;
    color: var(--sh-text);
}

.sh-product-price--sale {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.sh-product-price-current {
    color: var(--sh-error);
}

.sh-product-price-original {
    font-size: 0.875em;
    text-decoration: line-through;
    color: var(--sh-text-muted);
}

/* Cart Summary */
.sh-cart-summary {
    padding: 1.5rem;
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: 0.75rem;
}

.sh-cart-title {
    margin: 0 0 1rem;
    font-size: 1.125rem;
    font-weight: 600;
}

.sh-cart-items {
    list-style: none;
    margin: 0 0 1rem;
    padding: 0;
    border-bottom: 1px solid var(--sh-border);
}

.sh-cart-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 0;
}

.sh-cart-item-image {
    width: 48px;
    height: 48px;
    object-fit: cover;
    border-radius: 0.375rem;
}

.sh-cart-item-details {
    display: flex;
    flex-direction: column;
    flex: 1;
}

.sh-cart-item-name {
    font-weight: 500;
}

.sh-cart-item-qty {
    font-size: 0.875rem;
    color: var(--sh-text-muted);
}

.sh-cart-item-price {
    font-weight: 600;
}

.sh-cart-totals {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.sh-cart-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.875rem;
}

.sh-cart-row--total {
    margin-top: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid var(--sh-border);
    font-size: 1.125rem;
    font-weight: 600;
}

/* Quantity Stepper */
.sh-quantity-stepper {
    display: flex;
    align-items: center;
    border: 1px solid var(--sh-border);
    border-radius: 0.375rem;
    overflow: hidden;
}

.sh-quantity-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    padding: 0;
    font-size: 1.25rem;
    font-weight: 500;
    color: var(--sh-text);
    background: var(--sh-surface);
    border: none;
    cursor: pointer;
    transition: background 0.2s;
}

.sh-quantity-btn:hover:not(:disabled) {
    background: var(--sh-surface-alt);
}

.sh-quantity-btn:disabled {
    color: var(--sh-text-muted);
    cursor: not-allowed;
}

.sh-quantity-input {
    width: 3rem;
    height: 2.5rem;
    text-align: center;
    font-size: 0.875rem;
    color: var(--sh-text);
    background: var(--sh-surface);
    border: none;
    border-left: 1px solid var(--sh-border);
    border-right: 1px solid var(--sh-border);
    -moz-appearance: textfield;
}

.sh-quantity-input::-webkit-outer-spin-button,
.sh-quantity-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-product-card,
    .sh-product-image {
        transition: none;
    }
    
    .sh-product-card--link:hover {
        transform: none;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_price() {
        let regular = ProductPrice::regular(99.99);
        assert_eq!(regular.value(), 99.99);

        let sale = ProductPrice::sale(79.99, 99.99);
        assert_eq!(sale.value(), 79.99);

        let range = ProductPrice::range(49.99, 99.99);
        assert_eq!(range.value(), 49.99);
    }

    #[test]
    fn test_rating() {
        let rating = Rating::new(4.5, 128).size(RatingSize::Lg);

        assert_eq!(rating.value, 4.5);
        assert_eq!(rating.count, Some(128));
        assert_eq!(rating.size, RatingSize::Lg);
    }

    #[test]
    fn test_rating_clamping() {
        let high = Rating::without_count(7.0);
        assert_eq!(high.value, 5.0);

        let low = Rating::without_count(-2.0);
        assert_eq!(low.value, 0.0);
    }

    #[test]
    fn test_product_card() {
        let card = ProductCard::new("Test Product")
            .image("/test.jpg")
            .price(ProductPrice::regular(99.99))
            .rating(Rating::new(4.5, 100))
            .badge("New");

        assert_eq!(card.title, "Test Product");
        assert!(card.image.is_some());
        assert!(card.price.is_some());
    }

    #[test]
    fn test_cart_item() {
        let item = CartItem::new("Widget", 2, 29.99).image("/widget.jpg");

        assert_eq!(item.name, "Widget");
        assert_eq!(item.quantity, 2);
        assert_eq!(item.price, 29.99);
        assert_eq!(item.total(), 59.98);
    }

    #[test]
    fn test_cart_summary() {
        let items = vec![
            CartItem::new("Item 1", 1, 10.0),
            CartItem::new("Item 2", 2, 5.0),
        ];

        let summary = CartSummary::new(items).shipping(5.0).tax(2.0);

        assert_eq!(summary.subtotal, 20.0);
        assert_eq!(summary.total(), 27.0);
    }

    #[test]
    fn test_quantity_stepper() {
        let stepper = QuantityStepper::new("qty").value(5).min(1).max(10);

        assert_eq!(stepper.value, 5);
        assert_eq!(stepper.min, 1);
        assert_eq!(stepper.max, Some(10));
    }

    #[test]
    fn test_badge_styles() {
        assert_eq!(BadgeStyle::Sale.css_class(), "sh-badge--sale");
        assert_eq!(BadgeStyle::New.default_text(), "New");
    }
}
