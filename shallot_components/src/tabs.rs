//! Tabs Component with animated transitions

use maud::{html, Markup, Render};

/// Tab animation style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabAnimation {
    None,
    #[default]
    Fade,
    Slide,
    Scale,
}

/// Tab variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabVariant {
    #[default]
    Default,
    Pills,
    Underline,
    Bordered,
}

/// Tab size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Single tab item
pub struct Tab<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub content: Markup,
    pub disabled: bool,
    pub icon: Option<&'a str>,
}

impl<'a> Tab<'a> {
    pub fn new(id: &'a str, label: &'a str, content: Markup) -> Self {
        Self {
            id,
            label,
            content,
            disabled: false,
            icon: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Tabs Component
pub struct Tabs<'a> {
    pub name: &'a str,
    pub tabs: Vec<Tab<'a>>,
    pub selected: usize,
    pub animation: TabAnimation,
    pub variant: TabVariant,
    pub size: TabSize,
}

impl<'a> Tabs<'a> {
    pub fn new(name: &'a str, tabs: Vec<Tab<'a>>) -> Self {
        Self {
            name,
            tabs,
            selected: 0,
            animation: TabAnimation::default(),
            variant: TabVariant::default(),
            size: TabSize::default(),
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index.min(self.tabs.len().saturating_sub(1));
        self
    }

    pub fn animation(mut self, animation: TabAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: TabSize) -> Self {
        self.size = size;
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-tabs"];

        match self.variant {
            TabVariant::Default => classes.push("sh-tabs--default"),
            TabVariant::Pills => classes.push("sh-tabs--pills"),
            TabVariant::Underline => classes.push("sh-tabs--underline"),
            TabVariant::Bordered => classes.push("sh-tabs--bordered"),
        }

        match self.size {
            TabSize::Sm => classes.push("sh-tabs--sm"),
            TabSize::Md => classes.push("sh-tabs--md"),
            TabSize::Lg => classes.push("sh-tabs--lg"),
        }

        match self.animation {
            TabAnimation::None => classes.push("sh-tabs--no-animation"),
            TabAnimation::Fade => classes.push("sh-tabs--fade"),
            TabAnimation::Slide => classes.push("sh-tabs--slide"),
            TabAnimation::Scale => classes.push("sh-tabs--scale"),
        }

        classes.join(" ")
    }
}

impl<'a> Render for Tabs<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        // Pre-compute tab classes
        let tab_classes: Vec<String> = self.tabs.iter().enumerate().map(|(idx, tab)| {
            let mut c = vec!["sh-tabs__tab"];
            if idx == self.selected { c.push("sh-tabs__tab--active"); }
            if tab.disabled { c.push("sh-tabs__tab--disabled"); }
            c.join(" ")
        }).collect();

        // Pre-compute panel classes
        let panel_classes: Vec<String> = self.tabs.iter().enumerate().map(|(idx, _)| {
            let mut c = vec!["sh-tabs__panel"];
            if idx == self.selected { c.push("sh-tabs__panel--active"); }
            c.join(" ")
        }).collect();

        // Pre-compute aria-selected values
        let aria_selected: Vec<&'static str> = self.tabs.iter().enumerate().map(|(idx, _)| {
            if idx == self.selected { "true" } else { "false" }
        }).collect();

        html! {
            div class=(classes) data-tabs=(self.name) {
                div class="sh-tabs__list" role="tablist" {
                    @for (idx, tab) in self.tabs.iter().enumerate() {
                        button
                            class=(tab_classes[idx])
                            type="button"
                            role="tab"
                            id={(format!("{}-tab-{}", self.name, tab.id))}
                            aria-selected=(aria_selected[idx])
                            aria-controls={(format!("{}-panel-{}", self.name, tab.id))}
                            disabled?[tab.disabled]
                            data-tab-index=(idx)
                        {
                            @if let Some(icon) = tab.icon {
                                span class="sh-tabs__icon" { (maud::PreEscaped(icon)) }
                            }
                            span class="sh-tabs__label" { (tab.label) }
                        }
                    }

                    div class="sh-tabs__indicator" {}
                }

                div class="sh-tabs__panels" {
                    @for (idx, tab) in self.tabs.iter().enumerate() {
                        div
                            class=(panel_classes[idx])
                            role="tabpanel"
                            id={(format!("{}-panel-{}", self.name, tab.id))}
                            aria-labelledby={(format!("{}-tab-{}", self.name, tab.id))}
                            hidden?[idx != self.selected]
                            data-panel-index=(idx)
                        {
                            (tab.content.clone())
                        }
                    }
                }
            }
        }
    }
}



/// Animated Tabs with enhanced transitions
pub struct AnimatedTabs<'a> {
    pub tabs: Tabs<'a>,
    pub transition_duration: u16,
}

impl<'a> AnimatedTabs<'a> {
    pub fn new(name: &'a str, tabs: Vec<Tab<'a>>) -> Self {
        Self {
            tabs: Tabs::new(name, tabs),
            transition_duration: 300,
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.tabs = self.tabs.selected(index);
        self
    }

    pub fn animation(mut self, animation: TabAnimation) -> Self {
        self.tabs = self.tabs.animation(animation);
        self
    }

    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.tabs = self.tabs.variant(variant);
        self
    }

    pub fn transition_duration(mut self, duration: u16) -> Self {
        self.transition_duration = duration;
        self
    }
}

impl<'a> Render for AnimatedTabs<'a> {
    fn render(&self) -> Markup {
        let style = format!("--sh-tabs-duration: {}ms;", self.transition_duration);
        let rendered_tabs = self.tabs.render();

        html! {
            div class="sh-tabs-animated" style=(style) {
                (rendered_tabs)
            }
        }
    }
}

/// Generate CSS for tabs components
pub fn tabs_css() -> String {
    r#"
/* Tabs Component Styles */
.sh-tabs {
    width: 100%;
}

.sh-tabs__list {
    display: flex;
    gap: 0.25rem;
    position: relative;
    border-bottom: 1px solid var(--sh-border);
}

.sh-tabs__tab {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text-muted);
    background: transparent;
    border: none;
    cursor: pointer;
    position: relative;
    transition: color 0.2s, background 0.2s;
}

.sh-tabs__tab:hover:not(.sh-tabs__tab--disabled) {
    color: var(--sh-text);
}

.sh-tabs__tab--active {
    color: var(--sh-accent);
}

.sh-tabs__tab--disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.sh-tabs__icon {
    display: flex;
    align-items: center;
    width: 1rem;
    height: 1rem;
}

/* Variant: Pills */
.sh-tabs--pills .sh-tabs__list {
    border-bottom: none;
    background: var(--sh-surface-alt);
    padding: 0.25rem;
    border-radius: 0.5rem;
}

.sh-tabs--pills .sh-tabs__tab {
    border-radius: 0.375rem;
}

.sh-tabs--pills .sh-tabs__tab--active {
    background: var(--sh-primary);
    color: white;
}

/* Variant: Underline */
.sh-tabs--underline .sh-tabs__tab::after {
    content: '';
    position: absolute;
    bottom: -1px;
    left: 0;
    right: 0;
    height: 2px;
    background: transparent;
    transition: background 0.2s;
}

.sh-tabs--underline .sh-tabs__tab--active::after {
    background: var(--sh-accent);
}

/* Variant: Bordered */
.sh-tabs--bordered .sh-tabs__tab {
    border: 1px solid transparent;
    border-bottom: none;
    border-radius: 0.375rem 0.375rem 0 0;
    margin-bottom: -1px;
}

.sh-tabs--bordered .sh-tabs__tab--active {
    border-color: var(--sh-border);
    background: var(--sh-surface);
}

/* Sizes */
.sh-tabs--sm .sh-tabs__tab {
    padding: 0.5rem 0.75rem;
    font-size: 0.8125rem;
}

.sh-tabs--lg .sh-tabs__tab {
    padding: 1rem 1.5rem;
    font-size: 1rem;
}

/* Panels */
.sh-tabs__panels {
    padding: 1rem 0;
}

.sh-tabs__panel {
    display: none;
}

.sh-tabs__panel--active {
    display: block;
}

/* Animations */
.sh-tabs--fade .sh-tabs__panel {
    opacity: 0;
    transition: opacity var(--sh-tabs-duration, 300ms) ease;
}

.sh-tabs--fade .sh-tabs__panel--active {
    opacity: 1;
}

.sh-tabs--slide .sh-tabs__panel {
    transform: translateX(20px);
    opacity: 0;
    transition: transform var(--sh-tabs-duration, 300ms) ease, opacity var(--sh-tabs-duration, 300ms) ease;
}

.sh-tabs--slide .sh-tabs__panel--active {
    transform: translateX(0);
    opacity: 1;
}

.sh-tabs--scale .sh-tabs__panel {
    transform: scale(0.95);
    opacity: 0;
    transition: transform var(--sh-tabs-duration, 300ms) ease, opacity var(--sh-tabs-duration, 300ms) ease;
}

.sh-tabs--scale .sh-tabs__panel--active {
    transform: scale(1);
    opacity: 1;
}

/* Indicator line (animated) */
.sh-tabs__indicator {
    position: absolute;
    bottom: 0;
    height: 2px;
    background: var(--sh-accent);
    transition: left 0.3s, width 0.3s;
    pointer-events: none;
}

/* Animated wrapper */
.sh-tabs-animated {
    overflow: hidden;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-tabs__tab,
    .sh-tabs__panel,
    .sh-tabs__indicator {
        transition: none;
    }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maud::html;

    #[test]
    fn test_tabs_creation() {
        let tabs = Tabs::new(
            "test",
            vec![
                Tab::new("tab1", "Tab 1", html! { "Content 1" }),
                Tab::new("tab2", "Tab 2", html! { "Content 2" }),
            ],
        );

        assert_eq!(tabs.tabs.len(), 2);
        assert_eq!(tabs.selected, 0);
    }

    #[test]
    fn test_tabs_selected() {
        let tabs = Tabs::new(
            "test",
            vec![
                Tab::new("tab1", "Tab 1", html! { "Content 1" }),
                Tab::new("tab2", "Tab 2", html! { "Content 2" }),
            ],
        )
        .selected(1);

        assert_eq!(tabs.selected, 1);
    }

    #[test]
    fn test_tabs_variants() {
        let tabs = Tabs::new("test", vec![])
            .variant(TabVariant::Pills)
            .animation(TabAnimation::Slide)
            .size(TabSize::Lg);

        assert_eq!(tabs.variant, TabVariant::Pills);
        assert_eq!(tabs.animation, TabAnimation::Slide);
        assert_eq!(tabs.size, TabSize::Lg);
    }

    #[test]
    fn test_animated_tabs() {
        let animated =
            AnimatedTabs::new("test", vec![Tab::new("tab1", "Tab 1", html! { "Content" })])
                .transition_duration(500);

        assert_eq!(animated.transition_duration, 500);
    }

    #[test]
    fn test_tab_disabled() {
        let tab = Tab::new("tab1", "Tab 1", html! { "Content" }).disabled(true);

        assert!(tab.disabled);
    }
}
