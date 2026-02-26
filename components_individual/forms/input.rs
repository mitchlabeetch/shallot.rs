use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Search,
    Url,
    Tel,
    Date,
    Time,
    Color,
}

impl InputType {
    fn to_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Number => "number",
            InputType::Search => "search",
            InputType::Url => "url",
            InputType::Tel => "tel",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::Color => "color",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputVariant {
    Default,
    Outlined,
    Filled,
    Glass,
    Flat,
}

pub struct Input {
    name: String,
    input_type: InputType,
    value: Option<String>,
    placeholder: Option<String>,
    label: Option<String>,
    helper: Option<String>,
    error: Option<String>,
    variant: InputVariant,
    required: bool,
    disabled: bool,
    readonly: bool,
    autocomplete: Option<String>,
    min: Option<String>,
    max: Option<String>,
    minlength: Option<usize>,
    maxlength: Option<usize>,
    pattern: Option<String>,
}

impl Input {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_type: InputType::Text,
            value: None,
            placeholder: None,
            label: None,
            helper: None,
            error: None,
            variant: InputVariant::Outlined,
            required: false,
            disabled: false,
            readonly: false,
            autocomplete: None,
            min: None,
            max: None,
            minlength: None,
            maxlength: None,
            pattern: None,
        }
    }

    pub fn text(self) -> Self {
        Self {
            input_type: InputType::Text,
            ..self
        }
    }
    pub fn password(self) -> Self {
        Self {
            input_type: InputType::Password,
            ..self
        }
    }
    pub fn email(self) -> Self {
        Self {
            input_type: InputType::Email,
            ..self
        }
    }
    pub fn number(self) -> Self {
        Self {
            input_type: InputType::Number,
            ..self
        }
    }
    pub fn search(self) -> Self {
        Self {
            input_type: InputType::Search,
            ..self
        }
    }
    pub fn url(self) -> Self {
        Self {
            input_type: InputType::Url,
            ..self
        }
    }
    pub fn tel(self) -> Self {
        Self {
            input_type: InputType::Tel,
            ..self
        }
    }
    pub fn date(self) -> Self {
        Self {
            input_type: InputType::Date,
            ..self
        }
    }
    pub fn time(self) -> Self {
        Self {
            input_type: InputType::Time,
            ..self
        }
    }
    pub fn color(self) -> Self {
        Self {
            input_type: InputType::Color,
            ..self
        }
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
    pub fn variant(mut self, v: InputVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn outlined(self) -> Self {
        self.variant(InputVariant::Outlined)
    }
    pub fn filled(self) -> Self {
        self.variant(InputVariant::Filled)
    }
    pub fn glass(self) -> Self {
        self.variant(InputVariant::Glass)
    }
    pub fn flat(self) -> Self {
        self.variant(InputVariant::Flat)
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
    pub fn autocomplete(mut self, a: impl Into<String>) -> Self {
        self.autocomplete = Some(a.into());
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
    pub fn minlength(mut self, n: usize) -> Self {
        self.minlength = Some(n);
        self
    }
    pub fn maxlength(mut self, n: usize) -> Self {
        self.maxlength = Some(n);
        self
    }
    pub fn pattern(mut self, p: impl Into<String>) -> Self {
        self.pattern = Some(p.into());
        self
    }

    fn base_style(&self) -> String {
        let has_error = self.error.is_some();

        let (border, bg) = match self.variant {
            InputVariant::Outlined => ("1px solid var(--sh-border)", "transparent"),
            InputVariant::Filled => ("none", "var(--sh-surface)"),
            InputVariant::Glass => ("1px solid rgba(255,255,255,0.1)", "rgba(255,255,255,0.05)"),
            InputVariant::Flat => ("none", "transparent"),
            InputVariant::Default => ("none", "transparent"),
        };

        let border_color = if has_error {
            "var(--sh-error)"
        } else {
            "var(--sh-border)"
        };

        let mut s = format!("width:100%;padding:12px 16px;border:{};border-radius:var(--sh-radius-md);background:{};color:var(--sh-fg);font-size:1rem;transition:border-color var(--sh-transition-fast),box-shadow var(--sh-transition-fast);", border, bg);

        if self.variant == InputVariant::Glass {
            s.push_str("backdrop-filter:blur(8px);");
        }

        s
    }
}

impl Render for Input {
    fn render(&self) -> Markup {
        let has_error = self.error.is_some();

        html! {
            div class="sh-input-wrapper" {
                @if let Some(ref label) = self.label {
                    label style="display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;color:var(--sh-fg);" {
                        (label)
                        @if self.required {
                            span style="color:var(--sh-error);margin-left:4px;" { "*" }
                        }
                    }
                }

                input
                    type={(self.input_type.to_str())}
                    name={(self.name)}
                    value={(self.value.clone().unwrap_or_default())}
                    placeholder={(self.placeholder.clone().unwrap_or_default())}
                    autocomplete={(self.autocomplete.clone().unwrap_or_default())}
                    min={(self.min.clone().unwrap_or_default())}
                    max={(self.max.clone().unwrap_or_default())}
                    pattern={(self.pattern.clone().unwrap_or_default())}
                    class="sh-input"
                    style={(self.base_style())}
                    @if self.required { required }
                    @if self.disabled { disabled }
                    @if self.readonly { readonly }
                {}

                @if let Some(ref error) = self.error {
                    span class="sh-input-error" style="display:block;margin-top:4px;font-size:0.75rem;color:var(--sh-error);" {
                        (error)
                    }
                }

                @if let Some(ref helper) = self.helper {
                    @if self.error.is_none() {
                        span class="sh-input-helper" style="display:block;margin-top:4px;font-size:0.75rem;opacity:0.7;" {
                            (helper)
                        }
                    }
                }
            }
        }
    }
}

pub struct Textarea {
    name: String,
    value: Option<String>,
    placeholder: Option<String>,
    label: Option<String>,
    helper: Option<String>,
    error: Option<String>,
    rows: u32,
    cols: Option<u32>,
    required: bool,
    disabled: bool,
    readonly: bool,
    resize: bool,
}

impl Textarea {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            placeholder: None,
            label: None,
            helper: None,
            error: None,
            rows: 4,
            cols: None,
            required: false,
            disabled: false,
            readonly: false,
            resize: true,
        }
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
    pub fn rows(mut self, r: u32) -> Self {
        self.rows = r;
        self
    }
    pub fn cols(mut self, c: u32) -> Self {
        self.cols = Some(c);
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
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
    pub fn no_resize(self) -> Self {
        self.resize = false;
        self
    }
}

impl Render for Textarea {
    fn render(&self) -> Markup {
        html! {
            div class="sh-textarea-wrapper" {
                @if let Some(ref label) = self.label {
                    label style="display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;" {
                        (label)
                        @if self.required { span style="color:var(--sh-error);margin-left:4px;" { "*" } }
                    }
                }

                textarea
                    name={(self.name)}
                    rows={(self.rows)}
                    @if let Some(c) = self.cols { cols={(c)} }
                    placeholder={(self.placeholder.clone().unwrap_or_default())}
                    class="sh-textarea"
                    style={"width:100%;padding:12px 16px;border:1px solid var(--sh-border);border-radius:var(--sh-radius-md);background:transparent;color:var(--sh-fg);font-size:1rem;resize:{};transition:border-color var(--sh-transition-fast);".replace("{}", if self.resize { "vertical" } else { "none" })}
                    @if self.required { required }
                    @if self.disabled { disabled }
                    @if self.readonly { readonly }
                {
                    @if let Some(ref v) = self.value { (v) }
                }

                @if let Some(ref error) = self.error {
                    span style="display:block;margin-top:4px;font-size:0.75rem;color:var(--sh-error);" { (error) }
                }
            }
        }
    }
}
