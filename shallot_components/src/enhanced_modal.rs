use maud::{html, Markup, PreEscaped};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalSize {
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalPosition {
    Center,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackdropStyle {
    Blur,
    Dark,
    Transparent,
    Gradient,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModalConfig {
    pub close_on_esc: bool,
    pub close_on_backdrop: bool,
    pub trap_focus: bool,
    pub restore_focus: bool,
    pub prevent_scroll: bool,
    pub animation_duration: u32,
}

impl Default for ModalConfig {
    fn default() -> Self {
        Self {
            close_on_esc: true,
            close_on_backdrop: true,
            trap_focus: true,
            restore_focus: true,
            prevent_scroll: true,
            animation_duration: 300,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AriaModal {
    pub labelled_by: Option<String>,
    pub described_by: Option<String>,
    pub role: String,
    pub modal: bool,
}

impl Default for AriaModal {
    fn default() -> Self {
        Self {
            labelled_by: None,
            described_by: None,
            role: "dialog".to_string(),
            modal: true,
        }
    }
}

pub struct EnhancedModal<'a> {
    pub id: &'a str,
    pub title: Option<&'a str>,
    pub children: Markup,
    pub footer: Option<Markup>,
    pub size: ModalSize,
    pub position: ModalPosition,
    pub backdrop: BackdropStyle,
    pub config: ModalConfig,
    pub aria: AriaModal,
    pub custom_class: Option<&'a str>,
    pub data_attributes: HashMap<&'a str, &'a str>,
}

impl<'a> EnhancedModal<'a> {
    pub fn new(id: &'a str, children: Markup) -> Self {
        Self {
            id,
            title: None,
            children,
            footer: None,
            size: ModalSize::Md,
            position: ModalPosition::Center,
            backdrop: BackdropStyle::Blur,
            config: ModalConfig::default(),
            aria: AriaModal::default(),
            custom_class: None,
            data_attributes: HashMap::new(),
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }

    pub fn position(mut self, position: ModalPosition) -> Self {
        self.position = position;
        self
    }

    pub fn backdrop(mut self, backdrop: BackdropStyle) -> Self {
        self.backdrop = backdrop;
        self
    }

    pub fn config(mut self, config: ModalConfig) -> Self {
        self.config = config;
        self
    }

    pub fn aria(mut self, aria: AriaModal) -> Self {
        self.aria = aria;
        self
    }

    pub fn custom_class(mut self, class: &'a str) -> Self {
        self.custom_class = Some(class);
        self
    }

    pub fn data_attr(mut self, key: &'a str, value: &'a str) -> Self {
        self.data_attributes.insert(key, value);
        self
    }

    fn modal_classes(&self) -> String {
        let mut classes = vec!["sh-modal"];

        match self.size {
            ModalSize::Sm => classes.push("sh-modal--sm"),
            ModalSize::Md => classes.push("sh-modal--md"),
            ModalSize::Lg => classes.push("sh-modal--lg"),
            ModalSize::Xl => classes.push("sh-modal--xl"),
            ModalSize::Full => classes.push("sh-modal--full"),
        }

        match self.position {
            ModalPosition::Center => classes.push("sh-modal--center"),
            ModalPosition::Top => classes.push("sh-modal--top"),
            ModalPosition::Bottom => classes.push("sh-modal--bottom"),
            ModalPosition::Left => classes.push("sh-modal--left"),
            ModalPosition::Right => classes.push("sh-modal--right"),
        }

        if let Some(custom) = self.custom_class {
            classes.push(custom);
        }

        classes.join(" ")
    }

    fn backdrop_classes(&self) -> String {
        let mut classes = vec!["sh-modal-backdrop"];

        match self.backdrop {
            BackdropStyle::Blur => classes.push("sh-modal-backdrop--blur"),
            BackdropStyle::Dark => classes.push("sh-modal-backdrop--dark"),
            BackdropStyle::Transparent => classes.push("sh-modal-backdrop--transparent"),
            BackdropStyle::Gradient => classes.push("sh-modal-backdrop--gradient"),
        }

        classes.join(" ")
    }

    fn render_data_attributes(&self) -> Markup {
        let mut html = String::new();
        for (key, value) in &self.data_attributes {
            html.push_str(&format!(" data-{}=\"{}\"", key, html_escape(value)));
        }
        PreEscaped(html)
    }

    fn render_aria_attributes(&self) -> Markup {
        let mut html = String::new();
        html.push_str(&format!(" role=\"{}\"", self.aria.role));

        if self.aria.modal {
            html.push_str(" aria-modal=\"true\"");
        }

        if let Some(_title) = self.title {
            html.push_str(&format!(" aria-labelledby=\"{}-title\"", self.id));
        }

        if let Some(described_by) = &self.aria.described_by {
            html.push_str(&format!(" aria-describedby=\"{}\"", described_by));
        }

        PreEscaped(html)
    }

    fn render_header(&self) -> Markup {
        let close_button = html! {
            button
                class="sh-modal__close"
                type="button"
                aria-label="Close modal"
                data-modal-close=(self.id)
            {
                span aria-hidden="true" { (PreEscaped("×".to_string())) }
            }
        };

        if let Some(title) = self.title {
            html! {
                div class="sh-modal__header" {
                    h2
                        id=(format!("{}-title", self.id))
                        class="sh-modal__title"
                    {
                        (PreEscaped(title.to_string()))
                    }
                    (close_button)
                }
            }
        } else {
            html! {
                div class="sh-modal__header sh-modal__header--minimal" {
                    (close_button)
                }
            }
        }
    }

    pub fn render(&self) -> Markup {
        let aria_attrs = self.render_aria_attributes();
        let data_attrs = self.render_data_attributes();

        html! {
            div
                id=(self.id)
                class=(self.modal_classes())
            {
                (aria_attrs)
                (data_attrs)
                div class=(self.backdrop_classes()) {
                    div class="sh-modal__container" {
                        div class="sh-modal__content" {
                            (self.render_header())
                            div class="sh-modal__body" {
                                (self.children)
                            }
                            @if let Some(footer) = &self.footer {
                                div class="sh-modal__footer" {
                                    (footer)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Modal group for managing multiple modals
pub struct ModalGroup<'a> {
    pub modals: Vec<EnhancedModal<'a>>,
    pub group_config: ModalConfig,
}

impl<'a> ModalGroup<'a> {
    pub fn new(modals: Vec<EnhancedModal<'a>>) -> Self {
        Self {
            modals,
            group_config: ModalConfig::default(),
        }
    }

    pub fn render(&self) -> Markup {
        html! {
            div class="sh-modal-group" {
                @for modal in &self.modals {
                    (modal.render())
                }
            }
        }
    }
}

// Advanced modal CSS with animations and accessibility
pub fn enhanced_modal_css() -> String {
    format!(
        r#"
/* Modal base styles */
.sh-modal {{
  position: fixed;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
  opacity: 0;
  visibility: hidden;
  transition: all var(--sh-dur-med) var(--sh-ease-out);
}}

.sh-modal[data-open="true"] {{
  opacity: 1;
  visibility: visible;
}}

/* Backdrop styles */
.sh-modal-backdrop {{
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}}

.sh-modal-backdrop--blur {{
  backdrop-filter: blur(8px);
  background: rgba(0, 0, 0, 0.5);
}}

.sh-modal-backdrop--dark {{
  background: rgba(0, 0, 0, 0.8);
}}

.sh-modal-backdrop--transparent {{
  background: transparent;
}}

.sh-modal-backdrop--gradient {{
  background: linear-gradient(
    135deg,
    rgba(0, 0, 0, 0.7) 0%,
    rgba(0, 0, 0, 0.3) 50%,
    rgba(0, 0, 0, 0.7) 100%
  );
}}

/* Container and content */
.sh-modal__container {{
  position: relative;
  width: 100%;
  max-height: calc(100vh - 2rem);
  transform: scale(0.95) translateY(20px);
  transition: transform var(--sh-dur-med) var(--sh-ease-out);
}}

.sh-modal[data-open="true"] .sh-modal__container {{
  transform: scale(1) translateY(0);
}}

.sh-modal__content {{
  background: var(--sh-surface);
  border: 1px solid var(--sh-border);
  border-radius: var(--sh-radius-lg);
  box-shadow: var(--sh-shadow-xl);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  max-height: calc(100vh - 2rem);
}}

/* Size variants */
.sh-modal--sm .sh-modal__content {{ max-width: 320px; }}
.sh-modal--md .sh-modal__content {{ max-width: 480px; }}
.sh-modal--lg .sh-modal__content {{ max-width: 640px; }}
.sh-modal--xl .sh-modal__content {{ max-width: 800px; }}
.sh-modal--full .sh-modal__content {{
  max-width: 95vw;
  max-height: 95vh;
  border-radius: var(--sh-radius-xl);
}}

/* Position variants */
.sh-modal--top {{
  align-items: flex-start;
  padding-top: 4rem;
}}

.sh-modal--bottom {{
  align-items: flex-end;
  padding-bottom: 4rem;
}}

.sh-modal--left {{
  justify-content: flex-start;
  padding-left: 4rem;
}}

.sh-modal--right {{
  justify-content: flex-end;
  padding-right: 4rem;
}}

/* Header */
.sh-modal__header {{
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.5rem;
  border-bottom: 1px solid var(--sh-border);
  background: var(--sh-surface-2);
}}

.sh-modal__header--minimal {{
  justify-content: flex-end;
  padding: 1rem;
}}

.sh-modal__title {{
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--sh-text);
}}

.sh-modal__close {{
  background: none;
  border: none;
  color: var(--sh-text-muted);
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: var(--sh-radius-sm);
  transition: all var(--sh-dur-fast) var(--sh-ease-out);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
}}

.sh-modal__close:hover {{
  background: var(--sh-surface);
  color: var(--sh-text);
}}

.sh-modal__close:focus-visible {{
  outline: 2px solid var(--sh-accent);
  outline-offset: 2px;
}}

/* Body */
.sh-modal__body {{
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
  max-height: calc(100vh - 12rem);
}}

.sh-modal--full .sh-modal__body {{
  max-height: calc(95vh - 8rem);
}}

/* Footer */
.sh-modal__footer {{
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--sh-border);
  background: var(--sh-surface-2);
}}

/* Focus management */
.sh-modal__content:focus-visible {{
  outline: 3px solid var(--sh-accent);
  outline-offset: -3px;
}}

/* Animation variants */
@keyframes modal-enter {{
  from {{
    opacity: 0;
    transform: scale(0.9) translateY(20px);
  }}
  to {{
    opacity: 1;
    transform: scale(1) translateY(0);
  }}
}}

@keyframes modal-exit {{
  from {{
    opacity: 1;
    transform: scale(1) translateY(0);
  }}
  to {{
    opacity: 0;
    transform: scale(0.9) translateY(20px);
  }}
}}

.sh-modal[data-animation="enter"] {{
  animation: modal-enter var(--sh-dur-med) var(--sh-ease-out) forwards;
}}

.sh-modal[data-animation="exit"] {{
  animation: modal-exit var(--sh-dur-med) var(--sh-ease-out) forwards;
}}

/* Responsive design */
@media (max-width: 768px) {{
  .sh-modal {{
    padding: 0.5rem;
  }}

  .sh-modal__content {{
    max-width: calc(100vw - 1rem);
    max-height: calc(100vh - 1rem);
    border-radius: var(--sh-radius-md);
  }}

  .sh-modal--full .sh-modal__content {{
    max-width: 100vw;
    max-height: 100vh;
    border-radius: 0;
  }}

  .sh-modal__header {{
    padding: 1rem;
  }}

  .sh-modal__body {{
    padding: 1rem;
    max-height: calc(100vh - 10rem);
  }}

  .sh-modal__footer {{
    padding: 0.75rem 1rem;
  }}
}}

/* High contrast mode */
@media (prefers-contrast: high) {{
  .sh-modal__content {{
    border: 2px solid CanvasText;
  }}

  .sh-modal__header {{
    border-bottom: 2px solid CanvasText;
  }}

  .sh-modal__footer {{
    border-top: 2px solid CanvasText;
  }}
}}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {{
  .sh-modal,
  .sh-modal__container {{
    transition: none;
  }}

  .sh-modal[data-animation="enter"],
  .sh-modal[data-animation="exit"] {{
    animation: none;
  }}
}}

/* Focus trap indicators */
.sh-modal[data-focus-trap="true"] .sh-modal__content {{
  position: relative;
}}

.sh-modal[data-focus-trap="true"]::before {{
  content: '';
  position: absolute;
  inset: 0;
  pointer-events: none;
  border: 2px solid transparent;
  border-radius: var(--sh-radius-lg);
  transition: border-color var(--sh-dur-med) var(--sh-ease-out);
}}

.sh-modal[data-focus-trap="true"]:focus-within::before {{
  border-color: var(--sh-accent);
}}
"#
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_modal_creation() {
        let modal = EnhancedModal::new("test-modal", html! { "Content" });
        assert_eq!(modal.id, "test-modal");
        assert_eq!(modal.size, ModalSize::Md);
    }

    #[test]
    fn test_enhanced_modal_size() {
        let modal = EnhancedModal::new("test", html! {}).size(ModalSize::Lg);
        assert_eq!(modal.size, ModalSize::Lg);
    }

    #[test]
    fn test_enhanced_modal_position() {
        let modal = EnhancedModal::new("test", html! {}).position(ModalPosition::Center);
        assert_eq!(modal.position, ModalPosition::Center);
    }

    #[test]
    fn test_enhanced_modal_backdrop() {
        let modal = EnhancedModal::new("test", html! {}).backdrop(BackdropStyle::Dark);
        assert_eq!(modal.backdrop, BackdropStyle::Dark);
    }

    #[test]
    fn test_enhanced_modal_css() {
        let css = enhanced_modal_css();
        assert!(css.contains(".sh-modal"));
        assert!(css.contains(".sh-modal__container"));
    }
}
