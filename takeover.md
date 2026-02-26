# Shallot.rs - Takeover Documentation

## Project Overview

**Shallot.rs** is a zero-JavaScript Rust UI component library demonstrating that beautiful, accessible web interfaces can be built entirely without client-side JavaScript. The project uses Rust with Maud templates for HTML generation.

### Core Goal
Build 150+ production-ready UI components with:
- Zero JavaScript (must work in TOR browser)
- Beautiful, animated aesthetics
- Full theming via CSS custom properties
- Accessibility built-in (ARIA attributes)
- Type-safe Rust APIs with builder pattern

---

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Language | Rust (Edition 2021) | Core implementation |
| Templating | Maud 0.26 | Type-safe HTML generation |
| Web Framework | Axum 0.7 | Server-side routing |
| Styling | CSS Custom Properties | Theming and design tokens |

### Dependencies
- `maud` - HTML templating
- `axum` - Web framework
- `shallot_foundation` - Design tokens, theming, animations
- `serde` - Serialization
- `thiserror` - Error handling

---

## Core Principles

1. **Zero JS/TS** - No JavaScript, ever. All interactivity via CSS & rust.
2. **English Codebase** - All code and docs in English
3. **Full-Stack Rust** - Both frontend and backend in Rust
4. **Beauty + Customizability** - Animations/visuals with customization
5. **Best Practices** - Builder pattern, type safety, accessibility

---

## Project Structure

```
shallot/
├── Cargo.toml                    # Workspace definition
├── Cargo.lock                    # Dependency lock
├── AGENTS.md                     # Agent/developer documentation
├── takeover.md                   # Project handoff notes (THIS FILE)
├── DEVELOPMENT_NOTES.md          # Technical difficulties
│
├── crates/
│   ├── shallot/                  # Umbrella crate (re-exports)
│   │
│   ├── shallot_components/       # UI components (75 modules)
│   │   └── src/
│   │       ├── lib.rs            # Module declarations & exports
│   │       ├── component.rs      # Base traits and enums
│   │       ├── button.rs         # Button component
│   │       ├── input.rs          # Form inputs
│   │       ├── card.rs           # Card component
│   │       ├── table.rs          # Data tables (FIXED)
│   │       ├── accordion.rs      # Collapsible panels (FIXED)
│   │       ├── dialog.rs         # Modal dialogs (FIXED)
│   │       ├── tooltip.rs        # Tooltips (FIXED)
│   │       ├── menu.rs           # Navigation menus (FIXED)
│   │       ├── pagination.rs     # Page navigation (FIXED)
│   │       ├── avatar_group.rs   # Grouped avatars (FIXED)
│   │       ├── toggle_group.rs   # Toggle buttons (FIXED)
│   │       ├── search_input.rs   # Search input (FIXED)
│   │       ├── tag_input.rs      # Tag management (FIXED)
│   │       ├── color_picker.rs   # Color selection (FIXED)
│   │       ├── counter.rs        # Number counters (FIXED)
│   │       ├── list.rs           # List components
│   │       ├── file_upload.rs    # File upload
│   │       └── [60+ other modules]
│   │
│   ├── shallot_foundation/       # Design tokens, theme system
│   ├── shallot_build/            # Build system
│   └── shallot_testing/          # Testing framework
│
├── tests/
│   └── integration_tests.rs
│
├── components_individual/         # Alternative component structure
├── open_ressources/              # Fonts, icons
└── code_drafts/                  # Draft experiments
```

---

## Progress

### Current Status (February 2026)

| Metric | Value |
|--------|-------|
| **Component Files** | 75 |
| **Build Status** | PASSING (24 warnings, 0 errors) |
| **Target** | 150+ components |

### Components by Category

| Category | Components |
|----------|------------|
| **Layout** | Container, Grid, Stack, Divider, Bento, Section, Box, Layout, Spacer |
| **Forms** | Input, Textarea, Checkbox, Radio, Switch, Select, Slider, FileUpload, TagInput, ColorPicker, SearchInput, ToggleGroup |
| **Navigation** | Navbar, Sidebar, Tabs, Breadcrumbs, Dock, Pagination, Menu |
| **Data Display** | Card, Avatar, AvatarGroup, Badge, Table, List, Timeline, Calendar, Counter, Stats |
| **Feedback** | Alert, Toast, Progress, Rating, Dialog, Tooltip, Modal, Skeleton |
| **Effects** | Marquee, Confetti, BorderBeam, MagicCard, GlassCard, AnimatedBeam, OrbitingCircles, TextReveal, TypingAnimation |
| **Overlays** | Drawer, Dropdown, Popover, Dialog |
| **Typography** | Heading, Text, Code, Quote, List (Typography) |
| **Advanced** | Bento Grid, Carousel, Charts, Timeline, Dock |

### Components Added/Fixed This Session (13)

| Component | Description | Status |
|-----------|-------------|--------|
| `table.rs` | Data tables with sorting, selection, responsive design | FIXED |
| `accordion.rs` | Collapsible content panels with variants | FIXED |
| `dialog.rs` | Modal dialogs using CSS-only (details/summary) | FIXED |
| `tooltip.rs` | CSS-only tooltips with multiple positions/variants | FIXED |
| `menu.rs` | Navigation menus and dropdown menus | FIXED |
| `pagination.rs` | Page navigation with multiple variants | FIXED |
| `avatar_group.rs` | Stacked/grouped avatars with overflow | FIXED |
| `toggle_group.rs` | Group of toggleable buttons (single/multiple) | FIXED |
| `search_input.rs` | Search input with results dropdown | FIXED |
| `tag_input.rs` | Tag/chip input management | FIXED |
| `color_picker.rs` | Color selection with swatches | FIXED |
| `counter.rs` | Animated number counters and statistics | FIXED |
| `list.rs` | List, ListGroup, DefinitionList | Existing |

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `shallot_components/src/lib.rs` | Module exports (~150 exports) |
| `shallot_foundation/src/lib.rs` | Foundation exports |
| `shallot_foundation/src/theme.rs` | Theme system with 6 themes |
| `shallot_foundation/src/design_tokens.rs` | HSL colors, design tokens |
| `shallot_components/src/component.rs` | Base traits (Component, AriaAttrs, etc.) |
| `shallot_components/src/input.rs` | Form input components |
| `AGENTS.md` | Development guidelines |
| `DEVELOPMENT_NOTES.md` | Technical difficulties |

---

## Building & Testing

```bash
# Build entire workspace
cargo build

# Check components compile
cargo check --package shallot_components

# Run tests (note: some test code in existing files has issues)
cargo test --package shallot_components --lib

# Build release
cargo build --release
```

---

## Maud Template Syntax Guide

### Working Patterns

```rust
// Simple attributes - direct values
input type="text" name="field" id="my-id"

// String interpolation with format!
div class=(format!("sh-btn {} {}", variant, size))

// Optional values using brackets
input value=[self.value] placeholder=[self.placeholder]

// Boolean attributes with question mark
input disabled?[self.disabled] checked?[self.checked]

// Computed values outside html! macro
impl<'a> MyComponent<'a> {
    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-component"];
        if self.active { classes.push("sh-component--active"); }
        classes.join(" ")
    }
}

impl<'a> Render for MyComponent<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();  // Compute outside
        html! {
            div class=(classes) { "Content" }
        }
    }
}

// @let for computed values inside html!
html! {
    @let item_id = format!("item-{}", self.id);
    div id=(item_id) { "Content" }
}

// @for loops
@for item in &self.items {
    li { (item.name) }
}

// @if conditionals
@if self.show {
    div { "Visible" }
}

// @if let
@if let Some(value) = &self.value {
    span { (value) }
}

// @match
@match self.variant {
    Variant::A => "Type A",
    Variant::B => "Type B",
}
```

### Non-Working Patterns (AVOID)

```rust
// WRONG: Complex expressions in class attribute
div class={
    let mut cls = vec!["a"];
    cls.push("b");
    cls.join(" ")
}

// CORRECT: Pre-compute before html!
@let classes = { let mut cls = vec!["a"]; cls.push("b"); cls.join(" ") };
div class=(classes)

// WRONG: Direct boolean values
input value=(self.value)        // Not for optional strings

// CORRECT: Use brackets for optional
input value=[self.value]

// WRONG: Tuple interpolation
name={(self.name, "_suffix")}

// CORRECT: Use format!
name=(format!("{}{}", self.name, "_suffix"))
```

---

## Component Development Pattern

Use this template for new components:

```rust
//! ComponentName - Brief description
//!
//! Extended description of the component.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct ComponentName<'a> {
    field: &'a str,
    size: ComponentSize,
    variant: ComponentVariant,
    disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentVariant {
    #[default]
    Default,
    Primary,
    Secondary,
}

impl<'a> ComponentName<'a> {
    pub fn new(field: &'a str) -> Self {
        Self {
            field,
            size: ComponentSize::Md,
            variant: ComponentVariant::Default,
            disabled: false,
        }
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ComponentVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-component".to_string()];
        classes.push(format!("sh-component--{}", self.size.class_suffix()));
        classes.push(format!("sh-component--{}", self.variant.class_suffix()));
        if self.disabled {
            classes.push("sh-component--disabled".to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for ComponentName<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        
        html! {
            div class=(classes) {
                (self.field)
            }
        }
    }
}

pub fn component_name_css() -> String {
    r#"
.sh-component {
    /* Base styles */
}

.sh-component--md { /* Size variant */ }
.sh-component--primary { /* Variant */ }
.sh-component--disabled { /* State */ }
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let comp = ComponentName::new("test")
            .variant(ComponentVariant::Primary);
        
        assert_eq!(comp.field, "test");
    }
}
```

---

## Remaining Work

### To Reach 150+ Components (Need ~75 more)

**High Priority - New Components:**
- Date Picker / Date Range Picker
- Time Picker
- OTP Input
- Range Slider
- File Drop Zone (enhanced)
- Rich Text Editor (CSS-only display)

**Medium Priority - Data Display:**
- Data Grid
- Tree View
- Description List (exists in list.rs, could expand)
- Key Value List
- Property Table

**Medium Priority - Feedback:**
- Loading States (Skeleton variants)
- Empty States
- Error Boundaries
- Notification Stack

**Lower Priority - Effects:**
- Gradient Text
- Blur Background
- Spotlight Effect
- Text Gradient Animation

### Known Issues

1. **Test compilation errors** in `typography.rs` and `code_block.rs` - test code references methods that don't exist
2. **Unused import warnings** - 24 warnings that can be auto-fixed with `cargo fix`
3. **Missing showcase app** - needs to be recreated in `apps/showcase/`

---

## Recommendations for Future Development

1. **Test incrementally** - Run `cargo check` after each new component
2. **Use simpler templates** - Avoid complex Rust inside html! macros
3. **Extract logic first** - Compute classes/values in methods before templates
4. **Start minimal** - Create basic versions first, enhance later
5. **Reference working components** - Look at `table.rs`, `dialog.rs`, `menu.rs` for patterns
6. **Add Clone derive** - When nesting components, structs often need `#[derive(Clone)]`
7. **Use brackets for optional** - `value=[self.value]` not `value=(self.value)`

---

## Session History

### Session 1 (Previous)
- Initial project analysis
- Added `file_upload.rs`, `list.rs`
- Removed 14+ broken components to get build passing
- Documented Maud syntax issues

### Session 2 (Current - February 2026)
- Fixed workspace Cargo.toml (removed non-existent apps/)
- Re-added all 13 previously failed components with correct Maud syntax
- Total component files: 75
- Build: PASSING
- Updated `lib.rs` with all exports and CSS generation
- Updated takeover.md documentation

---

## Contact / Context

This project follows strict zero-JavaScript principles. All interactivity must be CSS-only. Animations use CSS keyframes. Accessibility requires proper ARIA attributes on all interactive elements.

**Last Updated:** February 2026
