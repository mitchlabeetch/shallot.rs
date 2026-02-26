use maud::{html, Markup};

pub struct ShinyButton<'a> {
    pub label: &'a str,
    pub href: &'a str,
}

impl<'a> ShinyButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, href: "#" }
    }
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = href;
        self
    }
    pub fn render(self) -> Markup {
        html! {
            a class="sh-shiny-btn" href=(self.href) {
                span class="sh-shiny-btn__label" { (self.label) }
                span class="sh-shiny-btn__ring" aria-hidden="true" {}
            }
        }
    }
}

pub struct ShimmerButton<'a> {
    pub label: &'a str,
    pub href: &'a str,
}

impl<'a> ShimmerButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, href: "#" }
    }
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = href;
        self
    }
    pub fn render(self) -> Markup {
        html! {
            a class="sh-shimmer-btn" href=(self.href) {
                span class="sh-shimmer-btn__label" { (self.label) }
                span class="sh-shimmer-btn__ring" aria-hidden="true" {}
            }
        }
    }
}

pub struct GlowCard {
    pub children: Markup,
}

impl GlowCard {
    pub fn new(children: Markup) -> Self {
        Self { children }
    }
    pub fn render(self) -> Markup {
        html! {
            div class="sh-glow-card" role="article" aria-label="Glow card" {
                (self.children)
            }
        }
    }
}

/// Generate CSS for fx components
pub fn fx_css() -> String {
    r#"
/* Shiny Button */
.sh-shiny-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.75rem 1.5rem;
    background: var(--sh-primary, #3b82f6);
    color: white;
    border-radius: var(--sh-radius-md, 0.375rem);
    text-decoration: none;
    overflow: hidden;
    transition: transform 0.2s ease;
}

.sh-shiny-btn:hover {
    transform: translateY(-2px);
}

.sh-shiny-btn__ring {
    position: absolute;
    inset: -50%;
    background: linear-gradient(45deg, transparent, rgba(255, 255, 255, 0.3), transparent);
    animation: sh-shiny-shine 3s ease-in-out infinite;
}

@keyframes sh-shiny-shine {
    0%, 100% { transform: translateX(-100%) rotate(45deg); }
    50% { transform: translateX(100%) rotate(45deg); }
}

/* Shimmer Button */
.sh-shimmer-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.75rem 1.5rem;
    background: var(--sh-surface-2, #f3f4f6);
    color: var(--sh-text, #1f2937);
    border-radius: var(--sh-radius-md, 0.375rem);
    text-decoration: none;
    overflow: hidden;
}

.sh-shimmer-btn__ring {
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.4), transparent);
    animation: sh-shimmer 2s ease-in-out infinite;
}

@keyframes sh-shimmer {
    0% { left: -100%; }
    100% { left: 100%; }
}

/* Glow Card */
.sh-glow-card {
    position: relative;
    padding: 1.5rem;
    background: var(--sh-surface, #fff);
    border-radius: var(--sh-radius-lg, 0.5rem);
    box-shadow: 0 0 20px rgba(59, 130, 246, 0.1);
    transition: box-shadow 0.3s ease;
}

.sh-glow-card:hover {
    box-shadow: 0 0 40px rgba(59, 130, 246, 0.2);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shiny_button_creation() {
        let btn = ShinyButton::new("Click");
        assert_eq!(btn.label, "Click");
        assert_eq!(btn.href, "#");
    }

    #[test]
    fn test_shimmer_button_creation() {
        let btn = ShimmerButton::new("Action");
        assert_eq!(btn.label, "Action");
    }

    #[test]
    fn test_glow_card_creation() {
        let card = GlowCard::new(html! { "Content" });
        let html = card.render().into_string();
        assert!(html.contains("sh-glow-card"));
    }

    #[test]
    fn test_fx_css() {
        let css = fx_css();
        assert!(css.contains(".sh-shiny-btn"));
        assert!(css.contains(".sh-shimmer-btn"));
        assert!(css.contains(".sh-glow-card"));
    }
}
