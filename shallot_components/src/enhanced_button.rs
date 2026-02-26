use maud::{html, Markup, PreEscaped};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
    Success,
    Warning,
    Glass,
    Shiny,
    Shimmer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonShape {
    Default,
    Square,
    Circle,
    Pill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    None,
    Hover,
    Focus,
    Loading,
    Pulse,
    Bounce,
}

#[derive(Debug, Clone)]
pub struct AriaConfig {
    pub label: Option<String>,
    pub described_by: Option<String>,
    pub expanded: Option<bool>,
    pub pressed: Option<bool>,
    pub controls: Option<String>,
}

impl Default for AriaConfig {
    fn default() -> Self {
        Self {
            label: None,
            described_by: None,
            expanded: None,
            pressed: None,
            controls: None,
        }
    }
}

pub struct EnhancedButton<'a> {
    pub label: &'a str,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub shape: ButtonShape,
    pub animation: AnimationType,
    pub disabled: bool,
    pub loading: bool,
    pub href: Option<&'a str>,
    pub aria: AriaConfig,
    pub custom_class: Option<&'a str>,
    pub icon_left: Option<Icon>,
    pub icon_right: Option<Icon>,
}

#[derive(Debug, Clone)]
pub struct Icon {
    pub name: &'static str,
    pub size: u8,
}

impl<'a> EnhancedButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            shape: ButtonShape::Default,
            animation: AnimationType::Hover,
            disabled: false,
            loading: false,
            href: None,
            aria: AriaConfig::default(),
            custom_class: None,
            icon_left: None,
            icon_right: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn animation(mut self, animation: AnimationType) -> Self {
        self.animation = animation;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub fn aria(mut self, aria: AriaConfig) -> Self {
        self.aria = aria;
        self
    }

    pub fn custom_class(mut self, class: &'a str) -> Self {
        self.custom_class = Some(class);
        self
    }

    pub fn icon_left(mut self, icon: Icon) -> Self {
        self.icon_left = Some(icon);
        self
    }

    pub fn icon_right(mut self, icon: Icon) -> Self {
        self.icon_right = Some(icon);
        self
    }

    fn base_classes(&self) -> String {
        let mut classes = vec!["sh-btn"];

        // Variant classes
        match self.variant {
            ButtonVariant::Primary => classes.push("sh-btn--primary"),
            ButtonVariant::Secondary => classes.push("sh-btn--secondary"),
            ButtonVariant::Ghost => classes.push("sh-btn--ghost"),
            ButtonVariant::Danger => classes.push("sh-btn--danger"),
            ButtonVariant::Success => classes.push("sh-btn--success"),
            ButtonVariant::Warning => classes.push("sh-btn--warning"),
            ButtonVariant::Glass => classes.push("sh-btn--glass"),
            ButtonVariant::Shiny => classes.push("sh-btn--shiny"),
            ButtonVariant::Shimmer => classes.push("sh-btn--shimmer"),
        }

        // Size classes
        match self.size {
            ButtonSize::Xs => classes.push("sh-btn--xs"),
            ButtonSize::Sm => classes.push("sh-btn--sm"),
            ButtonSize::Md => classes.push("sh-btn--md"),
            ButtonSize::Lg => classes.push("sh-btn--lg"),
            ButtonSize::Xl => classes.push("sh-btn--xl"),
        }

        // Shape classes
        match self.shape {
            ButtonShape::Square => classes.push("sh-btn--square"),
            ButtonShape::Circle => classes.push("sh-btn--circle"),
            ButtonShape::Pill => classes.push("sh-btn--pill"),
            ButtonShape::Default => {}
        }

        // Animation classes
        match self.animation {
            AnimationType::Hover => classes.push("sh-btn--hover"),
            AnimationType::Focus => classes.push("sh-btn--focus"),
            AnimationType::Loading => classes.push("sh-btn--loading"),
            AnimationType::Pulse => classes.push("sh-btn--pulse"),
            AnimationType::Bounce => classes.push("sh-btn--bounce"),
            AnimationType::None => {}
        }

        // State classes
        if self.disabled {
            classes.push("sh-btn--disabled");
        }
        if self.loading {
            classes.push("sh-btn--loading");
        }

        // Custom classes
        if let Some(custom) = self.custom_class {
            classes.push(custom);
        }

        classes.join(" ")
    }

    fn render_icon(&self, icon: &Icon, position: &str) -> Markup {
        html! {
            span class=(format!("sh-btn__icon sh-btn__icon--{}", position)) {
                svg
                    class=(format!("sh-icon sh-icon--{}", icon.size))
                    width=(format!("{}", icon.size))
                    height=(format!("{}", icon.size))
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                {
                    (PreEscaped(self.get_icon_path(icon.name)))
                }
            }
        }
    }

    fn get_icon_path(&self, name: &str) -> &'static str {
        match name {
            "arrow-right" => "<line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\"></line><polyline points=\"12 5 19 12 12 19\"></polyline>",
            "loading" => "<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"6\"></line><line x1=\"12\" y1=\"18\" x2=\"12\" y2=\"22\"></line><line x1=\"4.93\" y1=\"4.93\" x2=\"7.76\" y2=\"7.76\"></line><line x1=\"16.24\" y1=\"16.24\" x2=\"19.07\" y2=\"19.07\"></line><line x1=\"2\" y1=\"12\" x2=\"6\" y2=\"12\"></line><line x1=\"18\" y1=\"12\" x2=\"22\" y2=\"12\"></line><line x1=\"4.93\" y1=\"19.07\" x2=\"7.76\" y2=\"16.24\"></line><line x1=\"16.24\" y1=\"7.76\" x2=\"19.07\" y2=\"4.93\"></line>",
            _ => "",
        }
    }

    #[allow(dead_code)] // Reserved for future ARIA rendering
    fn render_aria_attributes(&self) -> Markup {
        let mut attrs = vec![];

        if let Some(label) = &self.aria.label {
            attrs.push(format!("aria-label=\"{}\"", html_escape(label)));
        }

        if let Some(described_by) = &self.aria.described_by {
            attrs.push(format!("aria-describedby=\"{}\"", described_by));
        }

        if let Some(expanded) = self.aria.expanded {
            attrs.push(format!("aria-expanded=\"{}\"", expanded));
        }

        if let Some(pressed) = self.aria.pressed {
            attrs.push(format!("aria-pressed=\"{}\"", pressed));
        }

        if let Some(controls) = &self.aria.controls {
            attrs.push(format!("aria-controls=\"{}\"", controls));
        }

        if self.disabled {
            attrs.push("aria-disabled=\"true\"".to_string());
        }

        if self.loading {
            attrs.push("aria-busy=\"true\"".to_string());
        }

        PreEscaped(attrs.join(" "))
    }

    pub fn render(&self) -> Markup {
        let base_class = self.base_classes();

        // Pre-compute aria attribute values
        let aria_label = self.aria.label.as_ref();
        let aria_describedby = self.aria.described_by.as_ref();
        let aria_expanded = self.aria.expanded.map(|v| v.to_string());
        let aria_pressed = self.aria.pressed.map(|v| v.to_string());
        let aria_controls = self.aria.controls.as_ref();
        let aria_disabled = if self.disabled { Some("true") } else { None };
        let aria_busy = if self.loading { Some("true") } else { None };
        let disabled_style = if self.disabled {
            Some("pointer-events: none;")
        } else {
            None
        };

        let content = html! {
            @if let Some(icon) = &self.icon_left {
                (self.render_icon(icon, "left"))
            }

            span class="sh-btn__label" { (self.label) }

            @if self.loading {
                span class="sh-btn__spinner" aria-hidden="true" {
                    (self.render_icon(&Icon { name: "loading", size: 16 }, "spinner"))
                }
            }

            @if let Some(icon) = &self.icon_right {
                (self.render_icon(icon, "right"))
            }
        };

        if let Some(href) = self.href {
            html! {
                a
                    class=(base_class)
                    href=(if self.disabled { "#" } else { href })
                    role="button"
                    tabindex=(if self.disabled { "-1" } else { "0" })
                    style=[disabled_style]
                    aria-label=[aria_label]
                    aria-describedby=[aria_describedby]
                    aria-expanded=[aria_expanded.as_deref()]
                    aria-pressed=[aria_pressed.as_deref()]
                    aria-controls=[aria_controls]
                    aria-disabled=[aria_disabled]
                    aria-busy=[aria_busy]
                {
                    (content)
                }
            }
        } else {
            html! {
                button
                    type="button"
                    class=(base_class)
                    disabled[self.disabled]
                    aria-label=[aria_label]
                    aria-describedby=[aria_describedby]
                    aria-expanded=[aria_expanded.as_deref()]
                    aria-pressed=[aria_pressed.as_deref()]
                    aria-controls=[aria_controls]
                    aria-disabled=[aria_disabled]
                    aria-busy=[aria_busy]
                {
                    (content)
                }
            }
        }
    }
}

fn html_escape(s: &str) -> String {
    #[allow(dead_code)] // Used by render_aria_attributes (reserved)
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

// Advanced CSS animations and effects
pub fn enhanced_button_css() -> String {
    format!(
        r#"
/* Base button styles */
.sh-btn {{
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  font-family: var(--sh-font-sans);
  font-weight: 500;
  line-height: 1;
  text-decoration: none;
  cursor: pointer;
  transition: all var(--sh-dur-med) var(--sh-ease-out);
  border: none;
  outline: none;
  transform: translateZ(0);
  will-change: transform, box-shadow;
}}

/* Size variants */
.sh-btn--xs {{ padding: 0.375rem 0.75rem; font-size: 0.75rem; min-height: 1.75rem; }}
.sh-btn--sm {{ padding: 0.5rem 1rem; font-size: 0.875rem; min-height: 2rem; }}
.sh-btn--md {{ padding: 0.75rem 1.5rem; font-size: 1rem; min-height: 2.5rem; }}
.sh-btn--lg {{ padding: 1rem 2rem; font-size: 1.125rem; min-height: 3rem; }}
.sh-btn--xl {{ padding: 1.25rem 2.5rem; font-size: 1.25rem; min-height: 3.5rem; }}

/* Shape variants */
.sh-btn--square {{ border-radius: var(--sh-radius-sm); }}
.sh-btn--circle {{ border-radius: 50%; aspect-ratio: 1; }}
.sh-btn--pill {{ border-radius: 9999px; }}
.sh-btn:not(.sh-btn--square):not(.sh-btn--circle):not(.sh-btn--pill) {{
  border-radius: var(--sh-radius-md);
}}

/* Color variants with advanced effects */
.sh-btn--primary {{
  background: linear-gradient(135deg, var(--sh-accent), var(--sh-accent-2));
  color: white;
  box-shadow: var(--sh-shadow-sm), 0 0 0 1px rgba(255,255,255,0.1) inset;
}}

.sh-btn--primary:hover:not(.sh-btn--disabled) {{
  transform: translateY(-2px) scale(1.02);
  box-shadow: var(--sh-shadow-md), 0 0 0 1px rgba(255,255,255,0.15) inset;
}}

.sh-btn--primary:active:not(.sh-btn--disabled) {{
  transform: translateY(0) scale(0.98);
}}

.sh-btn--secondary {{
  background: var(--sh-surface);
  color: var(--sh-text);
  border: 1px solid var(--sh-border);
  box-shadow: var(--sh-shadow-sm);
}}

.sh-btn--secondary:hover:not(.sh-btn--disabled) {{
  background: var(--sh-surface-2);
  border-color: color-mix(in srgb, var(--sh-border) 80%, var(--sh-accent));
  transform: translateY(-1px);
}}

.sh-btn--ghost {{
  background: transparent;
  color: var(--sh-accent);
  border: 1px solid transparent;
}}

.sh-btn--ghost:hover:not(.sh-btn--disabled) {{
  background: color-mix(in srgb, var(--sh-accent) 10%, transparent);
  border-color: color-mix(in srgb, var(--sh-accent) 20%, transparent);
}}

.sh-btn--glass {{
  background: rgba(255,255,255,0.1);
  backdrop-filter: blur(10px);
  color: white;
  border: 1px solid rgba(255,255,255,0.2);
  box-shadow: 0 8px 32px rgba(0,0,0,0.1);
}}

.sh-btn--glass:hover:not(.sh-btn--disabled) {{
  background: rgba(255,255,255,0.15);
  border-color: rgba(255,255,255,0.3);
  transform: translateY(-2px);
}}

.sh-btn--shiny {{
  background: linear-gradient(135deg, var(--sh-accent), var(--sh-accent-2));
  color: white;
  position: relative;
  overflow: hidden;
}}

.sh-btn--shiny::before {{
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
  transition: left 0.6s ease;
}}

.sh-btn--shiny:hover::before {{
  left: 100%;
}}

.sh-btn--shimmer {{
  background: linear-gradient(135deg, var(--sh-accent), var(--sh-accent-2));
  color: white;
  position: relative;
  overflow: hidden;
}}

.sh-btn--shimmer::after {{
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(
    45deg,
    transparent 30%,
    rgba(255,255,255,0.1) 50%,
    transparent 70%
  );
  animation: shimmer 2s infinite;
}}

@keyframes shimmer {{
  0% {{ transform: translateX(-100%) translateY(-100%); }}
  100% {{ transform: translateX(100%) translateY(100%); }}
}}

/* Animation variants */
.sh-btn--hover {{
  transition: all var(--sh-dur-med) var(--sh-ease-out);
}}

.sh-btn--focus:focus-visible {{
  outline: 3px solid color-mix(in srgb, var(--sh-accent) 30%, transparent);
  outline-offset: 3px;
}}

.sh-btn--loading {{
  pointer-events: none;
  opacity: 0.8;
}}

.sh-btn--pulse {{
  animation: pulse 2s infinite;
}}

@keyframes pulse {{
  0%, 100% {{ opacity: 1; }}
  50% {{ opacity: 0.7; }}
}}

.sh-btn--bounce {{
  animation: bounce 1s infinite;
}}

@keyframes bounce {{
  0%, 20%, 53%, 80%, 100% {{ transform: translateY(0); }}
  40%, 43% {{ transform: translateY(-8px); }}
  70% {{ transform: translateY(-4px); }}
}}

/* Disabled state */
.sh-btn--disabled {{
  opacity: 0.5;
  cursor: not-allowed !important;
  transform: none !important;
  box-shadow: none !important;
}}

/* Icon styles */
.sh-btn__icon {{
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: transform var(--sh-dur-fast) var(--sh-ease-out);
}}

.sh-btn__icon--left {{
  margin-right: 0.5rem;
}}

.sh-btn__icon--right {{
  margin-left: 0.5rem;
}}

.sh-btn__icon--spinner {{
  animation: spin 1s linear infinite;
}}

@keyframes spin {{
  to {{ transform: rotate(360deg); }}
}}

/* Label styles */
.sh-btn__label {{
  position: relative;
  z-index: 1;
}}

/* Spinner styles */
.sh-btn__spinner {{
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-left: 0.5rem;
}}

/* Responsive design */
@media (max-width: 768px) {{
  .sh-btn {{
    min-height: 2.25rem;
  }}

  .sh-btn--lg,
  .sh-btn--xl {{
    min-height: 2.5rem;
    font-size: 1rem;
  }}
}}

/* High contrast mode support */
@media (prefers-contrast: high) {{
  .sh-btn {{
    border: 2px solid currentColor;
  }}

  .sh-btn:focus-visible {{
    outline: 4px solid CanvasText;
    outline-offset: 2px;
  }}
}}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {{
  .sh-btn {{
    transition: none;
  }}

  .sh-btn--shimmer::after,
  .sh-btn--pulse,
  .sh-btn--bounce {{
    animation: none;
  }}
}}
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_button_creation() {
        let button = EnhancedButton::new("Click me");
        assert_eq!(button.label, "Click me");
        assert_eq!(button.variant, ButtonVariant::Primary);
        assert_eq!(button.size, ButtonSize::Md);
    }

    #[test]
    fn test_enhanced_button_variant() {
        let button = EnhancedButton::new("Test").variant(ButtonVariant::Danger);
        assert_eq!(button.variant, ButtonVariant::Danger);
    }

    #[test]
    fn test_enhanced_button_size() {
        let button = EnhancedButton::new("Test").size(ButtonSize::Lg);
        assert_eq!(button.size, ButtonSize::Lg);
    }

    #[test]
    fn test_enhanced_button_loading() {
        let button = EnhancedButton::new("Test").loading(true);
        assert!(button.loading);
    }

    #[test]
    fn test_enhanced_button_disabled() {
        let button = EnhancedButton::new("Test").disabled(true);
        assert!(button.disabled);
    }

    #[test]
    fn test_enhanced_button_css() {
        let css = enhanced_button_css();
        assert!(css.contains(".sh-btn"));
        assert!(css.contains(".sh-btn--primary"));
    }
}
