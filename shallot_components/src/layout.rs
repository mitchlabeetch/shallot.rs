use maud::{html, Markup, Render};
use shallot_foundation::{Breakpoint, ResponsiveValue};

/// Responsive Container component
pub struct Container {
    children: Markup,
    max_width: Option<u32>,
    fluid: bool,
    centered: bool,
    padding: ResponsiveValue<u16>,
    class: Option<String>,
}

impl Container {
    /// Create a new container
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            max_width: None,
            fluid: false,
            centered: true,
            padding: ResponsiveValue::new(16),
            class: None,
        }
    }

    /// Set maximum width in pixels
    pub fn max_width(mut self, width: u32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Make container fluid (full width)
    pub fn fluid(mut self, fluid: bool) -> Self {
        self.fluid = fluid;
        self
    }

    /// Set centered alignment
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = ResponsiveValue::new(padding);
        self
    }

    /// Set responsive padding
    pub fn padding_responsive(mut self, padding: ResponsiveValue<u16>) -> Self {
        self.padding = padding;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Render for Container {
    fn render(&self) -> Markup {
        let mut classes = vec!["sh-container"];

        if self.fluid {
            classes.push("sh-container--fluid");
        }

        if self.centered {
            classes.push("sh-container--centered");
        }

        if let Some(custom) = &self.class {
            classes.push(custom);
        }

        let style = if let Some(max_w) = self.max_width {
            format!("max-width: {}px;", max_w)
        } else {
            String::new()
        };

        html! {
            div class=(classes.join(" ")) style=(style) {
                (self.children.clone())
            }
        }
    }
}

/// Grid layout component with responsive columns
pub struct Grid {
    children: Markup,
    columns: ResponsiveValue<u8>,
    gap: u16,
    row_gap: Option<u16>,
    min_child_width: Option<u16>,
    auto_fit: bool,
    class: Option<String>,
}

impl Grid {
    /// Create a new grid
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            columns: ResponsiveValue::new(1).with_sm(2).with_md(3).with_lg(4),
            gap: 16,
            row_gap: None,
            min_child_width: None,
            auto_fit: false,
            class: None,
        }
    }

    /// Set number of columns
    pub fn columns(mut self, cols: u8) -> Self {
        self.columns = ResponsiveValue::new(cols);
        self
    }

    /// Set responsive columns
    pub fn columns_responsive(mut self, cols: ResponsiveValue<u8>) -> Self {
        self.columns = cols;
        self
    }

    /// Set gap between items
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Set row gap
    pub fn row_gap(mut self, gap: u16) -> Self {
        self.row_gap = Some(gap);
        self
    }

    /// Set minimum child width (for auto-fit)
    pub fn min_child_width(mut self, width: u16) -> Self {
        self.min_child_width = Some(width);
        self.auto_fit = true;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    fn build_style(&self) -> String {
        let gap = self.gap;
        let row_gap = self.row_gap.unwrap_or(gap);

        if self.auto_fit && self.min_child_width.is_some() {
            let min_w = self.min_child_width.unwrap();
            format!(
                "display: grid; grid-template-columns: repeat(auto-fit, minmax({}px, 1fr)); gap: {}px {};",
                min_w, gap,
                if row_gap != gap { format!("row-gap: {}px;", row_gap) } else { String::new() }
            )
        } else {
            let cols = self.columns.get(Breakpoint::Xs).unwrap_or(&1);
            format!(
                "display: grid; grid-template-columns: repeat({}, minmax(0, 1fr)); gap: {}px {};",
                cols,
                gap,
                if row_gap != gap {
                    format!("row-gap: {}px;", row_gap)
                } else {
                    String::new()
                }
            )
        }
    }
}

impl Render for Grid {
    fn render(&self) -> Markup {
        let mut classes = vec!["sh-grid"];
        if let Some(custom) = &self.class {
            classes.push(custom);
        }

        html! {
            div
                class=(classes.join(" "))
                style=(self.build_style())
                role="region"
                aria-label="Grid layout"
            {
                (self.children.clone())
            }
        }
    }
}

/// Stack layout component (flexbox)
pub struct Stack {
    children: Markup,
    gap: u16,
    horizontal: bool,
    wrap: bool,
    align_items: AlignItems,
    justify_content: JustifyContent,
    class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl AlignItems {
    fn css(&self) -> &'static str {
        match self {
            AlignItems::Start => "flex-start",
            AlignItems::Center => "center",
            AlignItems::End => "flex-end",
            AlignItems::Stretch => "stretch",
            AlignItems::Baseline => "baseline",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

impl JustifyContent {
    fn css(&self) -> &'static str {
        match self {
            JustifyContent::Start => "flex-start",
            JustifyContent::Center => "center",
            JustifyContent::End => "flex-end",
            JustifyContent::Between => "space-between",
            JustifyContent::Around => "space-around",
            JustifyContent::Evenly => "space-evenly",
        }
    }
}

impl Stack {
    /// Create a new vertical stack
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            gap: 16,
            horizontal: false,
            wrap: false,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Start,
            class: None,
        }
    }

    /// Create a new horizontal stack
    pub fn row(children: Markup) -> Self {
        Self {
            children,
            gap: 16,
            horizontal: true,
            wrap: false,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            class: None,
        }
    }

    /// Set gap between items
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Set horizontal layout
    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    /// Set flex wrap
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Set align items
    pub fn align(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    /// Set justify content
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Render for Stack {
    fn render(&self) -> Markup {
        let mut classes = vec!["sh-stack"];
        if self.horizontal {
            classes.push("sh-stack--horizontal");
        }
        if self.wrap {
            classes.push("sh-stack--wrap");
        }
        if let Some(custom) = &self.class {
            classes.push(custom);
        }

        let direction = if self.horizontal { "row" } else { "column" };
        let wrap_val = if self.wrap { "wrap" } else { "nowrap" };

        let style = format!(
            "display: flex; flex-direction: {}; gap: {}px; align-items: {}; justify-content: {}; flex-wrap: {};",
            direction, self.gap, self.align_items.css(), self.justify_content.css(), wrap_val
        );

        html! {
            div class=(classes.join(" ")) style=(style) {
                (self.children.clone())
            }
        }
    }
}

/// Divider component
pub struct Divider {
    vertical: bool,
    inset: bool,
    class: Option<String>,
}

impl Divider {
    /// Create a horizontal divider
    pub fn new() -> Self {
        Self {
            vertical: false,
            inset: false,
            class: None,
        }
    }

    /// Create a vertical divider
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            inset: false,
            class: None,
        }
    }

    /// Set inset mode
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Divider {
    fn render(&self) -> Markup {
        let mut classes = vec!["sh-divider"];

        if self.vertical {
            classes.push("sh-divider--vertical");
        }

        if self.inset {
            classes.push("sh-divider--inset");
        }

        if let Some(custom) = &self.class {
            classes.push(custom);
        }

        html! {
            hr class=(classes.join(" ")) {}
        }
    }
}

/// Spacer component for flexible spacing
pub struct Spacer {
    size: Option<u16>,
    flex: bool,
}

impl Spacer {
    /// Create a flexible spacer that fills available space
    pub fn flex() -> Self {
        Self {
            size: None,
            flex: true,
        }
    }

    /// Create a fixed-size spacer
    pub fn size(size: u16) -> Self {
        Self {
            size: Some(size),
            flex: false,
        }
    }
}

impl Render for Spacer {
    fn render(&self) -> Markup {
        let style = if self.flex {
            "flex: 1;".to_string()
        } else if let Some(s) = self.size {
            format!("width: {}px; height: {}px;", s, s)
        } else {
            String::new()
        };

        html! {
            div class="sh-spacer" style=(style) {}
        }
    }
}

/// Section component for page layout
pub struct Section {
    children: Markup,
    class: Option<String>,
    padding_y: u16,
    bg_color: Option<String>,
}

impl Section {
    /// Create a new section
    pub fn new(children: Markup) -> Self {
        Self {
            children,
            class: None,
            padding_y: 64,
            bg_color: None,
        }
    }

    /// Set vertical padding
    pub fn padding_y(mut self, padding: u16) -> Self {
        self.padding_y = padding;
        self
    }

    /// Set background color
    pub fn bg_color(mut self, color: impl Into<String>) -> Self {
        self.bg_color = Some(color.into());
        self
    }

    /// Add custom class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Render for Section {
    fn render(&self) -> Markup {
        let mut classes = vec!["sh-section"];
        if let Some(custom) = &self.class {
            classes.push(custom);
        }

        let mut style = format!(
            "padding-top: {}px; padding-bottom: {}px;",
            self.padding_y, self.padding_y
        );
        if let Some(bg) = &self.bg_color {
            style.push_str(&format!(" background-color: {};", bg));
        }

        html! {
            section class=(classes.join(" ")) style=(style) {
                (self.children.clone())
            }
        }
    }
}

/// Aspect ratio container
pub struct AspectRatio {
    children: Markup,
    ratio: f32,
    class: Option<String>,
}

impl AspectRatio {
    /// Create an aspect ratio container
    pub fn new(children: Markup, ratio: f32) -> Self {
        Self {
            children,
            ratio,
            class: None,
        }
    }

    /// Common 16:9 aspect ratio
    pub fn video(children: Markup) -> Self {
        Self::new(children, 16.0 / 9.0)
    }

    /// Common 4:3 aspect ratio
    pub fn standard(children: Markup) -> Self {
        Self::new(children, 4.0 / 3.0)
    }

    /// Common 1:1 square aspect ratio
    pub fn square(children: Markup) -> Self {
        Self::new(children, 1.0)
    }

    /// Add custom class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Render for AspectRatio {
    fn render(&self) -> Markup {
        let mut classes = vec!["sh-aspect-ratio"];
        if let Some(custom) = &self.class {
            classes.push(custom);
        }

        let padding_bottom = (1.0 / self.ratio) * 100.0;

        html! {
            div class=(classes.join(" ")) style=(format!("position: relative; width: 100%; padding-bottom: {}%;", padding_bottom)) {
                div style="position: absolute; inset: 0;" {
                    (self.children.clone())
                }
            }
        }
    }
}

/// Generate CSS for layout components
pub fn layout_css() -> String {
    r#"
/* Container */
.sh-container {
  width: 100%;
  max-width: 1200px;
  margin-left: auto;
  margin-right: auto;
  padding-left: 1rem;
  padding-right: 1rem;
}

.sh-container--fluid {
  max-width: none;
}

.sh-container--centered {
  display: flex;
  flex-direction: column;
  align-items: center;
}

@media (min-width: 640px) {
  .sh-container {
    padding-left: 1.5rem;
    padding-right: 1.5rem;
  }
}

@media (min-width: 1024px) {
  .sh-container {
    padding-left: 2rem;
    padding-right: 2rem;
  }
}

/* Grid */
.sh-grid {
  display: grid;
  gap: 1rem;
}

/* Stack */
.sh-stack {
  display: flex;
}

.sh-stack--horizontal {
  flex-direction: row;
}

.sh-stack--wrap {
  flex-wrap: wrap;
}

/* Divider */
.sh-divider {
  border: none;
  border-top: 1px solid var(--sh-border);
  margin: 0;
}

.sh-divider--vertical {
  border-top: none;
  border-left: 1px solid var(--sh-border);
  height: 100%;
  min-height: 1rem;
}

.sh-divider--inset {
  margin: 0 1rem;
}

/* Spacer */
.sh-spacer {
  display: block;
}

/* Section */
.sh-section {
  width: 100%;
}

/* Aspect Ratio */
.sh-aspect-ratio {
  overflow: hidden;
}

.sh-aspect-ratio > * {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

/* Responsive utilities */
@media (max-width: 640px) {
  .sh-hide-mobile {
    display: none !important;
  }
}

@media (min-width: 641px) {
  .sh-show-mobile-only {
    display: none !important;
  }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let container = Container::new(html! { "Content" })
            .max_width(800)
            .centered(true);

        assert_eq!(container.max_width, Some(800));
        assert!(container.centered);
    }

    #[test]
    fn test_grid() {
        let grid = Grid::new(html! { "Items" })
            .columns(3)
            .gap(24)
            .min_child_width(200);

        assert!(grid.auto_fit);
        assert_eq!(grid.gap, 24);
    }

    #[test]
    fn test_stack() {
        let stack = Stack::row(html! { "Items" })
            .gap(8)
            .align(AlignItems::Center)
            .justify(JustifyContent::Between);

        assert!(stack.horizontal);
        assert_eq!(stack.gap, 8);
        assert_eq!(stack.align_items, AlignItems::Center);
    }

    #[test]
    fn test_divider() {
        let divider = Divider::vertical().inset(true);
        assert!(divider.vertical);
        assert!(divider.inset);
    }

    #[test]
    fn test_aspect_ratio() {
        let video = AspectRatio::video(html! { "Video" });
        assert_eq!(video.ratio, 16.0 / 9.0);

        let square = AspectRatio::square(html! { "Image" });
        assert_eq!(square.ratio, 1.0);
    }

    #[test]
    fn test_spacer() {
        let flex = Spacer::flex();
        assert!(flex.flex);

        let fixed = Spacer::size(16);
        assert_eq!(fixed.size, Some(16));
    }
}
