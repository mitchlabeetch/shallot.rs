//! Charts & Data Visualization Components
//!
//! SVG-based data visualization components with zero JavaScript.
//! Implements research section 5.6.4 - Data Visualization.
//!
//! # Example
//! ```
//! use shallot_components::charts::{Sparkline, ChartColor};
//!
//! let sparkline = Sparkline::new(vec![10.0, 25.0, 15.0, 30.0, 45.0, 20.0])
//!     .width(120)
//!     .height(30)
//!     .color(ChartColor::Custom("#6366f1"))
//!     .stroke_width(2);
//! ```

use maud::{html, Markup};

/// Chart color scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartColor {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Custom(&'static str),
}

impl ChartColor {
    pub fn css_value(&self) -> String {
        match self {
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

/// Line curve type for smoothing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveType {
    Linear, // Straight lines between points
    Smooth, // Bezier curve smoothing
    Step,   // Step chart
}

/// Sparkline component - miniature line chart
#[derive(Debug, Clone)]
pub struct Sparkline {
    data: Vec<f64>,
    width: u16,
    height: u16,
    color: ChartColor,
    stroke_width: u8,
    fill: bool,
    fill_opacity: f32,
    curve: CurveType,
    show_dots: bool,
    dot_radius: u8,
    animate: bool,
    animation_duration: u16,
    aria_label: Option<String>,
    class: Option<String>,
}

impl Sparkline {
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            width: 120,
            height: 30,
            color: ChartColor::Primary,
            stroke_width: 2,
            fill: true,
            fill_opacity: 0.2,
            curve: CurveType::Linear,
            show_dots: false,
            dot_radius: 3,
            animate: true,
            animation_duration: 1000,
            aria_label: None,
            class: None,
        }
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = color;
        self
    }

    pub fn custom_color(mut self, color: &'static str) -> Self {
        self.color = ChartColor::Custom(color);
        self
    }

    pub fn stroke_width(mut self, width: u8) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.fill_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn curve(mut self, curve: CurveType) -> Self {
        self.curve = curve;
        self
    }

    pub fn show_dots(mut self, show: bool) -> Self {
        self.show_dots = show;
        self
    }

    pub fn dot_radius(mut self, radius: u8) -> Self {
        self.dot_radius = radius;
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    pub fn animation_duration(mut self, duration: u16) -> Self {
        self.animation_duration = duration;
        self
    }

    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// Get min and max values from data
    fn data_range(&self) -> (f64, f64) {
        if self.data.is_empty() {
            return (0.0, 1.0);
        }

        let min = self.data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = self.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        // Add some padding if min == max
        if min == max {
            (min - 1.0, max + 1.0)
        } else {
            (min, max)
        }
    }

    /// Convert data point to SVG coordinates
    fn point_to_coords(&self, index: usize, value: f64, padding: u8) -> (f64, f64) {
        let (min, max) = self.data_range();
        let range = max - min;

        let available_width = (self.width as f64) - (padding as f64 * 2.0);
        let available_height = (self.height as f64) - (padding as f64 * 2.0);

        let x = padding as f64
            + (index as f64 / (self.data.len().saturating_sub(1).max(1) as f64)) * available_width;
        let y = (self.height as f64) - padding as f64 - ((value - min) / range) * available_height;

        (x, y)
    }

    /// Generate SVG path for the data
    fn generate_path(&self, padding: u8) -> String {
        if self.data.len() < 2 {
            return String::new();
        }

        let points: Vec<(f64, f64)> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, &v)| self.point_to_coords(i, v, padding))
            .collect();

        match self.curve {
            CurveType::Linear => self.generate_linear_path(&points),
            CurveType::Smooth => self.generate_smooth_path(&points),
            CurveType::Step => self.generate_step_path(&points),
        }
    }

    fn generate_linear_path(&self, points: &[(f64, f64)]) -> String {
        let mut path = format!("M {:.1} {:.1}", points[0].0, points[0].1);

        for point in &points[1..] {
            path.push_str(&format!(" L {:.1} {:.1}", point.0, point.1));
        }

        path
    }

    fn generate_smooth_path(&self, points: &[(f64, f64)]) -> String {
        if points.len() < 2 {
            return String::new();
        }

        let mut path = format!("M {:.1} {:.1}", points[0].0, points[0].1);

        for i in 1..points.len() {
            let prev = if i > 1 { points[i - 1] } else { points[0] };
            let curr = points[i];
            let next = if i + 1 < points.len() {
                points[i + 1]
            } else {
                curr
            };

            // Simple bezier smoothing
            let cp1x = prev.0 + (curr.0 - prev.0) * 0.5;
            let cp1y = prev.1;
            let cp2x = curr.0 - (next.0 - prev.0) * 0.25;
            let cp2y = curr.1;

            path.push_str(&format!(
                " C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}",
                cp1x, cp1y, cp2x, cp2y, curr.0, curr.1
            ));
        }

        path
    }

    fn generate_step_path(&self, points: &[(f64, f64)]) -> String {
        let mut path = format!("M {:.1} {:.1}", points[0].0, points[0].1);

        for i in 1..points.len() {
            let _prev = points[i - 1];
            let curr = points[i];

            // Step: horizontal then vertical
            path.push_str(&format!(" H {:.1} V {:.1}", curr.0, curr.1));
        }

        path
    }

    /// Generate fill path (closes the area under the line)
    fn generate_fill_path(&self, padding: u8) -> String {
        let line_path = self.generate_path(padding);
        if line_path.is_empty() {
            return String::new();
        }

        let (min_x, _) = self.point_to_coords(0, self.data_range().0, padding);
        let (max_x, max_y) =
            self.point_to_coords(self.data.len() - 1, self.data_range().0, padding);

        format!(
            "{} L {:.1} {:.1} L {:.1} {:.1} Z",
            line_path, max_x, max_y, min_x, max_y
        )
    }

    pub fn render(self) -> Markup {
        let padding = self.stroke_width + 1;
        let color = self.color.css_value();
        let path_data = self.generate_path(padding);

        let data_len = self.data.len();
        let aria_label = match self.aria_label {
            Some(ref s) => s.clone(),
            None => format!("Sparkline showing {} data points", data_len),
        };

        let mut classes = vec!["sh-sparkline".to_string()];
        if self.animate {
            classes.push("sh-sparkline--animated".to_string());
        }
        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }
        let class = classes.join(" ");

        let animation_style = if self.animate {
            format!(
                "--sparkline-animation-duration:{}ms;",
                self.animation_duration
            )
        } else {
            String::new()
        };

        html! {
            svg
                class=(class)
                style=(animation_style)
                width=(self.width)
                height=(self.height)
                viewBox=(format!("0 0 {} {}", self.width, self.height))
                role="img"
                aria-label=(aria_label)
            {
                @if self.fill && !self.data.is_empty() {
                    @let fill_path = self.generate_fill_path(padding);
                    path
                        class="sh-sparkline-fill"
                        d=(fill_path)
                        fill=(color)
                        fill-opacity=(self.fill_opacity)
                        stroke="none"
                    {}
                }

                @if !path_data.is_empty() {
                    path
                        class="sh-sparkline-line"
                        d=(path_data)
                        fill="none"
                        stroke=(color)
                        stroke-width=(self.stroke_width)
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    {}
                }

                @if self.show_dots {
                    @for (i, &value) in self.data.iter().enumerate() {
                        @let (x, y) = self.point_to_coords(i, value, padding);
                        circle
                            class="sh-sparkline-dot"
                            cx=(x)
                            cy=(y)
                            r=(self.dot_radius)
                            fill=(color)
                        {}
                    }
                }
            }
        }
    }
}

/// Bar chart component
#[derive(Debug, Clone)]
pub struct BarChart {
    data: Vec<f64>,
    labels: Option<Vec<String>>,
    width: u16,
    height: u16,
    color: ChartColor,
    bar_width: Option<u8>, // None = auto
    bar_gap: u8,
    max_value: Option<f64>,
    show_values: bool,
    animate: bool,
    animation_duration: u16,
    aria_label: Option<String>,
}

impl BarChart {
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            labels: None,
            width: 200,
            height: 100,
            color: ChartColor::Primary,
            bar_width: None,
            bar_gap: 4,
            max_value: None,
            show_values: false,
            animate: true,
            animation_duration: 800,
            aria_label: None,
        }
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = color;
        self
    }

    pub fn max_value(mut self, value: f64) -> Self {
        self.max_value = Some(value);
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn render(self) -> Markup {
        let max = self
            .max_value
            .unwrap_or_else(|| self.data.iter().fold(0.0f64, |a, &b| a.max(b)).max(1.0));

        let bar_count = self.data.len().max(1);
        let available_width = self.width as f64 - (bar_count as f64 + 1.0) * self.bar_gap as f64;
        let bar_width = self
            .bar_width
            .unwrap_or_else(|| (available_width / bar_count as f64).max(4.0) as u8);

        let color = self.color.css_value();
        let aria_label = self
            .aria_label
            .unwrap_or_else(|| format!("Bar chart with {} bars", self.data.len()));

        let animation_style = if self.animate {
            format!(
                "--barchart-animation-duration:{}ms;",
                self.animation_duration
            )
        } else {
            String::new()
        };

        html! {
            svg
                class="sh-barchart"
                style=(animation_style)
                width=(self.width)
                height=(self.height)
                viewBox=(format!("0 0 {} {}", self.width, self.height))
                role="img"
                aria-label=(aria_label)
            {
                @for (i, &value) in self.data.iter().enumerate() {
                    @let bar_height = (value / max) * (self.height as f64 - 20.0);
                    @let x = self.bar_gap as f64 + i as f64 * (bar_width as f64 + self.bar_gap as f64);
                    @let y = self.height as f64 - bar_height - 20.0;

                    rect
                        class="sh-barchart-bar"
                        x=(x)
                        y=(y)
                        width=(bar_width)
                        height=(bar_height.max(0.0))
                        fill=(color)
                    {}

                    @if self.show_values {
                        text
                            x=(x + bar_width as f64 / 2.0)
                            y=(y - 5.0)
                            text-anchor="middle"
                            font-size="10"
                            fill="currentColor"
                        {
                            (format!("{:.0}", value))
                        }
                    }
                }
            }
        }
    }
}

/// Gauge chart - circular progress indicator
#[derive(Debug, Clone)]
pub struct Gauge {
    value: f64, // 0.0 to 100.0
    min: f64,
    max: f64,
    size: u16,
    stroke_width: u8,
    color: ChartColor,
    background_color: String,
    show_value: bool,
    animate: bool,
    animation_duration: u16,
}

impl Gauge {
    pub fn new(value: f64) -> Self {
        Self {
            value: value.clamp(0.0, 100.0),
            min: 0.0,
            max: 100.0,
            size: 100,
            stroke_width: 8,
            color: ChartColor::Primary,
            background_color: "var(--sh-surface-alt)".to_string(),
            show_value: true,
            animate: true,
            animation_duration: 1000,
        }
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = color;
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    pub fn render(self) -> Markup {
        let normalized_value = ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        let percentage = normalized_value * 100.0;

        let radius = (self.size as f64 - self.stroke_width as f64) / 2.0;
        let center = self.size as f64 / 2.0;
        let circumference = 2.0 * std::f64::consts::PI * radius;
        let stroke_dashoffset = circumference * (1.0 - normalized_value);

        let color = self.color.css_value();

        let animation_style = if self.animate {
            format!(
                "--gauge-circumference:{};--gauge-offset:{};--gauge-animation-duration:{}ms;",
                circumference, stroke_dashoffset, self.animation_duration
            )
        } else {
            String::new()
        };

        html! {
            div class="sh-gauge" style=(format!("width:{}px;height:{}px;", self.size, self.size)) {
                svg
                    class=(if self.animate { "sh-gauge--animated" } else { "" })
                    style=(animation_style)
                    width=(self.size)
                    height=(self.size)
                    viewBox=(format!("0 0 {} {}", self.size, self.size))
                    role="img"
                    aria-label=(format!("Gauge showing {:.0}%", percentage))
                {
                    // Background circle
                    circle
                        class="sh-gauge-bg"
                        cx=(center)
                        cy=(center)
                        r=(radius)
                        fill="none"
                        stroke=(self.background_color)
                        stroke-width=(self.stroke_width)
                    {}

                    // Progress circle
                    circle
                        class="sh-gauge-progress"
                        cx=(center)
                        cy=(center)
                        r=(radius)
                        fill="none"
                        stroke=(color)
                        stroke-width=(self.stroke_width)
                        stroke-linecap="round"
                        stroke-dasharray=(circumference)
                        stroke-dashoffset=(if self.animate { circumference } else { stroke_dashoffset })
                        transform=(format!("rotate(-90 {} {})", center, center))
                    {}
                }

                @if self.show_value {
                    div class="sh-gauge-value" {
                        (format!("{:.0}%", percentage))
                    }
                }
            }
        }
    }
}

/// Generate CSS for chart components
pub fn charts_css() -> String {
    r#"
/* Charts & Data Visualization Styles */

/* Sparkline */
.sh-sparkline {
    display: block;
}

.sh-sparkline-line {
    vector-effect: non-scaling-stroke;
}

.sh-sparkline--animated .sh-sparkline-line {
    stroke-dasharray: var(--path-length, 1000);
    stroke-dashoffset: var(--path-length, 1000);
    animation: sparkline-draw var(--sparkline-animation-duration, 1000ms) ease-out forwards;
}

.sh-sparkline--animated .sh-sparkline-fill {
    opacity: 0;
    animation: sparkline-fade-in 300ms ease-out var(--sparkline-animation-duration, 1000ms) forwards;
}

@keyframes sparkline-draw {
    to {
        stroke-dashoffset: 0;
    }
}

@keyframes sparkline-fade-in {
    to {
        opacity: 1;
    }
}

/* Bar Chart */
.sh-barchart {
    display: block;
}

.sh-barchart-bar {
    transform-origin: bottom;
    animation: barchart-grow var(--barchart-animation-duration, 800ms) ease-out forwards;
}

@keyframes barchart-grow {
    from {
        transform: scaleY(0);
    }
    to {
        transform: scaleY(1);
    }
}

/* Gauge */
.sh-gauge {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.sh-gauge svg {
    transform: rotate(-90deg);
}

.sh-gauge--animated .sh-gauge-progress {
    animation: gauge-fill var(--gauge-animation-duration, 1000ms) ease-out forwards;
}

@keyframes gauge-fill {
    to {
        stroke-dashoffset: var(--gauge-offset);
    }
}

.sh-gauge-value {
    position: absolute;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--sh-text);
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
    .sh-sparkline--animated .sh-sparkline-line,
    .sh-sparkline--animated .sh-sparkline-fill,
    .sh-barchart-bar,
    .sh-gauge--animated .sh-gauge-progress {
        animation: none;
    }
    
    .sh-sparkline--animated .sh-sparkline-fill {
        opacity: 1;
    }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_basic() {
        let sparkline = Sparkline::new(vec![10.0, 20.0, 15.0, 30.0])
            .width(200)
            .height(50)
            .color(ChartColor::Success);

        assert_eq!(sparkline.width, 200);
        assert_eq!(sparkline.height, 50);
        assert_eq!(sparkline.data_range(), (10.0, 30.0));
    }

    #[test]
    fn test_sparkline_empty_data() {
        let sparkline = Sparkline::new(vec![]);
        assert_eq!(sparkline.data_range(), (0.0, 1.0));
    }

    #[test]
    fn test_sparkline_single_value() {
        let sparkline = Sparkline::new(vec![42.0]);
        let (min, max) = sparkline.data_range();
        assert!(max > min); // Should have padding
    }

    #[test]
    fn test_chart_colors() {
        assert_eq!(ChartColor::Primary.css_value(), "var(--sh-primary)");
        assert_eq!(ChartColor::Custom("#ff0000").css_value(), "#ff0000");
    }

    #[test]
    fn test_bar_chart() {
        let chart = BarChart::new(vec![10.0, 20.0, 30.0])
            .width(300)
            .height(150)
            .show_values(true);

        assert_eq!(chart.width, 300);
        assert!(chart.show_values);
    }

    #[test]
    fn test_gauge() {
        let gauge = Gauge::new(75.0).size(120).color(ChartColor::Accent);

        assert_eq!(gauge.value, 75.0);
        assert_eq!(gauge.size, 120);
    }

    #[test]
    fn test_gauge_clamping() {
        let gauge = Gauge::new(150.0); // Over 100
        assert_eq!(gauge.value, 100.0);

        let gauge = Gauge::new(-10.0); // Under 0
        assert_eq!(gauge.value, 0.0);
    }
}
