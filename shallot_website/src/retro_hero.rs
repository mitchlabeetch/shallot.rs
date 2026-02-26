//! Retro Hero Section - 90s Geocities Style with Punchline
//!
//! A lovingly accurate recreation of 1998 Geocities/Netscape era,
//! built entirely in pure Rust + CSS. No JavaScript.
//!
//! Structure:
//!   1. The "90s Webpage" — authentically terrible, deliberately so
//!   2. The Punchline — "☝️ No JavaScript doesn't have to look like this."

use maud::{html, Markup};

/// Render the retro hero section - 90s Geocities style with punchline
pub fn render() -> Markup {
    html! {
        section class="sh-retro-hero scanlines" {
            /* CRT vignette overlay */
            div class="sh-retro-hero__crt" aria-hidden="true" {}

            div class="sh-retro-hero__content" {
                /* Under construction GIF */
                div class="sh-retro-hero__construction" {
                    img
                        src="https://web.archive.org/web/2009/http://www.geocities.com/SouthBeach/Marina/4942/underconstruction.gif"
                        alt="Under construction"
                        class="sh-retro-hero__gif"
                        loading="lazy";
                }

                /* Marquee - 90s style */
                div class="sh-retro-hero__marquee-container" {
                    div class="sh-retro-hero__marquee" {
                        span {
                            "⭐ Welcome to my homepage! ⭐ Best viewed in Netscape Navigator 4.0 at 800x600 ⭐ You are visitor #000,042 ⭐ This site is Y2K compliant ⭐ Sign my guestbook! ⭐"
                        }
                    }
                }

                hr class="sh-retro-hero__divider";

                /* Main heading - Comic Sans */
                h1 class="sh-retro-hero__title" {
                    "🚧 RustUI Component Library 🚧"
                }

                p class="sh-retro-hero__subtitle" {
                    i { "The " } b { "BEST" } " no-JavaScript UI framework!"
                }

                /* Classic 90s announcement table */
                table class="sh-retro-hero__table" role="presentation" {
                    tbody {
                        tr {
                            td class="sh-retro-hero__table-cell sh-retro-hero__table-cell--yellow" {
                                b { "📢 NEW!" } " v0.1.0 released!"
                            }
                        }
                        tr {
                            td class="sh-retro-hero__table-cell" {
                                a href="#showcase" class="sh-retro-hero__link" { "Click here" } " to download (56k modem friendly)"
                            }
                        }
                    }
                }

                /* Badge row - shields.io style */
                div class="sh-retro-hero__badges" {
                    img src="https://img.shields.io/badge/JavaScript-NONE-red?style=flat-square" alt="No JS" class="sh-retro-hero__badge" loading="lazy";
                    img src="https://img.shields.io/badge/Rust-🦀-orange?style=flat-square" alt="Rust" class="sh-retro-hero__badge" loading="lazy";
                    img src="https://img.shields.io/badge/bundle_size-0kb_JS-brightgreen?style=flat-square" alt="0kb" class="sh-retro-hero__badge" loading="lazy";
                }

                /* Classic 90s links */
                p class="sh-retro-hero__links" {
                    a href="#showcase" class="sh-retro-hero__link" { "Sign my guestbook!" }
                    " | "
                    a href="https://github.com/shallot-rs/shallot" target="_blank" rel="noopener noreferrer" class="sh-retro-hero__link" { "Webring" }
                    " | "
                    a href="https://github.com/shallot-rs/shallot" target="_blank" rel="noopener noreferrer" class="sh-retro-hero__link" { "Links" }
                }

                hr class="sh-retro-hero__divider";

                /* Footer - Times New Roman */
                p class="sh-retro-hero__footer" {
                    "© 1998-2026"
                    " | "
                    "Made with " code { "<table>" } " and love"
                }

                hr class="sh-retro-hero__divider";
            }
        }

        /* THE PUNCHLINE */
        div class="sh-retro-punchline" {
            p class="sh-retro-punchline__text" {
                "☝️ No JavaScript doesn't have to look like this."
            }
            p class="sh-retro-punchline__arrow" aria-hidden="true" {
                "👇"
            }
        }
    }
}

/// Generate retro hero CSS - 90s Geocities style
pub fn retro_css() -> String {
    r#"
/* ============================================
   RETRO HERO - 90s Geocities Style
   ============================================ */

.sh-retro-hero {
    position: relative;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    /* Classic Windows 95 grey background */
    background: #c0c0c0;
    overflow: hidden;
    font-family: "Comic Sans MS", "Chalkboard SE", "Comic Neue", sans-serif;
    color: #000;
}

/* Scanlines overlay */
.sh-retro-hero.scanlines::before {
    content: '';
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
        0deg,
        rgba(0, 0, 0, 0.03) 0px,
        rgba(0, 0, 0, 0.03) 1px,
        transparent 1px,
        transparent 2px
    );
    pointer-events: none;
    z-index: 1;
}

/* CRT vignette */
.sh-retro-hero__crt {
    position: absolute;
    inset: 0;
    background: radial-gradient(ellipse at center, transparent 50%, rgba(0,0,0,0.15) 100%);
    pointer-events: none;
    z-index: 2;
}

.sh-retro-hero__content {
    position: relative;
    z-index: 10;
    max-width: 42rem;
    width: 100%;
    text-align: center;
    background: #c0c0c0;
    padding: 1rem;
}

/* Under construction GIF */
.sh-retro-hero__construction {
    margin-bottom: 0.5rem;
}

.sh-retro-hero__gif {
    height: 3rem;
    width: auto;
    image-rendering: pixelated;
}

/* Marquee - classic 90s style */
.sh-retro-hero__marquee-container {
    overflow: hidden;
    margin-bottom: 0.5rem;
    background: #000080;
    color: #fff;
    padding: 0.25rem 0;
    border-top: 2px solid #000;
    border-bottom: 2px solid #000;
}

.sh-retro-hero__marquee {
    display: inline-block;
    white-space: nowrap;
    animation: sh-retro-marquee 15s linear infinite;
    font-family: "Courier New", Courier, monospace;
    font-size: 0.75rem;
}

@keyframes sh-retro-marquee {
    0% { transform: translateX(100%); }
    100% { transform: translateX(-100%); }
}

/* Classic HR divider */
.sh-retro-hero__divider {
    border: none;
    border-top: 2px groove #fff;
    border-bottom: 1px solid #808080;
    margin: 0.75rem auto;
    width: 90%;
}

/* Title - big and bold */
.sh-retro-hero__title {
    font-size: clamp(1.5rem, 5vw, 2.25rem);
    margin: 0.5rem 0;
    font-weight: bold;
    color: #000080;
    text-shadow: 1px 1px 0 #fff;
}

/* Subtitle */
.sh-retro-hero__subtitle {
    font-size: 1rem;
    margin: 0 0 1rem 0;
    color: #000;
}

/* Classic 90s table */
.sh-retro-hero__table {
    margin: 1rem auto;
    border: 2px solid #000;
    border-collapse: collapse;
    width: auto;
    display: inline-block;
}

.sh-retro-hero__table-cell {
    border: 1px solid #000;
    padding: 0.5rem 1rem;
    background: #c0c0c0;
    font-size: 0.9rem;
}

.sh-retro-hero__table-cell--yellow {
    background: #ffff00;
    color: #000;
    font-weight: bold;
}

.sh-retro-hero__link {
    color: #0000ff;
    text-decoration: underline;
}

.sh-retro-hero__link:visited {
    color: #551a8b;
}

.sh-retro-hero__link:hover {
    color: #ff0000;
    cursor: pointer;
}

/* Badge row */
.sh-retro-hero__badges {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin: 1rem 0;
    flex-wrap: wrap;
}

.sh-retro-hero__badge {
    height: 1.5rem;
    width: auto;
}

/* Links row */
.sh-retro-hero__links {
    font-size: 0.85rem;
    margin: 0.75rem 0;
}

/* Footer */
.sh-retro-hero__footer {
    font-size: 0.75rem;
    font-family: "Times New Roman", Times, serif;
    margin: 0.5rem 0;
    color: #000;
}

/* PUNCHLINE SECTION */
.sh-retro-punchline {
    text-align: center;
    padding: 2rem 1rem;
    background: linear-gradient(180deg, #f8fafc 0%, #e2e8f0 100%);
    position: relative;
}

.sh-retro-punchline__text {
    font-size: clamp(1.25rem, 4vw, 1.75rem);
    font-weight: bold;
    font-family: "Comic Sans MS", "Chalkboard SE", sans-serif;
    color: #1e293b;
    animation: sh-punchline-fade 0.8s ease-out forwards;
    opacity: 0;
    transform: translateY(20px);
}

@keyframes sh-punchline-fade {
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.sh-retro-punchline__arrow {
    font-size: 2.5rem;
    margin-top: 1.5rem;
    animation: sh-bounce 1s ease-in-out infinite;
    display: block;
}

@keyframes sh-bounce {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(10px); }
}

/* Responsive */
@media (max-width: 48rem) {
    .sh-retro-hero__content {
        padding: 0.5rem;
    }
    .sh-retro-hero__title {
        font-size: 1.25rem;
    }
    .sh-retro-hero__badges {
        gap: 0.25rem;
    }
    .sh-retro-hero__badge {
        height: 1.25rem;
    }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-retro-hero__marquee { animation: none; }
    .sh-retro-punchline__text { animation: none; opacity: 1; transform: none; }
    .sh-retro-punchline__arrow { animation: none; }
}
"#
    .to_string()
}
