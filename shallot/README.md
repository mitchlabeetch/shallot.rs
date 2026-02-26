# Shallot.rs 🦀✨

> **Iron & Glass.** A zero-JavaScript Rust UI component library proving that beautiful, accessible web interfaces can be built entirely without client-side JavaScript.

Shallot is a comprehensive UI ecosystem for Rust web development. It leverages the power of [Maud](https://maud.lambda.xyz/) for type-safe HTML generation and pure CSS for interactivity, animations, and state management.

## 🧅 The "Allium" Philosophy

Like the layers of a shallot, our system is fractal and self-contained:

1.  **The Bulb (Logic)**: Pure Rust structs and enums representing component state.
2.  **The Skin (Structure)**: Semantic HTML generated via type-safe Maud templates.
3.  **The Aura (Style)**: A typed design system that generates valid, hardware-accelerated CSS.

## 🚀 Key Features

-   **Zero JavaScript**: No runtime overhead, no XSS vectors, works perfectly with NoScript/Tor.
-   **Type-Safe**: Catch UI bugs at compile-time using Rust's ownership and type system.
-   **129+ Components**: From basic buttons to complex animated effects like `BorderBeam` and `LiquidButton`.
-   **Accessibility First**: WCAG AA+ compliant out of the box with semantic HTML landmarks and ARIA attributes.
-   **Performant**: Minimal CSS payloads, `content-visibility` optimizations, and no JS execution thread.

## 📦 Project Structure

-   `crates/shallot`: Umbrella crate for the entire library.
-   `crates/shallot_components`: The core UI library (Button, Card, Modal, etc.).
-   `crates/shallot_foundation`: Design tokens, theming, and animation primitives.
-   `crates/shallot_website`: The official showcase application.
-   `crates/shallot_build`: Multi-format build system (SSR, Static).

## 🛠 Quick Start

Add Shallot to your `Cargo.toml`:

```toml
[dependencies]
shallot = "0.1"
maud = "0.26"
```

### Basic Usage

```rust
use shallot::components::{Button, ButtonVariant};
use maud::html;

fn my_view() -> maud::Markup {
    html! {
        div {
            h1 { "Welcome to Shallot" }
            (Button::new("Get Started")
                .variant(ButtonVariant::Primary)
                .render())
        }
    }
}
```

### Injecting Styles

Shallot components generate their own CSS. You can aggregate it all or pull specific styles:

```rust
use shallot::components::all_component_css;

fn global_styles() -> String {
    let mut css = String::new();
    css.push_str(&all_component_css());
    css
}
```

## 🎨 Zero-JS Interactivity

Shallot uses advanced CSS techniques (the "checkbox hack", `:target`, and peer selectors) to handle interactive states:

-   **Modals**: Managed via `:target` or hidden checkboxes.
-   **Tabs**: Managed via radio button sibling selectors.
-   **Animations**: Hardware-accelerated CSS keyframes.

## 📜 License

Shallot is licensed under the MIT License.

---

*Built with ❤️ and 🦀 by the Shallot.rs contributors.*