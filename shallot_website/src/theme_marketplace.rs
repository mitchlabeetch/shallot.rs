//! Theme Marketplace
//!
//! Community-submitted theme presets.
//! Users can submit themes via GitHub PRs.

use maud::{html, Markup};

/// Community theme
pub struct CommunityTheme {
    pub name: &'static str,
    pub author: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
    pub accent: &'static str,
    pub downloads: u32,
}

/// Community themes
pub const COMMUNITY_THEMES: &[CommunityTheme] = &[
    CommunityTheme {
        name: "Cyberpunk",
        author: "@rustacean",
        primary: "#fcee0a",
        secondary: "#00f0ff",
        accent: "#ff003c",
        downloads: 1247,
    },
    CommunityTheme {
        name: "Synthwave",
        author: "@neon_dev",
        primary: "#ff71ce",
        secondary: "#01cdfe",
        accent: "#05ffa1",
        downloads: 892,
    },
    CommunityTheme {
        name: "Nordic",
        author: "@viking_coder",
        primary: "#5e81ac",
        secondary: "#81a1c1",
        accent: "#88c0d0",
        downloads: 654,
    },
    CommunityTheme {
        name: "Solarized",
        author: "@ethan_schoonover",
        primary: "#268bd2",
        secondary: "#2aa198",
        accent: "#cb4b16",
        downloads: 1523,
    },
    CommunityTheme {
        name: "Gruvbox",
        author: "@morhetz",
        primary: "#d79921",
        secondary: "#689d6a",
        accent: "#cc241d",
        downloads: 987,
    },
    CommunityTheme {
        name: "Dracula",
        author: "@dracula_theme",
        primary: "#bd93f9",
        secondary: "#50fa7b",
        accent: "#ff79c6",
        downloads: 2341,
    },
];

/// Render theme marketplace
pub fn render() -> Markup {
    html! {
        section class="sh-theme-marketplace" id="community-themes" {
            div class="sh-container" {
                h2 class="sh-theme-marketplace__title" {
                    "🎨 Community Themes"
                }
                p class="sh-theme-marketplace__description" {
                    "Themes submitted by the Shallot community. "
                    a href="https://github.com/shallot-rs/shallot/themes" target="_blank" rel="noopener noreferrer" {
                        "Submit your own!"
                        span class="sh-visually-hidden" { " (opens in new tab)" }
                        span aria-hidden="true" { " ↗" }
                    }
                }

                div class="sh-theme-grid" {
                    @for theme in COMMUNITY_THEMES {
                        article class="sh-theme-card" {
                            div
                                class="sh-theme-card__preview"
                                style=(format!("background: linear-gradient(135deg, {} 0%, {} 50%, {} 100%)", theme.primary, theme.secondary, theme.accent))
                            {}
                            div class="sh-theme-card__content" {
                                h3 class="sh-theme-card__name" {
                                    (theme.name)
                                }
                                p class="sh-theme-card__author" {
                                    "by " (theme.author)
                                }
                                div class="sh-theme-card__stats" {
                                    span class="sh-theme-card__downloads" {
                                        "📥 " (theme.downloads)
                                    }
                                }
                                button
                                    class="sh-theme-card__install"
                                    type="button"
                                    data-primary=(theme.primary)
                                    data-secondary=(theme.secondary)
                                    data-accent=(theme.accent)
                                {
                                    "Use Theme"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate theme marketplace CSS
pub fn theme_marketplace_css() -> String {
    r#"
/* Theme Marketplace */
.sh-theme-marketplace {
    padding: 4rem 0;
    background: var(--sh-surface-2);
}

.sh-theme-marketplace__title {
    font-size: 1.75rem;
    text-align: center;
    margin-bottom: 0.5rem;
}

.sh-theme-marketplace__description {
    text-align: center;
    color: var(--sh-text-secondary);
    margin-bottom: 2rem;
}

.sh-theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(16rem, 1fr));
    gap: 1.5rem;
}

.sh-theme-card {
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-lg);
    overflow: hidden;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.sh-theme-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
}

.sh-theme-card__preview {
    height: 6rem;
}

.sh-theme-card__content {
    padding: 1rem;
}

.sh-theme-card__name {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
}

.sh-theme-card__author {
    font-size: 0.75rem;
    color: var(--sh-text-muted);
    margin-bottom: 0.5rem;
}

.sh-theme-card__stats {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
}

.sh-theme-card__downloads {
    font-size: 0.75rem;
    color: var(--sh-text-muted);
}

.sh-theme-card__install {
    width: 100%;
    padding: 0.5rem;
    background: var(--sh-primary);
    color: white;
    border: none;
    border-radius: var(--sh-radius-md);
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s ease;
}

.sh-theme-card__install:hover {
    background: var(--sh-primary-hover);
}
"#
    .to_string()
}
