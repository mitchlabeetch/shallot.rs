//! Shallot Website — Zero-JS Component Showcase
//!
//! COLOR THEMING (zero JS):
//!   Seven `<input type="radio">` elements are the *first children of <body>*,
//!   before `<div id="sh-app">`. CSS general-sibling selectors then override
//!   `--sh-primary` / `--sh-accent` on #sh-app for every checked state.
//!   The visible swatch labels live inside the navbar (inside #sh-app) —
//!   HTML `<label for="…">` works across the entire document regardless of
//!   DOM position, so this is fully spec-compliant and requires zero JavaScript.
//!
//! NO COMMUNITY THEMES:
//!   The theme_marketplace module is no longer rendered anywhere.
//!   The theme_panel floating widget has been removed; theming lives in navbar.

pub mod component_docs;
pub mod retro_hero;
pub mod rss;
pub mod showcase;
pub mod theme_marketplace; // kept for compilation, not rendered
pub mod theme_panel; // kept for compilation, not rendered
pub mod theme_switcher;
pub mod webring;

use maud::{html, Markup, PreEscaped, DOCTYPE};

// ── Static CSS files (embedded at compile time) ───────────────────────────────

static MAIN_CSS_BASE: &str = include_str!("css/main_base.css");
static NAVBAR_CSS: &str = include_str!("css/navbar.css");
static MANIFESTO_CSS: &str = include_str!("css/manifesto.css");
static FOOTER_CSS: &str = include_str!("css/footer.css");
static SHOWCASE_CSS_FILE: &str = include_str!("css/showcase.css");

// ── Inline Shallot logo SVG ───────────────────────────────────────────────────

pub fn logo_svg(size: u32) -> PreEscaped<String> {
    PreEscaped(format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{sz}" height="{sz}" viewBox="0 0 200 200" aria-hidden="true" focusable="false" style="display:block;flex-shrink:0"><g><path fill="#6e3565" d="M 94.5,45.5 C 101.58,44.193 105.58,47.0263 106.5,54C 101.378,62.9146 97.0442,72.2479 93.5,82C 95.8297,83.3454 98.1631,83.5121 100.5,82.5C 104.207,71.962 111.541,65.462 122.5,63C 146.093,58.971 164.593,66.8043 178,86.5C 182.273,94.7116 186.273,103.045 190,111.5C 190.667,114.833 190.667,118.167 190,121.5C 181.15,129.842 170.983,136.008 159.5,140C 149.127,141.823 139.461,139.989 130.5,134.5C 108.142,162.585 84.6421,163.585 60,137.5C 57.0545,132.664 54.8878,127.498 53.5,122C 46.543,122.539 39.543,123.039 32.5,123.5C 22.5494,124.108 15.0494,120.108 10,111.5C 8.566,107.594 8.23266,103.594 9,99.5C 13.1869,84.62 20.3536,71.4533 30.5,60C 40.3235,51.8919 51.6568,48.0586 64.5,48.5C 71.1638,49.2226 77.8304,49.3893 84.5,49C 88.2451,48.6978 91.5784,47.5312 94.5,45.5 Z"/></g><g><path fill="#d983a2" d="M 96.5,50.5 C 98.8242,49.1674 100.491,49.8341 101.5,52.5C 92.8671,59.4654 84.8671,67.1321 77.5,75.5C 74.8739,77.8235 72.2072,80.1568 69.5,82.5C 63.049,80.2487 57.049,77.2487 51.5,73.5C 54.4835,70.6742 57.8168,68.3409 61.5,66.5C 72.9529,63.7123 83.9529,59.5457 94.5,54C 95.5,53 96.5,52 97.5,51C 97.2716,50.6012 96.9382,50.4346 96.5,50.5 Z"/></g><g><path fill="#b47b9a" d="M 101.5,52.5 C 101.657,53.8734 101.49,55.2068 101,56.5C 97.0414,62.084 93.8747,68.084 91.5,74.5C 86.7858,74.3531 82.1191,74.6864 77.5,75.5C 84.8671,67.1321 92.8671,59.4654 101.5,52.5 Z"/></g><g><path fill="#d884a0" d="M 162.5,83.5 C 166.35,86.2608 170.017,89.2608 173.5,92.5C 175.423,95.6032 177.59,98.6032 180,101.5C 182.119,105.402 183.952,109.402 185.5,113.5C 181.538,111.205 178.205,108.205 175.5,104.5C 172.149,101.152 168.482,98.1518 164.5,95.5C 161.55,93.8847 158.884,91.8847 156.5,89.5C 159.478,88.5223 161.478,86.5223 162.5,83.5 Z"/></g></svg>"##,
        sz = size
    ))
}

// ── Color theme definitions ───────────────────────────────────────────────────

struct ThemeColor {
    id: &'static str,
    name: &'static str,
    hex: &'static str,
    hex_hover: &'static str,
    rgb: &'static str,
}

const THEME_COLORS: &[ThemeColor] = &[
    ThemeColor {
        id: "violet",
        name: "Violet",
        hex: "#8b5cf6",
        hex_hover: "#7c3aed",
        rgb: "139,92,246",
    },
    ThemeColor {
        id: "azure",
        name: "Azure",
        hex: "#3b82f6",
        hex_hover: "#2563eb",
        rgb: "59,130,246",
    },
    ThemeColor {
        id: "emerald",
        name: "Emerald",
        hex: "#10b981",
        hex_hover: "#059669",
        rgb: "16,185,129",
    },
    ThemeColor {
        id: "rose",
        name: "Rose",
        hex: "#f43f5e",
        hex_hover: "#e11d48",
        rgb: "244,63,94",
    },
    ThemeColor {
        id: "amber",
        name: "Amber",
        hex: "#f59e0b",
        hex_hover: "#d97706",
        rgb: "245,158,11",
    },
    ThemeColor {
        id: "cyan",
        name: "Cyan",
        hex: "#06b6d4",
        hex_hover: "#0891b2",
        rgb: "6,182,212",
    },
    ThemeColor {
        id: "indigo",
        name: "Indigo",
        hex: "#6366f1",
        hex_hover: "#4f46e5",
        rgb: "99,102,241",
    },
];

// ── Hidden color-radio inputs (must be first children of <body>) ──────────────

fn color_radio_inputs() -> Markup {
    html! {
        @for (i, tc) in THEME_COLORS.iter().enumerate() {
            input
                type="radio"
                name="sh-theme-color"
                id=(format!("sh-tc-{}", tc.id))
                class="sh-tc-radio"
                checked?[i == 0];
        }
    }
}

// ── Sticky navbar ─────────────────────────────────────────────────────────────

fn navbar() -> Markup {
    html! {
        header class="sh-navbar" role="banner" {
            nav class="sh-navbar__inner" aria-label="Main navigation" {

                a href="#" class="sh-navbar__brand" aria-label="Shallot home" {
                    (logo_svg(32))
                    span class="sh-navbar__brand-name" { "Shallot" }
                    span class="sh-navbar__version" aria-label="version 0.1" { "v0.1" }
                }

                div class="sh-navbar__colors" role="group" aria-label="Choose accent color" {
                    span class="sh-navbar__colors-label" aria-hidden="true" { "🎨" }
                    @for tc in THEME_COLORS {
                        label
                            for=(format!("sh-tc-{}", tc.id))
                            class="sh-color-swatch"
                            title=(tc.name)
                            style=(format!("--swatch-color:{}", tc.hex))
                        {
                            span class="sh-visually-hidden" { (tc.name) " theme" }
                        }
                    }
                    div class="sh-navbar__custom-color" title="Pick any custom color" {
                        label class="sh-custom-color-label" for="sh-custom-color" {
                            "Custom:"
                        }
                        input
                            type="color"
                            id="sh-custom-color"
                            class="sh-custom-color-input"
                            value="#8b5cf6"
                            aria-label="Custom accent color"
                            oninput="document.getElementById('sh-app').style.setProperty('--sh-primary', this.value); document.getElementById('sh-app').style.setProperty('--sh-accent', this.value); document.getElementById('sh-app').style.setProperty('--sh-primary-hover', this.value); document.getElementById('sh-app').style.setProperty('--sh-primary-bg', this.value + '20'); document.getElementById('sh-app').style.setProperty('--sh-focus-ring', this.value + '66')";
                        input
                            type="color"
                            id="sh-custom-color"
                            class="sh-custom-color-input"
                            value="#8b5cf6"
                            aria-label="Custom accent color (decorative — use presets for zero-JS live theming)";
                    }
                }

                div class="sh-navbar__links" {
                    a href="#showcase" class="sh-navbar__link" { "Components" }
                    a
                        href="https://github.com/shallot-rs/shallot"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="sh-navbar__link sh-navbar__link--github"
                    {
                        "GitHub"
                        span aria-hidden="true" { " ↗" }
                        span class="sh-visually-hidden" { " (opens in new tab)" }
                    }
                }
            }
        }
    }
}

// ── Modern manifesto strip ────────────────────────────────────────────────────

fn manifesto_strip() -> Markup {
    html! {
        section class="sh-manifesto" aria-label="Shallot introduction" {
            div class="sh-manifesto__inner" {

                div class="sh-manifesto__brand" {
                    div class="sh-manifesto__logo-wrap" { (logo_svg(72)) }
                    div class="sh-manifesto__text" {
                        h2 class="sh-manifesto__headline" {
                            "Iron Logic. "
                            span class="sh-manifesto__glass" { "Glass Aesthetics." }
                        }
                        p class="sh-manifesto__subline" {
                            "129 production-ready UI components. "
                            strong { "0 bytes of JavaScript." }
                        }
                    }
                }

                div class="sh-manifesto__stats" aria-label="Performance metrics" {
                    div class="sh-manifesto__stat" {
                        span class="sh-manifesto__stat-value" { "0ms" }
                        span class="sh-manifesto__stat-label" { "Total Blocking Time" }
                    }
                    div class="sh-manifesto__stat-divider" aria-hidden="true" {}
                    div class="sh-manifesto__stat" {
                        span class="sh-manifesto__stat-value" { "0.00" }
                        span class="sh-manifesto__stat-label" { "Layout Shift" }
                    }
                    div class="sh-manifesto__stat-divider" aria-hidden="true" {}
                    div class="sh-manifesto__stat" {
                        span class="sh-manifesto__stat-value" { "0kb" }
                        span class="sh-manifesto__stat-label" { "JavaScript Bundle" }
                    }
                    div class="sh-manifesto__stat-divider" aria-hidden="true" {}
                    div class="sh-manifesto__stat" {
                        span class="sh-manifesto__stat-value" { "100" }
                        span class="sh-manifesto__stat-label" { "Lighthouse Score" }
                    }
                }

                div class="sh-manifesto__install" aria-label="Installation snippet" {
                    span class="sh-manifesto__install-label" aria-hidden="true" { "Cargo.toml" }
                    pre class="sh-manifesto__install-code" {
                        code {
                            "shallot_components = "
                            span class="sh-syn-string" { "\"0.1\"" }
                        }
                    }
                }

                div class="sh-manifesto__ctas" {
                    a href="#showcase" class="sh-manifesto__cta sh-manifesto__cta--primary" {
                        "Browse Components"
                    }
                    a
                        href="https://github.com/shallot-rs/shallot"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="sh-manifesto__cta sh-manifesto__cta--ghost"
                    {
                        "GitHub"
                        span aria-hidden="true" { " ↗" }
                        span class="sh-visually-hidden" { " (opens in new tab)" }
                    }
                }
            }
        }
    }
}

// ── Site footer ───────────────────────────────────────────────────────────────

fn site_footer() -> Markup {
    html! {
        footer class="sh-site-footer" {
            div class="sh-site-footer__inner" {
                div class="sh-site-footer__brand" {
                    (logo_svg(28))
                    span class="sh-site-footer__brand-name" { "Shallot" }
                }
                p class="sh-site-footer__tagline" {
                    "Iron logic. Glass aesthetics. Pure Rust."
                }
                p class="sh-site-footer__copy" {
                    "© 2026 Shallot. Built with "
                    span class="sh-heart" aria-label="love" { "❤" }
                    " and " strong { "zero JavaScript." }
                }
                nav class="sh-site-footer__links" aria-label="Footer navigation" {
                    a href="feed.xml" { "📰 RSS" }
                    span aria-hidden="true" { " · " }
                    a href="https://github.com/shallot-rs/shallot" target="_blank" rel="noopener noreferrer" {
                        "💾 GitHub"
                        span class="sh-visually-hidden" { " (opens in new tab)" }
                        span aria-hidden="true" { " ↗" }
                    }
                    span aria-hidden="true" { " · " }
                    a href="https://docs.rs/shallot" target="_blank" rel="noopener noreferrer" {
                        "📚 Docs"
                        span class="sh-visually-hidden" { " (opens in new tab)" }
                        span aria-hidden="true" { " ↗" }
                    }
                }
                (webring::render())
            }
        }
    }
}

// ── Public: full homepage ─────────────────────────────────────────────────────

/// Generate the full homepage HTML.
///
/// The seven hidden color-radio inputs appear **before** `#sh-app` in the DOM
/// so that `#sh-tc-X:checked ~ #sh-app { --sh-primary: … }` works without
/// any JavaScript. HTML `<label for="…">` works across the entire document
/// regardless of where the label lives relative to the input.
pub fn homepage() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="description" content="Shallot — Iron & Glass. 129 production-ready UI components built in Rust with zero JavaScript. Fully functional in TOR High Security.";
                meta name="theme-color" content="#8b5cf6";
                meta property="og:title" content="Shallot — Zero-JS Rust UI Components";
                meta property="og:description" content="Beautiful, accessible UI components. 0 bytes of JavaScript. Works in NoScript TOR browser.";
                title { "Shallot — Iron & Glass · Zero-JS Rust UI" }
                link rel="stylesheet" href="styles/main.css";
                link rel="stylesheet" href="styles/retro.css";
                link rel="stylesheet" href="styles/showcase.css";
                link rel="stylesheet" href="styles/components.css";
                link rel="alternate" type="application/rss+xml" title="Shallot RSS" href="feed.xml";
            }
            body {
                // Skip link for accessibility
                a href="#showcase" class="sh-skip-link" { "Skip to component showcase" }

                // CRITICAL: color-theme radio inputs MUST appear before #sh-app
                // so that `#sh-tc-X:checked ~ #sh-app { --sh-primary: … }` works.
                (color_radio_inputs())

                // Main app wrapper — the target of all theme sibling selectors
                div id="sh-app" {
                    (navbar())
                    (retro_hero::render())
                    (manifesto_strip())
                    (showcase::render())
                    (site_footer())
                }
            }
        }
    }
}

// ── CSS generation ────────────────────────────────────────────────────────────

/// Main CSS: color theme overrides + static base + navbar + manifesto + footer.
pub fn main_css() -> String {
    let mut css = String::new();
    // Dynamic: per-color theme variable overrides (7 rules, generated from THEME_COLORS)
    css.push_str(&color_theme_css());
    // Static: base reset, variables, utilities
    css.push_str(MAIN_CSS_BASE);
    // Static: navbar
    css.push_str(NAVBAR_CSS);
    // Static: manifesto strip
    css.push_str(MANIFESTO_CSS);
    // Static: site footer
    css.push_str(FOOTER_CSS);
    // Static: webring
    css.push_str(&webring::webring_css());
    css
}

/// Showcase CSS: column layout, code panels, scroll animations, replay.
pub fn showcase_css() -> String {
    String::from(SHOWCASE_CSS_FILE)
}

/// Dynamically generated color-theme override rules.
///
/// Pattern: `#sh-tc-X:checked ~ #sh-app { --sh-primary: …; --sh-accent: …; }`
///
/// The hidden radio inputs sit before `#sh-app` in the DOM, making them
/// valid general siblings. The `~` combinator requires no `:has()` support.
fn color_theme_css() -> String {
    let mut css = String::new();

    // Hide the body-level radio inputs
    css.push_str("/* Color-theme radio inputs — hidden, placed at body root */\n");
    css.push_str(".sh-tc-radio { display: none; }\n\n");

    // Per-theme variable block
    css.push_str("/* Per-theme CSS variable overrides (zero JS, CSS sibling selectors) */\n");
    for tc in THEME_COLORS {
        css.push_str(&format!(
            "#sh-tc-{id}:checked ~ #sh-app {{\n  \
             --sh-primary:       {hex};\n  \
             --sh-primary-hover: {hover};\n  \
             --sh-primary-bg:    rgba({rgb}, 0.12);\n  \
             --sh-accent:        {hex};\n  \
             --sh-focus-ring:    rgba({rgb}, 0.4);\n\
             }}\n",
            id = tc.id,
            hex = tc.hex,
            hover = tc.hex_hover,
            rgb = tc.rgb,
        ));
    }

    // Swatch active ring via :has() — degrades gracefully if unsupported
    css.push_str("\n/* Swatch selected-ring — uses :has(), degrades gracefully */\n");
    for tc in THEME_COLORS {
        css.push_str(&format!(
            "body:has(#sh-tc-{id}:checked) .sh-color-swatch[for='sh-tc-{id}'] {{\n  \
             box-shadow: 0 0 0 2px #fff, 0 0 0 4px {hex};\n  \
             transform: scale(1.15);\n\
             }}\n",
            id = tc.id,
            hex = tc.hex,
        ));
    }

    css
}
