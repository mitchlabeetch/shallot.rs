use super::hsl::Hsl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Light,
    Dark,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Dark
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    Obsidian,
    Frost,
    Ember,
    Ocean,
    Forest,
    Midnight,
}

pub struct Theme {
    pub name: ThemeName,
    pub mode: ColorMode,
    pub primary: Hsl,
    pub accent: Hsl,
    pub success: Hsl,
    pub warning: Hsl,
    pub error: Hsl,
    pub background: Hsl,
    pub surface: Hsl,
    pub foreground: Hsl,
    pub border: Hsl,
    pub radius_sm: &'static str,
    pub radius_md: &'static str,
    pub radius_lg: &'static str,
    pub radius_full: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        Self::obsidian()
    }
}

impl Theme {
    pub fn obsidian() -> Self {
        Self {
            name: ThemeName::Obsidian,
            mode: ColorMode::Dark,
            primary: Hsl::new(263.0, 70.0, 55.0),
            accent: Hsl::new(326.0, 80.0, 60.0),
            success: Hsl::new(145.0, 65.0, 45.0),
            warning: Hsl::new(38.0, 90.0, 55.0),
            error: Hsl::new(352.0, 75.0, 55.0),
            background: Hsl::new(240.0, 15.0, 4.0),
            surface: Hsl::new(240.0, 10.0, 10.0),
            foreground: Hsl::new(0.0, 0.0, 98.0),
            border: Hsl::new(240.0, 10.0, 18.0),
            radius_sm: "4px",
            radius_md: "8px",
            radius_lg: "16px",
            radius_full: "9999px",
        }
    }

    pub fn frost() -> Self {
        Self {
            name: ThemeName::Frost,
            mode: ColorMode::Light,
            primary: Hsl::new(210.0, 100.0, 50.0),
            accent: Hsl::new(280.0, 80.0, 60.0),
            success: Hsl::new(145.0, 60.0, 40.0),
            warning: Hsl::new(38.0, 85.0, 50.0),
            error: Hsl::new(352.0, 70.0, 50.0),
            background: Hsl::new(200.0, 20.0, 98.0),
            surface: Hsl::new(200.0, 15.0, 100.0),
            foreground: Hsl::new(200.0, 10.0, 10.0),
            border: Hsl::new(200.0, 10.0, 85.0),
            radius_sm: "4px",
            radius_md: "8px",
            radius_lg: "12px",
            radius_full: "9999px",
        }
    }

    pub fn ember() -> Self {
        Self {
            name: ThemeName::Ember,
            mode: ColorMode::Dark,
            primary: Hsl::new(15.0, 85.0, 55.0),
            accent: Hsl::new(35.0, 90.0, 55.0),
            success: Hsl::new(145.0, 65.0, 45.0),
            warning: Hsl::new(45.0, 95.0, 50.0),
            error: Hsl::new(0.0, 80.0, 55.0),
            background: Hsl::new(20.0, 30.0, 5.0),
            surface: Hsl::new(20.0, 20.0, 10.0),
            foreground: Hsl::new(20.0, 10.0, 95.0),
            border: Hsl::new(20.0, 25.0, 20.0),
            radius_sm: "4px",
            radius_md: "8px",
            radius_lg: "12px",
            radius_full: "9999px",
        }
    }

    pub fn ocean() -> Self {
        Self {
            name: ThemeName::Ocean,
            mode: ColorMode::Dark,
            primary: Hsl::new(200.0, 90.0, 50.0),
            accent: Hsl::new(170.0, 85.0, 55.0),
            success: Hsl::new(160.0, 70.0, 45.0),
            warning: Hsl::new(45.0, 90.0, 55.0),
            error: Hsl::new(0.0, 75.0, 55.0),
            background: Hsl::new(210.0, 40.0, 5.0),
            surface: Hsl::new(210.0, 25.0, 12.0),
            foreground: Hsl::new(200.0, 10.0, 96.0),
            border: Hsl::new(210.0, 30.0, 20.0),
            radius_sm: "2px",
            radius_md: "6px",
            radius_lg: "12px",
            radius_full: "9999px",
        }
    }

    pub fn forest() -> Self {
        Self {
            name: ThemeName::Forest,
            mode: ColorMode::Dark,
            primary: Hsl::new(145.0, 60.0, 45.0),
            accent: Hsl::new(170.0, 70.0, 50.0),
            success: Hsl::new(145.0, 55.0, 40.0),
            warning: Hsl::new(45.0, 85.0, 50.0),
            error: Hsl::new(352.0, 70.0, 55.0),
            background: Hsl::new(140.0, 20.0, 6.0),
            surface: Hsl::new(140.0, 15.0, 12.0),
            foreground: Hsl::new(140.0, 5.0, 96.0),
            border: Hsl::new(140.0, 15.0, 20.0),
            radius_sm: "4px",
            radius_md: "8px",
            radius_lg: "16px",
            radius_full: "9999px",
        }
    }

    pub fn midnight() -> Self {
        Self {
            name: ThemeName::Midnight,
            mode: ColorMode::Dark,
            primary: Hsl::new(270.0, 60.0, 60.0),
            accent: Hsl::new(300.0, 70.0, 55.0),
            success: Hsl::new(160.0, 60.0, 45.0),
            warning: Hsl::new(50.0, 80.0, 50.0),
            error: Hsl::new(350.0, 75.0, 55.0),
            background: Hsl::new(240.0, 30.0, 3.0),
            surface: Hsl::new(240.0, 20.0, 8.0),
            foreground: Hsl::new(240.0, 5.0, 98.0),
            border: Hsl::new(240.0, 25.0, 15.0),
            radius_sm: "2px",
            radius_md: "4px",
            radius_lg: "8px",
            radius_full: "9999px",
        }
    }

    pub fn with_mode(mut self, mode: ColorMode) -> Self {
        self.mode = mode;
        if mode == ColorMode::Light {
            self.background = Hsl::new(200.0, 15.0, 98.0);
            self.surface = Hsl::new(200.0, 10.0, 100.0);
            self.foreground = Hsl::new(200.0, 5.0, 10.0);
            self.border = Hsl::new(200.0, 10.0, 88.0);
        }
        self
    }

    pub fn to_css(&self) -> String {
        let primary_glass = self.primary.to_glass();

        format!(
            r#"
:root {{
    --sh-primary: {primary};
    --sh-primary-glass: {primary_glass};
    --sh-primary-light: {primary_light};
    --sh-primary-dark: {primary_dark};
    --sh-accent: {accent};
    --sh-accent-glass: {accent_glass};
    --sh-success: {success};
    --sh-warning: {warning};
    --sh-error: {error};
    --sh-bg: {bg};
    --sh-surface: {surface};
    --sh-surface-glass: {surface_glass};
    --sh-fg: {fg};
    --sh-fg-muted: {fg_muted};
    --sh-border: {border};
    --sh-border-light: {border_light};
    --sh-radius-sm: {radius_sm};
    --sh-radius-md: {radius_md};
    --sh-radius-lg: {radius_lg};
    --sh-radius-full: {radius_full};

    --sh-font-sans: 'Inter', 'Public Sans', system-ui, -apple-system, sans-serif;
    --sh-font-mono: 'JetBrains Mono', 'Fira Code', monospace;

    --sh-ease-bionic: cubic-bezier(0.16, 1, 0.3, 1);
    --sh-ease-out: cubic-bezier(0, 0, 0.2, 1);
    --sh-ease-in: cubic-bezier(0.4, 0, 1, 1);
    --sh-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);

    --sh-transition-fast: 150ms var(--sh-ease-bionic);
    --sh-transition-normal: 250ms var(--sh-ease-bionic);
    --sh-transition-slow: 400ms var(--sh-ease-bionic);

    --sh-shadow-sm: 0 1px 2px rgba(0,0,0,0.05);
    --sh-shadow-md: 0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1);
    --sh-shadow-lg: 0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -4px rgba(0,0,0,0.1);
    --sh-shadow-xl: 0 20px 25px -5px rgba(0,0,0,0.1), 0 8px 10px -6px rgba(0,0,0,0.1);
    --sh-shadow-glow: 0 0 20px {primary};
}}

*, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}

html {{ font-size: 16px; -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; }}

body {{
    background: var(--sh-bg);
    color: var(--sh-fg);
    font-family: var(--sh-font-sans);
    line-height: 1.6;
    min-height: 100vh;
}}

a {{ color: var(--sh-primary); text-decoration: none; transition: color var(--sh-transition-fast); }}
a:hover {{ color: var(--sh-primary-light); }}

button {{ font-family: inherit; cursor: pointer; }}

input, textarea, select {{ font-family: inherit; }}

::selection {{ background: var(--sh-primary); color: white; }}

:focus-visible {{ outline: 2px solid var(--sh-primary); outline-offset: 2px; }}

@media (prefers-reduced-motion: reduce) {{
    *, *::before, *::after {{
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }}
}}

@media (max-width: 640px) {{
    html {{ font-size: 14px; }}
}}
"#,
            primary = self.primary.to_css(),
            primary_glass = primary_glass.to_css(),
            primary_light = self.primary.lighten(15.0).to_css(),
            primary_dark = self.primary.darken(15.0).to_css(),
            accent = self.accent.to_css(),
            accent_glass = self.accent.to_glass().to_css(),
            success = self.success.to_css(),
            warning = self.warning.to_css(),
            error = self.error.to_css(),
            bg = self.background.to_css(),
            surface = self.surface.to_css(),
            surface_glass = self.surface.to_surface().to_css(),
            fg = self.foreground.to_css(),
            fg_muted = self.foreground.darken(20.0).to_css(),
            border = self.border.to_css(),
            border_light = self.border.lighten(10.0).to_css(),
            radius_sm = self.radius_sm,
            radius_md = self.radius_md,
            radius_lg = self.radius_lg,
            radius_full = self.radius_full,
        )
    }
}
