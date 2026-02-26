# Shallot.rs - Agent Documentation

> **Iron & Glass.** A zero-JavaScript Rust UI component library proving that beautiful, accessible web interfaces can be built entirely without client-side JavaScript.

---

## Project Overview

**Shallot.rs** is a comprehensive Rust-first component library with a strict philosophical commitment: **absolutely zero JavaScript/TypeScript**. The project demonstrates that modern, beautiful, and accessible web interfaces can be built using only Rust, CSS, and semantic HTML.

### Core Philosophy (from `base_principles.md`)

1. **Zero JS/TS** - Regardless of safety features, the goal is philosophical: proving beautiful websites can exist without client-side JavaScript
2. **English Codebase** - Although some references may be in French, all code and documentation is in English
3. **Full-Stack Rust** - Both frontend and backend are implemented in Rust (no JS frontend)
4. **Beauty + Customizability** - Perfect middle ground between animations/visuals and customization
5. **Best Practices** - Absolute best Rust practices implemented from the beginning

### Architecture: The "Allium" Model

Like the layers of a shallot, the system is fractal and self-contained:

- **The Bulb (Logic)**: Pure Rust components with type-safe APIs
- **The Skin (Structure)**: Maud templates that bind logic to HTML
- **The Aura (Style)**: Typed design system - no magic strings, compile-time CSS validation

---

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Language | Rust (Edition 2021) | Core language |
| Templating | Maud 0.26 | Type-safe HTML generation |
| Web Framework | Axum 0.7 | Server-side routing |
| Runtime | Tokio | Async runtime |
| Styling | CSS Custom Properties | Theming and design tokens |
| Assets | tower-http | Static file serving |

### Dependencies Summary

```toml
# Core dependencies across crates
axum = "0.7"           # Web framework
maud = "0.26"          # HTML templating
tokio = "1"            # Async runtime
tower-http = "0.5"     # HTTP utilities
serde = "1.0"          # Serialization
thiserror = "1.0"      # Error handling
walkdir = "2.4"        # File walking (build)
```

---

## Project Structure

```
shallot/
├── Cargo.toml                    # Workspace definition
├── Cargo.lock                   # Dependency lock
├── AGENTS.md                   # Developer documentation
├── takeover.md                 # Project handoff notes
├── DEVELOPMENT_NOTES.md        # Technical difficulties
├── shallot_logo_svg.svg       # Project logo
│
├── apps/                       # Applications
│   ├── showcase/               # Main SSR showcase (Axum + Maud)
│   │   ├── Cargo.toml
│   │   └── src/main.rs        # Demo application
│   └── enhanced_showcase/      # Enhanced showcase
│
├── crates/                     # Library crates (moved from shallot.rs)
│   ├── shallot/               # Umbrella crate
│   ├── shallot_components/     # UI components library (55+ modules)
│   │   └── src/
│   │       ├── lib.rs         # Module declarations
│   │       ├── button.rs
│   │       └── [50+ other modules]
│   │
│   ├── shallot_foundation/    # Design tokens, theming, animations
│   │
│   ├── shallot_build/         # Build system
│   │
│   └── shallot_testing/       # Testing framework
│
├── tests/                      # Integration tests
│   └── integration_tests.rs
│
├── components_individual/       # Alternative component structure
│
├── open_ressources/            # Fonts, icons, assets
│
└── code_drafts/               # Draft code experiments
```
│   │       ├── enhanced_button.rs  # Enhanced button with animations
│   │       ├── enhanced_modal.rs   # Enhanced modal component
│   │       ├── layout.rs        # Grid, Stack, Container
│   │       ├── bento.rs         # Bento grid layout
│   │       ├── card.rs          # Card component
│   │       ├── form.rs          # Form primitives
│   │       ├── input.rs         # Input, Checkbox, Switch, etc.
│   │       ├── feedback.rs      # Dialog, Tooltip, Skeleton
│   │       ├── fx.rs            # Visual effects (ShinyButton, etc.)
│   │       ├── navbar.rs        # Navigation bar
│   │       ├── dock.rs          # macOS-style dock
│   │       ├── carousel.rs      # Carousel component
│   │       ├── calendar.rs      # Calendar widget
│   │       ├── marquee.rs       # Marquee animation
│   │       ├── typing_animation.rs
│   │       ├── text_reveal.rs
│   │       ├── orbiting_circles.rs
│   │       ├── confetti.rs
│   │       ├── border_beam.rs
│   │       └── ... (47 total modules)
│   │
│   ├── shallot_foundation/       # Design tokens, theming, animations
│   │   ├── Cargo.toml           # No external dependencies
│   │   └── src/
│   │       ├── lib.rs           # Public exports
│   │       ├── theme.rs         # ColorMode, Theme with CSS generation
│   │       ├── design_tokens.rs # HSLColor, ColorPalette, DesignTokens
│   │       ├── responsive.rs    # Breakpoint, ResponsiveValue
│   │       └── animations.rs    # KeyframeAnimation, easing functions
│   │
│   ├── shallot_testing/          # Testing framework
│   │   ├── Cargo.toml           # deps: serde, thiserror, criterion
│   │   └── src/lib.rs           # TestCase, TestSuite, TestRunner
│   │                            # accessibility, performance, visual modules
│   │
│   └── shallot_build/            # Multi-format build system
│       ├── Cargo.toml           # deps: serde, thiserror, walkdir
│       └── src/lib.rs           # BuildConfig, BuildSystem, BuildTarget
│                                # Supports: WASM, Native, SSR, Static
│
├── tests/                        # Integration tests
│   └── integration_tests.rs     # Component integration tests
│
├── docs/                         # Documentation
│   └── PRINCIPLES.md            # Design principles
│
├── openressources/               # Fonts, icons, assets
│   ├── Outfit_Complete/         # Outfit font family
│   ├── PublicSans_Complete/     # Public Sans font
│   ├── ionicons/                # Ionicons icon set
│   ├── radix-icons/             # Radix UI icons
│   └── ...
│
├── daisyui/                      # Reference: DaisyUI components
├── magicui/                      # Reference: MagicUI effects
├── otherinsp/                    # Other inspiration sources
├── backup_V1/                    # Legacy backup
│
└── [Documentation Files]
    ├── README.md                 # Main README
    ├── README_ENHANCED.md        # Enhanced documentation
    ├── IMPLEMENTATION_REPORT.md  # Comprehensive audit report
    ├── TODO_IMPLEMENTATION.md    # Implementation plan
    ├── base_principles.md        # Core principles
    ├── compo-lib-best-practices.txt
    ├── rust-best-practices.md
    └── enhanced_demo.html        # Static demo page
```

---

## Build and Development Commands

### Prerequisites

```bash
# Rust and Cargo (standard toolchain)
rustc --version  # Should be 1.70+

# Install wasm-pack for WebAssembly builds
cargo install wasm-pack

# Optional: cargo-watch for development
cargo install cargo-watch
```

### Build Commands

```bash
# Build entire workspace
cargo build

# Build release version
cargo build --release

# Run tests
cargo test --all-features

# Run specific test
cargo test test_enhanced_button_creation

# Build with specific features
cargo build --features ssr
cargo build --features wasm
```

### Running Showcase Applications

```bash
# Run the main showcase (port 3000)
cargo run --bin showcase

# Run the enhanced showcase (port 3000)
cargo run --bin enhanced_showcase

# With hot reloading (requires cargo-watch)
cargo watch -x "run --bin showcase"
```

### WebAssembly Build

```bash
# Build for WebAssembly
wasm-pack build --target web

# Build with wasm-pack in release mode
wasm-pack build --release --target web
```

---

## Code Style Guidelines

### Component Pattern

All components follow a consistent builder pattern:

```rust
// 1. Define enums for variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    // ...
}

// 2. Define size/variant enums
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Xs, Sm, Md, Lg, Xl,
}

// 3. Define struct with lifetime if needed
pub struct Button<'a> {
    pub label: &'a str,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub href: Option<&'a str>,
}

// 4. Implement constructor and builder methods
impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            disabled: false,
            href: None,
        }
    }
    
    // Builder pattern methods
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    
    // 5. Render method returns maud::Markup
    pub fn render(self) -> Markup {
        let variant_class = match self.variant {
            ButtonVariant::Primary => "sh-btn sh-btn--primary",
            // ...
        };
        
        html! {
            button type="button" class=(variant_class) {
                (self.label)
            }
        }
    }
}
```

### CSS Conventions

- All CSS classes prefixed with `sh-` (e.g., `sh-btn`, `sh-card`)
- CSS custom properties use `--sh-` prefix (e.g., `--sh-accent`, `--sh-bg`)
- Component variants use BEM-like naming: `sh-btn--primary`, `sh-btn--lg`
- CSS is generated programmatically and injected via `theme.css()` and `component_css()`

### Naming Conventions

- **Files**: `snake_case.rs` (e.g., `enhanced_button.rs`, `bento.rs`)
- **Structs**: `PascalCase` (e.g., `EnhancedButton`, `ColorPalette`)
- **Enums/Variants**: `PascalCase` (e.g., `ButtonVariant::Primary`)
- **Functions**: `snake_case` (e.g., `render()`, `to_css_string()`)
- **CSS Classes**: `kebab-case` with `sh-` prefix (e.g., `sh-btn--primary`)
- **CSS Variables**: `--sh-` prefix (e.g., `--sh-accent`)

---

## Testing Instructions

### Running Tests

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run specific test suite
cargo test test_integration_suite

# Run tests with output
cargo test -- --nocapture
```

### Testing Framework (`shallot_testing`)

The crate provides specialized testing modules:

```rust
// Accessibility testing
use shallot_testing::accessibility;
let contrast = accessibility::test_color_contrast("#000", "#fff", 4.5);

// Performance testing
use shallot_testing::performance;
let (result, duration) = performance::measure_render_time(|| {
    // render code
});

// Visual regression testing
use shallot_testing::visual;
let result = visual::test_layout_consistency(expected, actual);
```

### Test Organization

- **Unit tests**: Inside each crate's source files (`#[cfg(test)]`)
- **Integration tests**: In `tests/integration_tests.rs`
- **Benchmarks**: In `shallot_testing` (commented out, requires criterion)

---

## Component Architecture

### Foundation Layer (`shallot_foundation`)

```rust
// Design Tokens
use shallot_foundation::{
    DesignTokens, HSLColor, ColorScheme, ColorPalette,
    TypographyScale, SpacingScale, BorderRadiusScale, ShadowScale
};

// Responsive Design
use shallot_foundation::{
    Breakpoint, ResponsiveValue, ContainerConfig, GridConfig
};

// Animations
use shallot_foundation::{
    EasingFunction, KeyframeAnimation, Keyframe,
    fade_in, slide_in_up, bounce_in, pulse, glow, float
};

// Theme
use shallot_foundation::{ColorMode, Theme};
```

### Component Layer (`shallot_components`)

Components are organized by category:

- **Layout**: `Container`, `Grid`, `Stack`, `Divider`, `Section`
- **Forms**: `Form`, `Input`, `Checkbox`, `Radio`, `Select`, `Switch`, `Textarea`
- **Data Display**: `Avatar`, `Badge`, `Chip`, `Table`, `Typography`
- **Feedback**: `Dialog`, `Tooltip`, `Spinner`, `Skeleton`, `Alert`, `Toast`
- **Navigation**: `Navbar`, `Menu`, `Link`, `Breadcrumbs`, `Tabs`, `Steps`
- **Overlays**: `Drawer`, `Dropdown`, `Popover`, `Modal`
- **Effects**: `ShinyButton`, `ShimmerButton`, `GlowCard`, `Marquee`
- **Advanced**: `BentoGrid`, `Dock`, `Carousel`, `Calendar`, `Timeline`

### Usage Example

```rust
use shallot_components::{Button, ButtonVariant, ButtonSize};
use shallot_foundation::{Theme, ColorMode};
use maud::html;

let theme = Theme::default()
    .with_mode(ColorMode::Light)
    .with_accent_hue(250.0);

let button = Button::new("Click Me")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Md)
    .render();

let html = html! {
    (button)
};
```

---

## Security Considerations

### Current Security Posture

1. **Zero JavaScript** - Eliminates entire class of XSS vulnerabilities
2. **Memory Safety** - Rust's ownership model prevents memory corruption
3. **Type Safety** - Compile-time guarantees for all component APIs
4. **No Unsafe Code** - Project uses zero `unsafe` blocks

### Best Practices Applied

- Input validation through Rust's type system
- Proper escaping via Maud's HTML templating
- Semantic HTML for accessibility
- CSS custom properties prevent injection attacks

---

## Development Workflow

### Adding a New Component

1. **Create file**: `crates/shallot_components/src/my_component.rs`
2. **Add to lib.rs**: Add `pub mod my_component;` to `crates/shallot_components/src/lib.rs`
3. **Follow pattern**: Use existing components as templates (e.g., `button.rs`)
4. **Export if needed**: Re-export in `shallot` umbrella crate
5. **Test**: Add tests in `tests/integration_tests.rs`
6. **Document**: Update relevant README/documentation files

### Adding to Showcase

Edit `apps/showcase/src/main.rs`:
- Import the component
- Add a `Section` demonstrating the component
- Include code examples with `code_pair()` helper

### Build System Usage

```rust
use shallot_build::{BuildConfig, BuildSystem, BuildTarget};

let config = BuildConfig::production_default("my-app", "1.0.0")
    .with_targets(vec![
        BuildTarget::WebAssembly,
        BuildTarget::ServerSideRendering,
    ]);

let system = BuildSystem::new(config);
let results = system.build_all()?;
```

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace definition |
| `base_principles.md` | Core philosophical principles |
| `TODO_IMPLEMENTATION.md` | Implementation roadmap |
| `IMPLEMENTATION_REPORT.md` | Comprehensive audit report |
| `crates/shallot_components/src/lib.rs` | Component exports |
| `crates/shallot_foundation/src/lib.rs` | Foundation exports |
| `apps/showcase/src/main.rs` | Main demo application |
| `tests/integration_tests.rs` | Integration tests |

---

## Notes for AI Agents

1. **No JavaScript** - Never introduce JS/TS dependencies; use CSS for animations/interactions
2. **Type Safety** - Leverage Rust's type system; avoid stringly-typed APIs
3. **Builder Pattern** - Use builder pattern for component configuration
4. **Semantic HTML** - Prioritize accessibility with proper ARIA attributes
5. **CSS Variables** - Use design tokens, never hardcode values
6. **Test Coverage** - Add tests for new components
7. **Documentation** - Document public APIs with examples
8. **English Only** - All code and docs in English

---

*Built with ❤️ and 🦀 in the spirit of the "Post-JS" movement.*
