# Shallot Foundation 🧱

The architectural bedrock of Shallot.rs. This crate provides the design tokens, theming engine, and animation primitives that power the entire ecosystem.

## Overview

Shallot Foundation is a pure Rust library designed to manage the "Aura" (Style) layer of the Allium model. It translates high-level design intent into optimized, type-safe CSS variables and keyframes, adhering strictly to the **Zero-JavaScript** philosophy.

## Features

- **Type-Safe Design Tokens**: Manage HSL colors, spacing scales, border radii, and shadows with Rust's type system.
- **Dynamic Theming**: Support for light/dark modes and custom color palettes via CSS Custom Properties.
- **Declarative Animations**: Generate hardware-accelerated CSS `@keyframes` and easing functions programmatically.
- **Responsive Utilities**: Define breakpoints and responsive value mappings that compile to standard media queries.
- **Zero Dependencies**: Lightweight and fast, focusing exclusively on design system logic.

## Core Modules

- **`theme`**: The core `Theme` engine that orchestrates global style state and generates the root CSS variables.
- **`design_tokens`**: Primitives for `HSLColor`, `ColorPalette`, and scales for spacing and typography.
- **`animations`**: Tools for creating sophisticated CSS animations like `fade_in`, `slide_in`, and custom `Keyframe` sequences.
- **`responsive`**: Breakpoint definitions and viewport-aware styling utilities.
- **`transitions`**: Standardized easing functions (Cubic-Bezier) and transition timing presets.

## Usage

### Defining a Custom Theme

```rust
use shallot_foundation::theme::{Theme, ColorMode};
use shallot_foundation::design_tokens::HSLColor;

fn generate_styles() -> String {
    let my_theme = Theme::default()
        .with_mode(ColorMode::Dark)
        .with_primary(HSLColor::new(250.0, 80.0, 60.0)); // Royal Purple
    
    my_theme.to_css()
}
```

### Creating Animations

```rust
use shallot_foundation::animations::{KeyframeAnimation, EasingFunction};

fn bounce_css() -> String {
    KeyframeAnimation::new("sh-bounce")
        .add_keyframe(0, "transform: translateY(0);")
        .add_keyframe(50, "transform: translateY(-10px);")
        .add_keyframe(100, "transform: translateY(0);")
        .with_easing(EasingFunction::CubicBezier(0.16, 1.0, 0.3, 1.0))
        .to_css_string()
}
```

## Why Foundation?

By moving design logic into Rust, we eliminate "magic strings" in CSS and ensure that changes to the design system are propagated across all components with compile-time safety. This allows for complex visual effects—like glassmorphism and refractive light—to be managed through a structured API rather than scattered, unmaintainable stylesheets.

## License

MIT