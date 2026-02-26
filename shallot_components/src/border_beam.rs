//! BorderBeam Component
//!
//! An animated border beam effect that creates a moving gradient around an element's border.

use maud::{html, Markup, Render};

/// BorderBeam variant for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderBeamVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Rainbow,
}

/// BorderBeam size options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderBeamSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// BorderBeam - Animated border beam effect
pub struct BorderBeam {
    pub duration_s: f32,
    pub variant: BorderBeamVariant,
    pub size: BorderBeamSize,
    pub border_width: u32,
    pub color_from: Option<&'static str>,
    pub color_to: Option<&'static str>,
    pub children: Option<Markup>,
}

impl Default for BorderBeam {
    fn default() -> Self {
        Self {
            duration_s: 2.5,
            variant: BorderBeamVariant::Default,
            size: BorderBeamSize::Md,
            border_width: 2,
            color_from: None,
            color_to: None,
            children: None,
        }
    }
}

impl BorderBeam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn duration(mut self, duration_s: f32) -> Self {
        self.duration_s = duration_s;
        self
    }

    pub fn variant(mut self, variant: BorderBeamVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: BorderBeamSize) -> Self {
        self.size = size;
        self
    }

    pub fn border_width(mut self, width: u32) -> Self {
        self.border_width = width;
        self
    }

    pub fn colors(mut self, from: &'static str, to: &'static str) -> Self {
        self.color_from = Some(from);
        self.color_to = Some(to);
        self
    }

    pub fn children(mut self, children: Markup) -> Self {
        self.children = Some(children);
        self
    }
}

impl Render for BorderBeam {
    fn render(&self) -> Markup {
        let size_class = match self.size {
            BorderBeamSize::Sm => "sh-border-beam--sm",
            BorderBeamSize::Md => "sh-border-beam--md",
            BorderBeamSize::Lg => "sh-border-beam--lg",
            BorderBeamSize::Xl => "sh-border-beam--xl",
        };

        let variant_class = match self.variant {
            BorderBeamVariant::Default => "sh-border-beam",
            BorderBeamVariant::Primary => "sh-border-beam sh-border-beam--primary",
            BorderBeamVariant::Secondary => "sh-border-beam sh-border-beam--secondary",
            BorderBeamVariant::Accent => "sh-border-beam sh-border-beam--accent",
            BorderBeamVariant::Rainbow => "sh-border-beam sh-border-beam--rainbow",
        };

        let (from, to) = match (&self.color_from, &self.color_to) {
            (Some(f), Some(t)) => (*f, *t),
            _ => match self.variant {
                BorderBeamVariant::Default => ("#ff80b5", "#9089fc"),
                BorderBeamVariant::Primary => ("#3b82f6", "#8b5cf6"),
                BorderBeamVariant::Secondary => ("#6b7280", "#9ca3af"),
                BorderBeamVariant::Accent => ("#10b981", "#06b6d4"),
                BorderBeamVariant::Rainbow => ("#ff0000", "#00ff00"),
            },
        };

        let style = format!(
            "--sh-beam-dur: {}s; --sh-beam-from: {}; --sh-beam-to: {}; --sh-beam-width: {}px;",
            self.duration_s.max(0.5),
            from,
            to,
            self.border_width
        );

        html! {
            div class=(format!("{} {}", variant_class, size_class)) style=(style) role="presentation" aria-hidden="true" {
                @if let Some(children) = &self.children {
                    div class="sh-border-beam__content" {
                        (children)
                    }
                }
                div class="sh-border-beam__beam" {}
            }
        }
    }
}

/// Generate border beam CSS
pub fn border_beam_css() -> String {
    r#"
.sh-border-beam {
    --sh-beam-dur: 2.5s;
    --sh-beam-from: #ff80b5;
    --sh-beam-to: #9089fc;
    --sh-beam-width: 2px;
    position: relative;
    overflow: hidden;
    border-radius: var(--sh-radius-md, 0.5rem);
}

.sh-border-beam--sm {
    border-radius: var(--sh-radius-sm, 0.25rem);
}

.sh-border-beam--md {
    border-radius: var(--sh-radius-md, 0.5rem);
}

.sh-border-beam--lg {
    border-radius: var(--sh-radius-lg, 0.75rem);
}

.sh-border-beam--xl {
    border-radius: var(--sh-radius-xl, 1rem);
}

.sh-border-beam__content {
    position: relative;
    z-index: 1;
}

.sh-border-beam__beam {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    border: var(--sh-beam-width) solid transparent;
    background: linear-gradient(var(--sh-surface, #ffffff), var(--sh-surface, #ffffff)) padding-box,
                conic-gradient(from var(--beam-angle, 0deg), var(--sh-beam-from), var(--sh-beam-to), var(--sh-beam-from)) border-box;
    animation: border-beam-spin var(--sh-beam-dur) linear infinite;
}

@keyframes border-beam-spin {
    from {
        --beam-angle: 0deg;
    }
    to {
        --beam-angle: 360deg;
    }
}

@supports not (background: paint(something)) {
    .sh-border-beam__beam {
        background: 
            linear-gradient(90deg, transparent, var(--sh-beam-from), transparent),
            linear-gradient(90deg, transparent, var(--sh-beam-to), transparent),
            linear-gradient(var(--sh-surface, #ffffff), var(--sh-surface, #ffffff)) padding-box,
            linear-gradient(var(--sh-surface, #ffffff), var(--sh-surface, #ffffff)) border-box;
        background-size: 50% 100%, 50% 100%, 100% 100%, 100% 100%;
        background-position: 0% 0, 100% 0, 0 0, 0 0;
        background-repeat: no-repeat;
        animation: border-beam-slide var(--sh-beam-dur) linear infinite;
    }
}

@keyframes border-beam-slide {
    0% {
        background-position: 200% 0, -100% 0, 0 0, 0 0;
    }
    100% {
        background-position: -100% 0, 200% 0, 0 0, 0 0;
    }
}

.sh-border-beam--primary {
    --sh-beam-from: #3b82f6;
    --sh-beam-to: #8b5cf6;
}

.sh-border-beam--secondary {
    --sh-beam-from: #6b7280;
    --sh-beam-to: #9ca3af;
}

.sh-border-beam--accent {
    --sh-beam-from: #10b981;
    --sh-beam-to: #06b6d4;
}

.sh-border-beam--rainbow {
    --sh-beam-from: #ff0000;
    --sh-beam-to: #00ff00;
}

.sh-border-beam--rainbow .sh-border-beam__beam {
    background: 
        linear-gradient(90deg, transparent, #ff0000, transparent),
        linear-gradient(90deg, transparent, #ffff00, transparent),
        linear-gradient(90deg, transparent, #00ff00, transparent),
        linear-gradient(90deg, transparent, #00ffff, transparent),
        linear-gradient(90deg, transparent, #0000ff, transparent),
        linear-gradient(90deg, transparent, #ff00ff, transparent),
        linear-gradient(var(--sh-surface, #ffffff), var(--sh-surface, #ffffff)) padding-box,
        linear-gradient(var(--sh-surface, #ffffff), var(--sh-surface, #ffffff)) border-box;
    background-size: 20% 100%, 20% 100%, 20% 100%, 20% 100%, 20% 100%, 20% 100%, 100% 100%, 100% 100%;
    background-repeat: no-repeat;
    animation: border-beam-rainbow var(--sh-beam-dur) linear infinite;
}

@keyframes border-beam-rainbow {
    0% {
        background-position: 0% 0, 20% 0, 40% 0, 60% 0, 80% 0, 100% 0, 0 0, 0 0;
    }
    100% {
        background-position: 100% 0, 120% 0, 140% 0, 160% 0, 180% 0, 200% 0, 0 0, 0 0;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_beam_default() {
        let beam = BorderBeam::new();
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("sh-border-beam"));
    }

    #[test]
    fn test_border_beam_variant_primary() {
        let beam = BorderBeam::new().variant(BorderBeamVariant::Primary);
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("sh-border-beam--primary"));
    }

    #[test]
    fn test_border_beam_variant_secondary() {
        let beam = BorderBeam::new().variant(BorderBeamVariant::Secondary);
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("sh-border-beam--secondary"));
    }

    #[test]
    fn test_border_beam_variant_accent() {
        let beam = BorderBeam::new().variant(BorderBeamVariant::Accent);
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("sh-border-beam--accent"));
    }

    #[test]
    fn test_border_beam_variant_rainbow() {
        let beam = BorderBeam::new().variant(BorderBeamVariant::Rainbow);
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("sh-border-beam--rainbow"));
    }

    #[test]
    fn test_border_beam_size_sm() {
        let beam = BorderBeam::new().size(BorderBeamSize::Sm);
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("sh-border-beam--sm"));
    }

    #[test]
    fn test_border_beam_size_lg() {
        let beam = BorderBeam::new().size(BorderBeamSize::Lg);
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("sh-border-beam--lg"));
    }

    #[test]
    fn test_border_beam_custom_duration() {
        let beam = BorderBeam::new().duration(5.0);
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("--sh-beam-dur: 5s"));
    }

    #[test]
    fn test_border_beam_custom_colors() {
        let beam = BorderBeam::new().colors("#ff0000", "#00ff00");
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("--sh-beam-from: #ff0000"));
        assert!(rendered.0.as_str().contains("--sh-beam-to: #00ff00"));
    }

    #[test]
    fn test_border_beam_a11y() {
        let beam = BorderBeam::new();
        let rendered = beam.render();
        assert!(rendered.0.as_str().contains("role=\"presentation\""));
        assert!(rendered.0.as_str().contains("aria-hidden=\"true\""));
    }

    #[test]
    fn test_border_beam_css_function() {
        let css = border_beam_css();
        assert!(css.contains(".sh-border-beam"));
        assert!(css.contains("@keyframes border-beam-spin"));
    }
}
