use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSLColor {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl HSLColor {
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self {
            h: h.clamp(0.0, 360.0),
            s: s.clamp(0.0, 100.0),
            l: l.clamp(0.0, 100.0),
        }
    }

    pub fn to_css(&self) -> String {
        format!("hsl({:.1} {:.1}% {:.1}%)", self.h, self.s, self.l)
    }

    pub fn lighten(&self, amount: f32) -> Self {
        Self::new(self.h, self.s, (self.l + amount).clamp(0.0, 100.0))
    }

    pub fn darken(&self, amount: f32) -> Self {
        Self::new(self.h, self.s, (self.l - amount).clamp(0.0, 100.0))
    }

    pub fn saturate(&self, amount: f32) -> Self {
        Self::new(self.h, (self.s + amount).clamp(0.0, 100.0), self.l)
    }

    pub fn desaturate(&self, amount: f32) -> Self {
        Self::new(self.h, (self.s - amount).clamp(0.0, 100.0), self.l)
    }

    pub fn rotate(&self, degrees: f32) -> Self {
        Self::new((self.h + degrees).rem_euclid(360.0), self.s, self.l)
    }

    pub fn complement(&self) -> Self {
        self.rotate(180.0)
    }

    pub fn analogous(&self, offset: f32) -> (Self, Self) {
        (self.rotate(-offset), self.rotate(offset))
    }

    pub fn triadic(&self) -> (Self, Self) {
        (self.rotate(120.0), self.rotate(240.0))
    }

    pub fn tetradic(&self) -> (Self, Self, Self) {
        (self.rotate(90.0), self.rotate(180.0), self.rotate(270.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorScheme {
    Monochromatic,
    Analogous,
    Complementary,
    Triadic,
    Tetradic,
    SplitComplementary,
}

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: HSLColor,
    pub secondary: HSLColor,
    pub accent: HSLColor,
    pub success: HSLColor,
    pub warning: HSLColor,
    pub error: HSLColor,
    pub info: HSLColor,
    pub scheme: ColorScheme,
}

impl ColorPalette {
    pub fn from_primary(primary: HSLColor, scheme: ColorScheme) -> Self {
        let (secondary, accent, success, warning, error, info) = match scheme {
            ColorScheme::Monochromatic => (
                primary.desaturate(20.0),
                primary.lighten(15.0),
                HSLColor::new(145.0, 63.0, 38.0),
                HSLColor::new(38.0, 92.0, 52.0),
                HSLColor::new(352.0, 78.0, 54.0),
                HSLColor::new(220.0, 75.0, 48.0),
            ),
            ColorScheme::Analogous => {
                let (sec, acc) = primary.analogous(30.0);
                (sec, acc,
                 HSLColor::new(145.0, 63.0, 38.0),
                 HSLColor::new(38.0, 92.0, 52.0),
                 HSLColor::new(352.0, 78.0, 54.0),
                 HSLColor::new(220.0, 75.0, 48.0))
            },
            ColorScheme::Complementary => {
                let comp = primary.complement();
                (comp, primary.lighten(20.0),
                 HSLColor::new(145.0, 63.0, 38.0),
                 HSLColor::new(38.0, 92.0, 52.0),
                 HSLColor::new(352.0, 78.0, 54.0),
                 HSLColor::new(220.0, 75.0, 48.0))
            },
            ColorScheme::Triadic => {
                let (sec, acc) = primary.triadic();
                (sec, acc,
                 HSLColor::new(145.0, 63.0, 38.0),
                 HSLColor::new(38.0, 92.0, 52.0),
                 HSLColor::new(352.0, 78.0, 54.0),
                 HSLColor::new(220.0, 75.0, 48.0))
            },
            ColorScheme::Tetradic => {
                let (sec, acc, _) = primary.tetradic();
                (sec, acc,
                 HSLColor::new(145.0, 63.0, 38.0),
                 HSLColor::new(38.0, 92.0, 52.0),
                 HSLColor::new(352.0, 78.0, 54.0),
                 HSLColor::new(220.0, 75.0, 48.0))
            },
            ColorScheme::SplitComplementary => {
                let (split1, split2) = primary.analogous(30.0);
                (split1, split2,
                 HSLColor::new(145.0, 63.0, 38.0),
                 HSLColor::new(38.0, 92.0, 52.0),
                 HSLColor::new(352.0, 78.0, 54.0),
                 HSLColor::new(220.0, 75.0, 48.0))
            },
        };

        Self {
            primary,
            secondary,
            accent,
            success,
            warning,
            error,
            info,
            scheme,
        }
    }

    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        // Primary colors
        vars.insert("--sh-color-primary".to_string(), self.primary.to_css());
        vars.insert("--sh-color-primary-light".to_string(), self.primary.lighten(10.0).to_css());
        vars.insert("--sh-color-primary-dark".to_string(), self.primary.darken(10.0).to_css());
        
        // Secondary colors
        vars.insert("--sh-color-secondary".to_string(), self.secondary.to_css());
        vars.insert("--sh-color-secondary-light".to_string(), self.secondary.lighten(10.0).to_css());
        vars.insert("--sh-color-secondary-dark".to_string(), self.secondary.darken(10.0).to_css());
        
        // Accent colors
        vars.insert("--sh-color-accent".to_string(), self.accent.to_css());
        vars.insert("--sh-color-accent-light".to_string(), self.accent.lighten(10.0).to_css());
        vars.insert("--sh-color-accent-dark".to_string(), self.accent.darken(10.0).to_css());
        
        // Semantic colors
        vars.insert("--sh-color-success".to_string(), self.success.to_css());
        vars.insert("--sh-color-warning".to_string(), self.warning.to_css());
        vars.insert("--sh-color-error".to_string(), self.error.to_css());
        vars.insert("--sh-color-info".to_string(), self.info.to_css());
        
        // Gradient variations
        vars.insert("--sh-gradient-primary".to_string(), format!(
            "linear-gradient(135deg, {}, {})",
            self.primary.to_css(),
            self.primary.lighten(15.0).to_css()
        ));
        
        vars.insert("--sh-gradient-secondary".to_string(), format!(
            "linear-gradient(135deg, {}, {})",
            self.secondary.to_css(),
            self.secondary.lighten(15.0).to_css()
        ));
        
        vars
    }
}

#[derive(Debug, Clone)]
pub struct TypographyScale {
    pub font_family_base: String,
    pub font_family_heading: String,
    pub font_family_mono: String,
    pub font_size_base: f32,
    pub line_height_base: f32,
    pub scale_ratio: f32,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self {
            font_family_base: "'Inter', system-ui, -apple-system, sans-serif".to_string(),
            font_family_heading: "'Outfit', system-ui, -apple-system, sans-serif".to_string(),
            font_family_mono: "'JetBrains Mono', 'Fira Code', monospace".to_string(),
            font_size_base: 16.0,
            line_height_base: 1.5,
            scale_ratio: 1.25,
        }
    }
}

impl TypographyScale {
    pub fn calculate_sizes(&self) -> HashMap<String, f32> {
        let mut sizes = HashMap::new();
        
        // Major scale (1.25 ratio)
        sizes.insert("xs".to_string(), self.font_size_base / (self.scale_ratio.powi(3)));
        sizes.insert("sm".to_string(), self.font_size_base / (self.scale_ratio.powi(2)));
        sizes.insert("base".to_string(), self.font_size_base);
        sizes.insert("lg".to_string(), self.font_size_base * self.scale_ratio);
        sizes.insert("xl".to_string(), self.font_size_base * (self.scale_ratio.powi(2)));
        sizes.insert("2xl".to_string(), self.font_size_base * (self.scale_ratio.powi(3)));
        sizes.insert("3xl".to_string(), self.font_size_base * (self.scale_ratio.powi(4)));
        sizes.insert("4xl".to_string(), self.font_size_base * (self.scale_ratio.powi(5)));
        sizes.insert("5xl".to_string(), self.font_size_base * (self.scale_ratio.powi(6)));
        
        sizes
    }

    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        let sizes = self.calculate_sizes();
        
        // Font families
        vars.insert("--sh-font-family-base".to_string(), self.font_family_base.clone());
        vars.insert("--sh-font-family-heading".to_string(), self.font_family_heading.clone());
        vars.insert("--sh-font-family-mono".to_string(), self.font_family_mono.clone());
        
        // Font sizes
        for (name, size) in sizes {
            vars.insert(format!("--sh-font-size-{}", name), format!("{}rem", size / 16.0));
        }
        
        // Line heights
        vars.insert("--sh-line-height-tight".to_string(), "1.25".to_string());
        vars.insert("--sh-line-height-normal".to_string(), self.line_height_base.to_string());
        vars.insert("--sh-line-height-relaxed".to_string(), "1.75".to_string());
        
        // Letter spacing
        vars.insert("--sh-letter-spacing-tight".to_string(), "-0.025em".to_string());
        vars.insert("--sh-letter-spacing-normal".to_string(), "0em".to_string());
        vars.insert("--sh-letter-spacing-wide".to_string(), "0.025em".to_string());
        
        vars
    }
}

#[derive(Debug, Clone)]
pub struct SpacingScale {
    pub base_unit: f32,
    pub scale_ratio: f32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            base_unit: 4.0,
            scale_ratio: 1.5,
        }
    }
}

impl SpacingScale {
    pub fn calculate_spacings(&self) -> HashMap<String, f32> {
        let mut spacings = HashMap::new();
        
        // Generate spacing scale
        for i in 0..12 {
            let value = self.base_unit * (self.scale_ratio.powi(i));
            spacings.insert(i.to_string(), value);
        }
        
        spacings
    }

    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        let spacings = self.calculate_spacings();
        
        for (name, value) in spacings {
            vars.insert(format!("--sh-spacing-{}", name), format!("{}px", value));
        }
        
        vars
    }
}

#[derive(Debug, Clone)]
pub struct BorderRadiusScale {
    pub base_radius: f32,
    pub scale_factor: f32,
}

impl Default for BorderRadiusScale {
    fn default() -> Self {
        Self {
            base_radius: 4.0,
            scale_factor: 1.414,
        }
    }
}

impl BorderRadiusScale {
    pub fn calculate_radii(&self) -> HashMap<String, f32> {
        let mut radii = HashMap::new();
        
        radii.insert("none".to_string(), 0.0);
        radii.insert("sm".to_string(), self.base_radius);
        radii.insert("base".to_string(), self.base_radius * self.scale_factor);
        radii.insert("md".to_string(), self.base_radius * (self.scale_factor.powi(2)));
        radii.insert("lg".to_string(), self.base_radius * (self.scale_factor.powi(3)));
        radii.insert("xl".to_string(), self.base_radius * (self.scale_factor.powi(4)));
        radii.insert("2xl".to_string(), self.base_radius * (self.scale_factor.powi(5)));
        radii.insert("3xl".to_string(), self.base_radius * (self.scale_factor.powi(6)));
        radii.insert("full".to_string(), 9999.0);
        
        radii
    }

    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        let radii = self.calculate_radii();
        
        for (name, value) in radii {
            if name == "full" {
                vars.insert(format!("--sh-radius-{}", name), "9999px".to_string());
            } else {
                vars.insert(format!("--sh-radius-{}", name), format!("{}px", value));
            }
        }
        
        vars
    }
}

#[derive(Debug, Clone)]
pub struct ShadowScale {
    pub base_blur: f32,
    pub base_spread: f32,
    pub intensity: f32,
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self {
            base_blur: 4.0,
            base_spread: 0.0,
            intensity: 0.1,
        }
    }
}

impl ShadowScale {
    pub fn calculate_shadows(&self) -> HashMap<String, String> {
        let mut shadows = HashMap::new();
        
        shadows.insert("sm".to_string(), format!(
            "0 1px 2px 0 rgba(0, 0, 0, {})",
            self.intensity
        ));
        
        shadows.insert("base".to_string(), format!(
            "0 1px 3px 0 rgba(0, 0, 0, {}), 0 1px 2px -1px rgba(0, 0, 0, {})",
            self.intensity * 1.5, self.intensity
        ));
        
        shadows.insert("md".to_string(), format!(
            "0 4px 6px -1px rgba(0, 0, 0, {}), 0 2px 4px -2px rgba(0, 0, 0, {})",
            self.intensity * 2.0, self.intensity * 1.5
        ));
        
        shadows.insert("lg".to_string(), format!(
            "0 10px 15px -3px rgba(0, 0, 0, {}), 0 4px 6px -4px rgba(0, 0, 0, {})",
            self.intensity * 2.5, self.intensity * 2.0
        ));
        
        shadows.insert("xl".to_string(), format!(
            "0 20px 25px -5px rgba(0, 0, 0, {}), 0 8px 10px -6px rgba(0, 0, 0, {})",
            self.intensity * 3.0, self.intensity * 2.5
        ));
        
        shadows.insert("2xl".to_string(), format!(
            "0 25px 50px -12px rgba(0, 0, 0, {})",
            self.intensity * 4.0
        ));
        
        shadows.insert("inner".to_string(), format!(
            "inset 0 2px 4px 0 rgba(0, 0, 0, {})",
            self.intensity * 2.0
        ));
        
        shadows.insert("none".to_string(), "none".to_string());
        
        shadows
    }

    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let shadows = self.calculate_shadows();
        
        shadows.into_iter()
            .map(|(k, v)| (format!("--sh-shadow-{}", k), v))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub color_palette: ColorPalette,
    pub typography: TypographyScale,
    pub spacing: SpacingScale,
    pub border_radius: BorderRadiusScale,
    pub shadows: ShadowScale,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self {
            color_palette: ColorPalette::from_primary(
                HSLColor::new(312.0, 35.0, 33.0),
                ColorScheme::Monochromatic,
            ),
            typography: TypographyScale::default(),
            spacing: SpacingScale::default(),
            border_radius: BorderRadiusScale::default(),
            shadows: ShadowScale::default(),
        }
    }
}

impl DesignTokens {
    pub fn new(primary_color: HSLColor, scheme: ColorScheme) -> Self {
        Self {
            color_palette: ColorPalette::from_primary(primary_color, scheme),
            typography: TypographyScale::default(),
            spacing: SpacingScale::default(),
            border_radius: BorderRadiusScale::default(),
            shadows: ShadowScale::default(),
        }
    }

    pub fn to_css_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        // Merge all component variables
        vars.extend(self.color_palette.to_css_variables());
        vars.extend(self.typography.to_css_variables());
        vars.extend(self.spacing.to_css_variables());
        vars.extend(self.border_radius.to_css_variables());
        vars.extend(self.shadows.to_css_variables());
        
        vars
    }

    pub fn to_css_string(&self) -> String {
        let vars = self.to_css_variables();
        let css_lines: Vec<String> = vars.iter()
            .map(|(key, value)| format!("  {}: {};", key, value))
            .collect();
        
        format!(":root {{\n{}\n}}", css_lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_color_creation() {
        let color = HSLColor::new(180.0, 50.0, 50.0);
        assert_eq!(color.h, 180.0);
        assert_eq!(color.s, 50.0);
        assert_eq!(color.l, 50.0);
    }

    #[test]
    fn test_hsl_color_clamping() {
        let color = HSLColor::new(400.0, 150.0, -20.0);
        assert_eq!(color.h, 360.0);
        assert_eq!(color.s, 100.0);
        assert_eq!(color.l, 0.0);
    }

    #[test]
    fn test_color_manipulations() {
        let color = HSLColor::new(180.0, 50.0, 50.0);
        let lighter = color.lighten(10.0);
        assert_eq!(lighter.l, 60.0);
        
        let darker = color.darken(10.0);
        assert_eq!(darker.l, 40.0);
        
        let more_saturated = color.saturate(10.0);
        assert_eq!(more_saturated.s, 60.0);
    }

    #[test]
    fn test_color_palette_generation() {
        let primary = HSLColor::new(200.0, 60.0, 50.0);
        let palette = ColorPalette::from_primary(primary, ColorScheme::Complementary);
        
        assert_eq!(palette.primary, primary);
        assert_eq!(palette.scheme, ColorScheme::Complementary);
        assert_ne!(palette.secondary, primary);
    }

    #[test]
    fn test_css_variable_generation() {
        let primary = HSLColor::new(200.0, 60.0, 50.0);
        let palette = ColorPalette::from_primary(primary, ColorScheme::Monochromatic);
        let vars = palette.to_css_variables();
        
        assert!(vars.contains_key("--sh-color-primary"));
        assert!(vars.contains_key("--sh-color-secondary"));
        assert!(vars.contains_key("--sh-gradient-primary"));
    }
}