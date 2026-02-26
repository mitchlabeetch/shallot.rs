# Shallot Components 🧅

The core UI component library for Shallot.rs. This crate provides 129+ production-ready, accessible, and beautiful UI components built with Rust, Maud, and pure CSS.

## Features

- **Zero JavaScript**: Every component is interactive using only CSS techniques (checkbox hack, `:target`, peer selectors).
- **Type-Safe Templates**: Built on [Maud](https://maud.lambda.xyz/), ensuring your HTML is valid at compile-time.
- **Accessible by Default**: Automatic ARIA attribute management, semantic HTML landmarks, and keyboard navigation support.
- **Typed CSS**: Styles are generated programmatically, avoiding magic strings and ensuring theme consistency.
- **Hardware Accelerated**: Animations use CSS transforms and opacity for 60fps performance even on low-end devices.

## Component Categories

- **Layout**: `Box`, `Grid`, `Stack`, `Container`, `Section`, `Masonry`, `ZStack`.
- **Forms**: `Input`, `Select`, `Checkbox`, `Radio`, `Switch`, `MultiSelect`, `RichText`, `CreditCardInput`.
- **Navigation**: `Navbar`, `Sidebar`, `Dock`, `Breadcrumbs`, `Pagination`, `Tabs`, `Steps`.
- **Data Display**: `Card`, `Table`, `Avatar`, `Badge`, `Timeline`, `Stats`, `Treeview`, `Calendar`.
- **Overlays**: `Modal`, `Drawer`, `Popover`, `Tooltip`, `Dropdown`, `Toast`, `BottomSheet`.
- **Feedback**: `Alert`, `Progress`, `Skeleton`, `Spinner`, `EmptyState`.
- **Effects (Magic)**: `BorderBeam`, `Confetti`, `OrbitingCircles`, `Shimmer`, `LiquidButton`, `MagicCard`.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
shallot_components = { version = "0.1", path = "../shallot_components" }
maud = "0.26"
```

### Basic Component

```rust
use shallot_components::button::{Button, ButtonVariant, ButtonSize};
use maud::html;

fn render_button() -> maud::Markup {
    html! {
        (Button::new("Explore Components")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Lg)
            .render())
    }
}
```

### CSS Management

To get the styles for your components, call `all_component_css()` to get a complete bundle, or call individual CSS functions for a minimal payload:

```rust
use shallot_components::all_component_css;

// In your server route or static generator
let styles = all_component_css();
```

## Development

### Adding a Component

1. Create a new module in `src/`.
2. Implement the `Render` trait for your struct.
3. Provide a `{component}_css()` function.
4. Export and register in `src/lib.rs`.

### Testing

Run the full suite of component tests (including visual regression snapshots):

```bash
cargo test
```

## License

MIT