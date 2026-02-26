//! Webring Widget
//!
//! A CSS-only webring navigation component.
//! Joins the "No-JS Webring" for 90s web authenticity.

use maud::{html, Markup};

/// Webring navigation
pub fn render() -> Markup {
    html! {
        div class="sh-webring" role="navigation" aria-label="Webring navigation" {
            h3 class="sh-webring__title" {
                "🌐 No-JS Webring"
            }
            p class="sh-webring__description" {
                "A collection of websites built without JavaScript"
            }
            div class="sh-webring__nav" {
                a href="#" class="sh-webring__link" aria-label="Previous site in webring" {
                    span aria-hidden="true" { "←" }
                    " Prev"
                }
                a href="#" class="sh-webring__link sh-webring__link--random" aria-label="Random site" {
                    "🎲 Random"
                }
                a href="#" class="sh-webring__link" aria-label="Next site in webring" {
                    "Next "
                    span aria-hidden="true" { "→" }
                }
            }
            p class="sh-webring__join" {
                "Want to join? "
                a href="https://github.com/no-js-webring" target="_blank" rel="noopener noreferrer" {
                    "Apply here"
                    span class="sh-visually-hidden" { " (opens in new tab)" }
                    span aria-hidden="true" { " ↗" }
                }
            }
        }
    }
}

/// Generate webring CSS
pub fn webring_css() -> String {
    r#"
/* Webring Widget */
.sh-webring {
    max-width: 24rem;
    margin: 2rem auto;
    padding: 1.5rem;
    background: var(--sh-surface-2);
    border: 2px dashed var(--sh-border);
    border-radius: var(--sh-radius-lg);
    text-align: center;
}

.sh-webring__title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--sh-text);
    margin-bottom: 0.5rem;
}

.sh-webring__description {
    font-size: 0.75rem;
    color: var(--sh-text-muted);
    margin-bottom: 1rem;
}

.sh-webring__nav {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-bottom: 1rem;
}

.sh-webring__link {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.5rem 0.75rem;
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-md);
    color: var(--sh-text-secondary);
    text-decoration: none;
    font-size: 0.75rem;
    font-weight: 500;
    transition: all 0.2s ease;
}

.sh-webring__link:hover {
    background: var(--sh-primary);
    border-color: var(--sh-primary);
    color: white;
}

.sh-webring__link--random {
    background: var(--sh-surface-2);
}

.sh-webring__join {
    font-size: 0.75rem;
    color: var(--sh-text-muted);
}

.sh-webring__join a {
    color: var(--sh-primary);
    text-decoration: underline;
}
"#
    .to_string()
}
