use maud::{html, Markup, Render};

pub struct Heading {
    level: u8,
    text: String,
    id: Option<String>,
    anchor: bool,
    size: Option<String>,
    weight: Option<u32>,
    color: Option<String>,
    align: Option<&'static str>,
}

impl Heading {
    pub fn h1(text: impl Into<String>) -> Self {
        Self {
            level: 1,
            text: text.into(),
            id: None,
            anchor: false,
            size: None,
            weight: None,
            color: None,
            align: None,
        }
    }
    pub fn h2(text: impl Into<String>) -> Self {
        Self {
            level: 2,
            text: text.into(),
            id: None,
            anchor: false,
            size: None,
            weight: None,
            color: None,
            align: None,
        }
    }
    pub fn h3(text: impl Into<String>) -> Self {
        Self {
            level: 3,
            text: text.into(),
            id: None,
            anchor: false,
            size: None,
            weight: None,
            color: None,
            align: None,
        }
    }
    pub fn h4(text: impl Into<String>) -> Self {
        Self {
            level: 4,
            text: text.into(),
            id: None,
            anchor: false,
            size: None,
            weight: None,
            color: None,
            align: None,
        }
    }
    pub fn h5(text: impl Into<String>) -> Self {
        Self {
            level: 5,
            text: text.into(),
            id: None,
            anchor: false,
            size: None,
            weight: None,
            color: None,
            align: None,
        }
    }
    pub fn h6(text: impl Into<String>) -> Self {
        Self {
            level: 6,
            text: text.into(),
            id: None,
            anchor: false,
            size: None,
            weight: None,
            color: None,
            align: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn anchor(mut self) -> Self {
        self.anchor = true;
        self
    }
    pub fn size(mut self, s: impl Into<String>) -> Self {
        self.size = Some(s.into());
        self
    }
    pub fn weight(mut self, w: u32) -> Self {
        self.weight = Some(w);
        self
    }
    pub fn color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }
    pub fn align(mut self, a: &'static str) -> Self {
        self.align = Some(a);
        self
    }
    pub fn center(self) -> Self {
        self.align("center")
    }
    pub fn left(self) -> Self {
        self.align("left")
    }
    pub fn right(self) -> Self {
        self.align("right")
    }

    fn generate_id(&self) -> String {
        self.text
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
    }

    fn base_style(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("margin:0 0 0.5em;line-height:1.2;"));

        match self.level {
            1 => {
                s.push_str("font-size:2.5rem;");
                if self.weight.is_none() {
                    s.push_str("font-weight:800;");
                }
            }
            2 => {
                s.push_str("font-size:2rem;");
                if self.weight.is_none() {
                    s.push_str("font-weight:700;");
                }
            }
            3 => {
                s.push_str("font-size:1.5rem;");
                if self.weight.is_none() {
                    s.push_str("font-weight:600;");
                }
            }
            4 => {
                s.push_str("font-size:1.25rem;");
                if self.weight.is_none() {
                    s.push_str("font-weight:600;");
                }
            }
            5 => {
                s.push_str("font-size:1rem;");
                if self.weight.is_none() {
                    s.push_str("font-weight:600;");
                }
            }
            6 => {
                s.push_str("font-size:0.875rem;");
                if self.weight.is_none() {
                    s.push_str("font-weight:600;");
                }
            }
            _ => {}
        }

        if let Some(ref sz) = self.size {
            s.push_str(&format!("font-size:{};", sz));
        }
        if let Some(w) = self.weight {
            s.push_str(&format!("font-weight:{};", w));
        }
        if let Some(ref c) = self.color {
            s.push_str(&format!("color:{};", c));
        }
        if let Some(a) = self.align {
            s.push_str(&format!("text-align:{};", a));
        }

        s
    }
}

impl Render for Heading {
    fn render(&self) -> Markup {
        let tag = format!("h{}", self.level);
        let id = self.id.clone().unwrap_or_else(|| self.generate_id());

        html! {
            @(match self.level {
                1 => html! { h1 id={(id)} class="sh-heading" style={(self.base_style())} { @if self.anchor { a href={(format!("#{}", id))} class="sh-anchor" style="margin-left:8px;opacity:0.3;font-weight:400;text-decoration:none;" { "#" } } (self.text) } },
                2 => html! { h2 id={(id)} class="sh-heading" style={(self.base_style())} { @if self.anchor { a href={(format!("#{}", id))} class="sh-anchor" style="margin-left:8px;opacity:0.3;font-weight:400;text-decoration:none;" { "#" } } (self.text) } },
                3 => html! { h3 id={(id)} class="sh-heading" style={(self.base_style())} { @if self.anchor { a href={(format!("#{}", id))} class="sh-anchor" style="margin-left:8px;opacity:0.3;font-weight:400;text-decoration:none;" { "#" } } (self.text) } },
                4 => html! { h4 id={(id)} class="sh-heading" style={(self.base_style())} { @if self.anchor { a href={(format!("#{}", id))} class="sh-anchor" style="margin-left:8px;opacity:0.3;font-weight:400;text-decoration:none;" { "#" } } (self.text) } },
                5 => html! { h5 id={(id)} class="sh-heading" style={(self.base_style())} { @if self.anchor { a href={(format!("#{}", id))} class="sh-anchor" style="margin-left:8px;opacity:0.3;font-weight:400;text-decoration:none;" { "#" } } (self.text) } },
                _ => html! { h6 id={(id)} class="sh-heading" style={(self.base_style())} { @if self.anchor { a href={(format!("#{}", id))} class="sh-anchor" style="margin-left:8px;opacity:0.3;font-weight:400;text-decoration:none;" { "#" } } (self.text) } },
            })
        }
    }
}

pub struct Text {
    text: String,
    size: Option<String>,
    weight: Option<u32>,
    color: Option<String>,
    align: Option<&'static str>,
    italic: bool,
    underline: bool,
    truncate: bool,
    line_clamp: Option<u32>,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: None,
            weight: None,
            color: None,
            align: None,
            italic: false,
            underline: false,
            truncate: false,
            line_clamp: None,
        }
    }
    pub fn size(mut self, s: impl Into<String>) -> Self {
        self.size = Some(s.into());
        self
    }
    pub fn weight(mut self, w: u32) -> Self {
        self.weight = Some(w);
        self
    }
    pub fn color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }
    pub fn align(mut self, a: &'static str) -> Self {
        self.align = Some(a);
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    pub fn truncate(mut self) -> Self {
        self.truncate = true;
        self
    }
    pub fn line_clamp(mut self, n: u32) -> Self {
        self.line_clamp = Some(n);
        self
    }
    pub fn muted(self) -> Self {
        self.color("var(--sh-fg-muted)")
    }
    pub fn primary(self) -> Self {
        self.color("var(--sh-primary)")
    }
    pub fn success(self) -> Self {
        self.color("var(--sh-success)")
    }
    pub fn warning(self) -> Self {
        self.color("var(--sh-warning)")
    }
    pub fn error(self) -> Self {
        self.color("var(--sh-error)")
    }

    fn build_style(&self) -> String {
        let mut s = String::from("margin:0;line-height:1.6;");
        if let Some(ref sz) = self.size {
            s.push_str(&format!("font-size:{};", sz));
        }
        if let Some(w) = self.weight {
            s.push_str(&format!("font-weight:{};", w));
        }
        if let Some(ref c) = self.color {
            s.push_str(&format!("color:{};", c));
        }
        if let Some(a) = self.align {
            s.push_str(&format!("text-align:{};", a));
        }
        if self.italic {
            s.push_str("font-style:italic;");
        }
        if self.underline {
            s.push_str("text-decoration:underline;");
        }
        if self.truncate {
            s.push_str("overflow:hidden;text-overflow:ellipsis;white-space:nowrap;");
        }
        if let Some(n) = self.line_clamp {
            s.push_str(&format!("display:-webkit-box;-webkit-line-clamp:{};-webkit-box-orient:vertical;overflow:hidden;", n));
        }
        s
    }
}

impl Render for Text {
    fn render(&self) -> Markup {
        html! { p class="sh-text" style={(self.build_style())} { (self.text) } }
    }
}

pub struct Span {
    text: String,
    size: Option<String>,
    weight: Option<u32>,
    color: Option<String>,
}

impl Span {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: None,
            weight: None,
            color: None,
        }
    }
    pub fn size(mut self, s: impl Into<String>) -> Self {
        self.size = Some(s.into());
        self
    }
    pub fn weight(mut self, w: u32) -> Self {
        self.weight = Some(w);
        self
    }
    pub fn color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }
    pub fn muted(self) -> Self {
        self.color("var(--sh-fg-muted)")
    }
    pub fn primary(self) -> Self {
        self.color("var(--sh-primary)")
    }
}

fn build_span_style(size: &Option<String>, weight: &Option<u32>, color: &Option<String>) -> String {
    let mut s = String::new();
    if let Some(ref sz) = size {
        s.push_str(&format!("font-size:{};", sz));
    }
    if let Some(w) = weight {
        s.push_str(&format!("font-weight:{};", w));
    }
    if let Some(ref c) = color {
        s.push_str(&format!("color:{};", c));
    }
    s
}

impl Render for Span {
    fn render(&self) -> Markup {
        html! { span style={(build_span_style(&self.size, &self.weight, &self.color))} { (self.text) } }
    }
}
