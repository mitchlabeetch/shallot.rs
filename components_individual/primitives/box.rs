use maud::{html, Markup, Render};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Flex,
    Grid,
    Inline,
    InlineFlex,
    InlineGrid,
    None,
}

impl Display {
    fn to_css(&self) -> &'static str {
        match self {
            Display::Block => "block",
            Display::Flex => "flex",
            Display::Grid => "grid",
            Display::Inline => "inline",
            Display::InlineFlex => "inline-flex",
            Display::InlineGrid => "inline-grid",
            Display::None => "none",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Direction {
    fn to_css(&self) -> &'static str {
        match self {
            Direction::Row => "row",
            Direction::RowReverse => "row-reverse",
            Direction::Column => "column",
            Direction::ColumnReverse => "column-reverse",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Stretch,
    Start,
    Center,
    End,
    Baseline,
}

impl Align {
    fn to_css(&self) -> &'static str {
        match self {
            Align::Stretch => "stretch",
            Align::Start => "flex-start",
            Align::Center => "center",
            Align::End => "flex-end",
            Align::Baseline => "baseline",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Justify {
    fn to_css(&self) -> &'static str {
        match self {
            Justify::Start => "flex-start",
            Justify::Center => "center",
            Justify::End => "flex-end",
            Justify::SpaceBetween => "space-between",
            Justify::SpaceAround => "space-around",
            Justify::SpaceEvenly => "space-evenly",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    fn to_css(&self) -> &'static str {
        match self {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Absolute => "absolute",
            Position::Fixed => "fixed",
            Position::Sticky => "sticky",
        }
    }
}

pub struct Box {
    display: Display,
    position: Position,
    direction: Direction,
    align: Align,
    justify: Justify,
    gap: u32,
    padding: u32,
    margin: Option<u32>,
    width: Option<String>,
    height: Option<String>,
    min_width: Option<String>,
    max_width: Option<String>,
    min_height: Option<String>,
    max_height: Option<String>,
    overflow: Option<String>,
    glass: bool,
    border: bool,
    border_radius: Option<String>,
    background: Option<String>,
    opacity: Option<f32>,
    z_index: Option<i32>,
    children: Vec<Markup>,
}

impl Box {
    pub fn new() -> Self {
        Self {
            display: Display::Block,
            position: Position::Static,
            direction: Direction::Row,
            align: Align::Stretch,
            justify: Justify::Start,
            gap: 0,
            padding: 0,
            margin: None,
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            overflow: None,
            glass: false,
            border: false,
            border_radius: None,
            background: None,
            opacity: None,
            z_index: None,
            children: Vec::new(),
        }
    }

    pub fn display(mut self, display: Display) -> Self {
        self.display = display;
        self
    }
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }
    pub fn gap(mut self, px: u32) -> Self {
        self.gap = px;
        self
    }
    pub fn padding(mut self, px: u32) -> Self {
        self.padding = px;
        self
    }
    pub fn margin(mut self, px: u32) -> Self {
        self.margin = Some(px);
        self
    }
    pub fn margin_top(mut self, px: u32) -> Self {
        self.margin = Some(px);
        self
    }
    pub fn margin_bottom(mut self, px: u32) -> Self {
        self.margin = Some(px);
        self
    }
    pub fn width(mut self, w: impl Into<String>) -> Self {
        self.width = Some(w.into());
        self
    }
    pub fn height(mut self, h: impl Into<String>) -> Self {
        self.height = Some(h.into());
        self
    }
    pub fn min_width(mut self, w: impl Into<String>) -> Self {
        self.min_width = Some(w.into());
        self
    }
    pub fn max_width(mut self, w: impl Into<String>) -> Self {
        self.max_width = Some(w.into());
        self
    }
    pub fn min_height(mut self, h: impl Into<String>) -> Self {
        self.min_height = Some(h.into());
        self
    }
    pub fn max_height(mut self, h: impl Into<String>) -> Self {
        self.max_height = Some(h.into());
        self
    }
    pub fn overflow(mut self, o: impl Into<String>) -> Self {
        self.overflow = Some(o.into());
        self
    }
    pub fn overflow_auto(self) -> Self {
        self.overflow("auto".to_string())
    }
    pub fn overflow_hidden(self) -> Self {
        self.overflow("hidden".to_string())
    }
    pub fn overflow_scroll(self) -> Self {
        self.overflow("scroll".to_string())
    }
    pub fn glass(mut self) -> Self {
        self.glass = true;
        self
    }
    pub fn border(mut self) -> Self {
        self.border = true;
        self
    }
    pub fn border_radius(mut self, r: impl Into<String>) -> Self {
        self.border_radius = Some(r.into());
        self
    }
    pub fn background(mut self, bg: impl Into<String>) -> Self {
        self.background = Some(bg.into());
        self
    }
    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = Some(o);
        self
    }
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = Some(z);
        self
    }

    pub fn full_width(self) -> Self {
        self.width("100%")
    }
    pub fn full_height(self) -> Self {
        self.height("100%")
    }
    pub fn full(self) -> Self {
        self.full_width().full_height()
    }

    pub fn flex(self) -> Self {
        self.display(Display::Flex)
    }
    pub fn grid(self) -> Self {
        self.display(Display::Grid)
    }
    pub fn inline_flex(self) -> Self {
        self.display(Display::InlineFlex)
    }

    pub fn horizontal(self) -> Self {
        self.direction(Direction::Row)
    }
    pub fn vertical(self) -> Self {
        self.direction(Direction::Column)
    }

    pub fn sticky(self) -> Self {
        self.position(Position::Sticky)
    }
    pub fn fixed(self) -> Self {
        self.position(Position::Fixed)
    }
    pub fn absolute(self) -> Self {
        self.position(Position::Absolute)
    }
    pub fn relative(self) -> Self {
        self.position(Position::Relative)
    }

    pub fn center(self) -> Self {
        self.align(Align::Center).justify(Justify::Center)
    }
    pub fn center_x(self) -> Self {
        self.justify(Justify::Center)
    }
    pub fn center_y(self) -> Self {
        self.align(Align::Center)
    }

    pub fn child(mut self, child: Markup) -> Self {
        self.children.push(child);
        self
    }

    fn build_style(&self) -> String {
        let mut s = String::new();

        if self.display != Display::Block {
            s.push_str(&format!("display:{};", self.display.to_css()));
        }

        if self.position != Position::Static {
            s.push_str(&format!("position:{};", self.position.to_css()));
        }

        if self.display == Display::Flex || self.display == Display::InlineFlex {
            s.push_str(&format!("flex-direction:{};", self.direction.to_css()));
            s.push_str(&format!("align-items:{};", self.align.to_css()));
            s.push_str(&format!("justify-content:{};", self.justify.to_css()));

            if self.gap > 0 {
                s.push_str(&format!("gap:{}px;", self.gap));
            }
        }

        if self.padding > 0 {
            s.push_str(&format!("padding:{}px;", self.padding));
        }

        if let Some(m) = self.margin {
            s.push_str(&format!("margin:{}px;", m));
        }

        if let Some(ref w) = self.width {
            s.push_str(&format!("width:{};", w));
        }
        if let Some(ref h) = self.height {
            s.push_str(&format!("height:{};", h));
        }
        if let Some(ref mw) = self.min_width {
            s.push_str(&format!("min-width:{};", mw));
        }
        if let Some(ref mw) = self.max_width {
            s.push_str(&format!("max-width:{};", mw));
        }
        if let Some(ref mh) = self.min_height {
            s.push_str(&format!("min-height:{};", mh));
        }
        if let Some(ref mh) = self.max_height {
            s.push_str(&format!("max-height:{};", mh));
        }
        if let Some(ref o) = self.overflow {
            s.push_str(&format!("overflow:{};", o));
        }
        if let Some(ref br) = self.border_radius {
            s.push_str(&format!("border-radius:{};", br));
        }
        if let Some(ref bg) = self.background {
            s.push_str(&format!("background:{};", bg));
        }
        if let Some(o) = self.opacity {
            s.push_str(&format!("opacity:{};", o));
        }
        if let Some(z) = self.z_index {
            s.push_str(&format!("z-index:{};", z));
        }

        if self.glass {
            s.push_str("background:rgba(255,255,255,0.05);");
            s.push_str("backdrop-filter:blur(12px) saturate(180%);");
            s.push_str("border:1px solid rgba(255,255,255,0.1);");
        }

        if self.border {
            s.push_str("border:1px solid var(--sh-border);");
        }

        s
    }
}

impl Default for Box {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Box {
    fn render(&self) -> Markup {
        html! {
            div class="sh-box" style={(self.build_style())} {
                @for child in &self.children { (child) }
            }
        }
    }
}

// Convenience wrappers

pub struct Flex(Box);
impl Flex {
    pub fn new() -> Self {
        Flex(Box::new().display(Display::Flex))
    }
    pub fn row(self) -> Flex {
        Flex(self.0.direction(Direction::Row))
    }
    pub fn column(self) -> Flex {
        Flex(self.0.direction(Direction::Column))
    }
    pub fn gap(mut self, px: u32) -> Flex {
        self.0.gap = px;
        self
    }
    pub fn center(self) -> Flex {
        Flex(self.0.align(Align::Center).justify(Justify::Center))
    }
    pub fn between(self) -> Flex {
        Flex(self.0.justify(Justify::SpaceBetween))
    }
    pub fn child(mut self, child: Markup) -> Flex {
        self.0.children.push(child);
        self
    }
}
impl Default for Flex {
    fn default() -> Self {
        Self::new()
    }
}
impl Render for Flex {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

pub struct Stack(Box);
impl Stack {
    pub fn new() -> Self {
        Stack(
            Box::new()
                .display(Display::Flex)
                .direction(Direction::Column),
        )
    }
    pub fn gap(mut self, px: u32) -> Self {
        self.0.gap = px;
        self
    }
    pub fn child(mut self, child: Markup) -> Self {
        self.0.children.push(child);
        self
    }
}
impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
impl Render for Stack {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

pub struct InlineFlex(Box);
impl InlineFlex {
    pub fn new() -> Self {
        InlineFlex(Box::new().display(Display::InlineFlex))
    }
    pub fn row(self) -> InlineFlex {
        InlineFlex(self.0.direction(Direction::Row))
    }
    pub fn column(self) -> InlineFlex {
        InlineFlex(self.0.direction(Direction::Column))
    }
    pub fn gap(mut self, px: u32) -> InlineFlex {
        self.0.gap = px;
        self
    }
    pub fn center(self) -> InlineFlex {
        InlineFlex(self.0.align(Align::Center).justify(Justify::Center))
    }
    pub fn child(mut self, child: Markup) -> InlineFlex {
        self.0.children.push(child);
        self
    }
}
impl Default for InlineFlex {
    fn default() -> Self {
        Self::new()
    }
}
impl Render for InlineFlex {
    fn render(&self) -> Markup {
        self.0.render()
    }
}
