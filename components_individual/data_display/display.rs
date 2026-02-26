use maud::{html, Markup, Render};

pub struct Card {
    children: Markup,
    padding: u32,
    glass: bool,
    bordered: bool,
    hover: bool,
    padding_x: Option<u32>,
    padding_y: Option<u32>,
}

impl Card {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            padding: 24,
            glass: false,
            bordered: true,
            hover: false,
            padding_x: None,
            padding_y: None,
        }
    }
    pub fn padding(mut self, p: u32) -> Self {
        self.padding = p;
        self
    }
    pub fn padding_x(mut self, p: u32) -> Self {
        self.padding_x = Some(p);
        self
    }
    pub fn padding_y(mut self, p: u32) -> Self {
        self.padding_y = Some(p);
        self
    }
    pub fn tight(self) -> Self {
        self.padding(16)
    }
    pub fn spacious(self) -> Self {
        self.padding(32)
    }
    pub fn none(self) -> Self {
        self.padding(0)
    }
    pub fn glass(mut self) -> Self {
        self.glass = true;
        self
    }
    pub fn border(mut self) -> Self {
        self.bordered = true;
        self
    }
    pub fn hover(mut self) -> Self {
        self.hover = true;
        self
    }
    pub fn no_border(self) -> Self {
        Self {
            bordered: false,
            ..self
        }
    }

    fn build_style(&self) -> String {
        let px = self
            .padding_x
            .map(|p| p.to_string())
            .unwrap_or_else(|| self.padding.to_string());
        let py = self
            .padding_y
            .map(|p| p.to_string())
            .unwrap_or_else(|| self.padding.to_string());

        let mut s = format!(
            "padding:{}px {}px;background:var(--sh-surface);border-radius:var(--sh-radius-lg);",
            py, px
        );

        if self.bordered {
            s.push_str("border:1px solid var(--sh-border);");
        }
        if self.glass {
            s.push_str("background:rgba(255,255,255,0.05);backdrop-filter:blur(12px);border:1px solid rgba(255,255,255,0.1);");
        }
        if self.hover {
            s.push_str("transition:transform var(--sh-transition-fast),box-shadow var(--sh-transition-fast);");
        }

        s
    }
}

impl Render for Card {
    fn render(&self) -> Markup {
        html! { div class="sh-card" style={(self.build_style())} { (self.children) } }
    }
}

pub struct GlassCard {
    children: Markup,
    blur: u32,
    intensity: f32,
}

impl GlassCard {
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            blur: 12,
            intensity: 0.1,
        }
    }
    pub fn blur(mut self, b: u32) -> Self {
        self.blur = b;
        self
    }
    pub fn intensity(mut self, i: f32) -> Self {
        self.intensity = i;
        self
    }
    pub fn heavy(self) -> Self {
        self.blur(24).intensity(0.2)
    }
}

impl Render for GlassCard {
    fn render(&self) -> Markup {
        html! {
            div class="sh-glass-card" style={"padding:24px;background:rgba(255,255,{},{});backdrop-filter:blur({}px) saturate(180%);border:1px solid rgba(255,255,255,0.2);border-radius:var(--sh-radius-lg);box-shadow:var(--sh-shadow-lg);".replace("{}", &self.intensity.to_string()).replace("{}", &self.intensity.to_string()).replace("{}", &self.blur.to_string())} {
                (self.children)
            }
        }
    }
}

pub struct Avatar {
    alt: String,
    src: Option<String>,
    size: u32,
    shape: &'static str,
}

impl Avatar {
    pub fn new(alt: impl Into<String>) -> Self {
        Self {
            alt: alt.into(),
            src: None,
            size: 40,
            shape: "circle",
        }
    }
    pub fn src(mut self, s: impl Into<String>) -> Self {
        self.src = Some(s.into());
        self
    }
    pub fn size(mut self, s: u32) -> Self {
        self.size = s;
        self
    }
    pub fn sm(self) -> Self {
        self.size(32)
    }
    pub fn md(self) -> Self {
        self.size(40)
    }
    pub fn lg(self) -> Self {
        self.size(64)
    }
    pub fn xl(self) -> Self {
        self.size(96)
    }
    pub fn square(self) -> Self {
        Self {
            shape: "square",
            ..self
        }
    }
}

impl Render for Avatar {
    fn render(&self) -> Markup {
        let fallback = self
            .alt
            .chars()
            .next()
            .unwrap_or('?')
            .to_uppercase()
            .to_string();
        let radius = if self.shape == "square" {
            "var(--sh-radius-md)"
        } else {
            "50%"
        };

        html! {
            div class="sh-avatar" style={"width:{}px;height:{}px;display:flex;align-items:center;justify-content:center;background:var(--sh-primary);color:white;font-weight:600;border-radius:{};overflow:hidden;".replace("{}", &self.size.to_string()).replace("{}", &self.size.to_string()).replace("{}", radius)} {
                @if let Some(ref src) = self.src {
                    img src={(src)} alt={(self.alt)} style="width:100%;height:100%;object-fit:cover;" {}
                } @else {
                    span style={"font-size:{}px;".replace("{}", &(self.size / 2).to_string())} { (fallback) }
                }
            }
        }
    }
}

pub struct AvatarGroup {
    avatars: Vec<Avatar>,
    max: u32,
}

impl AvatarGroup {
    pub fn new() -> Self {
        Self {
            avatars: Vec::new(),
            max: 4,
        }
    }
    pub fn avatar(mut self, a: Avatar) -> Self {
        self.avatars.push(a);
        self
    }
    pub fn max(mut self, m: u32) -> Self {
        self.max = m;
        self
    }
}

impl Default for AvatarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AvatarGroup {
    fn render(&self) -> Markup {
        let shown = &self.avatars[..self.max.min(self.avatars.len() as u32) as usize];
        let remaining = self.avatars.len() as u32 - self.max;
        let size = shown.first().map(|a| a.size).unwrap_or(40);

        html! {
            div class="sh-avatar-group" style="display:flex;" {
                @for avatar in shown {
                    div style="margin-left:-8px; &:first-child { margin-left: 0; }" {
                        (avatar.clone().render())
                    }
                }
                @if remaining > 0 {
                    div style={"margin-left:-8px;width:{}px;height:{}px;display:flex;align-items:center;justify-content:center;background:var(--sh-surface);border:2px solid var(--sh-bg);border-radius:50%;font-size:{}px;font-weight:600;".replace("{}", &size.to_string()).replace("{}", &size.to_string()).replace("{}", &(size / 3).to_string())} {
                        (format!("+{}", remaining))
                    }
                }
            }
        }
    }
}

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    striped: bool,
    hoverable: bool,
    bordered: bool,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            striped: false,
            hoverable: true,
            bordered: false,
        }
    }
    pub fn headers(mut self, h: Vec<&str>) -> Self {
        self.headers = h.into_iter().map(String::from).collect();
        self
    }
    pub fn row(mut self, r: Vec<&str>) -> Self {
        self.rows.push(r.into_iter().map(String::from).collect());
        self
    }
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }
    pub fn hoverable(mut self) -> Self {
        self.hoverable = true;
        self
    }
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Table {
    fn render(&self) -> Markup {
        let border = if self.bordered {
            "border:1px solid var(--sh-border);"
        } else {
            ""
        };

        html! {
            div class="sh-table-wrapper" style="overflow-x:auto;" {
                table class="sh-table" style={"width:100%;border-collapse:collapse;".to_string() + border} {
                    thead {
                        tr {
                            @for header in &self.headers {
                                th style="padding:12px 16px;text-align:left;font-weight:600;border-bottom:2px solid var(--sh-border);" { (header) }
                            }
                        }
                    }
                    tbody {
                        @for (i, row) in self.rows.iter().enumerate() {
                            tr style={(if self.striped && i % 2 == 1 { "background:var(--sh-surface);" } else { "" })} {
                                @for cell in row {
                                    td style="padding:12px 16px;border-bottom:1px solid var(--sh-border);" { (cell) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct Stat {
    label: String,
    value: String,
    trend: Option<(&'static str, String)>,
    description: Option<String>,
}

impl Stat {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            trend: None,
            description: None,
        }
    }
    pub fn trend(mut self, direction: &'static str, value: impl Into<String>) -> Self {
        self.trend = Some((direction, value.into()));
        self
    }
    pub fn up(self, value: impl Into<String>) -> Self {
        self.trend("up", value)
    }
    pub fn down(self, value: impl Into<String>) -> Self {
        self.trend("down", value)
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
}

impl Render for Stat {
    fn render(&self) -> Markup {
        html! {
            div class="sh-stat" {
                p class="sh-stat-label" style="margin:0;font-size:0.875rem;opacity:0.7;" { (self.label) }
                p class="sh-stat-value" style="margin:4px 0;font-size:2rem;font-weight:700;" { (self.value) }
                @if let Some((direction, value)) = &self.trend {
                    p class="sh-stat-trend" style="margin:0;font-size:0.875rem;color:{};" {
                        (format!("{} {}", if *direction == "up" { "↑" } else { "↓" }, value))
                    }
                }
                @if let Some(ref desc) = self.description {
                    p class="sh-stat-desc" style="margin:8px 0 0;font-size:0.75rem;opacity:0.5;" { (desc) }
                }
            }
        }
    }
}
