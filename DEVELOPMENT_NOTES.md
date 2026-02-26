# Component Development Notes

## Compilation Issues - RESOLVED

### 1. Maud Template Syntax Errors (FIXED)

**Problem:** Many components had Maud HTML template syntax errors that prevented compilation.

**Root Causes:**
- Using complex expressions in class attributes (blocks)
- Using `value=()` for optional values instead of `value=[]`
- Using `checked=` and `disabled=` without `?` suffix
- Pattern interpolation syntax errors (`&(format!(...))`)
- Tuple interpolation like `(name, "_suffix")` doesn't work

**Solution Applied:** All 13 previously failed components have been rewritten with correct syntax:

| Component | Status | Key Fix |
|-----------|--------|---------|
| `table.rs` | FIXED | Pre-compute classes in helper methods |
| `accordion.rs` | FIXED | Use `@let` for computed values |
| `dialog.rs` | FIXED | Use `details/summary` for CSS-only modals |
| `tooltip.rs` | FIXED | CSS-only via hover states |
| `menu.rs` | FIXED | Extract render_dropdown_item to impl block |
| `pagination.rs` | FIXED | Pre-compute page ranges |
| `avatar_group.rs` | FIXED | Pre-compute style strings |
| `toggle_group.rs` | FIXED | Use `checked?[bool]` syntax |
| `search_input.rs` | FIXED | `placeholder=(value)` not `placeholder=[value]` |
| `tag_input.rs` | FIXED | Use parentheses for required strings |
| `color_picker.rs` | FIXED | Use brackets for optional attributes |
| `counter.rs` | FIXED | Pre-compute formatted values |

### 2. Clone Trait Issues (FIXED)

**Problem:** Structs used in nested components need `Clone` derived.

**Solution:** Added `#[derive(Clone)]` to:
- `SearchInput<'a>` - used in `SearchWithResults`
- `ListItem<'a>` - used in `ListGroup`

### 3. Lifetime Parameter Issues (FIXED)

**Problem:** `TableFooter<'a>` had unused lifetime parameter.

**Solution:** Removed unused lifetime since `Markup` doesn't need it:
```rust
// Before
pub struct TableFooter<'a> { ... }

// After
pub struct TableFooter { ... }
```

### 4. Name Conflicts (FIXED)

**Problem:** Duplicate exports with same names in `lib.rs`.

**Solution:** Use `as` for renaming:
```rust
pub use avatar_group::{AvatarGroup as StackedAvatarGroup, ...};
```

---

## Maud Attribute Syntax Reference (CORRECTED)

### Working Patterns

```rust
// Required string attributes - use parentheses
name=(self.name)
id=(format!("item-{}", self.id))

// Optional attributes - use brackets
value=[self.value]
placeholder=[self.placeholder]

// Boolean attributes - use question mark
disabled?[self.disabled]
checked?[self.checked]
required?[self.required]

// String values (not optional)
placeholder=(self.placeholder.unwrap_or("Default"))

// Pre-computed classes
impl MyComponent {
    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-component"];
        if self.active { classes.push("sh-component--active"); }
        classes.join(" ")
    }
}

html! {
    div class=(self.build_classes()) { ... }
}

// @let for inline computation
html! {
    @let item_id = format!("item-{}", i);
    div id=(item_id) { ... }
}
```

### Non-Working Patterns (AVOID)

```rust
// WRONG: Block expression in attribute
div class={
    let mut cls = vec!["a"];
    cls.join(" ")
}

// CORRECT: Pre-compute or use @let
@let classes = { let mut cls = vec!["a"]; cls.join(" ") };
div class=(classes)

// WRONG: Optional value with parentheses
input value=(self.value)  // If value is Option<&str>

// CORRECT: Use brackets for optional
input value=[self.value]

// WRONG: Boolean without ?
input disabled=(self.disabled)

// CORRECT: Use ?
input disabled?[self.disabled]

// WRONG: Tuple interpolation
name={(self.name, "_suffix")}

// CORRECT: Use format!
name=(format!("{}{}", self.name, "_suffix"))
```

---

## Component Architecture Patterns

### Pattern 1: Simple Component

```rust
pub struct Label<'a> {
    text: &'a str,
}

impl<'a> Render for Label<'a> {
    fn render(&self) -> Markup {
        html! { span class="sh-label" { (self.text) } }
    }
}
```

### Pattern 2: Component with Variants

```rust
pub struct Button<'a> {
    label: &'a str,
    variant: ButtonVariant,
    size: ComponentSize,
}

impl<'a> Button<'a> {
    fn build_classes(&self) -> String {
        format!("sh-btn sh-btn--{} sh-btn--{}", 
            self.variant.class_suffix(),
            self.size.class_suffix())
    }
}

impl<'a> Render for Button<'a> {
    fn render(&self) -> Markup {
        html! {
            button class=(self.build_classes()) {
                (self.label)
            }
        }
    }
}
```

### Pattern 3: Component with Children

```rust
pub struct Card {
    children: Markup,
    title: Option<String>,
}

impl Render for Card {
    fn render(&self) -> Markup {
        html! {
            div class="sh-card" {
                @if let Some(title) = &self.title {
                    h3 class="sh-card__title" { (title) }
                }
                div class="sh-card__body" {
                    (self.children.clone())
                }
            }
        }
    }
}
```

### Pattern 4: CSS-Only Interactive Component

Use `details/summary` for CSS-only interactivity:

```rust
impl<'a> Render for Accordion<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-accordion" {
                @for item in &self.items {
                    details class="sh-accordion-item" open?[item.open] {
                        summary class="sh-accordion-header" {
                            (item.title)
                        }
                        div class="sh-accordion-content" {
                            (item.content.clone())
                        }
                    }
                }
            }
        }
    }
}
```

---

## CSS Generation Pattern

Each component should have a `*_css()` function:

```rust
pub fn component_css() -> String {
    r#"
.sh-component {
    /* Base styles using CSS custom properties */
    display: flex;
    padding: var(--sh-spacing-4, 1rem);
    background: var(--sh-surface, #fff);
    border-radius: var(--sh-radius-md, 0.5rem);
}

/* Variants */
.sh-component--primary {
    background: var(--sh-accent, #3b82f6);
    color: #fff;
}

/* States */
.sh-component--disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

/* Sizes */
.sh-component--sm { padding: var(--sh-spacing-2, 0.5rem); }
.sh-component--lg { padding: var(--sh-spacing-6, 1.5rem); }

/* Responsive */
@media (max-width: 640px) {
    .sh-component { flex-direction: column; }
}
"#.to_string()
}
```

---

## Pre-existing Issues (Low Priority)

1. **Unused imports** - 24 warnings, can auto-fix with `cargo fix --lib -p shallot_components`
2. **Test code errors** in `typography.rs` and `code_block.rs` - test functions reference non-existent methods
3. **Function pointer comparison** warning in `form_validation.rs` - minor

---

## Recommendations for Development

1. **Test incrementally** - Run `cargo check` after each component
2. **Use simpler templates** - Avoid complex Rust inside `html!` macro
3. **Extract logic first** - Compute classes/values in methods before templates
4. **Start minimal** - Create basic versions first, enhance later
5. **Use brackets for optional** - `value=[self.value]` for `Option<T>`
6. **Use ? for booleans** - `disabled?[self.disabled]`
7. **Derive Clone when needed** - For nested components
8. **Follow naming convention** - `sh-component`, `sh-component--variant`, `sh-component__element`

---

## Session History

### Session 1
- Initial analysis of project
- Added `file_upload.rs`, `list.rs`
- Removed 14+ broken components
- Documented issues

### Session 2 (February 2026)
- Fixed workspace Cargo.toml
- Re-added all 13 failed components with correct syntax
- Total: 75 component files
- Build: PASSING
- Updated documentation

**Last Updated:** February 2026
