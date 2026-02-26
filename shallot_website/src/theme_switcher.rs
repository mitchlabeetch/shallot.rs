//! Theme Switcher - CSS-Only Theme Customization
//!
//! A floating theme picker with 7 presets and custom color selection.
//! Uses CSS custom properties and the checkbox hack for interactivity.

use maud::{html, Markup};

/// Theme preset with name and primary color
pub struct ThemePreset {
    pub name: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
    pub accent: &'static str,
}

/// Available theme presets
pub const THEME_PRESETS: &[ThemePreset] = &[
    ThemePreset {
        name: "Ocean",
        primary: "#667eea",
        secondary: "#764ba2",
        accent: "#f093fb",
    },
    ThemePreset {
        name: "Forest",
        primary: "#22c55e",
        secondary: "#16a34a",
        accent: "#84cc16",
    },
    ThemePreset {
        name: "Sunset",
        primary: "#f59e0b",
        secondary: "#ef4444",
        accent: "#ec4899",
    },
    ThemePreset {
        name: "Midnight",
        primary: "#3b82f6",
        secondary: "#1e40af",
        accent: "#8b5cf6",
    },
    ThemePreset {
        name: "Cherry",
        primary: "#ec4899",
        secondary: "#be185d",
        accent: "#f43f5e",
    },
    ThemePreset {
        name: "Teal",
        primary: "#14b8a6",
        secondary: "#0d9488",
        accent: "#2dd4bf",
    },
    ThemePreset {
        name: "Amber",
        primary: "#f59e0b",
        secondary: "#d97706",
        accent: "#fbbf24",
    },
<<<<<<< HEAD
    ThemePreset {
        name: "Dark Ocean",
        primary: "#667eea",
        secondary: "#764ba2",
        accent: "#f093fb",
    },
    ThemePreset {
        name: "Dark Forest",
        primary: "#22c55e",
        secondary: "#16a34a",
        accent: "#84cc16",
    },
    ThemePreset {
        name: "Dark Sunset",
        primary: "#f59e0b",
        secondary: "#ef4444",
        accent: "#ec4899",
    },
=======
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
];

/// Render the theme switcher
pub fn render() -> Markup {
    html! {
        aside class="sh-theme-switcher" id="themes" aria-label="Theme customization" {
            h3 class="sh-theme-switcher__title" {
                "🎨 Theme"
            }

            /* Theme presets */
            div class="sh-theme-switcher__presets" role="group" aria-label="Theme presets" {
                @for (i, preset) in THEME_PRESETS.iter().enumerate() {
                    label class="sh-theme-preset-wrapper" {
                        input
                            type="radio"
                            name="theme-preset"
                            id=(format!("theme-{}", i))
                            class="sh-theme-preset__input"
                            data-primary=(preset.primary)
                            data-secondary=(preset.secondary)
                            data-accent=(preset.accent)
                            checked=(i == 0)
                        ;
                        span
                            class="sh-theme-preset"
                            style=(format!("background: linear-gradient(135deg, {} 0%, {} 100%)", preset.primary, preset.secondary))
                            aria-label=(format!("{} theme", preset.name))
                            tabindex="0"
                        {}
                    }
                }
            }

            /* Custom color picker */
            div class="sh-theme-switcher__custom" {
                label class="sh-theme-switcher__label" for="custom-color" {
                    "Custom primary color:"
                }
                input
                    type="color"
                    id="custom-color"
                    class="sh-color-input"
                    value="#667eea"
                    aria-label="Custom primary color"
                ;
                p class="sh-theme-switcher__hint" {
                    "Click to pick any color"
                }
            }

            /* Additional customization */
            div class="sh-theme-switcher__options" {
                h4 class="sh-theme-switcher__subtitle" {
                    "More options"
                }

                label class="sh-theme-option" {
<<<<<<< HEAD
                    input type="checkbox" id="option-dark-mode" class="sh-theme-option__input" ;
                    span class="sh-theme-option__label" { "Dark mode" }
                }

                label class="sh-theme-option" {
=======
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
                    input type="checkbox" id="option-radius" class="sh-theme-option__input" ;
                    span class="sh-theme-option__label" { "Rounded corners" }
                }

                label class="sh-theme-option" {
                    input type="checkbox" id="option-shadows" class="sh-theme-option__input" checked ;
                    span class="sh-theme-option__label" { "Shadows" }
                }

                label class="sh-theme-option" {
                    input type="checkbox" id="option-animations" class="sh-theme-option__input" checked ;
                    span class="sh-theme-option__label" { "Animations" }
                }
            }
        }
    }
}

/// Generate theme switcher CSS
pub fn theme_switcher_css() -> String {
    r#"
/* ============================================
   THEME SWITCHER - CSS-Only Customization
   ============================================ */

.sh-theme-switcher {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 1000;
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-lg);
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
    padding: 1.25rem;
    width: 14rem;
    max-width: calc(100vw - 2rem);
    transition: transform 0.3s ease, opacity 0.3s ease;
}

/* Hide on mobile initially, show when scrolled past hero */
@media (max-width: 48rem) {
    .sh-theme-switcher {
        top: auto;
        bottom: 1rem;
        transform: translateY(calc(100% - 3rem));
    }

    .sh-theme-switcher:hover,
    .sh-theme-switcher:focus-within {
        transform: translateY(0);
    }
}

.sh-theme-switcher__title {
    font-size: 0.875rem;
    font-weight: 700;
    color: var(--sh-text);
    margin-bottom: 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.sh-theme-switcher__subtitle {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--sh-text-secondary);
    margin: 1rem 0 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

/* Preset grid */
.sh-theme-switcher__presets {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.5rem;
}

.sh-theme-preset-wrapper {
    position: relative;
}

.sh-theme-preset__input {
    position: absolute;
    width: 1px;
    height: 1px;
    opacity: 0;
}

.sh-theme-preset {
    display: block;
    width: 100%;
    aspect-ratio: 1;
    border-radius: var(--sh-radius-md);
    border: 2px solid transparent;
    cursor: pointer;
    transition: transform 0.2s ease, border-color 0.2s ease, box-shadow 0.2s ease;
}

.sh-theme-preset:hover {
    transform: scale(1.1);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.sh-theme-preset__input:checked + .sh-theme-preset {
    border-color: var(--sh-text);
    box-shadow: 0 0 0 2px var(--sh-surface), 0 0 0 4px var(--sh-text);
}

.sh-theme-preset:focus-visible {
    outline: 2px solid var(--sh-primary);
    outline-offset: 2px;
}

/* Custom color picker */
.sh-theme-switcher__custom {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--sh-border);
}

.sh-theme-switcher__label {
    display: block;
    font-size: 0.75rem;
    color: var(--sh-text-muted);
    margin-bottom: 0.5rem;
}

.sh-color-input {
    width: 100%;
    height: 2.5rem;
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-md);
    cursor: pointer;
    background: var(--sh-surface);
    padding: 0.25rem;
}

.sh-color-input::-webkit-color-swatch-wrapper {
    padding: 0;
    border-radius: var(--sh-radius-md);
}

.sh-color-input::-webkit-color-swatch {
    border: none;
    border-radius: var(--sh-radius-md);
}

.sh-color-input::-moz-color-swatch {
    border: none;
    border-radius: var(--sh-radius-md);
}

.sh-theme-switcher__hint {
    font-size: 0.625rem;
    color: var(--sh-text-muted);
    margin-top: 0.5rem;
    text-align: center;
}

/* Additional options */
.sh-theme-switcher__options {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--sh-border);
}

.sh-theme-option {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    cursor: pointer;
}

.sh-theme-option__input {
    width: 1rem;
    height: 1rem;
    accent-color: var(--sh-primary);
    cursor: pointer;
}

.sh-theme-option__label {
    font-size: 0.75rem;
    color: var(--sh-text-secondary);
    cursor: pointer;
}

/* CSS-only theme switching using :has() and data attributes */
/* When a preset is selected, update CSS custom properties */
.sh-theme-switcher:has(#theme-0:checked) {
    --sh-primary: #667eea;
    --sh-primary-hover: #5a67d8;
    --sh-secondary: #764ba2;
    --sh-accent: #f093fb;
}

.sh-theme-switcher:has(#theme-1:checked) {
    --sh-primary: #22c55e;
    --sh-primary-hover: #16a34a;
    --sh-secondary: #16a34a;
    --sh-accent: #84cc16;
}

.sh-theme-switcher:has(#theme-2:checked) {
    --sh-primary: #f59e0b;
    --sh-primary-hover: #d97706;
    --sh-secondary: #ef4444;
    --sh-accent: #ec4899;
}

.sh-theme-switcher:has(#theme-3:checked) {
    --sh-primary: #3b82f6;
    --sh-primary-hover: #2563eb;
    --sh-secondary: #1e40af;
    --sh-accent: #8b5cf6;
}

.sh-theme-switcher:has(#theme-4:checked) {
    --sh-primary: #ec4899;
    --sh-primary-hover: #db2777;
    --sh-secondary: #be185d;
    --sh-accent: #f43f5e;
}

.sh-theme-switcher:has(#theme-5:checked) {
    --sh-primary: #14b8a6;
    --sh-primary-hover: #0d9488;
    --sh-secondary: #0d9488;
    --sh-accent: #2dd4bf;
}

.sh-theme-switcher:has(#theme-6:checked) {
    --sh-primary: #f59e0b;
    --sh-primary-hover: #d97706;
    --sh-secondary: #d97706;
    --sh-accent: #fbbf24;
}

<<<<<<< HEAD
.sh-theme-switcher:has(#theme-7:checked) {
    --sh-primary: #667eea;
    --sh-primary-hover: #5a67d8;
    --sh-secondary: #764ba2;
    --sh-accent: #f093fb;
}

.sh-theme-switcher:has(#theme-8:checked) {
    --sh-primary: #22c55e;
    --sh-primary-hover: #16a34a;
    --sh-secondary: #16a34a;
    --sh-accent: #84cc16;
}

.sh-theme-switcher:has(#theme-9:checked) {
    --sh-primary: #f59e0b;
    --sh-primary-hover: #d97706;
    --sh-secondary: #ef4444;
    --sh-accent: #ec4899;
}

/* Dark mode toggle */
#option-dark-mode:checked ~ html {
    --sh-text: #f9fafb;
    --sh-text-secondary: #d1d5db;
    --sh-text-muted: #6b7280;
    --sh-surface: #1f2937;
    --sh-surface-2: #111827;
    --sh-surface-hover: #374151;
    --sh-border: #374151;
    --sh-primary-bg: rgba(102, 126, 234, 0.2);
}

=======
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
/* Option toggles using checkbox hack */
#option-radius:not(:checked) ~ * {
    --sh-radius-sm: 0;
    --sh-radius-md: 0;
    --sh-radius-lg: 0;
    --sh-radius-xl: 0;
}

#option-shadows:not(:checked) ~ * {
    --sh-shadow-sm: none;
    --sh-shadow-md: none;
    --sh-shadow-lg: none;
    --sh-shadow-xl: none;
}

@media (prefers-reduced-motion: reduce) {
    .sh-theme-switcher {
        transition: none;
    }

    .sh-theme-preset {
        transition: none;
    }
}
"#
    .to_string()
}
