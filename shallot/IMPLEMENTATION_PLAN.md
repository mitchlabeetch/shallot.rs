# Shallot.rs — SOTA Implementation Plan
## "Iron & Glass: The Post-JS Revolution"

> *From prototype to world-class: a phased roadmap for transforming Shallot.rs into the definitive zero-JS Rust UI library.*

---

## Executive Summary

Shallot.rs proves that the modern web's JavaScript addiction is a choice, not a requirement. This plan transforms the current prototype into a production-grade component library and showcase website that will reshape how engineers think about web UI. Every line of code must be runnable in a NoScript TOR browser. Every animation must be CSS-only. Every interaction must be driven by the browser's native state engine — no runtime, no bundle, no bridge.

**Current State**: Working component library (~120 components), broken showcase website with placeholder previews, stripped hero, dysfunctional code panels, fake community section, no live theming.

**Target State**: World-class showcase demonstrating every component live, a zero-JS color theming system, IDE-quality code panels, scroll-driven animations, and a hero section that makes engineers simultaneously laugh and believe.

---

## Phase 0: Architectural Foundations (Days 1–2)

### 0.1 — CSS Variable Color Theming System

**The Problem**: The current theme panel uses a floating panel that doesn't actually change component colors site-wide.

**The Solution**: Seven hidden `<input type="radio">` elements placed as the *first children of `<body>`*, before all other content. Their `id`s are targeted by CSS sibling selectors to update CSS custom properties on the `#sh-app` wrapper.

```
<body>
  <input type="radio" id="sh-tc-violet" name="sh-theme-color" checked>  ← FIRST
  <input type="radio" id="sh-tc-azure"  name="sh-theme-color">
  ... (5 more)
  <div id="sh-app">
    <!-- Labels (color swatches) live here in the navbar -->
    <!-- All components live here -->
  </div>
</body>
```

**CSS Pattern**:
```css
#sh-tc-violet:checked ~ #sh-app { --sh-primary: #8b5cf6; --sh-accent: #8b5cf6; }
#sh-tc-azure:checked  ~ #sh-app { --sh-primary: #3b82f6; --sh-accent: #3b82f6; }
```

HTML `<label for="sh-tc-violet">` elements work across the full document regardless of DOM position — the labels live in the navbar but control inputs at the top of body. This is 100% spec-compliant and works in all browsers including TOR/NoScript.

**7 Curated Colors**:
| ID | Name | Hex | Personality |
|----|------|-----|-------------|
| `sh-tc-violet` | Violet | `#8b5cf6` | Elegant, signature |
| `sh-tc-azure` | Azure | `#3b82f6` | Trustworthy, classic |
| `sh-tc-emerald` | Emerald | `#10b981` | Growth, vitality |
| `sh-tc-rose` | Rose | `#f43f5e` | Bold, passionate |
| `sh-tc-amber` | Amber | `#f59e0b` | Warm, energetic |
| `sh-tc-cyan` | Cyan | `#06b6d4` | Futuristic, cool |
| `sh-tc-indigo` | Indigo | `#6366f1` | Deep, minimal |

**Custom Color Picker**: A native `<input type="color">` is included as a UI affordance that shows the selected preset color. Without JS it cannot apply arbitrary colors dynamically — this is clearly communicated to users. The 7 presets provide the zero-JS theming. The custom picker is a future enhancement gated on the Axum SSR mode (POST form submission to server-set CSS variable).

### 0.2 — CSS-Only Code Panel Architecture

**The Problem**: The current code tabs use nested radio inputs that break the CSS sibling selector chain. The `pre` code blocks are never shown because `input:checked ~ .code-block` cannot reach across containing wrappers.

**The Fix**: Strict DOM ordering within each `.sh-cp` (code panel):

```html
<div class="sh-cp">
  <input type="checkbox" id="cp-{name}" class="sh-cp-toggle">   ← toggle visibility
  <label for="cp-{name}" class="sh-cp-label">< > Code ▼</label>
  <div class="sh-cp-body">                                        ← shown when toggle:checked
    <input type="radio" name="t-{name}" class="sh-tab-r sh-tab-r-full" checked>  ← BEFORE .sh-tab-bar
    <input type="radio" name="t-{name}" class="sh-tab-r sh-tab-r-lib">           ← BEFORE .sh-tab-bar
    <div class="sh-tab-bar">
      <label class="sh-tab-btn">Full Code</label>                 ← nth-child(1)
      <label class="sh-tab-btn">Lib Import</label>                ← nth-child(2)
    </div>
    <div class="sh-cb sh-cb-full">   ← shown by: .sh-tab-r-full:checked ~ .sh-tab-bar ~ .sh-cb-full
      <!-- RUST section, CSS section -->
    </div>
    <div class="sh-cb sh-cb-lib">   ← shown by: .sh-tab-r-lib:checked ~ .sh-tab-bar ~ .sh-cb-lib
      <!-- Cargo.toml section, Rust import section -->
    </div>
  </div>
</div>
```

**Key Insight**: The radio inputs MUST be direct children of `.sh-cp-body` and appear *before* `.sh-tab-bar` and the `.sh-cb` blocks. The `~` (general sibling) selector skips any number of intermediate siblings.

**Active Tab Highlighting** (no JS):
```css
.sh-tab-r-full:checked ~ .sh-tab-bar .sh-tab-btn:nth-child(1) { 
  color: var(--sh-primary); border-bottom: 2px solid var(--sh-primary); 
}
.sh-tab-r-lib:checked  ~ .sh-tab-bar .sh-tab-btn:nth-child(2) { 
  color: var(--sh-primary); border-bottom: 2px solid var(--sh-primary); 
}
```

**Code Section Types**: Each "Full Code" block contains labeled sections separated by a visual divider:
- `RUST` — The component struct, impl, and render method
- `CSS` — The component's generated CSS (abbreviated for readability)

Each "Lib Import" block contains:
- `CARGO.TOML` — The dependency line
- `RUST` — The use statement and minimal usage example

### 0.3 — CSS-Only Scroll Animations

**Scroll Entry** (using CSS Scroll-Driven Animations, Chrome 115+, Firefox 110+):
```css
.sh-demo-row {
  animation: sh-row-appear linear both;
  animation-timeline: view();
  animation-range: entry 0% entry 30%;
}
@keyframes sh-row-appear {
  from { opacity: 0; transform: translateY(24px); }
  to   { opacity: 1; transform: translateY(0); }
}
/* Graceful degradation for unsupported browsers */
@supports not (animation-timeline: view()) {
  .sh-demo-row { opacity: 1; transform: none; }
}
```

**CSS-Only Animation Replay** (the 2-keyframe identity trick):

The browser restarts a CSS animation when the `animation-name` property changes, even if the new keyframes are identical. We exploit this:

```css
/* Two identical keyframe sets with different names */
@keyframes sh-btn-a { from { opacity:0; transform:scale(0.9); } to { opacity:1; transform:scale(1); } }
@keyframes sh-btn-b { from { opacity:0; transform:scale(0.9); } to { opacity:1; transform:scale(1); } }

/* Hidden checkbox per component demo */
.sh-replay-r { display: none; }

/* Default: animate with 'a' */
.sh-demo-zone-button .sh-anim { animation: sh-btn-a 0.5s cubic-bezier(0.16,1,0.3,1) both; }

/* When checked: animate with 'b' — same visually, but new name = browser restarts */
.sh-replay-r-button:checked ~ .sh-demo-zone-button .sh-anim { 
  animation: sh-btn-b 0.5s cubic-bezier(0.16,1,0.3,1) both; 
}
```

Each click on the "↺ Replay" label toggles checked/unchecked, alternating between `a` and `b`, restarting the animation every time. The user experiences a seamless replay on every click.

### 0.4 — Performance Architecture

Following the "Zero-JS Challenge" mandate:

- `content-visibility: auto` on all `.sh-demo-row` elements (skip off-screen rendering)
- `contain: layout style` on component preview zones (isolate reflow)
- `will-change: transform, opacity` only on actively animated elements
- `@media (prefers-reduced-motion: reduce)` wrapper on ALL animations, replacing motion with `opacity` crossfades
- `clamp()` for ALL font sizes — no media query breakpoints for typography
- `aspect-ratio` on all media containers — prevent CLS

---

## Phase 1: Showcase Website — Hero Section (Day 2)

### 1.1 — The Retro Hero

The hero is a two-act play. Act 1: A faithful, lovingly crafted replica of the 1998 Geocities/Netscape era. Act 2: The punchline.

**Act 1 — The "90s Webpage" (CSS-only recreation of RetroHero.tsx)**:

Pure CSS/HTML/Rust adaptation. No framer-motion — scroll-driven animations replace it. Specific elements:

| Element | Implementation |
|---------|----------------|
| Page background | `background: #c0c0c0` (Windows 95 grey) |
| Scanlines overlay | CSS `repeating-linear-gradient` pseudo-element |
| CRT vignette | CSS `radial-gradient` overlay |
| "Under Construction" bar | CSS `repeating-linear-gradient(-45deg, #000, #ffcc00)` animated `background-position` |
| Marquee text | CSS `@keyframes` translateX animation on inline-block |
| HR dividers | Plain `<hr>` with 1px solid black |
| Title font | `font-family: "Comic Sans MS", cursive` |
| Table layout | Native `<table>` with `border-collapse: collapse` |
| Yellow announcement | `background: #ffff00` table cell |
| Badge shields | Inline HTML `<span>` elements styled to look like shields.io badges |
| Links row | Plain `<a>` tags with `color: #0000ff; text-decoration: underline` |
| Copyright footer | `font-family: "Times New Roman"`, standard paragraph |

**Act 2 — The Punchline**:

```
☝️  No JavaScript doesn't have to look like this.
                          👇
```

The punchline section appears below the retro block and uses `animation-timeline: view()` to fade/slide in as the user scrolls to it. The arrow emoji has a CSS `bounce` animation. Below it, the modern showcase begins immediately.

### 1.2 — The Modern Hero / Manifesto Strip

Between the retro punchline and the component categories, a brief but powerful "what is Shallot" strip:

- Logo (SVG inline, with glassmorphism card behind it)
- Headline: **"Iron Logic. Glass Aesthetics."**
- Subheadline: *"129 production-ready UI components. 0 bytes of JavaScript."*
- Three stats: `0ms TBT` · `0.00 CLS` · `0kb JS`
- Install snippet (styled code block): `shallot_components = "0.1"`
- Two CTAs: `Browse Components` (anchor to #showcase) · `GitHub ↗`

---

## Phase 2: Showcase Website — Navigation (Day 2)

### 2.1 — Sticky Navbar

Fixed to top, `backdrop-filter: blur(12px)`, glassmorphism style:

```
┌──────────────────────────────────────────────────────────────────────┐
│ [🧅 Logo] Shallot  v0.1    ● ● ● ● ● ● ●  [🎨 Custom]  [GitHub ↗] │
└──────────────────────────────────────────────────────────────────────┘
```

- **Logo**: Inline SVG from `shallot_logo_svg.svg`, 32px height
- **Version badge**: `v0.1` as a small pill
- **Color swatches**: Seven `<label>` elements (each wrapping a colored circle), each `for` one of the hidden body-level radio inputs. When selected, shows a ring border using `:has()` or the `checked` state of adjacent hidden input.
- **Custom color**: `<input type="color">` — decorative, shows current color
- **GitHub**: External link, `target="_blank" rel="noopener noreferrer"`

### 2.2 — Category Navigation

Sticky below navbar (or part of it on scroll), horizontal pill list:

```
📐 Layout   📝 Typography   📋 Forms   ✨ Effects   📊 Data   🔔 Feedback   🧭 Navigation
```

Each is a plain `<a href="#category-id">` — anchor navigation, no JS. Active state via `:target` on the section.

---

## Phase 3: Showcase Website — Component Demos (Days 3–5)

### 3.1 — Column Layout

**Delete the grid**. Each component gets a full-width "demo row":

```
┌─────────────────────────────────────────────────────────────────────┐
│ ComponentName     [tag: sh-button]          Brief description here  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│         [  Live Preview — spacious, centered, themed  ]             │
│                                                                     │
│                                              [↺ Replay]            │
├─────────────────────────────────────────────────────────────────────┤
│  < > Code  ▼                                                        │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ [Full Code]  [Lib Import]                                   │    │
│  │─────────────────────────────────────────────────────────────│    │
│  │ RUST                                                        │    │
│  │  pub struct Button<'a> { ... }                              │    │
│  │                                                             │    │
│  │ CSS                                                         │    │
│  │  .sh-btn { display: inline-flex; ... }                      │    │
│  └─────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
```

- Previews are **ample** (min-height: 200px for most, 300px+ for complex components)
- Components are shown **in context** with realistic data (not lorem ipsum)
- Dark preview background for glass/dark components, light for light components
- Replay button is always bottom-right of preview zone

### 3.2 — Component Priority Map

**Tier 1 — Full live preview + working code panels** (~40 components):

| Category | Components |
|----------|-----------|
| Signature Effects | BorderBeam, Confetti, LiquidButton, MeshGradient, Marquee, GlitchText, OrbitingCircles, TypingAnimation, TextReveal, AnimatedBeam |
| Buttons & Actions | Button (all variants+sizes+states), EnhancedButton, ShinyButton, ShimmerButton, MagneticButton |
| Typography | Heading (H1-H6), GradientText, CapDrop, AnimatedGradientText, NumberTicker, WordRotate |
| Forms | Input (all types), Select/GlassSelect, Checkbox, Radio, Switch, Slider, RangeSlider, Rating, RatingInput |
| Navigation | Navbar, Breadcrumbs, Tabs, Pagination, Sidebar (toggle demo) |
| Data Display | Card (all variants), Avatar+AvatarGroup, Badge (all variants), Table (sample data), Timeline, Stats, Progress, ProgressCircle, Skeleton |
| Feedback | Alert (all kinds), Toast, Tooltip, Spinner, Dialog (CSS :target) |
| Layout | BentoGrid, Grid, Stack, Masonry, ZStack |

**Tier 2 — Preview shown + code panel** (~50 components):
Components that render but need more complex setup: Calendar, Carousel, Drawer, Accordion, ColorPicker, TreeView, etc.

**Tier 3 — Listed with description** (~40 components):
Lower-priority or very complex components that are documented but shown with an intentional "API Reference" card.

### 3.3 — Code Panel Content

**Full Code tab** shows two sections:

```
RUST
────────────────────────────────────────────
use maud::{html, Markup};

pub struct Button<'a> {
    pub label: &'a str,
    pub variant: ButtonVariant,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self { ... }
    pub fn variant(mut self, v: ButtonVariant) -> Self { ... }
    pub fn render(self) -> Markup { ... }
}

CSS
────────────────────────────────────────────
.sh-btn {
    display: inline-flex;
    align-items: center;
    padding: 0.5rem 1rem;
    border-radius: var(--sh-radius-md);
    transition: all 0.2s ease;
}
.sh-btn--primary { background: var(--sh-primary); color: #fff; }
```

**Lib Import tab** shows:

```
CARGO.TOML
────────────────────────────────────────────
[dependencies]
shallot_components = "0.1"

RUST
────────────────────────────────────────────
use shallot_components::button::{Button, ButtonVariant, ButtonSize};

// In your Maud template:
html! {
    (Button::new("Launch")
        .variant(ButtonVariant::Primary)
        .size(ButtonSize::Lg)
        .render())
}
```

---

## Phase 4: Showcase Website — Polish (Day 5)

### 4.1 — Delete Community Themes Section Entirely

The `theme_marketplace` module and its render call in `showcase.rs` are removed. No replacement. The color picker in the navbar is the only theming affordance.

### 4.2 — Glassmorphism Design System

All panels follow "Iron & Glass" aesthetic:

```css
/* Layered ambient shadow — 5 layers */
.sh-glass-panel {
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(12px) saturate(180%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow:
    0 1px 2px rgba(0,0,0,0.04),
    0 4px 8px rgba(0,0,0,0.06),
    0 12px 24px rgba(0,0,0,0.08),
    0 24px 48px rgba(0,0,0,0.04),
    inset 0 0 0 1px rgba(255,255,255,0.08);
}
```

### 4.3 — Accessibility Audit

Every component demo must:
- Have `aria-label` on the preview region describing what is shown
- Never trap keyboard focus
- Support `prefers-reduced-motion` (all animations → opacity fade)
- Have sufficient color contrast (WCAG 2.1 AA minimum)
- Use semantic HTML (nav, main, section, article, aside, header, footer)

---

## Phase 5: Library Package (Days 6–8)

### 5.1 — Design Token Completion

`shallot_foundation` must export a complete typed token set:

```rust
pub struct DesignSystem {
    pub colors: ColorScale,      // 50-950 shades per hue
    pub spacing: SpacingScale,   // 0, 1, 2, 4, 8, 12, 16, 24, 32, 48, 64
    pub typography: TypeScale,   // xs, sm, base, lg, xl, 2xl, 3xl, 4xl, 5xl
    pub radii: RadiusScale,      // none, sm, md, lg, xl, full
    pub shadows: ShadowScale,    // sm, md, lg, xl, 2xl + inner
    pub animation: AnimScale,    // fast(150ms), med(300ms), slow(500ms) + easings
}
```

### 5.2 — Component API Standardization

Every public component must implement:
- `fn new(...)` — sensible defaults
- Builder pattern with `pub fn field(mut self, val: T) -> Self`
- `fn render(self) -> Markup` — consumes and renders
- `impl Render for X` — allows use in `html! { (component) }`
- `fn {component}_css() -> String` — component's CSS contribution

### 5.3 — Component CSS Audit

All component CSS must:
- Use only `var(--sh-*)` custom properties — no hardcoded colors
- Use `clamp()` for font sizes
- Use `transform` + `opacity` for animations (no layout-triggering properties)
- Include `@media (prefers-reduced-motion: reduce)` block
- Use `logical properties` (`padding-inline`, `margin-block`) for RTL support

### 5.4 — `all_component_css()` Optimization

The current `all_component_css()` function concatenates ~60 CSS strings. Optimize to:
1. Deduplicate shared animation keyframes (currently duplicated across many components)
2. Add CSS custom property foundation block at the top
3. Order: foundation tokens → base → layout → components → animations
4. Total target: < 80kb unminified, < 20kb gzipped

---

## Phase 6: Deployment (Day 8)

### 6.1 — Vercel Configuration

The project is a static site generator (SSG). `cargo run --bin shallot_website` produces the `output/` directory.

**`vercel.json`**:
```json
{
  "buildCommand": "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal && export PATH=\"$HOME/.cargo/bin:$PATH\" && cargo build --release && ./target/release/shallot_website",
  "outputDirectory": "output",
  "installCommand": "echo 'Rust toolchain installed in buildCommand'",
  "headers": [
    {
      "source": "/styles/(.*)",
      "headers": [{ "key": "Cache-Control", "value": "public, max-age=31536000, immutable" }]
    },
    {
      "source": "/(.*).html",
      "headers": [{ "key": "Cache-Control", "value": "public, max-age=0, must-revalidate" }]
    }
  ]
}
```

### 6.2 — Build Artifact Structure

```
output/
├── index.html          ← Full showcase site
├── feed.xml            ← RSS feed
└── styles/
    ├── main.css        ← Global + layout styles
    ├── retro.css       ← Retro hero styles
    ├── showcase.css    ← Showcase layout + code panels
    └── components.css  ← All component CSS (from all_component_css())
```

### 6.3 — Performance Targets

| Metric | Target | How |
|--------|--------|-----|
| TTFB | < 100ms | Static file, CDN edge |
| LCP | < 1.2s | No render-blocking JS |
| TBT | 0ms | Zero JavaScript |
| CLS | 0.00 | `aspect-ratio` on all media |
| FID | 0ms | No JS event handlers |
| Lighthouse Score | 100/100/100/100 | Semantic HTML, no JS |

---

## Phase 7: Documentation & Developer Experience (Days 9–10)

### 7.1 — Component Documentation Standard

Each component module must have a doc comment following this template:

```rust
//! # ComponentName
//!
//! Brief description of what this component does.
//!
//! ## Usage
//!
//! ```rust
//! use shallot_components::component_name::{ComponentName, Variant};
//!
//! let markup = ComponentName::new("content")
//!     .variant(Variant::Primary)
//!     .render();
//! ```
//!
//! ## Zero-JS Interaction
//!
//! Describe how this component achieves interactivity without JavaScript
//! (e.g., ":checked hack", ":target", ":focus-within", CSS animations).
//!
//! ## Accessibility
//!
//! Note ARIA attributes, keyboard support, and screen reader behavior.
//!
//! ## CSS Custom Properties
//!
//! List all `--sh-*` variables the component respects.
```

### 7.2 — README Overhaul

The root `README.md` must:
- Lead with the Shallot logo
- State the zero-JS mandate in the first sentence
- Show the install command
- Show a 3-line usage example
- Link to the live showcase
- Explain the "Iron & Glass" duality
- Show performance metrics
- List all 129 components by category

### 7.3 — `cargo doc` Integration

Run `cargo doc --no-deps --open` and verify:
- All public types have doc comments
- Code examples in docs compile
- No broken intra-doc links

---

## Phase 8: Component Completion (Days 10–14)

### 8.1 — Priority Missing Implementations

Components currently returning placeholder or minimal HTML that need full implementations:

| Component | Current State | Target |
|-----------|--------------|--------|
| `calendar.rs` | Basic grid | Full month view, prev/next via :target |
| `carousel.rs` | Static list | CSS :checked sliding carousel |
| `accordion.rs` | Exists | Verify :checked expand/collapse |
| `dialog.rs` | Exists | CSS :target open/close |
| `drawer.rs` | Exists | CSS :checked slide in/out |
| `command_palette.rs` | Stub | :focus-within activated, keyboard nav |
| `tree_view.rs` | Stub | Nested :checked expand nodes |
| `date_picker.rs` | Stub | :target calendar popup |
| `otp_input.rs` | Stub | :focus-within chain highlighting |

### 8.2 — New Signature Components

Components to add that will make Shallot truly world-class:

| Component | CSS Technique | Visual Impact |
|-----------|--------------|---------------|
| `spotlight_card.rs` | CSS `radial-gradient` from mouse via `:hover` grid | High |
| `tilt_card.rs` | CSS `perspective` + `:hover` rotation | High |
| `particle_field.rs` | CSS `@keyframes` with `nth-child` timing offsets | High |
| `morphing_blob.rs` | CSS `border-radius` animation | Medium |
| `glow_cursor.rs` | CSS `radial-gradient` `:hover` detection grid | High |

---

## Phase 9: Testing & Quality (Ongoing)

### 9.1 — Test Coverage Targets

- Unit tests: 100% of public API surface
- Snapshot tests: HTML output for all 129 components
- CSS validation: All generated CSS parses without errors
- Accessibility: Automated axe-core equivalent checks via `shallot_testing`

### 9.2 — Browser Compatibility

Test matrix (all must work with NoScript enabled):
- Firefox (TOR browser engine) — primary target
- Chrome/Chromium — scroll animations primary support
- Safari — backdrop-filter verification
- Firefox ESR — `animation-timeline: view()` fallback verification

### 9.3 — Regression Prevention

```rust
// In tests/integration_tests.rs — snapshot test example
#[test]
fn test_button_html_output_stable() {
    let rendered = Button::new("Click").variant(ButtonVariant::Primary).render();
    assert_snapshot!("button_primary", rendered.0);
}
```

---

## Implementation Order (Critical Path)

```
Week 1:
  Day 1:  Phase 0 (Architecture) → CSS color system, code panel DOM structure
  Day 2:  Phase 1 + 2 (Hero + Nav) → Retro hero, modern intro, navbar
  Day 3:  Phase 3 part 1 → Signature Effects demos (Tier 1 components)
  Day 4:  Phase 3 part 2 → Buttons, Typography, Forms demos
  Day 5:  Phase 3 part 3 → Navigation, Data Display, Feedback demos
  Day 5:  Phase 4 (Polish) → Delete community themes, glassmorphism, a11y audit
  Day 6:  Phase 5 (Library) → Token audit, API standardization, CSS audit
  Day 7:  Phase 6 (Deploy) → Vercel config, performance verification
  Day 7:  Phase 7 (Docs) → README, cargo doc, per-component docs

Week 2:
  Days 8–10: Phase 8 (Component Completion) → Priority missing implementations
  Days 11–14: Phase 9 (Testing) → Full test coverage, browser testing
```

---

## Guiding Principles (Non-Negotiable)

1. **Absolute Zero JavaScript**: Not a single `<script>` tag, inline or external. Every feature must work in a NoScript TOR browser at maximum security level.

2. **English Codebase**: All code, comments, and documentation in English only.

3. **Compile-Time Safety**: If the UI compiles, it is structurally valid. Use Rust's type system to make invalid component states unrepresentable.

4. **The Two-Lens View**: Every component must look like a clean, logical HTML outline to screen readers AND a refractive glass masterpiece to sighted users.

5. **Bionic Naturalism**: `cubic-bezier(0.16, 1, 0.3, 1)` for ALL transitions. No linear motion. Nature is never linear.

6. **Composite-Only Animations**: Animate ONLY `transform` and `opacity`. Never `width`, `height`, `margin`, or `color` (use CSS variable transitions instead).

7. **`clamp()` Typography**: Never a static `px` font-size. Always `clamp(min, fluid, max)`.

8. **Layered Shadows**: At least 3 shadow layers on elevated elements. Single shadows are flat. Flat is dead.

---

*Built with ❤️ and 🦀 — proving that the "Post-JS" movement is not a limitation. It is liberation.*