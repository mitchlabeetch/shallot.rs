use maud::{html, Markup, Render};

// Special Form Components
pub struct DatePicker {
    name: String,
    value: Option<String>,
    min: Option<String>,
    max: Option<String>,
    label: Option<String>,
}

impl DatePicker {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            min: None,
            max: None,
            label: None,
        }
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    pub fn min(mut self, m: impl Into<String>) -> Self {
        self.min = Some(m.into());
        self
    }
    pub fn max(mut self, m: impl Into<String>) -> Self {
        self.max = Some(m.into());
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
}

impl Render for DatePicker {
    fn render(&self) -> Markup {
        html! {
            div {
                @if let Some(ref label) = self.label { label style="display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;" { (label) } }
                input type="date" name={(self.name)} value={(self.value.clone().unwrap_or_default())} min={(self.min.clone().unwrap_or_default())} max={(self.max.clone().unwrap_or_default())} style="width:100%;padding:12px 16px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);font-size:1rem;" {}
            }
        }
    }
}

pub struct TimePicker {
    name: String,
    value: Option<String>,
    label: Option<String>,
}

impl TimePicker {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            label: None,
        }
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
}

impl Render for TimePicker {
    fn render(&self) -> Markup {
        html! {
            div {
                @if let Some(ref label) = self.label { label style="display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;" { (label) } }
                input type="time" name={(self.name)} value={(self.value.clone().unwrap_or_default())} style="width:100%;padding:12px 16px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);font-size:1rem;" {}
            }
        }
    }
}

pub struct ColorPicker {
    name: String,
    value: Option<String>,
    label: Option<String>,
}

impl ColorPicker {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            label: None,
        }
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
}

impl Render for ColorPicker {
    fn render(&self) -> Markup {
        html! {
            div {
                @if let Some(ref label) = self.label { label style="display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;" { (label) } }
                div style="display:flex;align-items:center;gap:12px;" {
                    input type="color" name={(self.name)} value={(self.value.clone().unwrap_or_else(|| "#000000".to_string()))} style="width:48px;height:48px;border:none;border-radius:var(--sh-radius-md);cursor:pointer;background:none;" {}
                    input type="text" value={(self.value.clone().unwrap_or_else(|| "#000000".to_string()))} style="flex:1;padding:12px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;font-family:var(--sh-font-mono);" {}
                }
            }
        }
    }
}

pub struct PinInput {
    name: String,
    length: u32,
}

impl PinInput {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            length: 6,
        }
    }
    pub fn length(mut self, l: u32) -> Self {
        self.length = l;
        self
    }
}

impl Render for PinInput {
    fn render(&self) -> Markup {
        html! {
            div class="sh-pin-input" style="display:flex;gap:8px;justify-content:center;" {
                @for i in 0..self.length {
                    input type="text" name={(format!("{}_{}", self.name, i))} maxlength="1" inputmode="numeric" pattern="[0-9]*" class="sh-pin-digit" style="width:48px;height:56px;text-align:center;font-size:1.5rem;font-weight:600;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);" {}
                }
            }
        }
    }
}

pub struct CreditCardInput {
    name: String,
}

impl CreditCardInput {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Render for CreditCardInput {
    fn render(&self) -> Markup {
        html! {
            div class="sh-credit-card" style="display:flex;gap:8px;" {
                input type="text" name={(format!("{}_number", self.name))} placeholder="1234 5678 9012 3456" pattern="[0-9 ]*" maxlength="19" style="flex:2;padding:12px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;font-family:var(--sh-font-mono);" {}
                input type="text" name={(format!("{}_expiry", self.name))} placeholder="MM/YY" pattern="[0-9/]*" maxlength="5" style="flex:1;padding:12px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;" {}
                input type="text" name={(format!("{}_cvc", self.name))} placeholder="CVC" pattern="[0-9]*" maxlength="4" style="width:80px;padding:12px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;" {}
            }
        }
    }
}

pub struct SearchInput {
    name: String,
    placeholder: Option<String>,
}

impl SearchInput {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            placeholder: None,
        }
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = Some(p.into());
        self
    }
}

impl Render for SearchInput {
    fn render(&self) -> Markup {
        html! {
            div style="position:relative;" {
                input type="search" name={(self.name)} placeholder={(self.placeholder.clone().unwrap_or("Search...".to_string()))} style="width:100%;padding:12px 16px 12px 44px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);font-size:1rem;" {}
                svg style="position:absolute;left:14px;top:50%;transform:translateY(-50%);width:20px;height:20px;opacity:0.5;" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" { circle cx="11" cy="11" r="8" {} path d="m21 21-4.35-4.35" {} }
            }
        }
    }
}

pub struct PhoneInput {
    name: String,
    country_code: Option<String>,
}

impl PhoneInput {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            country_code: Some("+1".to_string()),
        }
    }
    pub fn country_code(mut self, c: impl Into<String>) -> Self {
        self.country_code = Some(c.into());
        self
    }
}

impl Render for PhoneInput {
    fn render(&self) -> Markup {
        html! {
            div style="display:flex;gap:8px;" {
                @if let Some(ref cc) = self.country_code {
                    select name={(format!("{}_country", self.name))} style="width:80px;padding:12px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);" {
                        option value="+1" { "+1" }
                        option value="+44" { "+44" }
                        option value="+33" { "+33" }
                    }
                }
                input type="tel" name={(self.name)} placeholder="555-123-4567" style="flex:1;padding:12px 16px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);font-size:1rem;" {}
            }
        }
    }
}

pub struct RangeSlider {
    name: String,
    min: f64,
    max: f64,
    min_value: f64,
    max_value: f64,
}

impl RangeSlider {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            min: 0.0,
            max: 100.0,
            min_value: 20.0,
            max_value: 80.0,
        }
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn values(mut self, min: f64, max: f64) -> Self {
        self.min_value = min;
        self.max_value = max;
        self
    }
}

impl Render for RangeSlider {
    fn render(&self) -> Markup {
        html! {
            div class="sh-range-slider" {
                div style="position:relative;height:8px;background:var(--sh-border);border-radius:4px;margin:24px 0 16px;" {
                    div style={"position:absolute;left:{}%;right:{}%;height:100%;background:var(--sh-primary);border-radius:4px;".replace("{}", &((self.min_value - self.min) / (self.max - self.min) * 100.0).to_string()).replace("{}", &(100.0 - (self.max_value - self.min) / (self.max - self.min) * 100.0).to_string())} {}
                }
                div style="display:flex;justify-content:space-between;font-size:0.875rem;opacity:0.7;" {
                    span { (self.min_value) }
                    span { (self.max_value) }
                }
            }
        }
    }
}
