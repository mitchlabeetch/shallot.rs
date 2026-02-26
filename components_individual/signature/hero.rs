use maud::{html, Markup, Render};

pub struct HeroSection {
    title: String,
    subtitle: Option<String>,
    actions: Vec<Markup>,
}

impl HeroSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            actions: Vec::new(),
        }
    }
    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into());
        self
    }
    pub fn action(mut self, a: Markup) -> Self {
        self.actions.push(a);
        self
    }
}

impl Render for HeroSection {
    fn render(&self) -> Markup {
        html! {
            section class="sh-hero" style="position:relative;min-height:80vh;display:flex;flex-direction:column;align-items:center;justify-content:center;text-align:center;padding:64px 24px;overflow:hidden;background:radial-gradient(ellipse at top, var(--sh-primary-glass) 0%, transparent 70%);" {
                div class="sh-hero-grid" style="position:absolute;inset:0;background-image:linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px);background-size:60px 60px;mask-image:radial-gradient(ellipse at center, black, transparent 80%);animation:sh-grid-drift 20s linear infinite;" {}

                h1 class="sh-hero-title" style="position:relative;z-index:1;font-size:clamp(2.5rem, 8vw, 5rem);font-weight:800;letter-spacing:-0.02em;margin:0 0 16px;background:linear-gradient(to bottom right, #fff 30%, rgba(255,255,255,0.5));-webkit-background-clip:text;-webkit-text-fill-color:transparent;animation:sh-fade-up 0.6s var(--sh-ease-bionic) both;" {
                    (self.title)
                }

                @if let Some(ref subtitle) = self.subtitle {
                    p class="sh-hero-subtitle" style="position:relative;z-index:1;font-size:1.25rem;opacity:0.7;margin:0 0 32px;max-width:600px;animation:sh-fade-up 0.6s var(--sh-ease-bionic) 0.1s both;" { (subtitle) }
                }

                @if !self.actions.is_empty() {
                    div class="sh-hero-actions" style="position:relative;z-index:1;display:flex;gap:16px;flex-wrap:wrap;justify-content:center;animation:sh-fade-up 0.6s var(--sh-ease-bionic) 0.2s both;" {
                        @for action in &self.actions { (action) }
                    }
                }
            }
        }
    }
}

pub struct GlowText {
    text: String,
    color: Option<String>,
}

impl GlowText {
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
}

impl Render for GlowText {
    fn render(&self) -> Markup {
        let c = self
            .color
            .clone()
            .unwrap_or_else(|| "var(--sh-primary)".to_string());

        html! {
            span class="sh-glow-text" style={"color:{};text-shadow:0 0 10px {}, 0 0 20px {}, 0 0 40px {};animation:sh-glow 2s ease-in-out infinite alternate;".replace("{}", &c).replace("{}", &c).replace("{}", &c).replace("{}", &c)} {
                (self.text)
            }
        }
    }
}

pub struct Button {
    label: String,
    variant: &'static str,
    size: &'static str,
    href: Option<String>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: "primary",
            size: "md",
            href: None,
        }
    }
    pub fn primary(self) -> Self {
        Self {
            variant: "primary",
            ..self
        }
    }
    pub fn secondary(self) -> Self {
        Self {
            variant: "secondary",
            ..self
        }
    }
    pub fn ghost(self) -> Self {
        Self {
            variant: "ghost",
            ..self
        }
    }
    pub fn outline(self) -> Self {
        Self {
            variant: "outline",
            ..self
        }
    }
    pub fn glass(self) -> Self {
        Self {
            variant: "glass",
            ..self
        }
    }
    pub fn size_sm(self) -> Self {
        Self { size: "sm", ..self }
    }
    pub fn size_lg(self) -> Self {
        Self { size: "lg", ..self }
    }
    pub fn href(mut self, h: impl Into<String>) -> Self {
        self.href = Some(h.into());
        self
    }
}

impl Render for Button {
    fn render(&self) -> Markup {
        let (padding, font_size) = match self.size {
            "sm" => ("8px 16px", "0.875rem"),
            "lg" => ("16px 32px", "1.125rem"),
            _ => ("12px 24px", "1rem"),
        };

        let bg = match self.variant {
            "primary" => "var(--sh-primary)",
            "secondary" => "var(--sh-surface)",
            "ghost" => "transparent",
            "glass" => "rgba(255,255,255,0.1)",
            "outline" => "transparent",
            _ => "var(--sh-primary)",
        };

        let border = if self.variant == "outline" {
            "1px solid var(--sh-border)"
        } else {
            "none"
        };

        let content = html! {
            span style="position:relative;z-index:1;" { (self.label) }
        };

        if let Some(ref href) = self.href {
            html! {
                a href={(href)} class="sh-btn" style={"display:inline-flex;align-items:center;justify-content:center;gap:8px;padding:{};background:{};color:white;border:{};border-radius:var(--sh-radius-md);font-weight:600;font-size:{};text-decoration:none;transition:all var(--sh-transition-fast);".replace("{}", &padding).replace("{}", &bg).replace("{}", &border).replace("{}", &font_size)} {
                    (content)
                }
            }
        } else {
            html! {
                button class="sh-btn" style={"display:inline-flex;align-items:center;justify-content:center;gap:8px;padding:{};background:{};color:white;border:{};border-radius:var(--sh-radius-md);font-weight:600;font-size:{};cursor:pointer;transition:all var(--sh-transition-fast);".replace("{}", &padding).replace("{}", &bg).replace("{}", &border).replace("{}", &font_size)} {
                    (content)
                }
            }
        }
    }
}

pub struct IconButton {
    icon: String,
    label: Option<String>,
    size: &'static str,
}

impl IconButton {
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            label: None,
            size: "md",
        }
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn size_sm(self) -> Self {
        Self { size: "sm", ..self }
    }
    pub fn size_lg(self) -> Self {
        Self { size: "lg", ..self }
    }
}

impl Render for IconButton {
    fn render(&self) -> Markup {
        let (s, fs) = match self.size {
            "sm" => ("32px", "1rem"),
            "lg" => ("48px", "1.5rem"),
            _ => ("40px", "1.25rem"),
        };

        html! {
            button class="sh-icon-btn" aria-label={(self.label.clone())} style={"width:{};height:{};display:flex;align-items:center;justify-content:center;background:transparent;border:none;border-radius:var(--sh-radius-md);cursor:pointer;transition:all var(--sh-transition-fast);font-size:{};color:var(--sh-fg);".replace("{}", &s).replace("{}", &s).replace("{}", &fs)} {
                (self.icon.clone())
            }
        }
    }
}

pub struct Marquee {
    items: Vec<String>,
    speed: u32,
    pause_on_hover: bool,
}

impl Marquee {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            speed: 30,
            pause_on_hover: true,
        }
    }
    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(text.into());
        self
    }
    pub fn speed(mut self, s: u32) -> Self {
        self.speed = s;
        self
    }
    pub fn pause_on_hover(mut self, b: bool) -> Self {
        self.pause_on_hover = b;
        self
    }
}

impl Default for Marquee {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Marquee {
    fn render(&self) -> Markup {
        html! {
            div class="sh-marquee" style="overflow:hidden;white-space:nowrap;" {
                @if self.pause_on_hover {
                    div style="display:inline-flex;animation:sh-marquee {}s linear infinite;".replace("{}", &(self.speed * 100).to_string()) {
                        @for _ in 0..4 {
                            @for item in &self.items {
                                span style="padding:0 24px;font-size:1.5rem;font-weight:700;opacity:0.8;" { (item.clone()) }
                            }
                        }
                    }
                }
            }
        }
    }
}
