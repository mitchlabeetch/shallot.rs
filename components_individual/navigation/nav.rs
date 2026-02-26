use maud::{html, Markup, Render};

pub struct Navbar {
    brand: Option<Markup>,
    links: Vec<(String, String)>,
    actions: Vec<Markup>,
    sticky: bool,
}

impl Navbar {
    pub fn new() -> Self {
        Self {
            brand: None,
            links: Vec::new(),
            actions: Vec::new(),
            sticky: false,
        }
    }
    pub fn brand(mut self, b: Markup) -> Self {
        self.brand = Some(b);
        self
    }
    pub fn brand_text(mut self, text: impl Into<String>) -> Self {
        self.brand = Some(
            html! { a href="/" style="font-weight:700;font-size:1.25rem;color:var(--sh-fg);text-decoration:none;" { (text.into()) } },
        );
        self
    }
    pub fn link(mut self, href: impl Into<String>, text: impl Into<String>) -> Self {
        self.links.push((href.into(), text.into()));
        self
    }
    pub fn action(mut self, a: Markup) -> Self {
        self.actions.push(a);
        self
    }
    pub fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }
}

impl Default for Navbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Navbar {
    fn render(&self) -> Markup {
        let pos = if self.sticky {
            "position:sticky;top:0;z-index:100;"
        } else {
            ""
        };

        html! {
            nav class="sh-navbar" style={"display:flex;align-items:center;justify-content:space-between;padding:16px 24px;background:var(--sh-surface);border-bottom:1px solid var(--sh-border);".to_string() + pos} {
                div style="display:flex;align-items:center;gap:32px;flex:1;" {
                    @if let Some(ref brand) = self.brand { div { (brand) } }
                    div style="display:flex;gap:8px;flex:1;" {
                        @for (href, text) in &self.links {
                            a href={(href)} style="color:var(--sh-fg);opacity:0.8;text-decoration:none;padding:8px 12px;border-radius:var(--sh-radius-md);transition:all var(--sh-transition-fast);" { (text) }
                        }
                    }
                }
                div style="display:flex;align-items:center;gap:12px;" {
                    @for action in &self.actions { (action) }
                }
            }
        }
    }
}

pub struct Sidebar {
    items: Vec<(String, String, bool)>,
    collapsible: bool,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            collapsible: false,
        }
    }
    pub fn item(mut self, href: impl Into<String>, text: impl Into<String>, active: bool) -> Self {
        self.items.push((href.into(), text.into(), active));
        self
    }
    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Sidebar {
    fn render(&self) -> Markup {
        html! {
            aside class="sh-sidebar" style="width:280px;height:100vh;position:sticky;top:0;padding:16px;background:var(--sh-surface);border-right:1px solid var(--sh-border);overflow-y:auto;" {
                nav style="display:flex;flex-direction:column;gap:4px;" {
                    @for (href, text, active) in &self.items {
                        a href={(href)} style={"display:block;padding:10px 12px;border-radius:var(--sh-radius-md);text-decoration:none;transition:all var(--sh-transition-fast);{}".replace("{}", if active { "background:var(--sh-primary-glass);color:var(--sh-primary);font-weight:500;" } else { "color:var(--sh-fg);opacity:0.8;" })} { (text) }
                    }
                }
            }
        }
    }
}

pub struct Breadcrumbs {
    items: Vec<(Option<String>, String)>,
}

impl Breadcrumbs {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn item(mut self, href: impl Into<String>, text: impl Into<String>) -> Self {
        self.items.push((Some(href.into()), text.into()));
        self
    }
    pub fn active(mut self, text: impl Into<String>) -> Self {
        self.items.push((None, text.into()));
        self
    }
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Breadcrumbs {
    fn render(&self) -> Markup {
        html! {
            nav class="sh-breadcrumbs" aria-label="Breadcrumb" style="display:flex;align-items:center;gap:8px;font-size:0.875rem;" {
                ol style="display:flex;list-style:none;margin:0;padding:0;gap:8px;" {
                    @for (i, (href, text)) in self.items.iter().enumerate() {
                        li style="display:flex;align-items:center;gap:8px;" {
                            @if let Some(ref h) = href {
                                a href={(h)} style="color:var(--sh-primary);text-decoration:none;transition:opacity var(--sh-transition-fast);" { (text) }
                            } @else {
                                span style="color:var(--sh-fg);opacity:0.7;" { (text) }
                            }
                            @if i < self.items.len() - 1 {
                                span style="opacity:0.5;" { "/" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct Tabs {
    tabs: Vec<(String, Markup)>,
    active: usize,
    variant: &'static str,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active: 0,
            variant: "line",
        }
    }
    pub fn tab(mut self, label: impl Into<String>, content: Markup) -> Self {
        self.tabs.push((label.into(), content));
        self
    }
    pub fn active(mut self, idx: usize) -> Self {
        self.active = idx;
        self
    }
    pub fn pills(mut self) -> Self {
        self.variant = "pills";
        self
    }
    pub fn enclosed(mut self) -> Self {
        self.variant = "enclosed";
        self
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Tabs {
    fn render(&self) -> Markup {
        html! {
            div class="sh-tabs" {
                div class="sh-tabs-list" role="tablist" style={"display:flex;gap:4px;{}".replace("{}", if self.variant == "line" { "border-bottom:1px solid var(--sh-border);" } else { "" })} {
                    @for (i, (label, _)) in self.tabs.iter().enumerate() {
                        button role="tab" aria-selected={(if i == self.active { "true" } else { "false" })} class="sh-tab-btn" style={"padding:12px 20px;border:none;background:transparent;cursor:pointer;font-weight:500;transition:all var(--sh-transition-fast);{}".replace("{}", match (self.variant, i == self.active) {
                            ("pills", true) => "background:var(--sh-primary);color:white;border-radius:var(--sh-radius-md);",
                            ("pills", false) => "color:var(--sh-fg);opacity:0.7;border-radius:var(--sh-radius-md);",
                            ("enclosed", true) => "background:var(--sh-surface);color:var(--sh-fg);border:1px solid var(--sh-border);border-bottom:none;border-radius:var(--sh-radius-md) var(--sh-radius-md) 0 0;",
                            ("enclosed", false) => "color:var(--sh-fg);opacity:0.7;",
                            (_, true) => "color:var(--sh-primary);border-bottom:2px solid var(--sh-primary);margin-bottom:-1px;",
                            _ => "color:var(--sh-fg);opacity:0.7;",
                        })} { (label) }
                    }
                }
                div class="sh-tabs-panels" style="padding:24px 0;" {
                    @if let Some((_, content)) = self.tabs.get(self.active) {
                        (content.clone())
                    }
                }
            }
        }
    }
}

pub struct Pagination {
    current: u32,
    total: u32,
    show_numbers: bool,
}

impl Pagination {
    pub fn new(total: u32) -> Self {
        Self {
            current: 1,
            total,
            show_numbers: true,
        }
    }
    pub fn current(mut self, page: u32) -> Self {
        self.current = page;
        self
    }
    pub fn show_numbers(mut self, b: bool) -> Self {
        self.show_numbers = b;
        self
    }
}

impl Render for Pagination {
    fn render(&self) -> Markup {
        html! {
            nav class="sh-pagination" aria-label="Pagination" style="display:flex;align-items:center;gap:8px;" {
                a href={(format!("?page={}", self.current.saturating_sub(1)))} style="padding:8px 12px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);text-decoration:none;color:var(--sh-fg);" { "← Prev" }

                @if self.show_numbers {
                    @for page in 1..=self.total.min(5) {
                        a href={(format!("?page={}", page))} style={"padding:8px 12px;border-radius:var(--sh-radius-md);text-decoration:none;{}".replace("{}", if page == self.current { "background:var(--sh-primary);color:white;" } else { "color:var(--sh-fg);" })} { (page) }
                    }
                }

                a href={(format!("?page={}", self.current + 1))} style="padding:8px 12px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);text-decoration:none;color:var(--sh-fg);" { "Next →" }
            }
        }
    }
}

pub struct Steps {
    items: Vec<(String, bool, bool)>,
}

impl Steps {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn step(mut self, label: impl Into<String>, completed: bool, current: bool) -> Self {
        self.items.push((label.into(), completed, current));
        self
    }
}

impl Default for Steps {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Steps {
    fn render(&self) -> Markup {
        html! {
            div class="sh-steps" style="display:flex;align-items:center;gap:8px;" {
                @for (i, (label, completed, current)) in self.items.iter().enumerate() {
                    @if i > 0 { div style="flex:1;height:2px;background:var(--sh-border);" {} }
                    div style="display:flex;align-items:center;gap:8px;" {
                        div style={"width:32px;height:32px;display:flex;align-items:center;justify-content:center;border-radius:50%;font-size:0.875rem;font-weight:600;{}".replace("{}", if *completed { "background:var(--sh-primary);color:white;" } else if *current { "border:2px solid var(--sh-primary);color:var(--sh-primary);" } else { "background:var(--sh-surface);color:var(--sh-fg);opacity:0.5;" })} {
                            @if *completed { "✓" } @else { (i + 1) }
                        }
                        span style={"font-size:0.875rem;{}".replace("{}", if *current { "font-weight:600;" } else { "opacity:0.7;" })} { (label) }
                    }
                }
            }
        }
    }
}
