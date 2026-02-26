//! Avatar Component
//!
//! User avatar display with image, fallback initials, and status indicator.
//! Supports groups, sizes, and shapes.

use crate::component::{Component, ComponentColor, ComponentSize};
use maud::{html, Markup, Render};

/// Avatar component for user/profile display
#[derive(Clone)]
pub struct Avatar<'a> {
    /// Image source URL
    src: Option<&'a str>,
    /// Alt text for accessibility
    alt: &'a str,
    /// Fallback initials (if no image)
    initials: Option<&'a str>,
    /// Size variant
    size: ComponentSize,
    /// Shape variant
    shape: AvatarShape,
    /// Border style
    border: AvatarBorder,
    /// Status indicator
    status: Option<AvatarStatus>,
    /// Custom CSS class
    custom_class: Option<&'a str>,
}

/// Avatar shape variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
    Rounded,
}

/// Avatar border styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarBorder {
    #[default]
    None,
    Thin,
    Thick,
    Ring,
}

/// Avatar status indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarStatus {
    Online,
    Offline,
    Away,
    Busy,
}

impl AvatarStatus {
    fn color(&self) -> ComponentColor {
        match self {
            AvatarStatus::Online => ComponentColor::Success,
            AvatarStatus::Offline => ComponentColor::Neutral,
            AvatarStatus::Away => ComponentColor::Warning,
            AvatarStatus::Busy => ComponentColor::Error,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            AvatarStatus::Online => "Online",
            AvatarStatus::Offline => "Offline",
            AvatarStatus::Away => "Away",
            AvatarStatus::Busy => "Busy",
        }
    }
}

impl<'a> Avatar<'a> {
    /// Create a new avatar
    pub fn new(alt: &'a str) -> Self {
        Self {
            src: None,
            alt,
            initials: None,
            size: ComponentSize::Md,
            shape: AvatarShape::Circle,
            border: AvatarBorder::None,
            status: None,
            custom_class: None,
        }
    }

    /// Set image source
    pub fn src(mut self, src: &'a str) -> Self {
        self.src = Some(src);
        self
    }

    /// Set fallback initials
    pub fn initials(mut self, initials: &'a str) -> Self {
        self.initials = Some(initials);
        self
    }

    /// Set size
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    /// Set shape
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set border style
    pub fn border(mut self, border: AvatarBorder) -> Self {
        self.border = border;
        self
    }

    /// Set status indicator
    pub fn status(mut self, status: AvatarStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Add custom class
    pub fn custom_class(mut self, class: &'a str) -> Self {
        self.custom_class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-avatar".to_string()];

        classes.push(format!("sh-avatar--{}", self.size.class_suffix()));
        classes.push(format!(
            "sh-avatar--{}",
            match self.shape {
                AvatarShape::Circle => "circle",
                AvatarShape::Square => "square",
                AvatarShape::Rounded => "rounded",
            }
        ));
        classes.push(format!(
            "sh-avatar--border-{}",
            match self.border {
                AvatarBorder::None => "none",
                AvatarBorder::Thin => "thin",
                AvatarBorder::Thick => "thick",
                AvatarBorder::Ring => "ring",
            }
        ));

        if let Some(custom) = self.custom_class {
            classes.push(custom.to_string());
        }

        classes.join(" ")
    }

    fn get_initials(&self) -> String {
        self.initials.map(|s| s.to_string()).unwrap_or_else(|| {
            // Generate initials from alt text
            self.alt
                .split_whitespace()
                .take(2)
                .map(|w| w.chars().next().unwrap_or('?').to_uppercase().to_string())
                .collect::<String>()
        })
    }
}

impl<'a> Render for Avatar<'a> {
    fn render(&self) -> Markup {
        let class = self.build_classes();
        let initials = self.get_initials();

        html! {
            div class=(class) role="img" aria-label=(self.alt) {
                @if let Some(src) = self.src {
                    img
                        class="sh-avatar__image"
                        src=(src)
                        alt=(self.alt)
                    {};
                } @else {
                    span class="sh-avatar__fallback" { (initials) }
                }

                @if let Some(status) = self.status {
                    span
                        class=(format!("sh-avatar__status sh-avatar__status--{}", status.color().class_suffix()))
                        aria-label=(status.label())
                    {}
                }
            }
        }
    }
}

impl<'a> Component for Avatar<'a> {
    fn classes(&self) -> String {
        self.build_classes()
    }
}

/// Avatar group for displaying multiple avatars
pub struct AvatarGroup<'a> {
    avatars: Vec<Avatar<'a>>,
    max: Option<usize>,
    size: ComponentSize,
    stacked: bool,
}

impl<'a> AvatarGroup<'a> {
    pub fn new(avatars: Vec<Avatar<'a>>) -> Self {
        Self {
            avatars,
            max: None,
            size: ComponentSize::Md,
            stacked: true,
        }
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = stacked;
        self
    }
}

impl<'a> Render for AvatarGroup<'a> {
    fn render(&self) -> Markup {
        let class = format!(
            "sh-avatar-group {}",
            if self.stacked {
                "sh-avatar-group--stacked"
            } else {
                ""
            }
        );

        let visible_avatars: Vec<_> = match self.max {
            Some(max) if self.avatars.len() > max => self.avatars[..max].to_vec(),
            _ => self.avatars.clone(),
        };

        let remaining = self.max.map(|max| self.avatars.len().saturating_sub(max));

        html! {
            div class=(class) {
                @for avatar in &visible_avatars {
                    (avatar.clone().size(self.size).render())
                }
                @if let Some(count) = remaining {
                    @if count > 0 {
                        div class=(format!("sh-avatar sh-avatar--more sh-avatar--{}", self.size.class_suffix())) {
                            span { "+" (count) }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for avatar components
pub fn avatar_css() -> String {
    r#"
/* Avatar Base */
.sh-avatar {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  overflow: hidden;
  background: var(--sh-surface-2);
  font-weight: 500;
  color: var(--sh-text);
  user-select: none;
}

/* Size variants */
.sh-avatar--xs {
  width: 1.5rem;
  height: 1.5rem;
  font-size: 0.5rem;
}

.sh-avatar--sm {
  width: 2rem;
  height: 2rem;
  font-size: 0.625rem;
}

.sh-avatar--md {
  width: 2.5rem;
  height: 2.5rem;
  font-size: 0.875rem;
}

.sh-avatar--lg {
  width: 3rem;
  height: 3rem;
  font-size: 1rem;
}

.sh-avatar--xl {
  width: 4rem;
  height: 4rem;
  font-size: 1.25rem;
}

/* Shape variants */
.sh-avatar--circle {
  border-radius: 50%;
}

.sh-avatar--square {
  border-radius: var(--sh-radius-sm);
}

.sh-avatar--rounded {
  border-radius: var(--sh-radius-lg);
}

/* Border variants */
.sh-avatar--border-thin {
  border: 2px solid var(--sh-surface);
}

.sh-avatar--border-thick {
  border: 4px solid var(--sh-surface);
}

.sh-avatar--border-ring {
  box-shadow: 0 0 0 2px var(--sh-surface), 0 0 0 4px var(--sh-accent);
}

/* Image */
.sh-avatar__image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

/* Fallback initials */
.sh-avatar__fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, var(--sh-accent), var(--sh-accent-2));
  color: white;
}

/* Status indicator */
.sh-avatar__status {
  position: absolute;
  bottom: 0;
  right: 0;
  width: 25%;
  height: 25%;
  min-width: 8px;
  min-height: 8px;
  border-radius: 50%;
  border: 2px solid var(--sh-surface);
}

.sh-avatar__status--primary { background: var(--sh-accent); }
.sh-avatar__status--success { background: var(--sh-success); }
.sh-avatar__status--warning { background: var(--sh-warning); }
.sh-avatar__status--error { background: var(--sh-error); }
.sh-avatar__status--neutral { background: var(--sh-text-muted); }

/* Avatar Group */
.sh-avatar-group {
  display: flex;
  align-items: center;
}

.sh-avatar-group--stacked {
  .sh-avatar {
    margin-left: -0.5rem;
  }
  .sh-avatar:first-child {
    margin-left: 0;
  }
}

.sh-avatar--more {
  background: var(--sh-surface-2);
  border: 2px dashed var(--sh-border);
}

/* Placeholder/placeholder animation */
.sh-avatar--placeholder {
  animation: avatar-pulse 2s ease-in-out infinite;
}

@keyframes avatar-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

/* Ring animation for online status */
.sh-avatar__status--success {
  animation: avatar-status-pulse 2s ease-out infinite;
}

@keyframes avatar-status-pulse {
  0% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--sh-success) 40%, transparent); }
  70% { box-shadow: 0 0 0 6px transparent; }
  100% { box-shadow: 0 0 0 0 transparent; }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_creation() {
        let avatar = Avatar::new("John Doe")
            .src("/avatar.jpg")
            .size(ComponentSize::Lg)
            .shape(AvatarShape::Rounded)
            .status(AvatarStatus::Online);

        assert_eq!(avatar.alt, "John Doe");
        assert_eq!(avatar.src, Some("/avatar.jpg"));
        assert_eq!(avatar.size, ComponentSize::Lg);
    }

    #[test]
    fn test_avatar_initials() {
        let avatar = Avatar::new("John Doe");
        assert_eq!(avatar.get_initials(), "JD");

        let avatar2 = Avatar::new("Jane");
        assert_eq!(avatar2.get_initials(), "J");
    }

    #[test]
    fn test_avatar_status() {
        assert_eq!(AvatarStatus::Online.color(), ComponentColor::Success);
        assert_eq!(AvatarStatus::Busy.color(), ComponentColor::Error);
        assert_eq!(AvatarStatus::Away.color(), ComponentColor::Warning);
    }
}
