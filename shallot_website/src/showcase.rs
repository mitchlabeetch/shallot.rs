//! Component Showcase - Interactive Component Gallery
//!
//! Displays all 129 components organized by category.
//! Each component has a preview, description, and expandable code view.

use crate::theme_marketplace;
use maud::{html, Markup, Render};
use shallot_components::{
    alert::{Alert, AlertKind},
    avatar::Avatar,
    badge::Badge,
    border_beam::BorderBeam,
    breadcrumbs::{BreadcrumbItem, Breadcrumbs},
    button::Button,
    capdrop::CapDrop,
    card::Card,
    confetti::Confetti,
    input::Input,
    liquid_button::LiquidButton,
    progress::ProgressBar,
    rating::Rating as StarRating,
    refractive_gauge::RefractiveGauge,
    shadow_elevator::{ShadowElevator, ShadowLevel},
    skeleton::Skeleton,
    timeline::{Timeline, TimelineItem},
};

/// Component category
pub struct ComponentCategory {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
}

/// All component categories
pub const CATEGORIES: &[ComponentCategory] = &[
    ComponentCategory {
        id: "layout",
        name: "Layout & Primitives",
        icon: "📐",
        description: "Foundation components for structural integrity and spacing",
    },
    ComponentCategory {
        id: "typography",
        name: "Typography",
        icon: "📝",
        description: "Fluid scaling and refractive text effects",
    },
    ComponentCategory {
        id: "forms",
        name: "Forms & Inputs",
        icon: "📋",
        description: "Interactive form elements with CSS-only validation feedback",
    },
    ComponentCategory {
        id: "navigation",
        name: "Navigation",
        icon: "🧭",
        description: "Deep-linkable navigation components",
    },
    ComponentCategory {
        id: "overlays",
        name: "Overlays & Feedback",
        icon: "🔔",
        description: "Glass-effect overlays and contextual feedback",
    },
    ComponentCategory {
        id: "data",
        name: "Data Display",
        icon: "📊",
        description: "High-performance information rendering",
    },
    ComponentCategory {
        id: "animated",
        name: "Signature Animated",
        icon: "✨",
        description: "Breathtaking CSS-only animations and effects",
    },
];

/// Sample components for showcase (representative from each category)
pub const SAMPLE_COMPONENTS: &[(&str, &str, &str, &str)] = &[
    // Layout
    (
        "Box",
        "layout",
        "Universal container primitive",
        "The fundamental building block",
    ),
    (
        "Grid",
        "layout",
        "Two-dimensional CSS Grid layout",
        "Responsive grid system",
    ),
    (
        "Masonry",
        "layout",
        "CSS-only masonry layout",
        "Pinterest-style columns",
    ),
    (
        "ZStack",
        "layout",
        "Layered overlay layout",
        "Stack elements with z-index",
    ),
    // Typography
    (
        "Heading",
        "typography",
        "Semantic heading levels",
        "H1-H6 with fluid sizing",
    ),
    (
        "GradientText",
        "typography",
        "Animated gradient text",
        "Multi-stop color aura",
    ),
    (
        "CapDrop",
        "typography",
        "Decorative drop cap",
        "Initial letter styling",
    ),
    // Forms
    (
        "Input",
        "forms",
        "Text input with variants",
        "Accessible form field",
    ),
    (
        "Select",
        "forms",
        "Custom styled dropdown",
        "GlassSelect component",
    ),
    (
        "MultiSelect",
        "forms",
        "Tag-based selection",
        "Multiple value picker",
    ),
    (
        "CreditCardInput",
        "forms",
        "Card input with preview",
        "Formatted card entry",
    ),
    (
        "ImageUpload",
        "forms",
        "Drag-drop image upload",
        "File upload interface",
    ),
    // Navigation
    (
        "Navbar",
        "navigation",
        "Top navigation bar",
        "Responsive header",
    ),
    (
        "Sidebar",
        "navigation",
        "Collapsible sidebar",
        "Drawer navigation",
    ),
    (
        "Breadcrumbs",
        "navigation",
        "Path navigation",
        "Hierarchical links",
    ),
    (
        "SiteMap",
        "navigation",
        "Hierarchical site map",
        "SEO-friendly navigation",
    ),
    (
        "TableOfContents",
        "navigation",
        "Auto-generated TOC",
        "Heading navigator",
    ),
    // Overlays
    ("Modal", "overlays", "Dialog overlay", "CSS-only modal"),
    ("Drawer", "overlays", "Slide-out panel", "Side drawer"),
    (
        "Toast",
        "overlays",
        "Notification toast",
        "Non-blocking alerts",
    ),
    ("Tooltip", "overlays", "Hover tooltip", "Contextual hint"),
    // Data Display
    ("Card", "data", "Universal container", "Content card"),
    ("Table", "data", "Data table", "Sortable table"),
    ("Avatar", "data", "User avatar", "Profile image"),
    (
        "Timeline",
        "data",
        "Chronological display",
        "Event timeline",
    ),
    (
        "VideoPlayer",
        "data",
        "Native video player",
        "HTML5 video wrapper",
    ),
    // Animated
    (
        "BorderBeam",
        "animated",
        "Animated border beam",
        "Traveling light effect",
    ),
    (
        "Confetti",
        "animated",
        "Celebration particles",
        "CSS confetti",
    ),
    (
        "LiquidButton",
        "animated",
        "SVG filter warp button",
        "Liquid effect",
    ),
    (
        "MeshGradient",
        "animated",
        "Animated mesh gradient",
        "Flowing background",
    ),
    (
        "RefractiveGauge",
        "animated",
        "CSS speedometer",
        "Gauge visualization",
    ),
    (
        "ShadowElevator",
        "animated",
        "Dynamic elevation",
        "Material shadows",
    ),
    (
        "MaskedImage",
        "animated",
        "Creative shape masks",
        "Image clipping",
    ),
];

/// Render a live preview for a component
fn render_preview(name: &str) -> Markup {
    match name {
        "Button" => html! {
            div style="display: flex; gap: 0.5rem; flex-wrap: wrap;" aria-label=(format!("{} component preview: Primary, Secondary, and Ghost button variants", name)) {
                (Button::new("Primary").render())
                (Button::new("Secondary").render())
                (Button::new("Ghost").render())
            }
        },
        "Badge" => html! {
            div style="display: flex; gap: 0.5rem; flex-wrap: wrap;" aria-label=(format!("{} component preview: New, Hot, and Free badge variants", name)) {
                (Badge::new("New").render())
                (Badge::new("Hot").render())
                (Badge::new("Free").render())
            }
        },
        "Skeleton" => html! {
            div style="display: flex; flex-direction: column; gap: 0.5rem;" aria-label=(format!("{} component preview: Three skeleton loading placeholders of varying widths", name)) {
                (Skeleton::new().width("200px").height("20px").render())
                (Skeleton::new().width("150px").height("15px").render())
                (Skeleton::new().width("100px").height("15px").render())
            }
        },
        "Alert" => html! {
            div style="display: flex; flex-direction: column; gap: 0.5rem;" aria-label=(format!("{} component preview: Info and Success alert variants", name)) {
                (Alert::new(AlertKind::Info, "Info", "This is an info message").render())
                (Alert::new(AlertKind::Success, "Success", "Operation completed!").render())
            }
        },
        "Input" => html! {
            div style="display: flex; flex-direction: column; gap: 0.5rem; max-width: 200px;" aria-label=(format!("{} component preview: Email input field with placeholder", name)) {
                (Input::new("email").placeholder("you@example.com").render())
            }
        },
        "Card" => html! {
            div style="max-width: 250px;" aria-label=(format!("{} component preview: Card with title and content", name)) {
                (Card::new(html! {
                    h3 { "Card Title" }
                    p { "Card content goes here." }
                }).render())
            }
        },
        "Avatar" => html! {
            div style="display: flex; gap: 0.5rem;" aria-label=(format!("{} component preview: User avatars for John Doe and Jane Smith with initials", name)) {
                (Avatar::new("John Doe").initials("JD").render())
                (Avatar::new("Jane Smith").initials("JS").render())
            }
        },
        "BorderBeam" => html! {
            div style="position: relative; width: 200px; height: 100px; background: #1a1a2e;" aria-label=(format!("{} component preview: Animated border beam effect on dark background", name)) {
                (BorderBeam::new().render())
                span style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; color: white;" {
                    "Border Beam"
                }
            }
        },
        "Confetti" => html! {
            div style="position: relative; width: 200px; height: 100px; overflow: hidden;" aria-label=(format!("{} component preview: Animated confetti celebration effect", name)) {
                (Confetti::new().render())
                span style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center;" {
                    "🎉"
                }
            }
        },
        "LiquidButton" => html! {
            div aria-label=(format!("{} component preview: Animated liquid warp effect", name)) {
                (LiquidButton::new("Liquid Effect").render())
            }
        },
        "RefractiveGauge" => html! {
            div style="width: 150px;" aria-label=(format!("{} component preview: Refractive CSS gauge", name)) {
                (RefractiveGauge::new(75.0).render())
            }
        },
        "ShadowElevator" => html! {
            div style="display: flex; gap: 1rem;" aria-label=(format!("{} component preview: Dynamic elevation shadows", name)) {
                (ShadowElevator::new(html! { "Elevated" }).level(ShadowLevel::Level4).render())
            }
        },
        "Timeline" => html! {
            div style="max-width: 200px;" aria-label=(format!("{} component preview: Chronological event list", name)) {
                (Timeline::new(vec![
                    TimelineItem::new("Step 1", "Started"),
                    TimelineItem::new("Step 2", "In progress"),
                ]).render())
            }
        },
        "CapDrop" => html! {
            div aria-label=(format!("{} component preview: Decorative drop cap", name)) {
                (CapDrop::new("S", "hallot brings iron logic to the web.").render())
            }
        },
        "Breadcrumbs" => html! {
            div aria-label=(format!("{} component preview: Path navigation links", name)) {
                (Breadcrumbs::new(vec![
                    BreadcrumbItem { label: "Home", href: Some("/") },
                    BreadcrumbItem { label: "Components", href: None },
                ]).render())
            }
        },
        "Rating" => html! {
            div aria-label=(format!("{} component preview: Star rating visualization", name)) {
                (StarRating::new(4).render())
            }
        },
        "Progress" => html! {
            div style="width: 200px;" aria-label=(format!("{} component preview: Visual progress bar", name)) {
                (ProgressBar::new(65).render())
            }
        },
        _ => html! {
            div class="sh-component-card__placeholder" aria-label=(format!("{} component preview placeholder — live preview coming soon", name)) {
                span { "Live preview" }
                span style="display: block; font-size: 0.75rem; color: #999; margin-top: 0.25rem;" {
                    "(coming soon)"
                }
            }
        },
    }
}

/// Render the showcase section
pub fn render() -> Markup {
    html! {
        main class="sh-showcase" id="showcase" {
            /* Search bar - CSS only using checkbox hack */
            div class="sh-search-container" {
                input type="checkbox" id="search-toggle" class="sh-search-toggle" ;
                label for="search-toggle" class="sh-search-label" {
                    span class="sh-search-icon" aria-hidden="true" { "🔍" }
                    span { "Search components..." }
                }
                div class="sh-search-results" {
                    @for category in CATEGORIES {
                        @for (name, cat, _description, _tagline) in SAMPLE_COMPONENTS {
                            @if cat == &category.id {
                                a href=(format!("#doc-{}", name.to_lowercase())) class="sh-search-result-item" data-name=(name.to_lowercase()) {
                                    span class="sh-search-result__name" { (name) }
                                    span class="sh-search-result__category" { (category.name) }
                                }
                            }
                        }
                    }
                }
            }

            /* Community themes section */
            (theme_marketplace::render())

            /* Category navigation */
            nav class="sh-categories" aria-label="Component categories" {
                ul class="sh-categories__list" role="tablist" {
                    @for category in CATEGORIES {
                        li class="sh-categories__item" {
                            a
                                href=(format!("#{}", category.id))
                                class="sh-category-link"
                                role="tab"
                            {
                                span aria-hidden="true" { (category.icon) }
                                " "
                                (category.name)
                            }
                        }
                    }
                }
            }

            /* Component sections */
            @for category in CATEGORIES {
                section id=(category.id) class="sh-component-section" {
                    div class="sh-container" {
                        h2 class="sh-component-section__title" {
                            span aria-hidden="true" { (category.icon) }
                            " "
                            (category.name)
                        }
                        p class="sh-component-section__description" {
                            (category.description)
                        }

                        div class="sh-component-grid" {
                            @for (name, cat, description, _tagline) in SAMPLE_COMPONENTS {
                                @if cat == &category.id {
                                    article class="sh-component-card" {
                                        div class="sh-component-card__preview" {
                                            /* Live component preview */
                                            (render_preview(name))
                                        }

                                        div class="sh-component-card__content" {
                                            h3 class="sh-component-card__title" {
                                                (name)
                                            }
                                            p class="sh-component-card__description" {
                                                (description)
                                            }

                                            /* Code dropdown */
                                            div class="sh-code-dropdown" {
                                                input
                                                    type="checkbox"
                                                    id=(format!("code-{}", name.to_lowercase()))
                                                    class="sh-code-dropdown__checkbox"
                                                ;
                                                label
                                                    for=(format!("code-{}", name.to_lowercase()))
                                                    class="sh-code-dropdown__toggle"
                                                {
                                                    span class="sh-visually-hidden" {
                                                        "View code for " (name) " component"
                                                    }
                                                    span aria-hidden="true" { "View code" }
                                                    span class="sh-code-dropdown__icon" aria-hidden="true" {
                                                        "▼"
                                                    }
                                                }

                                                div class="sh-code-dropdown__content" {
                                                    /* Download button */
                                                    a
                                                        href=(format!("data:text/plain;charset=utf-8,{}example%20code%20for%20{}", "%5B%20Example%20code%20%5D", name.to_lowercase()))
                                                        download=(format!("{}_example.rs", name.to_lowercase()))
                                                        class="sh-code-download"
                                                    {
                                                        span aria-hidden="true" { "📥" }
                                                        " Download"
                                                    }

                                                    /* Code tabs: moved radio inputs to be direct siblings of the code blocks
                                                       so the CSS sibling selectors (.sh-code-tab__radio[value="..."]:checked ~ .sh-code-block--...)
                                                       work reliably even when the original tab labels remain grouped visually. */
                                                    input
                                                        type="radio"
                                                        name=(format!("tabs-{}", name.to_lowercase()))
                                                        id=(format!("full-{}", name.to_lowercase()))
                                                        class="sh-code-tab__radio"
                                                        value="full"
                                                        checked
                                                    ;
                                                    label
                                                        for=(format!("full-{}", name.to_lowercase()))
                                                        class="sh-code-tab"
                                                        role="tab"
                                                        aria-selected="true"
                                                        tabindex="0"
                                                    {
                                                        "Full Code"
                                                    }

                                                    input
                                                        type="radio"
                                                        name=(format!("tabs-{}", name.to_lowercase()))
                                                        id=(format!("library-{}", name.to_lowercase()))
                                                        class="sh-code-tab__radio"
                                                        value="library"
                                                    ;
                                                    label
                                                        for=(format!("library-{}", name.to_lowercase()))
                                                        class="sh-code-tab"
                                                        role="tab"
                                                        aria-selected="false"
                                                        tabindex="-1"
                                                    {
                                                        "In Library"
                                                    }

                                                    /* Code blocks */
                                                    pre class="sh-code-block sh-code-block--full" {
                                                        code {
                                                            "// " (name) " Component\n"
                                                            "// Source: shallot_components/src/" (name.to_lowercase()) ".rs\n"
                                                            "\n"
                                                            "use maud::{html, Markup, Render};\n"
                                                            "use shallot_foundation::theme::Theme;\n"
                                                            "\n"
                                                            "pub struct " (name) " {\n"
                                                            "    pub text: String,\n"
                                                            "    pub variant: Option<String>,\n"
                                                            "}\n"
                                                            "\n"
                                                            "impl " (name) " {\n"
                                                            "    pub fn new(text: impl Into<String>) -> Self {\n"
                                                            "        Self {\n"
                                                            "            text: text.into(),\n"
                                                            "            variant: None,\n"
                                                            "        }\n"
                                                            "    }\n"
                                                            "\n"
                                                            "    pub fn variant(mut self, variant: impl Into<String>) -> Self {\n"
                                                            "        self.variant = Some(variant.into());\n"
                                                            "        self\n"
                                                            "    }\n"
                                                            "}\n"
                                                            "\n"
                                                            "impl Render for " (name) " {\n"
                                                            "    fn render(&self) -> Markup {\n"
                                                            "        html! {\n"
                                                            "            div class=(format!(\"sh-{}\", stringify!(" (name.to_lowercase()) ")) {\n"
                                                            "                (self.text)\n"
                                                            "            }\n"
                                                            "        }\n"
                                                            "    }\n"
                                                            "}"
                                                        }
                                                    }

                                                    pre class="sh-code-block sh-code-block--library" {
                                                        code {
                                                            "// 1. Add to your Cargo.toml:\n"
                                                            "// [dependencies]\n"
                                                            "// shallot_components = \"0.1\"\n"
                                                            "\n"
                                                            "// 2. Import the component:\n"
                                                            "use shallot_components::" (name.to_lowercase()) "::" (name) ";\n"
                                                            "\n"
                                                            "// 3. Use in your Maud template:\n"
                                                            "html! {\n"
                                                            "    (" (name) "::new(\"Example\").render())\n"
                                                            "}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate showcase CSS
pub fn showcase_css() -> String {
    r#"
/* ============================================
   COMPONENT SHOWCASE STYLES
   ============================================ */

.sh-showcase {
    background: var(--sh-surface);
}

/* Category Navigation */
/* Search Bar - CSS Only */
.sh-search-container {
    position: relative;
    max-width: 24rem;
    margin: 0 auto 2rem;
}

.sh-search-toggle {
    display: none;
}

.sh-search-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.875rem 1.25rem;
    background: var(--sh-surface);
    border: 2px solid var(--sh-border);
    border-radius: var(--sh-radius-full);
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--sh-text-muted);
}

.sh-search-label:hover {
    border-color: var(--sh-primary);
    color: var(--sh-text);
}

.sh-search-toggle:checked + .sh-search-label {
    border-color: var(--sh-primary);
    background: var(--sh-surface);
}

.sh-search-icon {
    font-size: 1.25rem;
}

.sh-search-results {
    display: none;
    position: absolute;
    top: calc(100% + 0.5rem);
    left: 0;
    right: 0;
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-lg);
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
    max-height: 24rem;
    overflow-y: auto;
    z-index: 1000;
    padding: 0.5rem;
}

.sh-search-toggle:checked ~ .sh-search-results {
    display: block;
}

.sh-search-result-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    border-radius: var(--sh-radius-md);
    text-decoration: none;
    color: var(--sh-text);
    transition: background 0.2s ease;
}

.sh-search-result-item:hover {
    background: var(--sh-surface-2);
}

.sh-search-result__name {
    font-weight: 600;
    color: var(--sh-primary);
}

.sh-search-result__category {
    font-size: 0.75rem;
    color: var(--sh-text-muted);
}

.sh-categories {
    position: sticky;
    top: 0;
    z-index: 100;
    background: var(--sh-surface);
    border-bottom: 1px solid var(--sh-border);
    padding: 1rem 0;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
}

.sh-categories__list {
    display: flex;
    gap: 0.5rem;
    overflow-x: auto;
    padding: 0 1rem;
    scrollbar-width: thin;
    list-style: none;
    margin: 0;
}

.sh-categories__item {
    flex-shrink: 0;
}

.sh-category-link {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--sh-surface-2);
    border-radius: var(--sh-radius-full);
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text-secondary);
    white-space: nowrap;
    transition: all 0.2s ease;
}

.sh-category-link:hover {
    background: var(--sh-primary-bg);
    color: var(--sh-primary);
}

/* Component Section */
.sh-component-section {
    padding: 4rem 0;
    scroll-margin-top: 5rem;
}

.sh-component-section__title {
    font-size: 1.75rem;
    margin-bottom: 0.5rem;
    color: var(--sh-text);
}

.sh-component-section__description {
    color: var(--sh-text-secondary);
    margin-bottom: 2rem;
    max-width: 48rem;
}

/* Component Grid */
.sh-component-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(22rem, 1fr));
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
    box-shadow: 0 15px 50px rgba(0, 0, 0, 0.1);
    transform: translateY(-4px);
}

.sh-component-card__preview {
    padding: 2rem;
    background: var(--sh-surface-2);
    border-bottom: 1px solid var(--sh-border);
    min-height: 10rem;
    display: flex;
    align-items: center;
    justify-content: center;
}

.sh-component-card__placeholder {
    text-align: center;
    color: var(--sh-text-muted);
    font-size: 0.875rem;
    font-style: italic;
}

.sh-component-card__content {
    padding: 1.5rem;
}

.sh-component-card__title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--sh-text);
    margin-bottom: 0.5rem;
}

.sh-component-card__description {
    font-size: 0.875rem;
    color: var(--sh-text-secondary);
    margin-bottom: 1rem;
    line-height: 1.5;
}

/* Code Dropdown */
.sh-code-dropdown {
    border-top: 1px solid var(--sh-border);
    margin-top: 1rem;
}

.sh-code-dropdown__checkbox {
    display: none;
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
    font-weight: 600;
    color: var(--sh-primary);
    cursor: pointer;
    transition: background 0.2s ease;
}

.sh-code-dropdown__toggle:hover {
    background: var(--sh-surface-2);
}

.sh-code-dropdown__icon {
    transition: transform 0.3s ease;
    font-size: 0.75rem;
}

.sh-code-dropdown__checkbox:checked ~ .sh-code-dropdown__toggle .sh-code-dropdown__icon {
    transform: rotate(180deg);
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

/* Code Download Button */
.sh-code-download {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--sh-primary);
    color: white;
    border-radius: var(--sh-radius-md);
    text-decoration: none;
    font-size: 0.75rem;
    font-weight: 500;
    margin-bottom: 1rem;
    transition: all 0.2s ease;
}

.sh-code-download:hover {
    background: var(--sh-primary-hover);
    transform: translateY(-1px);
}


/* Code Tabs */
.sh-code-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
}

.sh-code-tab__radio {
    display: none;
}

.sh-code-tab {
    padding: 0.5rem 1rem;
    background: var(--sh-surface);
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

.sh-code-tab:focus {
    outline: 2px solid var(--sh-primary);
    outline-offset: 2px;
}

.sh-code-tab:focus-visible {
    outline: 2px solid var(--sh-primary);
    outline-offset: 2px;
}

.sh-code-tab__radio:checked + .sh-code-tab {
    background: var(--sh-primary);
    color: white;
    border-color: var(--sh-primary);
}

/* Code Blocks */
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
    line-height: 1.5;
}

.sh-code-tab__radio[value="full"]:checked ~ .sh-code-block--full {
    display: block;
}

.sh-code-tab__radio[value="library"]:checked ~ .sh-code-block--library {
    display: block;
}

/* Responsive */
@media (max-width: 64rem) {
    .sh-component-grid {
        grid-template-columns: 1fr;
    }

    .sh-categories__list {
        padding: 0;
    }
}

/* Print Styles */
@media print {
    .sh-theme-switcher,
    .sh-categories,
    .sh-code-dropdown,
    .sh-site-footer {
        display: none !important;
    }

    .sh-component-card {
        break-inside: avoid;
        page-break-inside: avoid;
        border: 1px solid #000;
        margin-bottom: 1rem;
    }

    .sh-component-card__preview {
        background: #fff !important;
        border: 1px solid #000;
    }

    .sh-component-section {
        padding: 1rem 0;
    }

    .sh-component-section__title {
        font-size: 1.25rem;
        margin-top: 1.5rem;
        page-break-after: avoid;
    }

    body {
        background: #fff;
        color: #000;
        font-size: 10pt;
    }

    a {
        color: #000;
        text-decoration: underline;
    }

    .sh-container {
        max-width: 100%;
        padding: 0;
    }
}
"#
    .to_string()
}
