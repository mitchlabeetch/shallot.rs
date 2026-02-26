//! Typography System - Comprehensive Text Components
//!
//! This module provides semantic typography primitives as outlined in research section 5.1.2:
//! - Text: Semantic text spans with size, weight, color, alignment variants
//! - Heading: H1-H6 hierarchy with automatic id generation
//! - Code: Inline/block code with syntax highlighting hooks
//! - Quote: Blockquote with citation support
//! - List: Ordered/unordered/description lists
//!
//! All components ensure semantic HTML output for accessibility and SEO.

use maud::{html, Markup};

/// Text size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSize {
    Xs,   // 12px
    Sm,   // 14px
    Md,   // 16px (base)
    Lg,   // 18px
    Xl,   // 20px
    Xxl,  // 24px
    Xxxl, // 30px
}

impl TextSize {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Xs => "0.75rem",
            Self::Sm => "0.875rem",
            Self::Md => "1rem",
            Self::Lg => "1.125rem",
            Self::Xl => "1.25rem",
            Self::Xxl => "1.5rem",
            Self::Xxxl => "1.875rem",
        }
    }

    pub const fn line_height(&self) -> &'static str {
        match self {
            Self::Xs => "1rem",
            Self::Sm => "1.25rem",
            Self::Md => "1.5rem",
            Self::Lg => "1.75rem",
            Self::Xl => "1.75rem",
            Self::Xxl => "2rem",
            Self::Xxxl => "2.25rem",
        }
    }
}

/// Font weight variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    Normal,     // 400
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}

impl FontWeight {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Thin => "100",
            Self::ExtraLight => "200",
            Self::Light => "300",
            Self::Normal => "400",
            Self::Medium => "500",
            Self::SemiBold => "600",
            Self::Bold => "700",
            Self::ExtraBold => "800",
            Self::Black => "900",
        }
    }
}

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
    Start,
    End,
}

impl TextAlign {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Justify => "justify",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// Text color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextColor {
    Default,
    Muted,
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Custom(&'static str),
}

impl TextColor {
    pub fn css_value(&self) -> String {
        match self {
            Self::Default => "var(--sh-text)".to_string(),
            Self::Muted => "var(--sh-text-muted)".to_string(),
            Self::Primary => "var(--sh-primary)".to_string(),
            Self::Secondary => "var(--sh-secondary)".to_string(),
            Self::Accent => "var(--sh-accent)".to_string(),
            Self::Success => "var(--sh-success)".to_string(),
            Self::Warning => "var(--sh-warning)".to_string(),
            Self::Error => "var(--sh-error)".to_string(),
            Self::Custom(c) => c.to_string(),
        }
    }
}

/// Text transform variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

impl TextTransform {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Uppercase => "uppercase",
            Self::Lowercase => "lowercase",
            Self::Capitalize => "capitalize",
        }
    }
}

/// Text decoration variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecoration {
    None,
    Underline,
    LineThrough,
    Overline,
}

impl TextDecoration {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Underline => "underline",
            Self::LineThrough => "line-through",
            Self::Overline => "overline",
        }
    }
}

/// Font family variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    Sans,
    Serif,
    Mono,
    Custom(&'static str),
}

impl FontFamily {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Sans => "var(--sh-font-sans)",
            Self::Serif => "var(--sh-font-serif)",
            Self::Mono => "var(--sh-font-mono)",
            Self::Custom(f) => f,
        }
    }
}

/// Text component - Semantic text spans with comprehensive styling
#[derive(Debug, Clone)]
pub struct Text {
    content: String,
    size: TextSize,
    weight: FontWeight,
    color: TextColor,
    align: TextAlign,
    transform: TextTransform,
    decoration: TextDecoration,
    family: FontFamily,
    italic: bool,
    truncate: bool,
    line_clamp: Option<u8>,
    letter_spacing: Option<&'static str>,
    as_element: Option<&'static str>,
    id: Option<String>,
    class: Option<String>,
    aria_label: Option<String>,
    title: Option<String>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            size: TextSize::Md,
            weight: FontWeight::Normal,
            color: TextColor::Default,
            align: TextAlign::Start,
            transform: TextTransform::None,
            decoration: TextDecoration::None,
            family: FontFamily::Sans,
            italic: false,
            truncate: false,
            line_clamp: None,
            letter_spacing: None,
            as_element: None,
            id: None,
            class: None,
            aria_label: None,
            title: None,
        }
    }

    pub fn size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn color(mut self, color: TextColor) -> Self {
        self.color = color;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn transform(mut self, transform: TextTransform) -> Self {
        self.transform = transform;
        self
    }

    pub fn decoration(mut self, decoration: TextDecoration) -> Self {
        self.decoration = decoration;
        self
    }

    pub fn family(mut self, family: FontFamily) -> Self {
        self.family = family;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn truncate(mut self) -> Self {
        self.truncate = true;
        self
    }

    pub fn line_clamp(mut self, lines: u8) -> Self {
        self.line_clamp = Some(lines);
        self
    }

    pub fn letter_spacing(mut self, spacing: &'static str) -> Self {
        self.letter_spacing = Some(spacing);
        self
    }

    /// Render as specific element (span, p, strong, em, etc.)
    pub fn as_element(mut self, element: &'static str) -> Self {
        self.as_element = Some(element);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Build CSS styles
    fn build_styles(&self) -> String {
        let mut styles = format!(
            "font-size:{};line-height:{};font-weight:{};color:{};text-align:{};text-transform:{};text-decoration:{};font-family:{};",
            self.size.css_value(),
            self.size.line_height(),
            self.weight.css_value(),
            self.color.css_value(),
            self.align.css_value(),
            self.transform.css_value(),
            self.decoration.css_value(),
            self.family.css_value()
        );

        if self.italic {
            styles.push_str("font-style:italic;");
        }

        if self.truncate {
            styles.push_str("overflow:hidden;text-overflow:ellipsis;white-space:nowrap;");
        }

        if let Some(lines) = self.line_clamp {
            styles.push_str(&format!(
                "display:-webkit-box;-webkit-line-clamp:{};-webkit-box-orient:vertical;overflow:hidden;",
                lines
            ));
        }

        if let Some(spacing) = self.letter_spacing {
            styles.push_str(&format!("letter-spacing:{};", spacing));
        }

        styles
    }

    /// Build class attribute
    fn build_class(&self) -> String {
        let mut classes = vec!["sh-text".to_string()];
        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }
        classes.join(" ")
    }

    /// Render the text
    pub fn render(self) -> Markup {
        let styles = self.build_styles();
        let class = self.build_class();
        let element = self.as_element.unwrap_or("span");

        html! {
            (maud::PreEscaped(format!(
                "<{} class=\"{}\" style=\"{}\"{}>{}</{}>",
                element,
                class,
                styles,
                self.build_attributes(),
                self.content,
                element
            )))
        }
    }

    fn build_attributes(&self) -> String {
        let mut attrs = String::new();
        if let Some(ref id) = self.id {
            attrs.push_str(&format!(" id=\"{}\"", id));
        }
        if let Some(ref aria_label) = self.aria_label {
            attrs.push_str(&format!(" aria-label=\"{}\"", aria_label));
        }
        if let Some(ref title) = self.title {
            attrs.push_str(&format!(" title=\"{}\"", title));
        }
        attrs
    }
}

/// Heading component - H1-H6 with automatic id generation
#[derive(Debug, Clone)]
pub struct Heading {
    level: u8, // 1-6
    content: String,
    visual_size: Option<TextSize>,
    weight: FontWeight,
    color: TextColor,
    align: TextAlign,
    id: Option<String>,
    anchor: bool,
    class: Option<String>,
    margin_top: Option<&'static str>,
    margin_bottom: Option<&'static str>,
}

impl Heading {
    pub fn new(level: u8, content: impl Into<String>) -> Self {
        assert!((1..=6).contains(&level), "Heading level must be 1-6");
        Self {
            level,
            content: content.into(),
            visual_size: None,
            weight: FontWeight::Bold,
            color: TextColor::Default,
            align: TextAlign::Start,
            id: None,
            anchor: false,
            class: None,
            margin_top: None,
            margin_bottom: None,
        }
    }

    pub fn h1(content: impl Into<String>) -> Self {
        Self::new(1, content)
    }

    pub fn h2(content: impl Into<String>) -> Self {
        Self::new(2, content)
    }

    pub fn h3(content: impl Into<String>) -> Self {
        Self::new(3, content)
    }

    pub fn h4(content: impl Into<String>) -> Self {
        Self::new(4, content)
    }

    pub fn h5(content: impl Into<String>) -> Self {
        Self::new(5, content)
    }

    pub fn h6(content: impl Into<String>) -> Self {
        Self::new(6, content)
    }

    /// Set visual size (can differ from semantic level)
    pub fn visual_size(mut self, size: TextSize) -> Self {
        self.visual_size = Some(size);
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn color(mut self, color: TextColor) -> Self {
        self.color = color;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Enable anchor link generation
    pub fn with_anchor(mut self) -> Self {
        self.anchor = true;
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn margin_top(mut self, margin: &'static str) -> Self {
        self.margin_top = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin: &'static str) -> Self {
        self.margin_bottom = Some(margin);
        self
    }

    /// Generate id from content if not provided
    fn generate_id(&self) -> String {
        self.id.clone().unwrap_or_else(|| {
            self.content
                .to_lowercase()
                .replace(' ', "-")
                .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
        })
    }

    fn size_for_level(&self) -> TextSize {
        match self.level {
            1 => TextSize::Xxxl,
            2 => TextSize::Xxl,
            3 => TextSize::Xl,
            4 => TextSize::Lg,
            5 => TextSize::Md,
            6 => TextSize::Sm,
            _ => TextSize::Md,
        }
    }

    pub fn render(self) -> Markup {
        let size = self.visual_size.unwrap_or_else(|| self.size_for_level());
        let id = self.generate_id();
        let element = format!("h{}", self.level);

        let mut styles = format!(
            "font-size:{};line-height:{};font-weight:{};color:{};text-align:{};",
            size.css_value(),
            size.line_height(),
            self.weight.css_value(),
            self.color.css_value(),
            self.align.css_value()
        );

        if let Some(m) = self.margin_top {
            styles.push_str(&format!("margin-top:{};", m));
        }
        if let Some(m) = self.margin_bottom {
            styles.push_str(&format!("margin-bottom:{};", m));
        }

        let mut classes = vec![format!("sh-heading"), format!("sh-heading-{}", self.level)];
        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }
        let class = classes.join(" ");

        let anchor_link = if self.anchor {
            html! {
                a href=(format!("#{}", id)) class="sh-heading-anchor" aria-hidden="true" {
                    "#"
                }
            }
        } else {
            html! {}
        };

        html! {
            (maud::PreEscaped(format!(
                "<{} class=\"{}\" id=\"{}\" style=\"{}\">{}</{}}}",
                element,
                class,
                id,
                styles,
                html! { (anchor_link) (self.content) }.into_string(),
                element
            )))
        }
    }
}

/// Code component - Inline and block code with syntax highlighting hooks
#[derive(Debug, Clone)]
pub struct Code {
    content: String,
    language: Option<String>,
    inline: bool,
    show_line_numbers: bool,
    filename: Option<String>,
    highlight_lines: Vec<u32>,
}

impl Code {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            language: None,
            inline: true,
            show_line_numbers: false,
            filename: None,
            highlight_lines: Vec::new(),
        }
    }

    /// Create a block code element
    pub fn block(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            language: None,
            inline: false,
            show_line_numbers: false,
            filename: None,
            highlight_lines: Vec::new(),
        }
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    pub fn with_line_numbers(mut self) -> Self {
        self.show_line_numbers = true;
        self
    }

    pub fn filename(mut self, name: impl Into<String>) -> Self {
        self.filename = Some(name.into());
        self
    }

    pub fn highlight_lines(mut self, lines: Vec<u32>) -> Self {
        self.highlight_lines = lines;
        self
    }

    fn escape_html(&self, content: &str) -> String {
        content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    pub fn render(self) -> Markup {
        if self.inline {
            let class = if let Some(ref lang) = self.language {
                format!("sh-code sh-code--{}", lang)
            } else {
                "sh-code".to_string()
            };

            html! {
                code class=(class) {
                    (self.content)
                }
            }
        } else {
            let lang_class = self
                .language
                .as_ref()
                .map(|l| format!("language-{}", l))
                .unwrap_or_default();

            let escaped_content = self.escape_html(&self.content);
            let lines: Vec<&str> = escaped_content.lines().collect();

            html! {
                figure class="sh-code-block" {
                    @if let Some(filename) = self.filename {
                        figcaption class="sh-code-filename" {
                            (filename)
                        }
                    }
                    pre class=(format!("sh-pre {}", lang_class)) {
                        @if self.show_line_numbers {
                            div class="sh-line-numbers" aria-hidden="true" {
                                @for i in 1..=lines.len() {
                                    span class="sh-line-number" { (i) }
                                }
                            }
                        }
                        code class=(format!("sh-block-code {}", lang_class)) {
                            @for (i, line) in lines.iter().enumerate() {
                                @let line_num = i as u32 + 1;
                                @let is_highlighted = self.highlight_lines.contains(&line_num);
                                span class=(if is_highlighted { "sh-line sh-line--highlighted" } else { "sh-line" }) {
                                    (maud::PreEscaped(line.to_string()))
                                    @if i < lines.len() - 1 {
                                        "\n"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Quote component - Blockquote with citation
#[derive(Debug, Clone)]
pub struct Quote {
    content: Markup,
    citation: Option<String>,
    source: Option<String>,
    variant: QuoteVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteVariant {
    Default,
    Pull,
    Callout,
}

impl Quote {
    pub fn new(content: Markup) -> Self {
        Self {
            content,
            citation: None,
            source: None,
            variant: QuoteVariant::Default,
        }
    }

    pub fn citation(mut self, citation: impl Into<String>) -> Self {
        self.citation = Some(citation.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn variant(mut self, variant: QuoteVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn render(self) -> Markup {
        let variant_class = match self.variant {
            QuoteVariant::Default => "sh-quote",
            QuoteVariant::Pull => "sh-quote sh-quote--pull",
            QuoteVariant::Callout => "sh-quote sh-quote--callout",
        };

        html! {
            blockquote class=(variant_class) {
                div class="sh-quote-content" {
                    (self.content)
                }
                @if self.citation.is_some() || self.source.is_some() {
                    figcaption class="sh-quote-citation" {
                        @if let Some(ref citation) = self.citation {
                            cite { (citation) }
                        }
                        @if let Some(source) = self.source {
                            @if self.citation.is_some() {
                                " — "
                            }
                            (source)
                        }
                    }
                }
            }
        }
    }
}

/// List component - Ordered, unordered, and description lists
#[derive(Debug, Clone)]
pub struct List {
    items: Vec<ListItem>,
    variant: ListVariant,
    marker_style: Option<MarkerStyle>,
    #[allow(dead_code)] // Reserved for nested list styling
    nested_level: u8,
    class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListVariant {
    Unordered,
    Ordered,
    Description,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerStyle {
    Disc,
    Circle,
    Square,
    Decimal,
    DecimalLeadingZero,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha,
}

impl MarkerStyle {
    pub const fn css_value(&self) -> &'static str {
        match self {
            Self::Disc => "disc",
            Self::Circle => "circle",
            Self::Square => "square",
            Self::Decimal => "decimal",
            Self::DecimalLeadingZero => "decimal-leading-zero",
            Self::LowerRoman => "lower-roman",
            Self::UpperRoman => "upper-roman",
            Self::LowerAlpha => "lower-alpha",
            Self::UpperAlpha => "upper-alpha",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ListItem {
    Simple(Markup),
    WithDescription { term: String, details: Markup },
    Nested { content: Markup, children: List },
}

impl List {
    pub fn unordered() -> Self {
        Self {
            items: Vec::new(),
            variant: ListVariant::Unordered,
            marker_style: None,
            nested_level: 0,
            class: None,
        }
    }

    pub fn ordered() -> Self {
        Self {
            items: Vec::new(),
            variant: ListVariant::Ordered,
            marker_style: None,
            nested_level: 0,
            class: None,
        }
    }

    pub fn description() -> Self {
        Self {
            items: Vec::new(),
            variant: ListVariant::Description,
            marker_style: None,
            nested_level: 0,
            class: None,
        }
    }

    pub fn marker_style(mut self, style: MarkerStyle) -> Self {
        self.marker_style = Some(style);
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn add_item(mut self, item: impl Into<Markup>) -> Self {
        self.items.push(ListItem::Simple(item.into()));
        self
    }

    pub fn add_description_item(
        mut self,
        term: impl Into<String>,
        details: impl Into<Markup>,
    ) -> Self {
        self.items.push(ListItem::WithDescription {
            term: term.into(),
            details: details.into(),
        });
        self
    }

    pub fn add_nested(mut self, content: impl Into<Markup>, children: List) -> Self {
        self.items.push(ListItem::Nested {
            content: content.into(),
            children,
        });
        self
    }

    pub fn extend(mut self, items: Vec<impl Into<Markup>>) -> Self {
        for item in items {
            self.items.push(ListItem::Simple(item.into()));
        }
        self
    }

    fn build_styles(&self) -> Option<String> {
        self.marker_style
            .map(|s| format!("list-style-type:{};", s.css_value()))
    }

    pub fn render(self) -> Markup {
        let (tag, class) = match self.variant {
            ListVariant::Unordered => ("ul", "sh-list sh-list--unordered"),
            ListVariant::Ordered => ("ol", "sh-list sh-list--ordered"),
            ListVariant::Description => ("dl", "sh-list sh-list--description"),
        };

        let mut classes = vec![class.to_string()];
        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }
        let class = classes.join(" ");

        let styles = self.build_styles().unwrap_or_default();

        let content = html! {
            @for item in self.items {
                @match item {
                    ListItem::Simple(content) => {
                        li { (content) }
                    }
                    ListItem::WithDescription { term, details } => {
                        dt { (term) }
                        dd { (details) }
                    }
                    ListItem::Nested { content, children } => {
                        li {
                            (content)
                            (children.render())
                        }
                    }
                }
            }
        };

        html! {
            (maud::PreEscaped(format!(
                "<{} class=\"{}\" style=\"{}\">{}</{}>",
                tag,
                class,
                styles,
                content.into_string(),
                tag
            )))
        }
    }
}

/// Generate CSS for typography components
pub fn typography_css() -> String {
    r#"
/* Typography System Styles */

/* Text Component */
.sh-text {
    font-family: var(--sh-font-sans);
    color: var(--sh-text);
}

/* Heading Component */
.sh-heading {
    font-family: var(--sh-font-sans);
    color: var(--sh-text);
    margin: 0;
}

.sh-heading-anchor {
    margin-left: 0.5rem;
    text-decoration: none;
    opacity: 0;
    transition: opacity 0.2s;
}

.sh-heading:hover .sh-heading-anchor {
    opacity: 0.5;
}

.sh-heading-anchor:hover {
    opacity: 1 !important;
}

/* Inline Code */
.sh-code {
    font-family: var(--sh-font-mono);
    font-size: 0.875em;
    padding: 0.125rem 0.25rem;
    background: var(--sh-surface-alt);
    border-radius: 0.25rem;
    color: var(--sh-accent);
}

/* Code Block */
.sh-code-block {
    margin: 1rem 0;
    background: var(--sh-surface-alt);
    border-radius: 0.5rem;
    overflow: hidden;
}

.sh-code-filename {
    padding: 0.5rem 1rem;
    background: var(--sh-surface);
    border-bottom: 1px solid var(--sh-border);
    font-family: var(--sh-font-mono);
    font-size: 0.875rem;
    color: var(--sh-text-muted);
}

.sh-pre {
    display: flex;
    margin: 0;
    padding: 1rem;
    overflow-x: auto;
    font-family: var(--sh-font-mono);
    font-size: 0.875rem;
    line-height: 1.5;
}

.sh-line-numbers {
    display: flex;
    flex-direction: column;
    padding-right: 1rem;
    margin-right: 1rem;
    border-right: 1px solid var(--sh-border);
    color: var(--sh-text-muted);
    text-align: right;
    user-select: none;
}

.sh-line-number {
    display: block;
}

.sh-block-code {
    display: block;
    flex: 1;
}

.sh-line {
    display: block;
}

.sh-line--highlighted {
    background: var(--sh-accent-muted);
    margin: 0 -1rem;
    padding: 0 1rem;
}

/* Quote Component */
.sh-quote {
    margin: 1.5rem 0;
    padding: 1rem 1.5rem;
    border-left: 4px solid var(--sh-accent);
    background: var(--sh-surface-alt);
    font-style: italic;
}

.sh-quote--pull {
    float: left;
    width: 40%;
    margin: 0.5rem 1.5rem 0.5rem 0;
    font-size: 1.25rem;
}

.sh-quote--callout {
    border-left-width: 0;
    border-radius: 0.5rem;
    font-style: normal;
}

.sh-quote-content {
    margin-bottom: 0.5rem;
}

.sh-quote-citation {
    font-size: 0.875rem;
    color: var(--sh-text-muted);
    font-style: normal;
}

.sh-quote-citation cite {
    font-style: italic;
}

/* List Component */
.sh-list {
    margin: 1rem 0;
    padding-left: 1.5rem;
}

.sh-list--description {
    padding-left: 0;
}

.sh-list li {
    margin: 0.25rem 0;
}

.sh-list dt {
    font-weight: 600;
    margin-top: 0.75rem;
}

.sh-list dd {
    margin-left: 1rem;
    margin-bottom: 0.5rem;
    color: var(--sh-text-muted);
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
    .sh-heading-anchor {
        opacity: 0.5;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_basic() {
        let text = Text::new("Hello World")
            .size(TextSize::Lg)
            .weight(FontWeight::Bold)
            .color(TextColor::Primary);

        assert_eq!(text.content, "Hello World");
        assert_eq!(text.size, TextSize::Lg);
        assert_eq!(text.weight, FontWeight::Bold);
    }

    #[test]
    fn test_heading_levels() {
        let h1 = Heading::h1("Title");
        assert_eq!(h1.level, 1);

        let h6 = Heading::h6("Small");
        assert_eq!(h6.level, 6);
    }

    #[test]
    fn test_code_inline() {
        let code = Code::new("let x = 5;").language("rust");
        assert!(code.inline);
        assert_eq!(code.language, Some("rust".to_string()));
    }

    #[test]
    fn test_code_block() {
        let code = Code::block("fn main() {}").with_line_numbers();
        assert!(!code.inline);
        assert!(code.show_line_numbers);
    }

    #[test]
    fn test_list_building() {
        use maud::html;
        let list = List::unordered()
            .add_item(html! { "Item 1" })
            .add_item(html! { "Item 2" })
            .marker_style(MarkerStyle::Square);

        assert_eq!(list.items.len(), 2);
        assert_eq!(list.marker_style, Some(MarkerStyle::Square));
    }

    #[test]
    fn test_text_size_values() {
        assert_eq!(TextSize::Xs.css_value(), "0.75rem");
        assert_eq!(TextSize::Md.css_value(), "1rem");
        assert_eq!(TextSize::Xxxl.css_value(), "1.875rem");
    }

    #[test]
    fn test_font_weight_values() {
        assert_eq!(FontWeight::Normal.css_value(), "400");
        assert_eq!(FontWeight::Bold.css_value(), "700");
    }
}
