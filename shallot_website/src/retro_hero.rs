//! Retro Hero Section - 90s Web Aesthetic (updated)
//!
//! Changes in this version:
//! 1) Hero background changed to a plain, lighter grey (no gradient).
//! 2) Added global CSS fallbacks to ensure code blocks are visible by default
//!    (addresses cases where the CSS-only tab selector structure prevents
//!    code panels from being shown because radio inputs are nested).
//! 3) Improved placeholder styling for component previews so the "Live preview"
//!    placeholder is clearer and includes a subtle hint when an actual preview
//!    is not rendered server-side.
//!
//! Notes on the "audit" findings (also represented below as comments in CSS):
//! - Many components displayed the placeholder because the server-side showcase
//!   render function falls back to a placeholder when a concrete component
//!   renderer is not matched/imported. That is a logic/templating issue in
//!   `showcase.rs` (no runtime JS involved).
//! - Code snippets were present in the HTML but not visible because the CSS
//!   selector used to show/hide them assumes the radio inputs are siblings of
//!   the code blocks. In the generated markup the inputs are nested inside a
//!   `.sh-code-tabs` element while the `<pre>` code blocks are later siblings of
//!   that element, which breaks the intended sibling selector. Two practical
//!   non-destructive fixes:
//!     a) Move the radio inputs out to be direct siblings of the code blocks
//!        (change in `showcase.rs`) — the most correct fix.
//!     b) Provide a CSS fallback that shows code blocks by default (done here)
//!        so the site is usable even without changing markup structure.
//!
//! If you want, I can open a follow-up edit that moves radio inputs in the
//! `showcase.rs` markup to make the CSS tab-toggle pattern reliable and still
//! keep the CSS fallback as a safety net.

use maud::{html, Markup};

/// Render the retro hero section
pub fn render() -> Markup {
    html! {
        section class="sh-retro-hero" {
            /* Scanlines overlay - decorative */
            div class="sh-retro-hero__scanlines" aria-hidden="true" {}

            /* CRT flicker effect - decorative */
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
                            "⭐ Welcome to the Shallot showcase — elegant Rust UI components, zero JS. ⭐"
                        }
                    }
                }

                hr class="sh-retro-hero__divider" aria-hidden="true";

                /* Main heading */
                h1 class="sh-retro-hero__title" {
                    "Shallot — Zero-JS Rust UI Components"
                }

                p class="sh-retro-hero__subtitle" {
                    i {
                        "Beautiful, accessible UI components built in Rust and CSS (no client JS)."
                    }
                }

                /* Small CTA row */
                div class="sh-retro-hero__cta" {
                    a href="#showcase" class="sh-retro-hero__cta-link" { "Browse components" }
                    " "
                    a href="https://github.com/shallot-rs/shallot" target="_blank" rel="noopener noreferrer" {
                        "View on GitHub"
                    }
                }
            }
        }
    }
}

/// Generate retro hero CSS + a few global fallbacks for showcase usability.
///
/// Important: this file deliberately includes a small set of global fallbacks
/// (below the hero styles) which are safe, non-destructive, and ensure the
/// static prebuilt `output/` site remains usable in browsers even when the
/// markup structure for the code-tabs is not ideal. The correct long-term fix
/// is to update the `showcase.rs` markup so radio inputs are siblings of the
/// code blocks.
pub fn retro_css() -> String {
    r#"
/* ============================================
   RETRO HERO - updated background + fallbacks
   ============================================ */

.sh-retro-hero {
    position: relative;
    min-height: 60vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 3rem 1rem;
    /* Changed: plain lighter grey background (no gradient) */
    background: #f5f7fa; /* light neutral grey, good contrast with retro styles */
    overflow: hidden;
    font-family: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial;
    color: #222;
}

/* Subtle scanlines and CRT effects kept but toned down to be unobtrusive */
.sh-retro-hero__scanlines {
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
        0deg,
        rgba(0, 0, 0, 0.03) 0px,
        rgba(0, 0, 0, 0.03) 1px,
        transparent 1px,
        transparent 3px
    );
    pointer-events: none;
    z-index: 0;
}

.sh-retro-hero__crt {
    position: absolute;
    inset: 0;
    background: radial-gradient(
        ellipse at center,
        rgba(255,255,255,0.02) 0%,
        rgba(0,0,0,0.04) 100%
    );
    pointer-events: none;
    z-index: 1;
}

/* Content sits above decorations */
.sh-retro-hero__content {
    position: relative;
    z-index: 10;
    max-width: 72rem;
    width: 100%;
    text-align: center;
    color: #0f1724;
    padding: 2rem;
    box-sizing: border-box;
}

.sh-retro-hero__title {
    font-size: clamp(1.75rem, 4vw, 2.5rem);
    margin: 0 0 0.5rem 0;
    font-weight: 700;
    color: #0b1220;
}

.sh-retro-hero__subtitle {
    color: #374151;
    margin: 0 0 1.25rem 0;
}

.sh-retro-hero__marquee-container {
    overflow: hidden;
    margin-bottom: 1rem;
    background: transparent;
}

.sh-retro-hero__marquee {
    display: inline-block;
    white-space: nowrap;
    animation: sh-retro-marquee 18s linear infinite;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, "Roboto Mono", "Courier New", monospace;
    font-size: 0.9rem;
    color: #6b7280;
}

@keyframes sh-retro-marquee {
    0% { transform: translateX(100%); }
    100% { transform: translateX(-100%); }
}

.sh-retro-hero__divider {
    border: none;
    border-top: 1px solid rgba(15,23,36,0.06);
    margin: 1.25rem auto;
    width: 60%;
}

.sh-retro-hero__construction { margin-bottom: 0.75rem; }
.sh-retro-hero__gif { height: 2.5rem; display:block; margin: 0 auto; }

.sh-retro-hero__cta {
    margin-top: 1rem;
}

.sh-retro-hero__cta-link {
    display: inline-block;
    padding: 0.6rem 1rem;
    background: #0b74ff;
    color: #fff;
    border-radius: 8px;
    text-decoration: none;
    font-weight: 600;
    margin-right: 0.5rem;
}

.sh-retro-hero__cta-link:hover {
    background: #095ec1;
}

/* Reduced motion respects user preference */
@media (prefers-reduced-motion: reduce) {
    .sh-retro-hero__marquee { animation: none; }
}

/* ============================================
   Global fallbacks & showcase improvements
   (safe, non-invasive CSS to improve UX)
   ============================================ */

/* Improve "Live preview" placeholder visuals so missing previews are clearer */
.sh-component-card__placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    color: #6b7280;
    font-size: 0.95rem;
    font-style: italic;
    padding: 1rem;
    background: linear-gradient(180deg, rgba(15,23,36,0.02), rgba(15,23,36,0.01));
    border-radius: 8px;
    border: 1px dashed rgba(15,23,36,0.06);
}

/* Add a subtle hint inside the placeholder (visually only) */
.sh-component-card__placeholder::after {
    content: " — preview not rendered server-side";
    display: block;
    font-size: 0.75rem;
    color: #9ca3af;
    margin-top: 0.25rem;
    font-style: normal;
}

/* Make code blocks visible by default as a robust fallback.
   Root cause: the CSS-only tab switching assumes radio inputs are siblings
   of code panels. In the generated markup the inputs were nested (inside
   `.sh-code-tabs`) and the code panels were separate siblings of that
   container, which broke the sibling selector used to toggle visibility.

   The correct fix is to move the radio inputs so they are siblings of the
   code panels (edit `showcase.rs`). As a non-destructive, backwards
   compatible remediation we show code blocks by default and keep tab
   styling behavior intact where it still works. */
.sh-code-block {
    display: block !important;
    background: #0f1724;
    color: #e6eef8;
    padding: 1rem;
    border-radius: 8px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, "Roboto Mono", "Courier New", monospace;
    font-size: 0.82rem;
    overflow-x: auto;
    line-height: 1.45;
    margin-bottom: 0.75rem;
}

/* Improve readability for download button in code area */
.sh-code-download {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.75rem;
    background: #111827;
    color: #f8fafc;
    border-radius: 6px;
    text-decoration: none;
    font-size: 0.81rem;
    margin-bottom: 0.75rem;
}

/* Ensure the code tab labels remain keyboard focusable and visible */
.sh-code-tab:focus {
    outline: 2px solid rgba(11,116,255,0.9);
    outline-offset: 2px;
}

/* Small responsive tweaks */
@media (max-width: 64rem) {
    .sh-retro-hero__content { padding: 1.25rem; }
    .sh-retro-hero__title { font-size: 1.5rem; }
}

/* End of CSS fallbacks and hero styles */
"#
    .to_string()
}
