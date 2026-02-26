use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    Xs,  // 0px and up
    Sm,  // 640px and up
    Md,  // 768px and up
    Lg,  // 1024px and up
    Xl,  // 1280px and up
    Xxl, // 1536px and up
}

impl Breakpoint {
    pub fn min_width(&self) -> u32 {
        match self {
            Breakpoint::Xs => 0,
            Breakpoint::Sm => 640,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 1024,
            Breakpoint::Xl => 1280,
            Breakpoint::Xxl => 1536,
        }
    }

    pub fn max_width(&self) -> Option<u32> {
        match self {
            Breakpoint::Xs => Some(639),
            Breakpoint::Sm => Some(767),
            Breakpoint::Md => Some(1023),
            Breakpoint::Lg => Some(1279),
            Breakpoint::Xl => Some(1535),
            Breakpoint::Xxl => None,
        }
    }

    pub fn media_query(&self) -> String {
        let min = self.min_width();
        let max = self.max_width();
        
        match (min, max) {
            (min, Some(max)) => format!("@media (min-width: {}px) and (max-width: {}px)", min, max),
            (min, None) => format!("@media (min-width: {}px)", min),
        }
    }

    pub fn container_class(&self) -> &'static str {
        match self {
            Breakpoint::Xs => "sh-container-xs",
            Breakpoint::Sm => "sh-container-sm",
            Breakpoint::Md => "sh-container-md",
            Breakpoint::Lg => "sh-container-lg",
            Breakpoint::Xl => "sh-container-xl",
            Breakpoint::Xxl => "sh-container-xxl",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveProperty {
    Display,
    FlexDirection,
    JustifyContent,
    AlignItems,
    TextAlign,
    Position,
    Overflow,
    Width,
    Height,
    Margin,
    Padding,
    FontSize,
    Gap,
    GridTemplateColumns,
    GridTemplateRows,
}

#[derive(Debug, Clone)]
pub struct ResponsiveValue<T> {
    pub xs: Option<T>,
    pub sm: Option<T>,
    pub md: Option<T>,
    pub lg: Option<T>,
    pub xl: Option<T>,
    pub xxl: Option<T>,
}

impl<T> ResponsiveValue<T> {
    pub fn new(base: T) -> Self 
    where
        T: Clone,
    {
        Self {
            xs: Some(base.clone()),
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }

    pub fn with_sm(mut self, value: T) -> Self {
        self.sm = Some(value);
        self
    }

    pub fn with_md(mut self, value: T) -> Self {
        self.md = Some(value);
        self
    }

    pub fn with_lg(mut self, value: T) -> Self {
        self.lg = Some(value);
        self
    }

    pub fn with_xl(mut self, value: T) -> Self {
        self.xl = Some(value);
        self
    }

    pub fn with_xxl(mut self, value: T) -> Self {
        self.xxl = Some(value);
        self
    }

    pub fn get(&self, breakpoint: Breakpoint) -> Option<&T> {
        match breakpoint {
            Breakpoint::Xs => self.xs.as_ref(),
            Breakpoint::Sm => self.sm.as_ref().or(self.xs.as_ref()),
            Breakpoint::Md => self.md.as_ref().or(self.sm.as_ref()).or(self.xs.as_ref()),
            Breakpoint::Lg => self.lg.as_ref().or(self.md.as_ref()).or(self.sm.as_ref()).or(self.xs.as_ref()),
            Breakpoint::Xl => self.xl.as_ref().or(self.lg.as_ref()).or(self.md.as_ref()).or(self.sm.as_ref()).or(self.xs.as_ref()),
            Breakpoint::Xxl => self.xxl.as_ref().or(self.xl.as_ref()).or(self.lg.as_ref()).or(self.md.as_ref()).or(self.sm.as_ref()).or(self.xs.as_ref()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub max_widths: HashMap<Breakpoint, u32>,
    pub padding: ResponsiveValue<u32>,
    pub center: bool,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        let mut max_widths = HashMap::new();
        max_widths.insert(Breakpoint::Sm, 640);
        max_widths.insert(Breakpoint::Md, 768);
        max_widths.insert(Breakpoint::Lg, 1024);
        max_widths.insert(Breakpoint::Xl, 1280);
        max_widths.insert(Breakpoint::Xxl, 1536);

        Self {
            max_widths,
            padding: ResponsiveValue::new(16).with_md(24).with_lg(32),
            center: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridConfig {
    pub columns: ResponsiveValue<u8>,
    pub gap: ResponsiveValue<u32>,
    pub row_gap: Option<ResponsiveValue<u32>>,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            columns: ResponsiveValue::new(1).with_sm(2).with_md(3).with_lg(4),
            gap: ResponsiveValue::new(16).with_md(24).with_lg(32),
            row_gap: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlexConfig {
    pub direction: ResponsiveValue<FlexDirection>,
    pub wrap: ResponsiveValue<FlexWrap>,
    pub justify: ResponsiveValue<JustifyContent>,
    pub align: ResponsiveValue<AlignItems>,
    pub gap: ResponsiveValue<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl Default for FlexConfig {
    fn default() -> Self {
        Self {
            direction: ResponsiveValue::new(FlexDirection::Row),
            wrap: ResponsiveValue::new(FlexWrap::NoWrap),
            justify: ResponsiveValue::new(JustifyContent::Start),
            align: ResponsiveValue::new(AlignItems::Stretch),
            gap: ResponsiveValue::new(0),
        }
    }
}

pub fn generate_responsive_css() -> String {
    let breakpoints = vec![
        Breakpoint::Xs,
        Breakpoint::Sm,
        Breakpoint::Md,
        Breakpoint::Lg,
        Breakpoint::Xl,
        Breakpoint::Xxl,
    ];

    let mut css_sections = vec![];

    // Container classes
    css_sections.push(r#"
/* Container */
.sh-container {
  width: 100%;
  margin-left: auto;
  margin-right: auto;
  padding-left: var(--sh-container-padding, 1rem);
  padding-right: var(--sh-container-padding, 1rem);
}

.sh-container-center {
  display: flex;
  flex-direction: column;
  align-items: center;
}
"#.to_string());

    for breakpoint in &breakpoints {
        if let Some(max_width) = ContainerConfig::default().max_widths.get(breakpoint) {
            css_sections.push(format!(
                r#"
@media (min-width: {}px) {{
  .sh-container {{
    max-width: {}px;
    padding-left: var(--sh-container-padding-md, 1.5rem);
    padding-right: var(--sh-container-padding-md, 1.5rem);
  }}
  
  .sh-container-{} {{
    max-width: {}px;
  }}
}}
"#,
                breakpoint.min_width(),
                max_width,
                breakpoint.container_class().replace("sh-container-", ""),
                max_width
            ));
        }
    }

    // Grid system
    css_sections.push(r#"
/* Grid System */
.sh-grid {
  display: grid;
  gap: var(--sh-grid-gap, 1rem);
}

.sh-grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
.sh-grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
.sh-grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
.sh-grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
.sh-grid-cols-5 { grid-template-columns: repeat(5, minmax(0, 1fr)); }
.sh-grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
"#.to_string());

    for breakpoint in &breakpoints {
        if breakpoint.min_width() > 0 {
            css_sections.push(format!(
                r#"
@media (min-width: {}px) {{
  .sh-grid-cols-sm-1 {{ grid-template-columns: repeat(1, minmax(0, 1fr)); }}
  .sh-grid-cols-sm-2 {{ grid-template-columns: repeat(2, minmax(0, 1fr)); }}
  .sh-grid-cols-sm-3 {{ grid-template-columns: repeat(3, minmax(0, 1fr)); }}
  .sh-grid-cols-sm-4 {{ grid-template-columns: repeat(4, minmax(0, 1fr)); }}
}}
"#,
                breakpoint.min_width()
            ));
        }
    }

    // Flexbox utilities
    css_sections.push(r#"
/* Flexbox */
.sh-flex { display: flex; }
.sh-inline-flex { display: inline-flex; }
.sh-flex-row { flex-direction: row; }
.sh-flex-col { flex-direction: column; }
.sh-flex-wrap { flex-wrap: wrap; }
.sh-flex-nowrap { flex-wrap: nowrap; }
.sh-items-start { align-items: flex-start; }
.sh-items-center { align-items: center; }
.sh-items-end { align-items: flex-end; }
.sh-justify-start { justify-content: flex-start; }
.sh-justify-center { justify-content: center; }
.sh-justify-end { justify-content: flex-end; }
.sh-justify-between { justify-content: space-between; }
.sh-gap-1 { gap: 0.25rem; }
.sh-gap-2 { gap: 0.5rem; }
.sh-gap-4 { gap: 1rem; }
.sh-gap-6 { gap: 1.5rem; }
.sh-gap-8 { gap: 2rem; }
"#.to_string());

    // Spacing utilities
    css_sections.push(r#"
/* Spacing */
.sh-m-0 { margin: 0; }
.sh-m-1 { margin: 0.25rem; }
.sh-m-2 { margin: 0.5rem; }
.sh-m-4 { margin: 1rem; }
.sh-m-6 { margin: 1.5rem; }
.sh-m-8 { margin: 2rem; }
.sh-p-0 { padding: 0; }
.sh-p-1 { padding: 0.25rem; }
.sh-p-2 { padding: 0.5rem; }
.sh-p-4 { padding: 1rem; }
.sh-p-6 { padding: 1.5rem; }
.sh-p-8 { padding: 2rem; }
"#.to_string());

    // Text utilities
    css_sections.push(r#"
/* Typography */
.sh-text-xs { font-size: 0.75rem; line-height: 1rem; }
.sh-text-sm { font-size: 0.875rem; line-height: 1.25rem; }
.sh-text-base { font-size: 1rem; line-height: 1.5rem; }
.sh-text-lg { font-size: 1.125rem; line-height: 1.75rem; }
.sh-text-xl { font-size: 1.25rem; line-height: 1.75rem; }
.sh-text-2xl { font-size: 1.5rem; line-height: 2rem; }
.sh-text-3xl { font-size: 1.875rem; line-height: 2.25rem; }
.sh-text-4xl { font-size: 2.25rem; line-height: 2.5rem; }
.sh-text-center { text-align: center; }
.sh-text-left { text-align: left; }
.sh-text-right { text-align: right; }
"#.to_string());

    // Display utilities
    css_sections.push(r#"
/* Display */
.sh-block { display: block; }
.sh-inline-block { display: inline-block; }
.sh-inline { display: inline; }
.sh-hidden { display: none; }
.sh-sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

/* Responsive display */
@media (min-width: 640px) {
  .sh-sm\:block { display: block; }
  .sh-sm\:hidden { display: none; }
  .sh-sm\:flex { display: flex; }
}

@media (min-width: 768px) {
  .sh-md\:block { display: block; }
  .sh-md\:hidden { display: none; }
  .sh-md\:flex { display: flex; }
}

@media (min-width: 1024px) {
  .sh-lg\:block { display: block; }
  .sh-lg\:hidden { display: none; }
  .sh-lg\:flex { display: flex; }
}
"#.to_string());

    css_sections.join("\n")
}

pub fn generate_container_css(config: &ContainerConfig) -> String {
    let mut css = String::new();
    
    // Base container styles
    css.push_str(&format!(r#"
.sh-container {{
  width: 100%;
  margin-left: auto;
  margin-right: auto;
  padding-left: {}px;
  padding-right: {}px;
}}
"#,
        config.padding.get(Breakpoint::Xs).unwrap_or(&16),
        config.padding.get(Breakpoint::Xs).unwrap_or(&16)
    ));

    if config.center {
        css.push_str(r#"
.sh-container-center {
  display: flex;
  flex-direction: column;
  align-items: center;
}
"#);
    }

    // Responsive container widths
    for breakpoint in [Breakpoint::Sm, Breakpoint::Md, Breakpoint::Lg, Breakpoint::Xl, Breakpoint::Xxl] {
        if let Some(max_width) = config.max_widths.get(&breakpoint) {
            css.push_str(&format!(
                r#"
@media (min-width: {}px) {{
  .sh-container {{
    max-width: {}px;
    padding-left: {}px;
    padding-right: {}px;
  }}
}}
"#,
                breakpoint.min_width(),
                max_width,
                config.padding.get(breakpoint).unwrap_or(&24),
                config.padding.get(breakpoint).unwrap_or(&24)
            ));
        }
    }

    css
}

pub fn generate_grid_css(config: &GridConfig) -> String {
    let mut css = String::new();
    
    // Base grid styles
    css.push_str(r#"
.sh-grid {
  display: grid;
  gap: var(--sh-grid-gap, 1rem);
}
"#);

    // Responsive grid columns
    for breakpoint in [Breakpoint::Xs, Breakpoint::Sm, Breakpoint::Md, Breakpoint::Lg, Breakpoint::Xl, Breakpoint::Xxl] {
        if let Some(cols) = config.columns.get(breakpoint) {
            let class_suffix = match breakpoint {
                Breakpoint::Xs => String::new(),
                _ => format!("-{}", breakpoint.container_class().replace("sh-container-", "")),
            };
            
            if breakpoint.min_width() == 0 {
                css.push_str(&format!(
                    r#"
.sh-grid-cols{} {{
  grid-template-columns: repeat({}, minmax(0, 1fr));
}}
"#,
                    class_suffix,
                    cols
                ));
            } else {
                css.push_str(&format!(
                    r#"
@media (min-width: {}px) {{
  .sh-grid-cols{} {{
    grid-template-columns: repeat({}, minmax(0, 1fr));
  }}
}}
"#,
                    breakpoint.min_width(),
                    class_suffix,
                    cols
                ));
            }
        }
    }

    // Responsive gaps
    if let Some(gap) = config.gap.get(Breakpoint::Xs) {
        css.push_str(&format!(
            r#"
.sh-grid-gap {{
  gap: {}px;
}}
"#,
            gap
        ));
    }

    for breakpoint in [Breakpoint::Sm, Breakpoint::Md, Breakpoint::Lg, Breakpoint::Xl, Breakpoint::Xxl] {
        if let Some(gap) = config.gap.get(breakpoint) {
            css.push_str(&format!(
                r#"
@media (min-width: {}px) {{
  .sh-grid-gap-{breakpoint} {{
    gap: {}px;
  }}
}}
"#,
                breakpoint.min_width(),
                gap,
                breakpoint = breakpoint.container_class().replace("sh-container-", "")
            ));
        }
    }

    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_widths() {
        assert_eq!(Breakpoint::Xs.min_width(), 0);
        assert_eq!(Breakpoint::Sm.min_width(), 640);
        assert_eq!(Breakpoint::Md.min_width(), 768);
        assert_eq!(Breakpoint::Lg.min_width(), 1024);
    }

    #[test]
    fn test_responsive_value() {
        let value = ResponsiveValue::new(1)
            .with_sm(2)
            .with_md(3)
            .with_lg(4);

        assert_eq!(value.get(Breakpoint::Xs), Some(&1));
        assert_eq!(value.get(Breakpoint::Sm), Some(&2));
        assert_eq!(value.get(Breakpoint::Md), Some(&3));
        assert_eq!(value.get(Breakpoint::Lg), Some(&4));
        assert_eq!(value.get(Breakpoint::Xl), Some(&4)); // Falls back to lg
    }

    #[test]
    fn test_media_query_generation() {
        let sm_query = Breakpoint::Sm.media_query();
        assert!(sm_query.contains("(min-width: 640px)"));
        assert!(sm_query.contains("(max-width: 767px)"));

        let xxl_query = Breakpoint::Xxl.media_query();
        assert!(xxl_query.contains("(min-width: 1536px)"));
        assert!(!xxl_query.contains("max-width"));
    }
}