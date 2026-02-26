//! Pulse Ring Component - Pulsing ring animation
//! CSS-only animated pulse effect

use maud::{html, Markup, Render};

/// Pulse ring size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PulseRingSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Pulse ring color variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PulseRingVariant {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

/// Pulse ring component
#[derive(Debug, Clone)]
pub struct PulseRing<'a> {
    pub children: Option<Markup>,
    pub size: PulseRingSize,
    pub variant: PulseRingVariant,
    pub color: Option<&'a str>,
    pub duration: &'a str,
    pub rings: usize,
}

impl<'a> PulseRing<'a> {
    pub fn new() -> Self {
        Self {
            children: None,
            size: PulseRingSize::default(),
            variant: PulseRingVariant::default(),
            color: None,
            duration: "2s",
            rings: 3,
        }
    }

    pub fn children(mut self, children: Markup) -> Self {
        self.children = Some(children);
        self
    }

    pub fn size(mut self, size: PulseRingSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: PulseRingVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: &'a str) -> Self {
        self.color = Some(color);
        self
    }

    pub fn duration(mut self, duration: &'a str) -> Self {
        self.duration = duration;
        self
    }

    pub fn rings(mut self, rings: usize) -> Self {
        self.rings = rings.max(1);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-pulse-ring"];

        match self.size {
            PulseRingSize::Sm => classes.push("sh-pulse-ring--sm"),
            PulseRingSize::Md => classes.push("sh-pulse-ring--md"),
            PulseRingSize::Lg => classes.push("sh-pulse-ring--lg"),
            PulseRingSize::Xl => classes.push("sh-pulse-ring--xl"),
        }

        match self.variant {
            PulseRingVariant::Primary => classes.push("sh-pulse-ring--primary"),
            PulseRingVariant::Secondary => classes.push("sh-pulse-ring--secondary"),
            PulseRingVariant::Success => classes.push("sh-pulse-ring--success"),
            PulseRingVariant::Warning => classes.push("sh-pulse-ring--warning"),
            PulseRingVariant::Danger => classes.push("sh-pulse-ring--danger"),
        }

        classes.join(" ")
    }
}

impl<'a> Default for PulseRing<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for PulseRing<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let color_style = self
            .color
            .map(|c| format!("--sh-pulse-color: {};", c))
            .unwrap_or_default();
        let duration_style = format!("--sh-pulse-duration: {};", self.duration);

        html! {
            div
                class=(classes)
                style=(format!("{}{}", color_style, duration_style))
                role="status"
                aria-label="Pulse animation"
            {
                @for i in 0..self.rings {
                    div class="sh-pulse-ring__ring" style=(format!("--sh-pulse-delay: {}s;", i as f32 * 0.3)) aria-hidden="true" {}
                }
                div class="sh-pulse-ring__content" {
                    @if let Some(ref children) = self.children {
                        (children)
                    }
                }
            }
        }
    }
}

pub fn pulse_ring_css() -> String {
    r#"
.sh-pulse-ring {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    --sh-pulse-color: var(--sh-color-primary, #3b82f6);
    --sh-pulse-duration: 2s;
}

.sh-pulse-ring--sm {
    width: 32px;
    height: 32px;
}

.sh-pulse-ring--md {
    width: 48px;
    height: 48px;
}

.sh-pulse-ring--lg {
    width: 64px;
    height: 64px;
}

.sh-pulse-ring--xl {
    width: 96px;
    height: 96px;
}

.sh-pulse-ring__ring {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    border: 2px solid var(--sh-pulse-color);
    animation: sh-pulse-expand var(--sh-pulse-duration) ease-out infinite;
    animation-delay: var(--sh-pulse-delay, 0s);
    opacity: 0;
}

.sh-pulse-ring--secondary .sh-pulse-ring__ring {
    --sh-pulse-color: var(--sh-color-secondary, #6b7280);
}

.sh-pulse-ring--success .sh-pulse-ring__ring {
    --sh-pulse-color: var(--sh-color-success, #22c55e);
}

.sh-pulse-ring--warning .sh-pulse-ring__ring {
    --sh-pulse-color: var(--sh-color-warning, #f59e0b);
}

.sh-pulse-ring--danger .sh-pulse-ring__ring {
    --sh-pulse-color: var(--sh-color-danger, #ef4444);
}

@keyframes sh-pulse-expand {
    0% {
        transform: scale(0.8);
        opacity: 0.8;
    }
    100% {
        transform: scale(2);
        opacity: 0;
    }
}

.sh-pulse-ring__content {
    position: relative;
    z-index: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 60%;
    height: 60%;
    background: var(--sh-pulse-color);
    border-radius: 50%;
    color: white;
}

.sh-pulse-ring__content:empty {
    background: transparent;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pulse_ring_creation() {
        let pulse = PulseRing::new()
            .size(PulseRingSize::Lg)
            .variant(PulseRingVariant::Success);

        assert_eq!(pulse.size, PulseRingSize::Lg);
        assert_eq!(pulse.variant, PulseRingVariant::Success);
    }

    #[test]
    fn test_pulse_ring_render() {
        let pulse = PulseRing::new();
        let rendered = pulse.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-pulse-ring"));
    }

    #[test]
    fn test_pulse_ring_with_children() {
        let pulse = PulseRing::new().children(html! { span { "!" } });

        let rendered = pulse.render();
        let html = rendered.into_string();

        assert!(html.contains("!"));
    }

    #[test]
    fn test_pulse_ring_rings() {
        let pulse = PulseRing::new().rings(5);
        assert_eq!(pulse.rings, 5);
    }

    #[test]
    fn test_pulse_ring_min_rings() {
        let pulse = PulseRing::new().rings(0);
        assert_eq!(pulse.rings, 1);
    }

    #[test]
    fn test_css_generation() {
        let css = pulse_ring_css();
        assert!(css.contains(".sh-pulse-ring"));
        assert!(css.contains("@keyframes"));
    }
}
