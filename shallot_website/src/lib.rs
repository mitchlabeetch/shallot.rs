//! Shallot Website - Zero-JS Component Showcase
//!
//! A fully interactive, themeable website showcasing the Shallot component library.
//! Built with pure HTML/CSS - no JavaScript, fully Tor-compatible.

pub mod component_docs;
pub mod retro_hero;
pub mod rss;
pub mod showcase;
pub mod theme_marketplace;
pub mod theme_panel;
pub mod theme_switcher;
pub mod webring;

use maud::{html, Markup, DOCTYPE};

/// Main website layout
pub fn website_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="description" content="Shallot - Zero-JS Rust UI Component Library";
                meta name="theme-color" content="#667eea";
                title { (title) " - Shallot" }
                link rel="stylesheet" href="/styles/main.css";
                link rel="stylesheet" href="/styles/retro.css";
                link rel="stylesheet" href="/styles/showcase.css";
            }
            body {
                (content)

                footer class="sh-site-footer" {
                    div class="sh-site-footer__content" {
                        p {
                            "© 2026 Shallot. Built with "
                            span class="sh-heart" aria-label="love" { "❤" }
                            " and zero JavaScript."
                        }
                        p class="sh-site-footer__tagline" {
                            "Iron logic. Glass aesthetics. Pure Rust."
                        }
                    }
                }
            }
        }
    }
}

/// Homepage with hero and showcase
pub fn homepage() -> Markup {
    let hero = retro_hero::render();
    let showcase = showcase::render();

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="description" content="Shallot - Zero-JS Rust UI Component Library. 129 production-ready components with CSS-only animations.";
                meta name="theme-color" content="#667eea";
                title { "Shallot - Zero-JS Rust UI Components" }
                link rel="stylesheet" href="styles/main.css";
                link rel="stylesheet" href="styles/retro.css";
                link rel="stylesheet" href="styles/showcase.css";
                link rel="stylesheet" href="styles/components.css";
            }
            body {
                /* Skip to content link for accessibility */
                a href="#showcase" class="sh-skip-link" {
                    "Skip to content"
                }

                /* Theme panel for zero-JS preview customization */
                (theme_panel::render())

                (hero)
                (showcase)

                footer class="sh-site-footer" {
                    div class="sh-site-footer__content" {
                        p {
                            "© 2026 Shallot. Built with "
                            span class="sh-heart" aria-label="love" { "❤" }
                            " and zero JavaScript."
                        }
                        p class="sh-site-footer__tagline" {
                            "Iron logic. Glass aesthetics. Pure Rust."
                        }
                        p class="sh-site-footer__links" {
                            a href="feed.xml" { "📰 RSS Feed" }
                            " | "
                            a href="#community-themes" { "🎨 Community Themes" }
                            " | "
                            a href="https://github.com/shallot-rs/shallot" target="_blank" rel="noopener noreferrer" {
                                "💾 GitHub"
                                span class="sh-visually-hidden" { " (opens in new tab)" }
                                span aria-hidden="true" { " ↗" }
                            }
                        }
                        /* Webring widget */
                        (webring::render())
                    }
                }
            }
        }
    }
}

/// Generate main CSS
pub fn main_css() -> String {
    let mut css = String::new();
    css.push_str(&main_css_content());
    css.push_str(&webring::webring_css());
    css.push_str(&theme_marketplace::theme_marketplace_css());
    css.push_str(&theme_panel::theme_panel_css());
    css
}

fn main_css_content() -> String {
    r#"
/* ============================================
   SHALLOT WEBSITE - Main Styles
   Zero JavaScript. Pure CSS Magic.
   ============================================ */

:root {
    /* Default theme - will be overridden by theme switcher */
    --sh-primary: #667eea;
    --sh-primary-hover: #5a67d8;
    --sh-primary-bg: rgba(102, 126, 234, 0.1);
    --sh-secondary: #764ba2;
    --sh-accent: #f093fb;
    --sh-success: #22c55e;
    --sh-warning: #f59e0b;
    --sh-error: #ef4444;
    --sh-text: #1f2937;
    --sh-text-secondary: #4b5563;
    --sh-text-muted: #9ca3af;
    --sh-surface: #ffffff;
    --sh-surface-2: #f9fafb;
    --sh-surface-hover: #f3f4f6;
    --sh-border: #e5e7eb;
    --sh-radius-sm: 0.25rem;
    --sh-radius-md: 0.375rem;
    --sh-radius-lg: 0.5rem;
    --sh-radius-xl: 0.75rem;
    --sh-radius-full: 9999px;
}

/* Dark mode support via data attribute */
[data-theme="dark"] {
    --sh-text: #f9fafb;
    --sh-text-secondary: #d1d5db;
    --sh-text-muted: #6b7280;
    --sh-surface: #1f2937;
    --sh-surface-2: #111827;
    --sh-surface-hover: #374151;
    --sh-border: #374151;
    --sh-primary-bg: rgba(102, 126, 234, 0.2);
}

/* Reset & Base */
*, *::before, *::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html {
    scroll-behavior: smooth;
    scroll-padding-top: 100px;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    font-size: 1rem;
    line-height: 1.6;
    color: var(--sh-text);
    background: var(--sh-surface);
}

/* Typography */
h1, h2, h3, h4, h5, h6 {
    font-weight: 700;
    line-height: 1.2;
}

a {
    color: var(--sh-primary);
    text-decoration: none;
    transition: color 0.2s ease;
}

a:hover {
    color: var(--sh-primary-hover);
}

/* Site Footer */
.sh-site-footer {
    background: var(--sh-surface-2);
    border-top: 1px solid var(--sh-border);
    padding: 2rem 1rem;
    text-align: center;
}

.sh-site-footer__content {
    max-width: 64rem;
    margin: 0 auto;
}

.sh-site-footer p {
    color: var(--sh-text-secondary);
    font-size: 0.875rem;
}

.sh-site-footer__tagline {
    margin-top: 0.5rem;
    font-style: italic;
    color: var(--sh-text-muted);
}

.sh-site-footer__links {
    margin-top: 0.75rem;
    font-size: 0.75rem;
}

.sh-site-footer__links a {
    color: var(--sh-primary);
    text-decoration: none;
}

.sh-site-footer__links a:hover {
    text-decoration: underline;
}

.sh-heart {
    display: inline-block;
    animation: sh-heartbeat 1s ease-in-out infinite;
}

@keyframes sh-heartbeat {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.2); }
}

/* Utility Classes */
.sh-container {
    max-width: 80rem;
    margin: 0 auto;
    padding: 0 1rem;
}

.sh-sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}

/* Focus styles */
:focus-visible {
    outline: 2px solid var(--sh-primary);
    outline-offset: 2px;
}

/* Skip Link - Accessibility */
.sh-skip-link {
    position: absolute;
    top: -100%;
    left: 50%;
    transform: translateX(-50%);
    background: var(--sh-primary);
    color: white;
    padding: 0.75rem 1.5rem;
    border-radius: var(--sh-radius-md);
    text-decoration: none;
    font-weight: 600;
    z-index: 9999;
    transition: top 0.2s ease;
}

.sh-skip-link:focus {
    top: 1rem;
}

/* Visually Hidden - for screen readers */
.sh-visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
    pointer-events: none;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    *, *::before, *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }

    html {
        scroll-behavior: auto;
    }
}
"#
    .to_string()
}

/// Generate showcase CSS
pub fn showcase_css() -> String {
    r#"
/* ============================================
   SHALLOT SHOWCASE STYLES
   ============================================ */

/* Theme Switcher */
.sh-theme-switcher {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 1000;
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-lg);
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
    padding: 1rem;
    max-width: 20rem;
}

.sh-theme-switcher__title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--sh-text);
    margin-bottom: 0.75rem;
}

.sh-theme-switcher__presets {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.5rem;
    margin-bottom: 1rem;
}

.sh-theme-preset {
    width: 100%;
    aspect-ratio: 1;
    border-radius: var(--sh-radius-md);
    border: 2px solid transparent;
    cursor: pointer;
    transition: transform 0.2s ease, border-color 0.2s ease;
}

.sh-theme-preset:hover {
    transform: scale(1.1);
}

.sh-theme-preset:focus-visible {
    outline: 2px solid var(--sh-primary);
    outline-offset: 2px;
}

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
}

/* Category Navigation */
.sh-categories {
    position: sticky;
    top: 0;
    z-index: 100;
    background: var(--sh-surface);
    border-bottom: 1px solid var(--sh-border);
    padding: 1rem 0;
}

.sh-categories__list {
    display: flex;
    gap: 0.5rem;
    overflow-x: auto;
    padding-bottom: 0.5rem;
    scrollbar-width: thin;
}

.sh-category-link {
    display: inline-block;
    padding: 0.5rem 1rem;
    background: var(--sh-surface-2);
    border-radius: var(--sh-radius-full);
    font-size: 0.875rem;
    font-weight: 500;
    white-space: nowrap;
    transition: all 0.2s ease;
}

.sh-category-link:hover {
    background: var(--sh-primary-bg);
    color: var(--sh-primary);
}

/* Component Grid */
.sh-component-section {
    padding: 3rem 0;
}

.sh-component-section__title {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    color: var(--sh-text);
}

.sh-component-section__description {
    color: var(--sh-text-secondary);
    margin-bottom: 2rem;
}

.sh-component-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr));
    gap: 1.5rem;
}

/* Component Card */
.sh-component-card {
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-lg);
    overflow: hidden;
    transition: box-shadow 0.3s ease, transform 0.3s ease;
}

.sh-component-card:hover {
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
}

.sh-component-card__preview {
    padding: 2rem;
    background: var(--sh-surface-2);
    border-bottom: 1px solid var(--sh-border);
    min-height: 8rem;
    display: flex;
    align-items: center;
    justify-content: center;
}

.sh-component-card__content {
    padding: 1.5rem;
}

.sh-component-card__title {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
}

.sh-component-card__description {
    font-size: 0.875rem;
    color: var(--sh-text-secondary);
    margin-bottom: 1rem;
}

/* Code Dropdown */
.sh-code-dropdown {
    border-top: 1px solid var(--sh-border);
}

.sh-code-dropdown__toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 1rem 1.5rem;
    background: transparent;
    border: none;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-primary);
    cursor: pointer;
    transition: background 0.2s ease;
}

.sh-code-dropdown__toggle:hover {
    background: var(--sh-surface-2);
}

.sh-code-dropdown__icon {
    transition: transform 0.3s ease;
}

/* CSS-only dropdown using checkbox hack */
.sh-code-dropdown__checkbox {
    display: none;
}

.sh-code-dropdown__content {
    display: none;
    padding: 1rem 1.5rem;
    background: var(--sh-surface-2);
    border-top: 1px solid var(--sh-border);
}

.sh-code-dropdown__checkbox:checked ~ .sh-code-dropdown__content {
    display: block;
}

.sh-code-dropdown__checkbox:checked ~ .sh-code-dropdown__toggle .sh-code-dropdown__icon {
    transform: rotate(180deg);
}

/* Code Tabs */
.sh-code-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
}

.sh-code-tab {
    padding: 0.5rem 1rem;
    background: transparent;
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-md);
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.sh-code-tab:hover {
    background: var(--sh-surface-2);
}

/* CSS-only tabs using radio buttons */
.sh-code-tab__radio {
    display: none;
}

.sh-code-tab__radio:checked + .sh-code-tab {
    background: var(--sh-primary);
    color: white;
    border-color: var(--sh-primary);
}

.sh-code-block {
    display: none;
    background: #1f2937;
    color: #f3f4f6;
    padding: 1rem;
    border-radius: var(--sh-radius-md);
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.75rem;
    overflow-x: auto;
    white-space: pre;
}

.sh-code-tab__radio[value="full"]:checked ~ .sh-code-block--full {
    display: block;
}

.sh-code-tab__radio[value="library"]:checked ~ .sh-code-block--library {
    display: block;
}

/* Responsive */
@media (max-width: 64rem) {
    .sh-theme-switcher {
        top: auto;
        bottom: 1rem;
        right: 1rem;
        max-width: 16rem;
    }

    .sh-component-grid {
        grid-template-columns: 1fr;
    }
}
"#
    .to_string()
}
