//! Box Component - Universal Layout Primitive
//!
//! The Box component is the universal layout primitive that implements the CSS box model
//! properties through a typed API. It provides compile-time validation of value combinations
//! and serves as the foundation for all other layout components.
//!
//! Research Reference: Section 5.1.1 - Primitives (Layout Foundation)
//!
//! # Example
//! ```
//! use shallot_components::r#box::{Box, Display, Position};
//! use maud::html;
//!
//! let container = Box::new()
//!     .display(Display::Flex)
//!     .position(Position::Relative)
//!     .padding(16)
//!     .margin_bottom(24)
//!     .render(html! { "Content" });
//! ```

use maud::{html, Markup};

/// Display property variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    None,
    Contents,
}

impl Display {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Block => "block",
            Self::Inline => "inline",
            Self::InlineBlock => "inline-block",
            Self::Flex => "flex",
            Self::InlineFlex => "inline-flex",
            Self::Grid => "grid",
            Self::InlineGrid => "inline-grid",
            Self::None => "none",
            Self::Contents => "contents",
        }
    }
}

/// Position property variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Relative => "relative",
            Self::Absolute => "absolute",
            Self::Fixed => "fixed",
            Self::Sticky => "sticky",
        }
    }
}

/// Overflow property variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

impl Overflow {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Hidden => "hidden",
            Self::Clip => "clip",
            Self::Scroll => "scroll",
            Self::Auto => "auto",
        }
    }
}

/// Box sizing variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

impl BoxSizing {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::ContentBox => "content-box",
            Self::BorderBox => "border-box",
        }
    }
}

/// Size value with typed units
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeValue {
    Auto,
    Px(f32),
    Rem(f32),
    Percent(f32),
    Vw(f32),
    Vh(f32),
    MinContent,
    MaxContent,
    FitContent,
}

impl SizeValue {
    pub fn css_value(&self) -> String {
        match self {
            Self::Auto => "auto".to_string(),
            Self::Px(v) => format!("{}px", v),
            Self::Rem(v) => format!("{}rem", v),
            Self::Percent(v) => format!("{}%", v),
            Self::Vw(v) => format!("{}vw", v),
            Self::Vh(v) => format!("{}vh", v),
            Self::MinContent => "min-content".to_string(),
            Self::MaxContent => "max-content".to_string(),
            Self::FitContent => "fit-content".to_string(),
        }
    }
}

/// Spacing value for margins and padding
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpacingValue {
    Zero,
    Px(u32),
    Rem(f32),
    Auto,
}

impl SpacingValue {
    pub fn css_value(&self) -> String {
        match self {
            Self::Zero => "0".to_string(),
            Self::Px(v) => format!("{}px", v),
            Self::Rem(v) => format!("{}rem", v),
            Self::Auto => "auto".to_string(),
        }
    }
}

/// Border style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    None,
    Hidden,
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl BorderStyle {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Hidden => "hidden",
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
            Self::Groove => "groove",
            Self::Ridge => "ridge",
            Self::Inset => "inset",
            Self::Outset => "outset",
        }
    }
}

/// Cursor variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cursor {
    Auto,
    Default,
    Pointer,
    Text,
    Move,
    NotAllowed,
    Grab,
    Grabbing,
}

impl Cursor {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Default => "default",
            Self::Pointer => "pointer",
            Self::Text => "text",
            Self::Move => "move",
            Self::NotAllowed => "not-allowed",
            Self::Grab => "grab",
            Self::Grabbing => "grabbing",
        }
    }
}

/// Universal layout primitive implementing CSS box model
#[derive(Debug, Clone)]
pub struct Box {
    // Display and positioning
    display: Option<Display>,
    position: Option<Position>,

    // Sizing
    width: Option<SizeValue>,
    height: Option<SizeValue>,
    min_width: Option<SizeValue>,
    min_height: Option<SizeValue>,
    max_width: Option<SizeValue>,
    max_height: Option<SizeValue>,
    box_sizing: Option<BoxSizing>,

    // Spacing (margin)
    margin: Option<SpacingValue>,
    margin_top: Option<SpacingValue>,
    margin_right: Option<SpacingValue>,
    margin_bottom: Option<SpacingValue>,
    margin_left: Option<SpacingValue>,

    // Spacing (padding)
    padding: Option<SpacingValue>,
    padding_top: Option<SpacingValue>,
    padding_right: Option<SpacingValue>,
    padding_bottom: Option<SpacingValue>,
    padding_left: Option<SpacingValue>,

    // Position offsets
    top: Option<SpacingValue>,
    right: Option<SpacingValue>,
    bottom: Option<SpacingValue>,
    left: Option<SpacingValue>,
    inset: Option<SpacingValue>,

    // Overflow
    overflow: Option<Overflow>,
    overflow_x: Option<Overflow>,
    overflow_y: Option<Overflow>,

    // Visual
    background: Option<String>,
    border_radius: Option<u32>,
    border_width: Option<u32>,
    border_style: Option<BorderStyle>,
    border_color: Option<String>,
    opacity: Option<f32>,
    cursor: Option<Cursor>,

    // CSS custom properties
    custom_properties: Vec<(String, String)>,

    // HTML attributes
    id: Option<String>,
    classes: Vec<String>,
    title: Option<String>,
    tabindex: Option<i8>,

    // ARIA attributes
    aria_label: Option<String>,
    aria_hidden: Option<bool>,
    role: Option<String>,
}

impl Default for Box {
    fn default() -> Self {
        Self {
            display: None,
            position: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            box_sizing: None,
            margin: None,
            margin_top: None,
            margin_right: None,
            margin_bottom: None,
            margin_left: None,
            padding: None,
            padding_top: None,
            padding_right: None,
            padding_bottom: None,
            padding_left: None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            inset: None,
            overflow: None,
            overflow_x: None,
            overflow_y: None,
            background: None,
            border_radius: None,
            border_width: None,
            border_style: None,
            border_color: None,
            opacity: None,
            cursor: None,
            custom_properties: Vec::new(),
            id: None,
            classes: Vec::new(),
            title: None,
            tabindex: None,
            aria_label: None,
            aria_hidden: None,
            role: None,
        }
    }
}

impl Box {
    /// Create a new Box with default settings
    pub fn new() -> Self {
        Self::default()
    }

    // Display and positioning

    pub fn display(mut self, value: Display) -> Self {
        self.display = Some(value);
        self
    }

    pub fn position(mut self, value: Position) -> Self {
        self.position = Some(value);
        self
    }

    // Sizing

    pub fn width(mut self, value: impl Into<SizeValue>) -> Self {
        self.width = Some(value.into());
        self
    }

    pub fn height(mut self, value: impl Into<SizeValue>) -> Self {
        self.height = Some(value.into());
        self
    }

    pub fn min_width(mut self, value: impl Into<SizeValue>) -> Self {
        self.min_width = Some(value.into());
        self
    }

    pub fn min_height(mut self, value: impl Into<SizeValue>) -> Self {
        self.min_height = Some(value.into());
        self
    }

    pub fn max_width(mut self, value: impl Into<SizeValue>) -> Self {
        self.max_width = Some(value.into());
        self
    }

    pub fn max_height(mut self, value: impl Into<SizeValue>) -> Self {
        self.max_height = Some(value.into());
        self
    }

    pub fn box_sizing(mut self, value: BoxSizing) -> Self {
        self.box_sizing = Some(value);
        self
    }

    // Spacing - margin

    pub fn margin(mut self, value: impl Into<SpacingValue>) -> Self {
        self.margin = Some(value.into());
        self
    }

    pub fn margin_top(mut self, value: impl Into<SpacingValue>) -> Self {
        self.margin_top = Some(value.into());
        self
    }

    pub fn margin_right(mut self, value: impl Into<SpacingValue>) -> Self {
        self.margin_right = Some(value.into());
        self
    }

    pub fn margin_bottom(mut self, value: impl Into<SpacingValue>) -> Self {
        self.margin_bottom = Some(value.into());
        self
    }

    pub fn margin_left(mut self, value: impl Into<SpacingValue>) -> Self {
        self.margin_left = Some(value.into());
        self
    }

    pub fn margin_x(mut self, value: impl Into<SpacingValue>) -> Self {
        let v = value.into();
        self.margin_left = Some(v);
        self.margin_right = Some(v);
        self
    }

    pub fn margin_y(mut self, value: impl Into<SpacingValue>) -> Self {
        let v = value.into();
        self.margin_top = Some(v);
        self.margin_bottom = Some(v);
        self
    }

    // Spacing - padding

    pub fn padding(mut self, value: impl Into<SpacingValue>) -> Self {
        self.padding = Some(value.into());
        self
    }

    pub fn padding_top(mut self, value: impl Into<SpacingValue>) -> Self {
        self.padding_top = Some(value.into());
        self
    }

    pub fn padding_right(mut self, value: impl Into<SpacingValue>) -> Self {
        self.padding_right = Some(value.into());
        self
    }

    pub fn padding_bottom(mut self, value: impl Into<SpacingValue>) -> Self {
        self.padding_bottom = Some(value.into());
        self
    }

    pub fn padding_left(mut self, value: impl Into<SpacingValue>) -> Self {
        self.padding_left = Some(value.into());
        self
    }

    pub fn padding_x(mut self, value: impl Into<SpacingValue>) -> Self {
        let v = value.into();
        self.padding_left = Some(v);
        self.padding_right = Some(v);
        self
    }

    pub fn padding_y(mut self, value: impl Into<SpacingValue>) -> Self {
        let v = value.into();
        self.padding_top = Some(v);
        self.padding_bottom = Some(v);
        self
    }

    // Position offsets

    pub fn top(mut self, value: impl Into<SpacingValue>) -> Self {
        self.top = Some(value.into());
        self
    }

    pub fn right(mut self, value: impl Into<SpacingValue>) -> Self {
        self.right = Some(value.into());
        self
    }

    pub fn bottom(mut self, value: impl Into<SpacingValue>) -> Self {
        self.bottom = Some(value.into());
        self
    }

    pub fn left(mut self, value: impl Into<SpacingValue>) -> Self {
        self.left = Some(value.into());
        self
    }

    pub fn inset(mut self, value: impl Into<SpacingValue>) -> Self {
        self.inset = Some(value.into());
        self
    }

    // Overflow

    pub fn overflow(mut self, value: Overflow) -> Self {
        self.overflow = Some(value);
        self
    }

    pub fn overflow_x(mut self, value: Overflow) -> Self {
        self.overflow_x = Some(value);
        self
    }

    pub fn overflow_y(mut self, value: Overflow) -> Self {
        self.overflow_y = Some(value);
        self
    }

    // Visual

    pub fn background(mut self, value: impl Into<String>) -> Self {
        self.background = Some(value.into());
        self
    }

    pub fn border_radius(mut self, value: u32) -> Self {
        self.border_radius = Some(value);
        self
    }

    pub fn border_width(mut self, value: u32) -> Self {
        self.border_width = Some(value);
        self
    }

    pub fn border_style(mut self, value: BorderStyle) -> Self {
        self.border_style = Some(value);
        self
    }

    pub fn border_color(mut self, value: impl Into<String>) -> Self {
        self.border_color = Some(value.into());
        self
    }

    pub fn opacity(mut self, value: f32) -> Self {
        self.opacity = Some(value.clamp(0.0, 1.0));
        self
    }

    pub fn cursor(mut self, value: Cursor) -> Self {
        self.cursor = Some(value);
        self
    }

    // Custom properties

    pub fn custom_property(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_properties.push((name.into(), value.into()));
        self
    }

    // HTML attributes

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn class(mut self, value: impl Into<String>) -> Self {
        self.classes.push(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn tabindex(mut self, value: i8) -> Self {
        self.tabindex = Some(value);
        self
    }

    // ARIA attributes

    pub fn aria_label(mut self, value: impl Into<String>) -> Self {
        self.aria_label = Some(value.into());
        self
    }

    pub fn aria_hidden(mut self, value: bool) -> Self {
        self.aria_hidden = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    /// Build CSS styles from box properties
    fn build_styles(&self) -> String {
        let mut styles = String::new();

        if let Some(ref d) = self.display {
            styles.push_str(&format!("display:{};", d.css_value()));
        }
        if let Some(ref p) = self.position {
            styles.push_str(&format!("position:{};", p.css_value()));
        }

        if let Some(ref w) = self.width {
            styles.push_str(&format!("width:{};", w.css_value()));
        }
        if let Some(ref h) = self.height {
            styles.push_str(&format!("height:{};", h.css_value()));
        }
        if let Some(ref w) = self.min_width {
            styles.push_str(&format!("min-width:{};", w.css_value()));
        }
        if let Some(ref h) = self.min_height {
            styles.push_str(&format!("min-height:{};", h.css_value()));
        }
        if let Some(ref w) = self.max_width {
            styles.push_str(&format!("max-width:{};", w.css_value()));
        }
        if let Some(ref h) = self.max_height {
            styles.push_str(&format!("max-height:{};", h.css_value()));
        }
        if let Some(ref bs) = self.box_sizing {
            styles.push_str(&format!("box-sizing:{};", bs.css_value()));
        }

        // Margin
        if let Some(ref m) = self.margin {
            styles.push_str(&format!("margin:{};", m.css_value()));
        } else {
            if let Some(ref m) = self.margin_top {
                styles.push_str(&format!("margin-top:{};", m.css_value()));
            }
            if let Some(ref m) = self.margin_right {
                styles.push_str(&format!("margin-right:{};", m.css_value()));
            }
            if let Some(ref m) = self.margin_bottom {
                styles.push_str(&format!("margin-bottom:{};", m.css_value()));
            }
            if let Some(ref m) = self.margin_left {
                styles.push_str(&format!("margin-left:{};", m.css_value()));
            }
        }

        // Padding
        if let Some(ref p) = self.padding {
            styles.push_str(&format!("padding:{};", p.css_value()));
        } else {
            if let Some(ref p) = self.padding_top {
                styles.push_str(&format!("padding-top:{};", p.css_value()));
            }
            if let Some(ref p) = self.padding_right {
                styles.push_str(&format!("padding-right:{};", p.css_value()));
            }
            if let Some(ref p) = self.padding_bottom {
                styles.push_str(&format!("padding-bottom:{};", p.css_value()));
            }
            if let Some(ref p) = self.padding_left {
                styles.push_str(&format!("padding-left:{};", p.css_value()));
            }
        }

        // Position offsets
        if let Some(ref i) = self.inset {
            styles.push_str(&format!("inset:{};", i.css_value()));
        } else {
            if let Some(ref t) = self.top {
                styles.push_str(&format!("top:{};", t.css_value()));
            }
            if let Some(ref r) = self.right {
                styles.push_str(&format!("right:{};", r.css_value()));
            }
            if let Some(ref b) = self.bottom {
                styles.push_str(&format!("bottom:{};", b.css_value()));
            }
            if let Some(ref l) = self.left {
                styles.push_str(&format!("left:{};", l.css_value()));
            }
        }

        // Overflow
        if let Some(ref o) = self.overflow {
            styles.push_str(&format!("overflow:{};", o.css_value()));
        }
        if let Some(ref o) = self.overflow_x {
            styles.push_str(&format!("overflow-x:{};", o.css_value()));
        }
        if let Some(ref o) = self.overflow_y {
            styles.push_str(&format!("overflow-y:{};", o.css_value()));
        }

        // Visual
        if let Some(ref bg) = self.background {
            styles.push_str(&format!("background:{};", bg));
        }
        if let Some(ref r) = self.border_radius {
            styles.push_str(&format!("border-radius:{}px;", r));
        }
        if let Some(ref w) = self.border_width {
            styles.push_str(&format!("border-width:{}px;", w));
        }
        if let Some(ref s) = self.border_style {
            styles.push_str(&format!("border-style:{};", s.css_value()));
        }
        if let Some(ref c) = self.border_color {
            styles.push_str(&format!("border-color:{};", c));
        }
        if let Some(ref o) = self.opacity {
            styles.push_str(&format!("opacity:{};", o));
        }
        if let Some(ref c) = self.cursor {
            styles.push_str(&format!("cursor:{};", c.css_value()));
        }

        // Custom properties
        for (name, value) in &self.custom_properties {
            styles.push_str(&format!("--{}:{};", name, value));
        }

        styles
    }

    /// Build class attribute
    fn build_class(&self) -> String {
        let mut classes = vec!["sh-box".to_string()];
        classes.extend(self.classes.clone());
        classes.join(" ")
    }

    /// Render the box with children
    pub fn render(self, children: Markup) -> Markup {
        let styles = self.build_styles();
        let class = self.build_class();

        html! {
            div
                class=(class)
                style=(styles)
                id=[self.id]
                title=[self.title]
                tabindex=[self.tabindex.map(|t| t.to_string())]
                aria-label=[self.aria_label]
                aria-hidden=[self.aria_hidden.map(|h| h.to_string())]
                role=[self.role]
            {
                (children)
            }
        }
    }

    /// Render as semantic element
    pub fn render_as(self, element: &str, children: Markup) -> Markup {
        let styles = self.build_styles();
        let class = self.build_class();

        html! {
            (maud::PreEscaped(format!(
                "<{} class=\"{}\" style=\"{}\"{}>{}</{}>",
                element,
                class,
                styles,
                self.build_attributes(),
                children.into_string(),
                element
            )))
        }
    }

    fn build_attributes(&self) -> String {
        let mut attrs = String::new();
        if let Some(ref id) = self.id {
            attrs.push_str(&format!(" id=\"{}\"", id));
        }
        if let Some(ref title) = self.title {
            attrs.push_str(&format!(" title=\"{}\"", title));
        }
        if let Some(tabindex) = self.tabindex {
            attrs.push_str(&format!(" tabindex=\"{}\"", tabindex));
        }
        if let Some(ref aria_label) = self.aria_label {
            attrs.push_str(&format!(" aria-label=\"{}\"", aria_label));
        }
        if let Some(aria_hidden) = self.aria_hidden {
            attrs.push_str(&format!(" aria-hidden=\"{}\"", aria_hidden));
        }
        if let Some(ref role) = self.role {
            attrs.push_str(&format!(" role=\"{}\"", role));
        }
        attrs
    }
}

/// Convenience impls for Into<SizeValue>
impl From<u32> for SizeValue {
    fn from(v: u32) -> Self {
        SizeValue::Px(v as f32)
    }
}

impl From<f32> for SizeValue {
    fn from(v: f32) -> Self {
        SizeValue::Rem(v)
    }
}

impl From<&str> for SizeValue {
    fn from(v: &str) -> Self {
        if v.ends_with("px") {
            v.trim_end_matches("px")
                .parse::<f32>()
                .map(SizeValue::Px)
                .unwrap_or(SizeValue::Auto)
        } else if v.ends_with("rem") {
            v.trim_end_matches("rem")
                .parse::<f32>()
                .map(SizeValue::Rem)
                .unwrap_or(SizeValue::Auto)
        } else if v.ends_with('%') {
            v.trim_end_matches('%')
                .parse::<f32>()
                .map(SizeValue::Percent)
                .unwrap_or(SizeValue::Auto)
        } else if v == "auto" {
            SizeValue::Auto
        } else {
            SizeValue::Auto
        }
    }
}

/// Convenience impls for Into<SpacingValue>
impl From<u32> for SpacingValue {
    fn from(v: u32) -> Self {
        SpacingValue::Px(v)
    }
}

impl From<f32> for SpacingValue {
    fn from(v: f32) -> Self {
        SpacingValue::Rem(v)
    }
}

/// Center component - Flexbox/Grid centered content
#[derive(Debug, Clone)]
pub struct Center {
    horizontal: bool,
    vertical: bool,
    inline: bool,
    box_props: Box,
}

impl Default for Center {
    fn default() -> Self {
        Self {
            horizontal: true,
            vertical: true,
            inline: false,
            box_props: Box::new(),
        }
    }
}

impl Center {
    pub fn new() -> Self {
        Self::default()
    }

    /// Center horizontally only
    pub fn horizontal() -> Self {
        Self {
            horizontal: true,
            vertical: false,
            inline: false,
            box_props: Box::new(),
        }
    }

    /// Center vertically only
    pub fn vertical() -> Self {
        Self {
            horizontal: false,
            vertical: true,
            inline: false,
            box_props: Box::new(),
        }
    }

    /// Use inline-flex instead of flex
    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }

    /// Delegate box methods
    pub fn width(mut self, value: impl Into<SizeValue>) -> Self {
        self.box_props = self.box_props.width(value);
        self
    }

    pub fn height(mut self, value: impl Into<SizeValue>) -> Self {
        self.box_props = self.box_props.height(value);
        self
    }

    pub fn padding(mut self, value: impl Into<SpacingValue>) -> Self {
        self.box_props = self.box_props.padding(value);
        self
    }

    pub fn class(mut self, value: impl Into<String>) -> Self {
        self.box_props = self.box_props.class(value);
        self
    }

    /// Render with children
    pub fn render(self, children: Markup) -> Markup {
        let display = if self.inline {
            Display::InlineFlex
        } else {
            Display::Flex
        };

        let justify = if self.horizontal {
            "center"
        } else {
            "flex-start"
        };

        let align = if self.vertical {
            "center"
        } else {
            "flex-start"
        };

        let box_component = self
            .box_props
            .display(display)
            .custom_property("sh-center-justify", justify)
            .custom_property("sh-center-align", align);

        // Apply justify-content and align-items via inline style
        let mut styles = box_component.build_styles();
        styles.push_str(&format!(
            "justify-content:{};align-items:{};",
            justify, align
        ));

        let class = box_component.build_class();

        html! {
            div class=(class) style=(styles) {
                (children)
            }
        }
    }
}

/// Spacer component - Empty space for flex/grid layouts
#[derive(Debug, Clone)]
pub struct Spacer {
    flex: Option<f32>,
    width: Option<SizeValue>,
    height: Option<SizeValue>,
}

impl Default for Spacer {
    fn default() -> Self {
        Self {
            flex: Some(1.0),
            width: None,
            height: None,
        }
    }
}

impl Spacer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a spacer with specific flex value
    pub fn flex(value: f32) -> Self {
        Self {
            flex: Some(value),
            width: None,
            height: None,
        }
    }

    /// Create a spacer with fixed width
    pub fn width(value: impl Into<SizeValue>) -> Self {
        Self {
            flex: None,
            width: Some(value.into()),
            height: None,
        }
    }

    /// Create a spacer with fixed height
    pub fn height(value: impl Into<SizeValue>) -> Self {
        Self {
            flex: None,
            width: None,
            height: Some(value.into()),
        }
    }

    /// Render the spacer
    pub fn render(self) -> Markup {
        let mut styles = String::from("display:block;");

        if let Some(flex) = self.flex {
            styles.push_str(&format!("flex:{};", flex));
        }
        if let Some(width) = self.width {
            styles.push_str(&format!(
                "width:{};min-width:{};",
                width.css_value(),
                width.css_value()
            ));
        }
        if let Some(height) = self.height {
            styles.push_str(&format!(
                "height:{};min-height:{};",
                height.css_value(),
                height.css_value()
            ));
        }

        html! {
            div class="sh-spacer" style=(styles) aria-hidden="true" {}
        }
    }
}

/// Generate CSS for box components
pub fn box_css() -> String {
    r#"
/* Box Component Styles */
.sh-box {
    box-sizing: border-box;
}

/* Spacer Component Styles */
.sh-spacer {
    flex-shrink: 0;
}

/* Center Component Styles */
.sh-center {
    display: flex;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_basic() {
        let box_component = Box::new()
            .display(Display::Flex)
            .padding(16)
            .width(100)
            .height(SizeValue::Percent(100.0));

        assert!(box_component.display.is_some());
        assert!(box_component.padding.is_some());
    }

    #[test]
    fn test_size_value_conversions() {
        let px: SizeValue = 100.into();
        assert_eq!(px.css_value(), "100px");

        let rem: SizeValue = 1.5f32.into();
        assert_eq!(rem.css_value(), "1.5rem");

        let percent = SizeValue::Percent(50.0);
        assert_eq!(percent.css_value(), "50%");
    }

    #[test]
    fn test_spacing_value_conversions() {
        let px: SpacingValue = 16.into();
        assert_eq!(px.css_value(), "16px");

        let rem: SpacingValue = 1.0f32.into();
        assert_eq!(rem.css_value(), "1rem");
    }

    #[test]
    fn test_display_variants() {
        assert_eq!(Display::Flex.css_value(), "flex");
        assert_eq!(Display::Grid.css_value(), "grid");
        assert_eq!(Display::None.css_value(), "none");
    }

    #[test]
    fn test_position_variants() {
        assert_eq!(Position::Relative.css_value(), "relative");
        assert_eq!(Position::Absolute.css_value(), "absolute");
        assert_eq!(Position::Sticky.css_value(), "sticky");
    }

    #[test]
    fn test_center_builder() {
        let center = Center::new().width(200).height(100).padding(16);

        assert!(center.horizontal);
        assert!(center.vertical);
    }

    #[test]
    fn test_spacer_variants() {
        let flex_spacer = Spacer::flex(2.0);
        assert_eq!(flex_spacer.flex, Some(2.0));

        let width_spacer = Spacer::width(50);
        assert!(width_spacer.width.is_some());
        assert!(width_spacer.flex.is_none());
    }
}
