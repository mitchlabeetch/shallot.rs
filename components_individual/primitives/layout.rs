use maud::{html, Markup, Render};

pub struct Container {
    size: &'static str,
    padding: u32,
    children: Vec<Markup>,
}

impl Container {
    pub fn xs() -> Self {
        Self {
            size: "480px",
            padding: 16,
            children: Vec::new(),
        }
    }
    pub fn sm() -> Self {
        Self {
            size: "640px",
            padding: 16,
            children: Vec::new(),
        }
    }
    pub fn md() -> Self {
        Self {
            size: "768px",
            padding: 16,
            children: Vec::new(),
        }
    }
    pub fn lg() -> Self {
        Self {
            size: "1024px",
            padding: 24,
            children: Vec::new(),
        }
    }
    pub fn xl() -> Self {
        Self {
            size: "1280px",
            padding: 24,
            children: Vec::new(),
        }
    }
    pub fn xxl() -> Self {
        Self {
            size: "1536px",
            padding: 32,
            children: Vec::new(),
        }
    }
    pub fn full() -> Self {
        Self {
            size: "100%",
            padding: 16,
            children: Vec::new(),
        }
    }

    pub fn padding(mut self, px: u32) -> Self {
        self.padding = px;
        self
    }
    pub fn fluid(self) -> Self {
        Self {
            size: "100%".to_string(),
            ..self
        }
    }

    pub fn child(mut self, child: Markup) -> Self {
        self.children.push(child);
        self
    }
}

impl Render for Container {
    fn render(&self) -> Markup {
        html! {
            div class="sh-container" style={"max-width:{};margin:0 auto;padding-left:{}px;padding-right:{}px;".replace("{}", self.size).replace("{}", &self.padding.to_string()).replace("{}", &self.padding.to_string())} {
                @for child in &self.children { (child) }
            }
        }
    }
}

pub struct Section {
    padding_y: u32,
    padding_x: u32,
    background: Option<String>,
    children: Vec<Markup>,
}

impl Section {
    pub fn new() -> Self {
        Self {
            padding_y: 64,
            padding_x: 24,
            background: None,
            children: Vec::new(),
        }
    }
    pub fn tight() -> Self {
        Self {
            padding_y: 32,
            padding_x: 16,
            background: None,
            children: Vec::new(),
        }
    }
    pub fn spacious() -> Self {
        Self {
            padding_y: 96,
            padding_x: 32,
            background: None,
            children: Vec::new(),
        }
    }
    pub fn none() -> Self {
        Self {
            padding_y: 0,
            padding_x: 0,
            background: None,
            children: Vec::new(),
        }
    }

    pub fn padding_y(mut self, px: u32) -> Self {
        self.padding_y = px;
        self
    }
    pub fn padding_x(mut self, px: u32) -> Self {
        self.padding_x = px;
        self
    }
    pub fn background(mut self, bg: impl Into<String>) -> Self {
        self.background = Some(bg.into());
        self
    }

    pub fn child(mut self, child: Markup) -> Self {
        self.children.push(child);
        self
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Section {
    fn render(&self) -> Markup {
        let bg = self
            .background
            .as_ref()
            .map(|b| format!("background:{};", b))
            .unwrap_or_default();
        html! {
            section class="sh-section" style={"padding:{}px {}px;{}".replace("{}", &self.padding_y.to_string()).replace("{}", &self.padding_x.to_string())} {
                @for child in &self.children { (child) }
            }
        }
    }
}

pub struct Divider {
    orientation: &'static str,
    margin: u32,
    color: Option<String>,
}

impl Divider {
    pub fn horizontal() -> Self {
        Self {
            orientation: "horizontal",
            margin: 16,
            color: None,
        }
    }
    pub fn vertical() -> Self {
        Self {
            orientation: "vertical",
            margin: 16,
            color: None,
        }
    }

    pub fn margin(mut self, px: u32) -> Self {
        self.margin = px;
        self
    }
    pub fn color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }

    pub fn subtle(self) -> Self {
        self.color("var(--sh-border)")
    }
    pub fn bold(self) -> Self {
        self.color("var(--sh-fg)")
    }
}

impl Render for Divider {
    fn render(&self) -> Markup {
        let c = self
            .color
            .clone()
            .unwrap_or_else(|| "var(--sh-border)".to_string());
        if self.orientation == "horizontal" {
            html! { hr class="sh-divider" style={"margin:{}px 0;border:none;border-top:1px solid {};".replace("{}", &self.margin.to_string()).replace("{}", &c)} {} }
        } else {
            html! { div class="sh-divider-v" style={"width:1px;height:100%;margin:0 {}px;background:{};".replace("{}", &self.margin.to_string()).replace("{}", &c)} {} }
        }
    }
}

pub struct Spacer {
    size: u32,
    axis: &'static str,
}

impl Spacer {
    pub fn xs() -> Self {
        Self { size: 4, axis: "y" }
    }
    pub fn sm() -> Self {
        Self { size: 8, axis: "y" }
    }
    pub fn md() -> Self {
        Self {
            size: 16,
            axis: "y",
        }
    }
    pub fn lg() -> Self {
        Self {
            size: 24,
            axis: "y",
        }
    }
    pub fn xl() -> Self {
        Self {
            size: 32,
            axis: "y",
        }
    }
    pub fn xxl() -> Self {
        Self {
            size: 48,
            axis: "y",
        }
    }

    pub fn x(mut self) -> Self {
        Self { axis: "x", ..self }
    }
    pub fn y(self) -> Self {
        Self { axis: "y", ..self }
    }
}

impl Render for Spacer {
    fn render(&self) -> Markup {
        if self.axis == "x" {
            html! { div style={"width:{}px;height:1px;flex-shrink:0;" replace="{}" } {} }
        } else {
            html! { div style={"width:1px;height:{}px;flex-shrink:0;" replace="{}" } {} }
        }
    }
}

pub struct Center {
    inline_: bool,
    children: Vec<Markup>,
}

impl Center {
    pub fn new() -> Self {
        Self {
            inline_: false,
            children: Vec::new(),
        }
    }
    pub fn inline_(self) -> Self {
        Self {
            inline_: true,
            ..self
        }
    }
    pub fn child(mut self, child: Markup) -> Self {
        self.children.push(child);
        self
    }
}

impl Default for Center {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Center {
    fn render(&self) -> Markup {
        let display = if self.inline_ { "inline-flex" } else { "flex" };
        html! {
            div class="sh-center" style={"display:{};align-items:center;justify-content:center;".replace("{}", display)} {
                @for child in &self.children { (child) }
            }
        }
    }
}

pub struct AspectRatio {
    ratio: String,
    children: Vec<Markup>,
}

impl AspectRatio {
    pub fn square() -> Self {
        Self {
            ratio: "1/1".to_string(),
            children: Vec::new(),
        }
    }
    pub fn video() -> Self {
        Self {
            ratio: "16/9".to_string(),
            children: Vec::new(),
        }
    }
    pub fn portrait() -> Self {
        Self {
            ratio: "3/4".to_string(),
            children: Vec::new(),
        }
    }
    pub fn wide() -> Self {
        Self {
            ratio: "21/9".to_string(),
            children: Vec::new(),
        }
    }
    pub fn cinematic() -> Self {
        Self {
            ratio: "2.35/1".to_string(),
            children: Vec::new(),
        }
    }

    pub fn custom(n: u32, d: u32) -> Self {
        Self {
            ratio: format!("{}/{}", n, d),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: Markup) -> Self {
        self.children.push(child);
        self
    }
}

impl Render for AspectRatio {
    fn render(&self) -> Markup {
        html! {
            div class="sh-aspect-ratio" style={"aspect-ratio:{};position:relative;width:100%;".replace("{}", &self.ratio)} {
                @for child in &self.children { (child) }
            }
        }
    }
}

pub struct ResponsiveHide {
    hide_on: Vec<&'static str>,
}

impl ResponsiveHide {
    pub fn on_mobile() -> Self {
        Self {
            hide_on: vec!["mobile"],
        }
    }
    pub fn on_tablet() -> Self {
        Self {
            hide_on: vec!["tablet"],
        }
    }
    pub fn on_desktop() -> Self {
        Self {
            hide_on: vec!["desktop"],
        }
    }
    pub fn on_all() -> Self {
        Self {
            hide_on: vec!["mobile", "tablet", "desktop"],
        }
    }
}

impl Render for ResponsiveHide {
    fn render(&self) -> Markup {
        html! {
            style {
                "@media (max-width: 639px) { .hide-mobile { display: none !important; } }
                 @media (min-width: 640px) and (max-width: 1023px) { .hide-tablet { display: none !important; } }
                 @media (min-width: 1024px) { .hide-desktop { display: none !important; } }"
            }
        }
    }
}
