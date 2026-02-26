//! Component Documentation - Detailed Component Pages
//!
//! Generates individual documentation pages for each component.

use maud::{html, Markup};

/// Component documentation structure
pub struct ComponentDoc {
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub full_code: &'static str,
    pub library_code: &'static str,
}

/// Render a single component documentation page
pub fn render_component_doc(doc: &ComponentDoc) -> Markup {
    html! {
        article class="sh-component-doc" id=(format!("doc-{}", doc.name.to_lowercase())) {
            header class="sh-component-doc__header" {
                h2 class="sh-component-doc__title" {
                    (doc.name)
                }
                p class="sh-component-doc__category" {
                    "Category: " (doc.category)
                }
            }

            div class="sh-component-doc__body" {
                p class="sh-component-doc__description" {
                    (doc.description)
                }

                /* Live preview placeholder */
                div class="sh-component-doc__preview" {
                    p { "[Live component preview would render here]" }
                }

                /* Code section */
                section class="sh-component-doc__code" {
                    h3 { "Implementation" }

                    div class="sh-code-tabs" {
                        input
                            type="radio"
                            name=(format!("doc-tabs-{}", doc.name.to_lowercase()))
                            id=(format!("doc-full-{}", doc.name.to_lowercase()))
                            class="sh-code-tab__radio"
                            checked
                        ;
                        label
                            for=(format!("doc-full-{}", doc.name.to_lowercase()))
                            class="sh-code-tab"
                        {
                            "Full Code"
                        }

                        input
                            type="radio"
                            name=(format!("doc-tabs-{}", doc.name.to_lowercase()))
                            id=(format!("doc-library-{}", doc.name.to_lowercase()))
                            class="sh-code-tab__radio"
                        ;
                        label
                            for=(format!("doc-library-{}", doc.name.to_lowercase()))
                            class="sh-code-tab"
                        {
                            "Library Usage"
                        }
                    }

                    pre class="sh-code-block sh-code-block--full" {
                        code { (doc.full_code) }
                    }

                    pre class="sh-code-block sh-code-block--library" {
                        code { (doc.library_code) }
                    }
                }
            }
        }
    }
}

/// Generate component docs CSS
pub fn component_docs_css() -> String {
    r#"
/* ============================================
   COMPONENT DOCUMENTATION STYLES
   ============================================ */

.sh-component-doc {
    background: var(--sh-surface);
    border: 1px solid var(--sh-border);
    border-radius: var(--sh-radius-lg);
    overflow: hidden;
    margin-bottom: 2rem;
}

.sh-component-doc__header {
    padding: 1.5rem 2rem;
    background: var(--sh-surface-2);
    border-bottom: 1px solid var(--sh-border);
}

.sh-component-doc__title {
    font-size: 1.5rem;
    color: var(--sh-text);
    margin-bottom: 0.5rem;
}

.sh-component-doc__category {
    font-size: 0.875rem;
    color: var(--sh-text-muted);
}

.sh-component-doc__body {
    padding: 2rem;
}

.sh-component-doc__description {
    font-size: 1rem;
    color: var(--sh-text-secondary);
    margin-bottom: 2rem;
    line-height: 1.7;
}

.sh-component-doc__preview {
    padding: 3rem;
    background: var(--sh-surface-2);
    border-radius: var(--sh-radius-md);
    text-align: center;
    color: var(--sh-text-muted);
    margin-bottom: 2rem;
}

.sh-component-doc__code h3 {
    font-size: 1.125rem;
    margin-bottom: 1rem;
    color: var(--sh-text);
}
"#
    .to_string()
}
