use maud::{html, Markup, Render};

pub struct Code {
    text: String,
    inline: bool,
    language: Option<String>,
    show_line_numbers: bool,
}

impl Code {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            inline: true,
            language: None,
            show_line_numbers: false,
        }
    }
    pub fn block(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            inline: false,
            language: None,
            show_line_numbers: false,
        }
    }
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }
    pub fn line_numbers(mut self) -> Self {
        self.show_line_numbers = true;
        self
    }
}

impl Render for Code {
    fn render(&self) -> Markup {
        if self.inline {
            html! {
                code class="sh-code-inline" style="font-family:var(--sh-font-mono);font-size:0.875em;padding:2px 6px;background:var(--sh-surface);border-radius:var(--sh-radius-sm);color:var(--sh-primary);" {
                    (self.text)
                }
            }
        } else {
            html! {
                pre class="sh-code-block" style="font-family:var(--sh-font-mono);font-size:0.875rem;padding:16px;background:var(--sh-surface);border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);overflow-x:auto;" {
                    code { (self.text) }
                }
            }
        }
    }
}

pub struct Kbd {
    keys: Vec<String>,
}

impl Kbd {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.keys.push(key.into());
        self
    }
}

impl Default for Kbd {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Kbd {
    fn render(&self) -> Markup {
        html! {
            kbd class="sh-kbd" style="display:inline-flex;gap:4px;font-family:var(--sh-font-mono);font-size:0.75rem;padding:4px 8px;background:var(--sh-surface);border:1px solid var(--sh-border);border-radius:var(--sh-radius-sm);box-shadow:0 1px 2px rgba(0,0,0,0.1);" {
                @for (i, key) in self.keys.iter().enumerate() {
                    @if i > 0 { span style="opacity:0.5;" { "+" } }
                    (key)
                }
            }
        }
    }
}

pub struct Quote {
    text: String,
    citation: Option<String>,
    variant: &'static str,
}

impl Quote {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            citation: None,
            variant: "default",
        }
    }
    pub fn citation(mut self, c: impl Into<String>) -> Self {
        self.citation = Some(c.into());
        self
    }
    pub fn pull(self) -> Self {
        Self {
            variant: "pull",
            ..self
        }
    }
    pub fn border(self) -> Self {
        Self {
            variant: "border",
            ..self
        }
    }
}

impl Render for Quote {
    fn render(&self) -> Markup {
        let style = match self.variant {
            "pull" => "font-size:1.25rem;font-style:italic;padding:24px;",
            "border" => "border:1px solid var(--sh-border);padding:16px 24px;",
            _ => "border-left:4px solid var(--sh-primary);padding:16px 24px;",
        };

        html! {
            blockquote class="sh-quote" style={"margin:16px 0;".to_string() + style} {
                p style="margin:0 0 12px;" { (self.text) }
                @if let Some(ref c) = self.citation {
                    cite style="font-size:0.875rem;color:var(--sh-fg-muted);font-style:normal;" { (c) }
                }
            }
        }
    }
}

pub struct List {
    items: Vec<String>,
    ordered: bool,
    marker: Option<&'static str>,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            ordered: false,
            marker: None,
        }
    }
    pub fn ordered(self) -> Self {
        Self {
            ordered: true,
            ..self
        }
    }
    pub fn unordered(self) -> Self {
        Self {
            ordered: false,
            ..self
        }
    }
    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(text.into());
        self
    }
    pub fn marker(mut self, m: &'static str) -> Self {
        self.marker = Some(m);
        self
    }
    pub fn circle(self) -> Self {
        self.marker("circle")
    }
    pub fn square(self) -> Self {
        self.marker("square")
    }
    pub fn disc(self) -> Self {
        self.marker("disc")
    }
    pub fn decimal(self) -> Self {
        self.marker("decimal")
    }
    pub fn none(self) -> Self {
        self.marker("none")
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for List {
    fn render(&self) -> Markup {
        let style = if let Some(m) = self.marker {
            format!("list-style:{};", m)
        } else if self.ordered {
            "list-style:decimal;".to_string()
        } else {
            "list-style:disc;".to_string()
        };

        if self.ordered {
            html! { ol class="sh-list" style={"padding-left:24px;".to_string() + &style + "margin:16px 0;"} { @for item in &self.items { li style="margin-bottom:8px;" { (item) } } } }
        } else {
            html! { ul class="sh-list" style={"padding-left:24px;".to_string() + &style + "margin:16px 0;"} { @for item in &self.items { li style="margin-bottom:8px;" { (item) } } } }
        }
    }
}

pub struct Lead {
    text: String,
    align: Option<&'static str>,
}

impl Lead {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            align: None,
        }
    }
    pub fn align(mut self, a: &'static str) -> Self {
        self.align = Some(a);
        self
    }
    pub fn center(self) -> Self {
        self.align("center")
    }
}

impl Render for Lead {
    fn render(&self) -> Markup {
        let align = self
            .align
            .map(|a| format!("text-align:{};", a))
            .unwrap_or_default();
        html! {
            p class="sh-lead" style={"font-size:1.25rem;color:var(--sh-fg);opacity:0.85;line-height:1.7;margin:0 0 24px;".to_string() + &align} {
                (self.text)
            }
        }
    }
}

pub struct Highlight {
    text: String,
    color: Option<String>,
}

impl Highlight {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: None,
        }
    }
    pub fn color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }
    pub fn primary(self) -> Self {
        self.color("var(--sh-primary)")
    }
    pub fn accent(self) -> Self {
        self.color("var(--sh-accent)")
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
}

impl Render for Highlight {
    fn render(&self) -> Markup {
        let c = self
            .color
            .clone()
            .unwrap_or_else(|| "var(--sh-primary)".to_string());
        html! {
            mark class="sh-highlight" style={"background:{};padding:2px 8px;border-radius:var(--sh-radius-sm);".replace("{}", &c)} {
                (self.text)
            }
        }
    }
}

pub struct GradientText {
    text: String,
    from: Option<String>,
    to: Option<String>,
}

impl GradientText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            from: None,
            to: None,
        }
    }
    pub fn from(mut self, c: impl Into<String>) -> Self {
        self.from = Some(c.into());
        self
    }
    pub fn to(mut self, c: impl Into<String>) -> Self {
        self.to = Some(c.into());
        self
    }
    pub fn colors(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self.to = Some(to.into());
        self
    }
}

impl Render for GradientText {
    fn render(&self) -> Markup {
        let from = self
            .from
            .clone()
            .unwrap_or_else(|| "var(--sh-primary)".to_string());
        let to = self
            .to
            .clone()
            .unwrap_or_else(|| "var(--sh-accent)".to_string());
        html! {
            span class="sh-gradient-text" style={"background:linear-gradient(120deg, {}, {});-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text;font-weight:700;".replace("{}", &from).replace("{}", &to)} {
                (self.text)
            }
        }
    }
}
