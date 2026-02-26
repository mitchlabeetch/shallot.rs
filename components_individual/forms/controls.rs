use maud::{html, Markup, Render};

pub struct Checkbox {
    name: String,
    value: Option<String>,
    checked: bool,
    label: Option<String>,
    required: bool,
    disabled: bool,
}

impl Checkbox {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            checked: false,
            label: None,
            required: false,
            disabled: false,
        }
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Render for Checkbox {
    fn render(&self) -> Markup {
        html! {
            label class="sh-checkbox" style="display:flex;align-items:center;gap:10px;cursor:pointer;user-select:none;" {
                input type="checkbox" name={(self.name)} value={(self.value.clone().unwrap_or("on"))} class="sh-checkbox-input" style="width:20px;height:20px;accent-color:var(--sh-primary);" @if self.checked { checked } @if self.required { required } @if self.disabled { disabled } {}
                @if let Some(ref label) = self.label { span style="color:var(--sh-fg);" { (label) } }
            }
        }
    }
}

pub struct Radio {
    name: String,
    value: String,
    checked: bool,
    label: Option<String>,
    required: bool,
    disabled: bool,
}

impl Radio {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            checked: false,
            label: None,
            required: false,
            disabled: false,
        }
    }
    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Render for Radio {
    fn render(&self) -> Markup {
        html! {
            label class="sh-radio" style="display:flex;align:10px;-items:center;gapcursor:pointer;user-select:none;" {
                input type="radio" name={(self.name)} value={(self.value)} class="sh-radio-input" style="width:20px;height:20px;accent-color:var(--sh-primary);" @if self.checked { checked } @if self.required { required } @if self.disabled { disabled } {}
                @if let Some(ref label) = self.label { span style="color:var(--sh-fg);" { (label) } }
            }
        }
    }
}

pub struct Switch {
    name: String,
    checked: bool,
    label: Option<String>,
    required: bool,
    disabled: bool,
}

impl Switch {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            checked: false,
            label: None,
            required: false,
            disabled: false,
        }
    }
    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Render for Switch {
    fn render(&self) -> Markup {
        html! {
            label class="sh-switch" style="display:flex;align-items:center;gap:12px;cursor:pointer;user-select:none;" {
                input type="checkbox" name={(self.name)} class="sh-switch-input" style="display:none;" @if self.checked { checked } @if self.required { required } @if self.disabled { disabled } {}
                span class="sh-switch-track" style="position:relative;width:48px;height:26px;background:var(--sh-border);border-radius:13px;transition:background var(--sh-transition-fast);" {
                    span class="sh-switch-thumb" style="position:absolute;top:3px;left:3px;width:20px;height:20px;background:white;border-radius:50%;transition:transform var(--sh-transition-fast);box-shadow:0 1px 3px rgba(0,0,0,0.2);" {}
                }
                @if let Some(ref label) = self.label { span style="color:var(--sh-fg);" { (label) } }
            }
        }
    }
}

pub struct Select {
    name: String,
    options: Vec<(String, String)>,
    value: Option<String>,
    placeholder: Option<String>,
    label: Option<String>,
    helper: Option<String>,
    error: Option<String>,
    required: bool,
    disabled: bool,
}

impl Select {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            options: Vec::new(),
            value: None,
            placeholder: None,
            label: None,
            helper: None,
            error: None,
            required: false,
            disabled: false,
        }
    }
    pub fn option(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        self.options.push((value.into(), label.into()));
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = Some(p.into());
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn helper(mut self, h: impl Into<String>) -> Self {
        self.helper = Some(h.into());
        self
    }
    pub fn error(mut self, e: impl Into<String>) -> Self {
        self.error = Some(e.into());
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Render for Select {
    fn render(&self) -> Markup {
        html! {
            div class="sh-select-wrapper" {
                @if let Some(ref label) = self.label {
                    label style="display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;" {
                        (label)
                        @if self.required { span style="color:var(--sh-error);margin-left:4px;" { "*" } }
                    }
                }

                select name={(self.name)} class="sh-select" style="width:100%;padding:12px 40px 12px 16px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);font-size:1rem;appearance:none;background-image:url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23888' stroke-width='2'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E\");background-repeat:no-repeat;background-position:right 12px center;cursor:pointer;" @if self.required { required } @if self.disabled { disabled } {
                    @if let Some(ref placeholder) = self.placeholder {
                        option value="" disabled selected { (placeholder) }
                    }
                    @for (value, label) in &self.options {
                        option value={(value)} @if self.value.as_ref() == Some(value) { selected } { (label) }
                    }
                }

                @if let Some(ref error) = self.error {
                    span style="display:block;margin-top:4px;font-size:0.75rem;color:var(--sh-error);" { (error) }
                }
            }
        }
    }
}

pub struct Slider {
    name: String,
    min: f64,
    max: f64,
    step: f64,
    value: f64,
    show_value: bool,
    label: Option<String>,
    required: bool,
    disabled: bool,
}

impl Slider {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 50.0,
            show_value: true,
            label: None,
            required: false,
            disabled: false,
        }
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn step(mut self, s: f64) -> Self {
        self.step = s;
        self
    }
    pub fn value(mut self, v: f64) -> Self {
        self.value = v;
        self
    }
    pub fn show_value(mut self, b: bool) -> Self {
        self.show_value = b;
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Render for Slider {
    fn render(&self) -> Markup {
        let percent = ((self.value - self.min) / (self.max - self.min)) * 100.0;

        html! {
            div class="sh-slider-wrapper" {
                @if let Some(ref label) = self.label {
                    div style="display:flex;justify-content:space-between;margin-bottom:8px;" {
                        label style="font-size:0.875rem;font-weight:500;" { (label) }
                        @if self.show_value { span style="font-size:0.875rem;opacity:0.7;" { (self.value) } }
                    }
                }

                input type="range" name={(self.name)} min={(self.min)} max={(self.max)} step={(self.step)} value={(self.value)} class="sh-slider" style="width:100%;height:6px;appearance:none;background:linear-gradient(to right, var(--sh-primary) 0%, var(--sh-primary) {}%, var(--sh-border) {}%, var(--sh-border) 100%);border-radius:3px;cursor:pointer;".replace("{}", &percent.to_string()).replace("{}", &percent.to_string()) @if self.required { required } @if self.disabled { disabled } {}
            }
        }
    }
}

pub struct FileUpload {
    name: String,
    accept: Option<String>,
    multiple: bool,
    label: Option<String>,
    required: bool,
    disabled: bool,
}

impl FileUpload {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            accept: None,
            multiple: false,
            label: None,
            required: false,
            disabled: false,
        }
    }
    pub fn accept(mut self, a: impl Into<String>) -> Self {
        self.accept = Some(a.into());
        self
    }
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }
    pub fn image(self) -> Self {
        self.accept("image/*")
    }
    pub fn video(self) -> Self {
        self.accept("video/*")
    }
    pub fn pdf(self) -> Self {
        self.accept(".pdf")
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Render for FileUpload {
    fn render(&self) -> Markup {
        html! {
            label class="sh-file-upload" style="display:flex;flex-direction:column;align-items:center;justify-content:center;padding:32px;border:2px dashed var(--sh-border);border-radius:var(--sh-radius-lg);cursor:pointer;transition:all var(--sh-transition-fast);" {
                input type="file" name={(self.name)} accept={(self.accept.clone().unwrap_or_default())} @if self.multiple { multiple } style="display:none;" @if self.required { required } @if self.disabled { disabled } {}

                svg style="width:48px;height:48px;margin-bottom:16px;opacity:0.5;" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" {
                    path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" {}
                    path d="M17 8l-5-5-5 5" {}
                    path d="M12 3v12" {}
                }

                @if let Some(ref label) = self.label {
                    span style="text-align:center;color:var(--sh-fg-muted);" { (label) }
                } @else {
                    span style="text-align:center;opacity:0.7;" { "Drop files here or click to upload" }
                }
            }
        }
    }
}

pub struct FormLabel {
    text: String,
    required: bool,
    for_id: Option<String>,
}

impl FormLabel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            required: false,
            for_id: None,
        }
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn for_id(mut self, id: impl Into<String>) -> Self {
        self.for_id = Some(id.into());
        self
    }
}

impl Render for FormLabel {
    fn render(&self) -> Markup {
        html! {
            label @if let Some(ref id) = self.for_id { for={(id)} } style="display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;color:var(--sh-fg);" {
                (self.text)
                @if self.required { span style="color:var(--sh-error);margin-left:4px;" { "*" } }
            }
        }
    }
}

pub struct FormError {
    text: String,
}

impl FormError {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Render for FormError {
    fn render(&self) -> Markup {
        html! { span style="display:block;margin-top:4px;font-size:0.75rem;color:var(--sh-error);" { (self.text) } }
    }
}

pub struct FormHelper {
    text: String,
}

impl FormHelper {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Render for FormHelper {
    fn render(&self) -> Markup {
        html! { span style="display:block;margin-top:4px;font-size:0.75rem;opacity:0.7;" { (self.text) } }
    }
}
