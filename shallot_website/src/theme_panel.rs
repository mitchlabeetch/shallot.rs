//! Theme Panel - CSS-only floating control for preview theming
//!
//! Provides a zero-JavaScript theme selector that uses CSS sibling selectors
//! to update CSS variables for component previews.

use maud::{html, Markup};

/// Predefined theme definitions
pub struct ThemeDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub accent: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
}

pub const THEMES: &[ThemeDefinition] = &[
    ThemeDefinition {
        id: "default",
        name: "Default",
        accent: "#667eea",
        primary: "#667eea",
        secondary: "#764ba2",
    },
    ThemeDefinition {
        id: "cyberpunk",
        name: "Cyberpunk",
        accent: "#ff003c",
        primary: "#fcee0a",
        secondary: "#00f0ff",
    },
    ThemeDefinition {
        id: "synthwave",
        name: "Synthwave",
        accent: "#05ffa1",
        primary: "#ff71ce",
        secondary: "#01cdfe",
    },
    ThemeDefinition {
        id: "nordic",
        name: "Nordic",
        accent: "#88c0d0",
        primary: "#5e81ac",
        secondary: "#81a1c1",
    },
    ThemeDefinition {
        id: "solarized",
        name: "Solarized",
        accent: "#cb4b16",
        primary: "#268bd2",
        secondary: "#2aa198",
    },
];

/// Render the CSS-only floating theme panel
pub fn render() -> Markup {
    html! {
        aside class="sh-theme-panel" aria-label="Theme customization panel" {
            div class="sh-theme-panel__header" {
                h3 class="sh-theme-panel__title" { "🎨 Theme" }
                p class="sh-theme-panel__subtitle" { "Zero-JS preview theming" }
            }

            div class="sh-theme-panel__presets" role="group" aria-label="Preset themes" {
                @for theme in THEMES {
                    label class="sh-theme-preset" title=(theme.name) {
                        input
                            type="radio"
                            name="theme"
                            value=(theme.id)
                            id=(format!("theme-{}", theme.id))
                            class="sh-theme-radio";

                        span class="sh-theme-preset__swatch" style=(format!(
                            "background: linear-gradient(135deg, {} 0%, {} 50%, {} 100%)",
                            theme.primary, theme.secondary, theme.accent
                        )) {}

                        span { (theme.name) }
                    }
                }
            }

            div class="sh-theme-panel__custom" {
                label for="custom-accent" class="sh-theme-panel__label" { "Custom:" }
                input
                    type="color"
                    id="custom-accent"
                    class="sh-color-input"
                    value="#667eea"
                    aria-label="Custom accent color picker";
            }

            p class="sh-theme-panel__note" {
                "Zero-JS theme control via CSS variables."
            }
        }
    }
}

/// Generate CSS for theme panel
pub fn theme_panel_css() -> String {
    r#"
/* ============================================
   THEME PANEL - Zero-JS Floating Control
   ============================================ */

/* Hidden radio inputs for theme selection */
.sh-theme-radio {
    display: none;
}

/* Floating theme panel - fixed to right side */
.sh-theme-panel {
    position: fixed;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    z-index: 900;
    background: var(--sh-surface);
    border: 2px solid var(--sh-border);
    border-radius: var(--sh-radius-lg);
    padding: 1.25rem;
    width: 200px;
    max-height: 80vh;
    overflow-y: auto;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
    font-size: 0.875rem;
}

.sh-theme-panel__header {
    margin-bottom: 1rem;
    text-align: center;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--sh-border);
}

.sh-theme-panel__title {
    font-size: 1.125rem;
    margin: 0;
    font-weight: 700;
    color: var(--sh-text);
}

.sh-theme-panel__subtitle {
    font-size: 0.75rem;
    color: var(--sh-text-muted);
    margin: 0.25rem 0 0 0;
    font-weight: 500;
}

/* Theme presets grid */
.sh-theme-panel__presets {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
    margin-bottom: 1.25rem;
}

.sh-theme-preset {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    border-radius: var(--sh-radius-md);
    padding: 0.5rem;
    transition: all 0.2s ease;
}

.sh-theme-preset:hover {
    background: var(--sh-surface-2);
}

.sh-theme-preset__swatch {
    display: block;
    width: 100%;
    aspect-ratio: 1;
    border: 3px solid transparent;
    border-radius: var(--sh-radius-md);
    transition: all 0.2s ease;
    box-sizing: border-box;
}

.sh-theme-radio:checked ~ .sh-theme-preset__swatch {
    border-color: var(--sh-text);
    box-shadow: 0 0 0 2px var(--sh-surface),
                0 0 8px rgba(0, 0, 0, 0.2);
}

.sh-theme-preset__name {
    font-size: 0.7rem;
    font-weight: 500;
    color: var(--sh-text-secondary);
    white-space: nowrap;
}

/* Custom color section */
.sh-theme-panel__custom {
    padding: 1rem 0;
    border-top: 1px solid var(--sh-border);
    border-bottom: 1px solid var(--sh-border);
    margin-bottom: 1rem;
}

.sh-theme-panel__label {
    display: block;
    margin: 0 0 0.5rem 0;
    font-weight: 600;
    color: var(--sh-text);
    font-size: 0.813rem;
}

.sh-color-input {
    width: 100%;
    height: 2.5rem;
    border: 2px solid var(--sh-border);
    border-radius: var(--sh-radius-md);
    cursor: pointer;
    transition: border-color 0.2s ease;
}

.sh-color-input:hover {
    border-color: var(--sh-text-muted);
}

.sh-color-input:focus {
    outline: 2px solid var(--sh-primary);
    outline-offset: 2px;
}

.sh-theme-panel__note {
    margin: 0;
    font-size: 0.75rem;
    color: var(--sh-text-muted);
    line-height: 1.4;
    text-align: center;
}

/* Responsive: hide panel on narrow screens */
@media (max-width: 768px) {
    .sh-theme-panel {
        position: fixed;
        right: 1rem;
        bottom: 1rem;
        top: auto;
        transform: none;
        width: auto;
        max-width: calc(100vw - 2rem);
        max-height: auto;
    }
}

/* Accessibility: respect prefers-reduced-motion */
@media (prefers-reduced-motion: reduce) {
    .sh-theme-preset,
    .sh-color-input {
        transition: none;
    }
}
"#
    .to_string()
}
