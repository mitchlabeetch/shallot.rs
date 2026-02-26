#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub mode: ColorMode,
    pub accent_h: f32,
    pub accent_s: f32,
    pub accent_l: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mode: ColorMode::Light,
            accent_h: 312.0,
            accent_s: 35.0,
            accent_l: 33.0,
        }
    }
}

impl Theme {
    pub fn css(&self) -> String {
        let (bg, surface, surface_2, text, text_muted, border) = match self.mode {
            ColorMode::Light => (
                "hsl(240 20% 98%)",
                "hsl(0 0% 100%)",
                "hsl(240 14% 96%)",
                "hsl(240 10% 12%)",
                "hsl(240 6% 40%)",
                "hsl(240 10% 88%)",
            ),
            ColorMode::Dark => (
                "hsl(240 10% 8%)",
                "hsl(240 10% 12%)",
                "hsl(240 10% 16%)",
                "hsl(0 0% 98%)",
                "hsl(240 6% 70%)",
                "hsl(240 10% 22%)",
            ),
        };

        let accent = format!(
            "hsl({} {}% {}%)",
            self.accent_h, self.accent_s, self.accent_l
        );
        let accent_2 = format!(
            "hsl({} {}% {}%)",
            self.accent_h,
            self.accent_s.max(18.0),
            (self.accent_l + if self.mode == ColorMode::Light { 18.0 } else { -10.0 })
                .clamp(10.0, 90.0)
        );

        format!(
            ":root {{\
  --sh-font-sans: 'Outfit', ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, 'Apple Color Emoji', 'Segoe UI Emoji';\
  --sh-radius-sm: 10px;\
  --sh-radius-md: 14px;\
  --sh-radius-lg: 18px;\
  --sh-radius-xl: 24px;\
  --sh-shadow-sm: 0 6px 18px hsl(240 30% 10% / 0.08);\
  --sh-shadow-md: 0 16px 48px hsl(240 30% 10% / 0.10);\
  --sh-shadow-xl: 0 28px 80px hsl(240 30% 10% / 0.18);\
  --sh-shadow-glow: 0 0 0 4px color-mix(in srgb, var(--sh-accent) 22%, transparent), 0 18px 54px color-mix(in srgb, var(--sh-accent) 18%, transparent);\
  --sh-dur-fast: 120ms;\
  --sh-dur-med: 200ms;\
  --sh-dur-slow: 360ms;\
  --sh-ease-out: cubic-bezier(0.16, 1, 0.3, 1);\
  --sh-ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);\
  --sh-bg: {bg};\
  --sh-surface: {surface};\
  --sh-surface-2: {surface_2};\
  --sh-border: {border};\
  --sh-text: {text};\
  --sh-text-muted: {text_muted};\
  --sh-accent: {accent};\
  --sh-accent-2: {accent_2};\
  --sh-page-gradient: radial-gradient(900px circle at 15% 10%, color-mix(in srgb, var(--sh-accent) 14%, transparent), transparent 60%), radial-gradient(900px circle at 90% 20%, color-mix(in srgb, var(--sh-accent-2) 12%, transparent), transparent 55%), var(--sh-bg);\
  --sh-success: hsl(145 63% 38%);\
  --sh-warning: hsl(38 92% 52%);\
  --sh-error: hsl(352 78% 54%);\
  --color-primary: var(--sh-accent);\
  --color-secondary: var(--sh-accent-2);\
  --color-background: var(--sh-bg);\
  --color-surface: var(--sh-surface);\
  --color-surface-2: var(--sh-surface-2);\
  --color-border: var(--sh-border);\
  --color-text: var(--sh-text);\
  --color-text-muted: var(--sh-text-muted);\
  --color-primary-content: white;\
  --shadow-sm: var(--sh-shadow-sm);\
  --shadow-md: var(--sh-shadow-md);\
  --shadow-xl: var(--sh-shadow-xl);\
}}\
\
html, body {{ height: 100%; }}\
body {{\
  margin: 0;\
  font-family: var(--sh-font-sans);\
  background: var(--sh-page-gradient);\
  background-attachment: fixed;\
  color: var(--sh-text);\
  line-height: 1.45;\
}}\
\
* {{ box-sizing: border-box; }}\
::selection {{ background: color-mix(in srgb, var(--sh-accent) 35%, transparent); }}\
:focus-visible {{ outline: 3px solid color-mix(in srgb, var(--sh-accent) 30%, transparent); outline-offset: 3px; }}\
a {{ color: var(--sh-accent); text-decoration: none; }}\
a:hover {{ text-decoration: underline; }}\
",
            bg = bg,
            surface = surface,
            surface_2 = surface_2,
            text = text,
            text_muted = text_muted,
            border = border,
            accent = accent,
            accent_2 = accent_2,
        )
    }
}
