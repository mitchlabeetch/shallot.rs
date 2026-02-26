use maud::{html, Markup, Render};

pub struct Badge {
    text: String,
    variant: &'static str,
    color: Option<String>,
    size: &'static str,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: "default",
            color: None,
            size: "md",
        }
    }
    pub fn primary(self) -> Self {
        Self {
            variant: "primary",
            ..self
        }
    }
    pub fn success(self) -> Self {
        Self {
            variant: "success",
            ..self
        }
    }
    pub fn warning(self) -> Self {
        Self {
            variant: "warning",
            ..self
        }
    }
    pub fn error(self) -> Self {
        Self {
            variant: "error",
            ..self
        }
    }
    pub fn info(self) -> Self {
        Self {
            variant: "info",
            ..self
        }
    }
    pub fn outline(self) -> Self {
        Self {
            variant: "outline",
            ..self
        }
    }
    pub fn color(mut self, c: impl Into<String>) -> Self {
        self.color = Some(c.into());
        self
    }
    pub fn size_sm(self) -> Self {
        Self { size: "sm", ..self }
    }
    pub fn size_lg(self) -> Self {
        Self { size: "lg", ..self }
    }
}

impl Render for Badge {
    fn render(&self) -> Markup {
        let (bg, color, padding) = if let Some(ref c) = self.color {
            (format!("{}20", c), c.clone(), "2px 8px")
        } else {
            match self.variant {
                "primary" => ("var(--sh-primary-glass)", "var(--sh-primary)", "2px 8px"),
                "success" => ("rgba(34,197,94,0.1)", "#22c55e", "2px 8px"),
                "warning" => ("rgba(251,191,36,0.1)", "#fbbf24", "2px 8px"),
                "error" => ("rgba(239,68,68,0.1)", "#ef4444", "2px 8px"),
                "info" => ("rgba(59,130,246,0.1)", "#3b82f6", "2px 8px"),
                "outline" => ("transparent", "var(--sh-fg)", "2px 8px"),
                _ => ("var(--sh-surface)", "var(--sh-fg)", "2px 8px"),
            }
        };

        let font_size = match self.size {
            "sm" => "0.625rem",
            "lg" => "0.875rem",
            _ => "0.75rem",
        };

        html! {
            span class="sh-badge" style={"display:inline-flex;align-items:center;padding:{};font-size:{};font-weight:600;border-radius:var(--sh-radius-full);background:{};color:{};".replace("{}", &padding).replace("{}", &font_size).replace("{}", &bg).replace("{}", &color)} {
                (self.text)
            }
        }
    }
}

pub struct Tag {
    text: String,
    removable: bool,
}

impl Tag {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            removable: false,
        }
    }
    pub fn removable(mut self) -> Self {
        self.removable = true;
        self
    }
}

impl Render for Tag {
    fn render(&self) -> Markup {
        html! {
            span class="sh-tag" style="display:inline-flex;align-items:center;gap:6px;padding:4px 12px;font-size:0.875rem;background:var(--sh-surface);border:1px solid var(--sh-border);border-radius:var(--sh-radius-full);" {
                (self.text)
                @if self.removable {
                    button type="button" style="background:none;border:none;cursor:pointer;opacity:0.5;font-size:1rem;line-height:1;padding:0;width:16px;height:16px;display:flex;align-items:center;justify-content:center;border-radius:50%;" { "×" }
                }
            }
        }
    }
}

pub struct Tooltip {
    text: String,
    children: Markup,
    position: &'static str,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            children: html! {},
            position: "top",
        }
    }
    pub fn child(mut self, c: Markup) -> Self {
        self.children = c;
        self
    }
    pub fn bottom(self) -> Self {
        Self {
            position: "bottom",
            ..self
        }
    }
    pub fn left(self) -> Self {
        Self {
            position: "left",
            ..self
        }
    }
    pub fn right(self) -> Self {
        Self {
            position: "right",
            ..self
        }
    }
}

impl Render for Tooltip {
    fn render(&self) -> Markup {
        let (top, left, transform) = match self.position {
            "bottom" => ("100%", "50%", "translateX(-50%) translateY(8px)"),
            "left" => ("50%", "0", "translateX(-8px) translateY(-50%)"),
            "right" => ("50%", "100%", "translateX(8px) translateY(-50%)"),
            _ => ("0", "50%", "translateX(-50%) translateY(-8px)"),
        };

        html! {
            div class="sh-tooltip-wrapper" style="position:relative;display:inline-block;" {
                (self.children)
                div class="sh-tooltip" style={"position:absolute;top:{};left:{};transform:{};padding:8px 12px;background:var(--sh-fg);color:var(--sh-bg);font-size:0.75rem;border-radius:var(--sh-radius-md);white-space:nowrap;opacity:0;visibility:hidden;transition:all var(--sh-transition-fast);z-index:100;pointer-events:none;margin:8px;".replace("{}", &top).replace("{}", &left).replace("{}", &transform)} {
                    (self.text)
                }
            }
        }
    }
}

pub struct EmptyState {
    icon: Option<String>,
    title: String,
    description: Option<String>,
    action: Option<Markup>,
}

impl EmptyState {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            icon: None,
            title: title.into(),
            description: None,
            action: None,
        }
    }
    pub fn icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn action(mut self, a: Markup) -> Self {
        self.action = Some(a);
        self
    }
}

impl Render for EmptyState {
    fn render(&self) -> Markup {
        html! {
            div class="sh-empty-state" style="display:flex;flex-direction:column;align-items:center;justify-content:center;padding:64px 32px;text-align:center;" {
                @if let Some(ref icon) = self.icon {
                    div style="font-size:4rem;margin-bottom:16px;opacity:0.3;" { (icon) }
                }
                h3 style="margin:0 0 8px;font-size:1.25rem;font-weight:600;" { (self.title) }
                @if let Some(ref desc) = self.description {
                    p style="margin:0 0 24px;opacity:0.7;max-width:400px;" { (desc) }
                }
                @if let Some(action) = self.action {
                    (action)
                }
            }
        }
    }
}
