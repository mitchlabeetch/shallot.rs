//! RSS Feed Generator
//!
//! Generates an Atom/RSS feed for component updates.

use maud::{html, Markup};

/// RSS feed entry
pub struct FeedEntry {
    pub title: String,
    pub link: String,
    pub published: String,
    pub summary: String,
}

/// Generate RSS feed
pub fn generate_rss() -> Markup {
    let entries = vec![
        FeedEntry {
            title: "Shallot v1.0 Released - 129 COMPLETE Components!".to_string(),
            link: "https://shallot.rs/#showcase".to_string(),
            published: "2026-02-24T00:00:00Z".to_string(),
            summary: "All 129 components are now COMPLETE with CSS, tests, and a11y!".to_string(),
        },
        FeedEntry {
            title: "New Component: ZStack".to_string(),
            link: "https://shallot.rs/#layout".to_string(),
            published: "2026-02-23T00:00:00Z".to_string(),
            summary: "Layered overlay layout component with z-index control.".to_string(),
        },
        FeedEntry {
            title: "New Component: CapDrop".to_string(),
            link: "https://shallot.rs/#typography".to_string(),
            published: "2026-02-23T00:00:00Z".to_string(),
            summary: "Decorative drop cap for elegant initial letter styling.".to_string(),
        },
        FeedEntry {
            title: "New Component: LiquidButton".to_string(),
            link: "https://shallot.rs/#animated".to_string(),
            published: "2026-02-23T00:00:00Z".to_string(),
            summary: "SVG filter warp button with liquid animation effect.".to_string(),
        },
        FeedEntry {
            title: "Website Launch".to_string(),
            link: "https://shallot.rs/".to_string(),
            published: "2026-02-22T00:00:00Z".to_string(),
            summary: "Official website launched with interactive component showcase.".to_string(),
        },
    ];

    html! {
        (maud::PreEscaped(r#"<?xml version="1.0" encoding="UTF-8"?>"#))
        feed xmlns="http://www.w3.org/2005/Atom" {
            title { "Shallot Component Library - Updates" }
            link href="https://shallot.rs/" rel="alternate";
            link href="https://shallot.rs/feed.xml" rel="self";
            id { "urn:uuid:shallot-rs" }
            updated { "2026-02-24T00:00:00Z" }
            author {
                name { "Shallot Team" }
            }
            subtitle { "Zero-JS Rust UI Component Library" }

            @for entry in entries {
                entry {
                    title { (entry.title) }
                    link href=(entry.link) rel="alternate";
                    id { (entry.link) }
                    published { (entry.published) }
                    summary { (entry.summary) }
                }
            }
        }
    }
}

/// Generate RSS feed as string
pub fn rss_string() -> String {
    generate_rss().into_string()
}
