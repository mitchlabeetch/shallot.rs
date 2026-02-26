use maud::{html, Markup, Render};

pub struct Modal {
    id: String,
    title: Option<String>,
    content: Markup,
    footer: Option<Markup>,
    size: &'static str,
}

impl Modal {
    pub fn new(id: impl Into<String>, content: Markup) -> Self { Self { id: id.into(), title: None, content, footer: None, size: "md" } }
    pub fn title(mut self, t: impl Into<String>) -> Self { self.title = Some(t.into()); self }
    pub fn footer(mut self, f: Markup) -> Self { self.footer = Some(f); self }
    pub fn size_sm(self) -> Self { Self { size: "sm", ..self } }
    pub fn size_lg(self) -> Self { Self { size: "lg", ..self } }
    pub fn size_xl(self) -> Self { Self { size: "xl", ..self } }
    pub fn size_full(self) -> Self { Self { size: "full", ..self } }
}

impl Render for Modal {
    fn render(&self) -> Markup {
        let max_w = match self.size { "sm" => "400px", "lg" => "800px", "xl" => "95vw", "full" => "95vw", _ => "500px" };
        
        html! {
            div id={(self.id)} class="sh-modal-overlay" style="position:fixed;inset:0;background:rgba(0,0,0,0.8);display:flex;align-items:center;justify-content:center;opacity:0;pointer-events:none;transition:opacity var(--sh-transition-normal);z-index:1000;" {
                a href="#" class="sh-modal-backdrop" style="position:absolute;inset:0;cursor:default;" {}
                div class="sh-modal-card" style={"position:relative;width:90%;max-width:{};max-height:90vh;overflow-y:auto;background:var(--sh-surface);border:1px solid var(--sh-border);border-radius:var(--sh-radius-lg);padding:24px;transform:scale(0.95);transition:transform var(--sh-transition-normal);".replace("{}", max_w)} {
                    a href="#" class="sh-modal-close" style="position:absolute;top:16px;right:16px;width:32px;height:32px;display:flex;align-items:center;justify-content:center;border-radius:var(--sh-radius-md);text-decoration:none;color:var(--sh-fg);opacity:0.7;transition:opacity var(--sh-transition-fast);font-size:1.5rem;line-height:1;" { "×" }
                    
                    @if let Some(ref title) = self.title {
                        h2 style="margin:0 0 16px;font-size:1.25rem;font-weight:600;" { (title) }
                    }
                    
                    (self.content)
                    
                    @if let Some(ref footer) = self.footer {
                        div style="margin-top:24px;padding-top:16px;border-top:1px solid var(--sh-border);display:flex;justify-content:flex-end;gap:12px;" { (footer) }
                    }
                }
            }
        }
    }
}

pub struct Drawer {
    id: String,
    title: Option<String>,
    content: Markup,
    position: &'static str,
    size: &'static str,
}

impl Drawer {
    pub fn new(id: impl Into<String>, content: Markup) -> Self { Self { id: id.into(), title: None, content, position: "right", size: "md" } }
    pub fn title(mut self, t: impl Into<String>) -> Self { self.title = Some(t.into()); self }
    pub fn left(self) -> Self { Self { position: "left", ..self } }
    pub fn top(self) -> Self { Self { position: "top", ..self } }
    pub fn bottom(self) -> Self { Self { position: "bottom", ..self } }
    pub fn size_sm(self) -> Self { Self { size: "sm", ..self } }
    pub fn size_lg(self) -> Self { Self { size: "lg", ..self } }
}

impl Render for Drawer {
    fn render(&self) -> Markup {
        let (pos, transform, width, height) = match (self.position, self.size) {
            ("left", "sm") => ("left:0;top:0;", "translateX(-100%)", "300px", "100vh"),
            ("left", "lg") => ("left:0;top:0;", "translateX(-100%)", "500px", "100vh"),
            ("right", "sm") => ("right:0;top:0;", "translateX(100%)", "300px", "100vh"),
            ("right", "lg") => ("right:0;top:0;", "translateX(100%)", "500px", "100vh"),
            ("top", _) => ("top:0;left:0;", "translateY(-100%)", "100vw", "300px"),
            ("bottom", _) => ("bottom:0;left:0;", "translateY(100%)", "100vw", "300px"),
            _ => ("right:0;top:0;", "translateX(100%)", "400px", "100vh"),
        };
        
        html! {
            div id={(self.id)} class="sh-drawer-overlay" style="position:fixed;inset:0;background:rgba(0,0,0,0.5);opacity:0;pointer-events:none;transition:opacity var(--sh-transition-normal);z-index:999;" {}
            div class="sh-drawer" style={"position:fixed;{}width:{};height:{};background:var(--sh-surface);border-right:1px solid var(--sh-border);transform:{};transition:transform var(--sh-transition-normal);z-index:1000;".replace("{}", &pos).replace("{}", &width).replace("{}", &height).replace("{}", &transform)} {
                @if let Some(ref title) = self.title {
                    div style="display:flex;align-items:center;justify-content:space-between;padding:16px;border-bottom:1px solid var(--sh-border);" {
                        h3 style="margin:0;font-size:1.125rem;font-weight:600;" { (title) }
                        a href="#" style="text-decoration:none;color:var(--sh-fg);font-size:1.5rem;opacity:0.7;" { "×" }
                    }
                }
                div style="padding:16px;overflow-y:auto;height:calc(100% - 60px);" { (self.content) }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AlertKind { Info, Success, Warning, Error }

pub struct Alert {
    kind: AlertKind,
    title: Option<String>,
    message: String,
    icon: bool,
    dismissible: bool,
}

impl Alert {
    pub fn new(message: impl Into<String>) -> Self { Self { kind: AlertKind::Info, title: None, message: message.into(), icon: true, dismissible: false } }
    pub fn info(self) -> Self { Self { kind: AlertKind::Info, ..self } }
    pub fn success(self) -> Self { Self { kind: AlertKind::Success, ..self } }
    pub fn warning(self) -> Self { Self { kind: AlertKind::Warning, ..self } }
    pub fn error(self) -> Self { Self { kind: AlertKind::Error, ..self } }
    pub fn title(mut self, t: impl Into<String>) -> Self { self.title = Some(t.into()); self }
    pub fn icon(mut self, b: bool) -> Self { self.icon = b; self }
    pub fn dismissible(mut self) -> Self { self.dismissible = true; self }
}

impl Render for Alert {
    fn render(&self) -> Markup {
        let (bg, border, icon) = match self.kind {
            AlertKind::Info => ("rgba(59,130,246,0.1)", "rgb(59,130,246)", "ℹ"),
            AlertKind::Success => ("rgba(34,197,94,0.1)", "rgb(34,197,94)", "✓"),
            AlertKind::Warning => ("rgba(251,191,36,0.1)", "rgb(251,191,36)", "⚠"),
            AlertKind::Error => ("rgba(239,68,68,0.1)", "rgb(239,68,68)", "✕"),
        };
        
        html! {
            div class="sh-alert" role="alert" style={"display:flex;gap:12px;padding:16px;background:{};border:1px solid {};border-radius:var(--sh-radius-lg);".replace("{}", &bg).replace("{}", &border)} {
                @if self.icon {
                    span style="font-size:1.25rem;flex-shrink:0;" { (icon) }
                }
                div style="flex:1;" {
                    @if let Some(ref title) = self.title {
                        h4 style="margin:0 0 4px;font-weight:600;" { (title) }
                    }
                    p style="margin:0;font-size:0.875rem;opacity:0.9;" { (self.message) }
                }
                @if self.dismissible {
                    button type="button" style="background:none;border:none;cursor:pointer;opacity:0.7;font-size:1.25rem;flex-shrink:0;" { "×" }
                }
            }
        }
    }
}

pub struct Toast {
    message: String,
    kind: AlertKind,
    duration: u32,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self { Self { message: message.into(), kind: AlertKind::Info, duration: 5000 } }
    pub fn success(self) -> Self { Self { kind: AlertKind::Success, ..self } }
    pub fn warning(self) -> Self { Self { kind: AlertKind::Warning, ..self } }
    pub fn error(self) -> Self { Self { kind: AlertKind::Error, ..self } }
    pub fn duration(mut self, ms: u32) -> Self { self.duration = ms; self }
}

impl Render for Toast {
    fn render(&self) -> Markup {
        let border = match self.kind {
            AlertKind::Info => "var(--sh-primary)",
            AlertKind::Success =>e",
            Alert "#22c55Kind::Warning => "#fbbf24",
            AlertKind::Error => "#ef4444",
        };
        
        html! {
            div class="sh-toast" role="status" style={"position:fixed;bottom:24px;right:24px;max-width:400px;padding:16px;background:var(--sh-surface);border:1px solid {};border-radius:var(--sh-radius-lg);box-shadow:var(--sh-shadow-lg);z-index:1100;animation:sh-slide-up 0.3s var(--sh-ease-bionic);".replace("{}", border)} {
                p style="margin:0;font-size:0.875rem;" { (self.message) }
            }
        }
    }
}

pub struct Spinner {
    size: u32,
}

impl Spinner {
    pub fn new() -> Self { Self { size: 24 } }
    pub fn size(mut self, s: u32) -> Self { self.size = s; self }
    pub fn sm(self) -> Self { self.size(16) }
    pub fn md(self) -> Self { self.size(24) }
    pub fn lg(self) -> Self { self.size(40) }
}

impl Default for Spinner { fn default() -> Self { Self::new() } }

impl Render for Spinner {
    fn render(&self) -> Markup {
        html! {
            svg class="sh-spinner" style={"width:{}px;height:{}px;animation:sh-spin 1s linear infinite;color:var(--sh-primary);".replace("{}", &self.size.to_string())} viewBox="0 0 24 24" fill="none" {
                circle style="opacity:0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" {}
                path style="opacity:0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" {}
            }
        }
    }
}

pub struct ProgressBar {
    value: u32,
    max: u32,
    show_value: bool,
    striped: bool,
    animated: bool,
}

impl ProgressBar {
    pub fn new() -> Self { Self { value: 0, max: 100, show_value: false, striped: false, animated: false } }
    pub fn value(mut self, v: u32) -> Self { self.value = v; self }
    pub fn max(mut self, m: u32) -> Self { self.max = m; self }
    pub fn show_value(mut self) -> Self { self.show_value = true; self }
    pub fn striped(mut self) -> Self { self.striped = true; self }
    pub fn animated(mut self) -> Self { self.animated = true; self }
}

impl Default for ProgressBar { fn default() -> Self { Self::new() } }

impl Render for ProgressBar {
    fn render(&self) -> Markup {
        let percent = (self.value as f32 / self.max as f32 * 100.0) as u32;
        let bg = if self.striped { "linear-gradient(45deg,rgba(255,255,255,.15) 25%,transparent 25%,transparent 50%,rgba(255,255,255,.15) 50%,rgba(255,255,255,.15) 75%,transparent 75%,transparent);background-size:1rem 1rem;" } else { "" };
        
        html! {
            div class="sh-progress-wrapper" {
                @if self.show_value {
                    div style="display:flex;justify-content:space-between;margin-bottom:8px;font-size:0.875rem;" {
                        span { "Progress" }
                        span { (format!("{}%", percent)) }
                    }
                }
                div class="sh-progress-track" style="height:8px;background:var(--sh-border);border-radius:var(--sh-radius-full);overflow:hidden;" {
                    div class="sh-progress-bar" style={"height:100%;width:{}%;background:var(--sh-primary);border-radius:var(--sh-radius-full);transition:width var(--sh-transition-slow);{}".replace("{}", &percent.to_string())} {}
                }
            }
        }
    }
}

pub struct CircularProgress {
    value: u32,
    max: u32,
    size: u32,
    stroke_width: u32,
    show_value: bool,
}

impl CircularProgress {
    pub fn new() -> Self { Self { value: 0, max: 100, size: 120, stroke_width: 8, show_value: true } }
    pub fn value(mut self, v: u32) -> Self { self.value = v; self }
    pub fn size(mut self, s: u32) -> Self { self.size = s; self }
    pub fn show_value(mut self, b: bool) -> Self { self.show_value = b; self }
}

impl Default for CircularProgress { fn default() -> Self { Self::new() } }

impl Render for CircularProgress {
    fn render(&self) -> Markup {
        let radius = (self.size - self.stroke_width) / 2;
        let circumference = 2.0 * std::f32::consts::PI * (radius as f32);
        let percent = self.value as f32 / self.max as f32;
        let offset = circumference * (1.0 - percent);
        
        html! {
            div class="sh-circular-progress" style={"position:relative;width:{}px;height:{}px;".replace("{}", &self.size.to_string()).replace("{}", &self.size.to_string())} {
                svg style={"transform:rotate(-90deg);width:{}px;height:{}px;".replace("{}", &self.size.to_string()).replace("{}", &self.size.to_string())} {
                    circle fill="none" stroke="var(--sh-border)" cx={(self.size/2)} cy={(self.size/2)} r={(radius)} stroke-width={(self.stroke_width)} {}
                    circle fill="none" stroke="var(--sh-primary)" cx={(self.size/2)} cy={(self.size/2)} r={(radius)} stroke-width={(self.stroke_width)} stroke-dasharray={(circumference)} stroke-dashoffset={(offset)} stroke-linecap="round" style="transition:stroke-dashoffset 0.5s var(--sh-ease-bionic);" {}
                }
                @if self.show_value {
                    span style="position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);font-size:1.5rem;font-weight:600;" { (format!("{}%", (percent * 100.0) as u32)) }
                }
            }
        }
    }
}
