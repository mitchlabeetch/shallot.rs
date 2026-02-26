use maud::{html, Markup, Render};

pub struct Grid {
    cols: Option<u32>,
    rows: Option<u32>,
    gap: u32,
    col_gap: Option<u32>,
    row_gap: Option<u32>,
    min_col_width: Option<String>,
    children: Vec<Markup>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cols: None,
            rows: None,
            gap: 16,
            col_gap: None,
            row_gap: None,
            min_col_width: None,
            children: Vec::new(),
        }
    }

    pub fn cols(mut self, c: u32) -> Self {
        self.cols = Some(c);
        self
    }
    pub fn rows(mut self, r: u32) -> Self {
        self.rows = Some(r);
        self
    }
    pub fn gap(mut self, px: u32) -> Self {
        self.gap = px;
        self
    }
    pub fn col_gap(mut self, px: u32) -> Self {
        self.col_gap = Some(px);
        self
    }
    pub fn row_gap(mut self, px: u32) -> Self {
        self.row_gap = Some(px);
        self
    }
    pub fn min_col(mut self, w: impl Into<String>) -> Self {
        self.min_col_width = Some(w.into());
        self
    }
    pub fn auto_cols(self) -> Self {
        self.min_col("250px")
    }
    pub fn auto_fit(self) -> Self {
        self.min_col("minmax(250px, 1fr)")
    }

    pub fn child(mut self, child: Markup) -> Self {
        self.children.push(child);
        self
    }

    fn build_style(&self) -> String {
        let mut s = String::from("display:grid;");

        if let Some(c) = self.cols {
            s.push_str(&format!("grid-template-columns:repeat({}, 1fr);", c));
        } else if let Some(ref min) = self.min_col_width {
            s.push_str(&format!(
                "grid-template-columns:repeat(auto-fill, minmax({}, 1fr));",
                min
            ));
        }

        if let Some(r) = self.rows {
            s.push_str(&format!("grid-template-rows:repeat({}, 1fr);", r));
        }

        if let Some(cg) = self.col_gap {
            s.push_str(&format!("column-gap:{}px;", cg));
        } else {
            s.push_str(&format!("gap:{}px;", self.gap));
        }

        if let Some(rg) = self.row_gap {
            s.push_str(&format!("row-gap:{}px;", rg));
        }

        s
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Grid {
    fn render(&self) -> Markup {
        html! { div class="sh-grid" style={(self.build_style())} { @for child in &self.children { (child) } } }
    }
}

// Responsive Grid variants
pub struct Grid2 {
    children: Vec<Markup>,
}
impl Grid2 {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    pub fn child(mut self, c: Markup) -> Self {
        self.children.push(c);
        self
    }
}
impl Default for Grid2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Render for Grid2 {
    fn render(&self) -> Markup {
        html! { div class="sh-grid-2" style="display:grid;grid-template-columns:repeat(2, 1fr);gap:16px;" { @for child in &self.children { (child) } } }
    }
}

pub struct Grid3 {
    children: Vec<Markup>,
}
impl Grid3 {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    pub fn child(mut self, c: Markup) -> Self {
        self.children.push(c);
        self
    }
}
impl Default for Grid3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Render for Grid3 {
    fn render(&self) -> Markup {
        html! { div class="sh-grid-3" style="display:grid;grid-template-columns:repeat(3, 1fr);gap:16px;" { @for child in &self.children { (child) } } }
    }
}

pub struct Grid4 {
    children: Vec<Markup>,
}
impl Grid4 {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    pub fn child(mut self, c: Markup) -> Self {
        self.children.push(c);
        self
    }
}
impl Default for Grid4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Render for Grid4 {
    fn render(&self) -> Markup {
        html! { div class="sh-grid-4" style="display:grid;grid-template-columns:repeat(4, 1fr);gap:16px;" { @for child in &self.children { (child) } } }
    }
}

pub struct AutoGrid {
    children: Vec<Markup>,
    min_width: u32,
    gap: u32,
}
impl AutoGrid {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            min_width: 250,
            gap: 16,
        }
    }
    pub fn min_width(mut self, w: u32) -> Self {
        self.min_width = w;
        self
    }
    pub fn gap(mut self, g: u32) -> Self {
        self.gap = g;
        self
    }
    pub fn child(mut self, c: Markup) -> Self {
        self.children.push(c);
        self
    }
}
impl Default for AutoGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Render for AutoGrid {
    fn render(&self) -> Markup {
        html! {
            div class="sh-auto-grid" style={"display:grid;grid-template-columns:repeat(auto-fill, minmax({}, 1fr));gap:{}px;".replace("{}", &self.min_width.to_string()).replace("{}", &self.gap.to_string())} {
                @for child in &self.children { (child) }
            }
        }
    }
}
