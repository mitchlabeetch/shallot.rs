//! Retro Hero Section - 90s Web Aesthetic
//!
//! A humorous homage to the early web era.
//! Pure CSS animations, no JavaScript required.

use maud::{html, Markup};

/// Render the retro hero section
pub fn render() -> Markup {
    html! {
        section class="sh-retro-hero" {
            /* Scanlines overlay */
            div class="sh-retro-hero__scanlines" aria-hidden="true" {}

            /* CRT flicker effect */
            div class="sh-retro-hero__crt" aria-hidden="true" {}

            div class="sh-retro-hero__content" {
                /* Under construction GIF placeholder */
                div class="sh-retro-hero__construction" {
                    img
                        src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='200' height='50'%3E%3Crect fill='%23ffcc00' width='200' height='50'/%3E%3Ctext x='50%25' y='50%25' dominant-baseline='middle' text-anchor='middle' font-family='Arial' font-size='14' fill='%23000'%3E🚧 UNDER CONSTRUCTION 🚧%3C/text%3E%3C/svg%3E"
                        alt="Under construction"
                        class="sh-retro-hero__gif"
                    ;
                }

                /* Marquee-style scrolling text */
                div class="sh-retro-hero__marquee-container" {
                    div class="sh-retro-hero__marquee" {
                        span {
                            "⭐ Welcome to my homepage! ⭐ Best viewed in Netscape Navigator 4.0 at 800x600 ⭐ You are visitor #000,042 ⭐ Now with 100% more Rust! ⭐"
                        }
                    }
                }

                hr class="sh-retro-hero__divider" aria-hidden="true";

                /* Main heading with Comic Sans */
                h1 class="sh-retro-hero__title" {
                    "🚧 RustUI Component Library 🚧"
                }

                p class="sh-retro-hero__subtitle" {
                    i {
                        "The "
                        b { "BEST" }
                        " no-JavaScript UI framework!"
                    }
                }

                /* Retro table layout */
                table class="sh-retro-hero__table" cellpadding="8" {
                    tbody {
                        tr {
                            td class="sh-retro-hero__table-cell sh-retro-hero__table-cell--highlight" {
                                b { "📢 NEW!" }
                                " v1.0 released with 129 COMPLETE components!"
                            }
                        }
                        tr {
                            td class="sh-retro-hero__table-cell" {
                                a href="#showcase" { "👇 Scroll down to see the magic" }
                                " (56k modem friendly)"
                            }
                        }
                    }
                }

                /* Badge row */
                div class="sh-retro-hero__badges" {
                    div class="sh-retro-hero__badge" {
                        span class="sh-retro-hero__badge-text sh-retro-hero__badge-text--red" {
                            "JavaScript: NONE"
                        }
                    }
                    div class="sh-retro-hero__badge" {
                        span class="sh-retro-hero__badge-text sh-retro-hero__badge-text--orange" {
                            "Rust: 🦀"
                        }
                    }
                    div class="sh-retro-hero__badge" {
                        span class="sh-retro-hero__badge-text sh-retro-hero__badge-text--green" {
                            "Bundle: 0kb JS"
                        }
                    }
                    div class="sh-retro-hero__badge" {
                        span class="sh-retro-hero__badge-text sh-retro-hero__badge-text--blue" {
                            "Tor: ✓ Compatible"
                        }
                    }
                }

                /* Navigation links */
                p class="sh-retro-hero__links" {
                    a href="#showcase" { "📖 Components" }
                    " | "
                    a href="#themes" { "🎨 Themes" }
                    " | "
<<<<<<< HEAD
                    a href="https://github.com/shallot-rs/shallot" target="_blank" rel="noopener noreferrer" {
                        "💾 GitHub"
                        span class="sh-visually-hidden" { " (opens in new tab)" }
                        span aria-hidden="true" { " ↗" }
                    }
=======
                    a href="https://github.com/shallot-rs/shallot" { "💾 GitHub" }
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
                }

                hr class="sh-retro-hero__divider" aria-hidden="true";

                /* Footer of retro section */
                div class="sh-retro-hero__footer" {
                    span { "© 1998-2026" }
                    span aria-hidden="true" { " | " }
                    span { "Made with " b { "&lt;table&gt;" } " and love" }
                }

                /* The punchline */
                div class="sh-retro-hero__punchline" {
                    p class="sh-retro-hero__punchline-text" {
                        "☝️ No JavaScript doesn't have to look like this."
                    }

                    /* Animated arrow pointing down */
                    div class="sh-retro-hero__arrow" aria-hidden="true" {
                        "👇"
                    }
                }
            }
        }
    }
}

/// Generate retro hero CSS
pub fn retro_css() -> String {
    r#"
/* ============================================
   RETRO HERO SECTION - 90s Web Aesthetic
   Pure CSS. Zero JavaScript. Maximum Nostalgia.
   ============================================ */

.sh-retro-hero {
    position: relative;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
<<<<<<< HEAD
    background: linear-gradient(180deg, #808080 0%, #9a9a9a 50%, #696969 100%);
=======
    background: linear-gradient(180deg, #000080 0%, #0000aa 50%, #000080 100%);
>>>>>>> bdf16c3 (Initial commit: Shallot.rs project)
    overflow: hidden;
    font-family: 'Times New Roman', Times, serif;
}

/* Scanlines effect */
.sh-retro-hero__scanlines {
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
        0deg,
        rgba(0, 0, 0, 0.15) 0px,
        rgba(0, 0, 0, 0.15) 1px,
        transparent 1px,
        transparent 2px
    );
    pointer-events: none;
    z-index: 10;
    animation: sh-scanline-scroll 8s linear infinite;
}

@keyframes sh-scanline-scroll {
    0% { transform: translateY(0); }
    100% { transform: translateY(4px); }
}

/* CRT flicker effect */
.sh-retro-hero__crt {
    position: absolute;
    inset: 0;
    background: radial-gradient(
        ellipse at center,
        transparent 0%,
        transparent 50%,
        rgba(0, 0, 0, 0.3) 100%
    );
    pointer-events: none;
    z-index: 11;
    animation: sh-crt-flicker 0.15s infinite;
}

@keyframes sh-crt-flicker {
    0% { opacity: 0.97; }
    50% { opacity: 1; }
    100% { opacity: 0.98; }
}

.sh-retro-hero__content {
    position: relative;
    z-index: 20;
    max-width: 42rem;
    width: 100%;
    text-align: center;
    color: #00ff00;
    text-shadow: 0 0 10px #00ff00, 0 0 20px #00ff00;
}

/* Under construction GIF */
.sh-retro-hero__construction {
    margin-bottom: 1rem;
}

.sh-retro-hero__gif {
    height: 3rem;
    margin: 0 auto;
    display: block;
    image-rendering: pixelated;
}

/* Marquee scrolling text */
.sh-retro-hero__marquee-container {
    overflow: hidden;
    margin-bottom: 1rem;
    background: #000;
    border: 2px inset #808080;
    padding: 0.5rem;
}

.sh-retro-hero__marquee {
    display: inline-block;
    white-space: nowrap;
    animation: sh-marquee 15s linear infinite;
    font-family: 'Courier New', Courier, monospace;
    font-size: 0.875rem;
    color: #ffff00;
}

@keyframes sh-marquee {
    0% { transform: translateX(100%); }
    100% { transform: translateX(-100%); }
}

/* Divider */
.sh-retro-hero__divider {
    border: none;
    border-top: 3px ridge #808080;
    border-bottom: 3px ridge #ffffff;
    margin: 1.5rem 0;
}

/* Main title with Comic Sans */
.sh-retro-hero__title {
    font-family: 'Comic Sans MS', 'Comic Sans', cursive;
    font-size: clamp(1.75rem, 5vw, 2.5rem);
    font-weight: bold;
    margin-bottom: 0.5rem;
    color: #ffff00;
    text-shadow:
        3px 3px 0 #ff00ff,
        -3px -3px 0 #00ffff,
        0 0 20px #ffff00;
    animation: sh-title-rainbow 3s linear infinite;
}

@keyframes sh-title-rainbow {
    0%, 100% { filter: hue-rotate(0deg); }
    50% { filter: hue-rotate(180deg); }
}

/* Subtitle */
.sh-retro-hero__subtitle {
    font-size: 1.125rem;
    margin-bottom: 1.5rem;
    color: #00ffff;
}

.sh-retro-hero__subtitle b {
    color: #ff00ff;
    text-decoration: underline;
}

/* Retro table */
.sh-retro-hero__table {
    margin: 1.5rem auto;
    border: 3px outset #808080;
    background: #c0c0c0;
    color: #000;
    text-shadow: none;
    font-family: Arial, sans-serif;
    font-size: 0.875rem;
}

.sh-retro-hero__table-cell {
    border: 2px inset #ffffff;
    padding: 0.75rem;
    text-align: left;
}

.sh-retro-hero__table-cell--highlight {
    background: #ffff00;
    font-weight: bold;
}

.sh-retro-hero__table a {
    color: #0000ff;
    text-decoration: underline;
}

.sh-retro-hero__table a:hover {
    color: #ff0000;
}

/* Badges */
.sh-retro-hero__badges {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.5rem;
    margin: 1.5rem 0;
}

.sh-retro-hero__badge {
    display: inline-block;
    background: #000;
    border: 2px outset #808080;
    padding: 0.25rem 0.75rem;
}

.sh-retro-hero__badge-text {
    font-family: 'Courier New', Courier, monospace;
    font-size: 0.75rem;
    font-weight: bold;
}

.sh-retro-hero__badge-text--red { color: #ff0000; }
.sh-retro-hero__badge-text--orange { color: #ffa500; }
.sh-retro-hero__badge-text--green { color: #00ff00; }
.sh-retro-hero__badge-text--blue { color: #0000ff; }

/* Navigation links */
.sh-retro-hero__links {
    font-size: 0.875rem;
    margin: 1.5rem 0;
}

.sh-retro-hero__links a {
    color: #00ffff;
    text-decoration: underline;
}

.sh-retro-hero__links a:hover {
    color: #ffff00;
    text-decoration: none;
}

/* Retro footer */
.sh-retro-hero__footer {
    font-size: 0.75rem;
    color: #808080;
    margin-top: 1rem;
}

/* Punchline */
.sh-retro-hero__punchline {
    margin-top: 3rem;
    padding-top: 2rem;
    border-top: 2px dashed #808080;
}

.sh-retro-hero__punchline-text {
    font-family: 'Comic Sans MS', 'Comic Sans', cursive;
    font-size: clamp(1.25rem, 3vw, 1.5rem);
    font-weight: bold;
    color: #ffffff;
    text-shadow: 0 0 10px #ffffff;
    margin-bottom: 1rem;
}

/* Bouncing arrow */
.sh-retro-hero__arrow {
    font-size: 2.5rem;
    animation: sh-bounce 1s ease-in-out infinite;
}

@keyframes sh-bounce {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(10px); }
}

/* Responsive adjustments */
@media (max-width: 40rem) {
    .sh-retro-hero__table {
        font-size: 0.75rem;
    }

    .sh-retro-hero__badges {
        flex-direction: column;
        align-items: center;
    }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-retro-hero__scanlines,
    .sh-retro-hero__crt,
    .sh-retro-hero__marquee,
    .sh-retro-hero__title,
    .sh-retro-hero__arrow {
        animation: none;
    }

    .sh-retro-hero__crt {
        opacity: 0.97;
    }
}
"#
    .to_string()
}
