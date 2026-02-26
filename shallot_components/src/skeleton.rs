//! Skeleton Loading Component
//!
//! Placeholder loading states for content that is loading.
//! Creates visual placeholders with shimmer animations.

use crate::component::Component;
use maud::{html, Markup, Render};

/// Skeleton loading placeholder
pub struct Skeleton {
    /// Width (CSS value)
    width: String,
    /// Height (CSS value)
    height: String,
    /// Border radius
    border_radius: String,
    /// Animation variant
    animation: SkeletonAnimation,
    /// Custom class
    custom_class: Option<String>,
}

/// Skeleton animation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonAnimation {
    #[default]
    Pulse,
    Shimmer,
    Wave,
    None,
}

impl Skeleton {
    /// Create a new skeleton placeholder
    pub fn new() -> Self {
        Self {
            width: "100%".to_string(),
            height: "1rem".to_string(),
            border_radius: "var(--sh-radius-md)".to_string(),
            animation: SkeletonAnimation::Pulse,
            custom_class: None,
        }
    }

    /// Set width
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = width.into();
        self
    }

    /// Set height
    pub fn height(mut self, height: impl Into<String>) -> Self {
        self.height = height.into();
        self
    }

    /// Set size (convenience for width + height)
    pub fn size(mut self, width: impl Into<String>, height: impl Into<String>) -> Self {
        self.width = width.into();
        self.height = height.into();
        self
    }

    /// Set border radius
    pub fn border_radius(mut self, radius: impl Into<String>) -> Self {
        self.border_radius = radius.into();
        self
    }

    /// Make it a circle (for avatars)
    pub fn circle(mut self, size: impl Into<String>) -> Self {
        let size = size.into();
        self.width = size.clone();
        self.height = size;
        self.border_radius = "50%".to_string();
        self
    }

    /// Set animation type
    pub fn animation(mut self, animation: SkeletonAnimation) -> Self {
        self.animation = animation;
        self
    }

    /// Add custom class
    pub fn custom_class(mut self, class: impl Into<String>) -> Self {
        self.custom_class = Some(class.into());
        self
    }

    fn build_style(&self) -> String {
        format!(
            "width: {}; height: {}; border-radius: {};",
            self.width, self.height, self.border_radius
        )
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-skeleton".to_string()];

        classes.push(format!(
            "sh-skeleton--{}",
            match self.animation {
                SkeletonAnimation::Pulse => "pulse",
                SkeletonAnimation::Shimmer => "shimmer",
                SkeletonAnimation::Wave => "wave",
                SkeletonAnimation::None => "static",
            }
        ));

        if let Some(custom) = &self.custom_class {
            classes.push(custom.clone());
        }

        classes.join(" ")
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Skeleton {
    fn render(&self) -> Markup {
        html! {
            div
                class=(self.build_classes())
                style=(self.build_style())
                aria-hidden="true"
            {}
        }
    }
}

impl Component for Skeleton {
    fn classes(&self) -> String {
        self.build_classes()
    }
}

/// Skeleton text paragraph (multiple lines)
pub struct SkeletonText {
    lines: u8,
    line_height: String,
    last_line_width: String,
    animation: SkeletonAnimation,
}

impl SkeletonText {
    pub fn new(lines: u8) -> Self {
        Self {
            lines,
            line_height: "1rem".to_string(),
            last_line_width: "60%".to_string(),
            animation: SkeletonAnimation::Pulse,
        }
    }

    pub fn line_height(mut self, height: impl Into<String>) -> Self {
        self.line_height = height.into();
        self
    }

    pub fn last_line_width(mut self, width: impl Into<String>) -> Self {
        self.last_line_width = width.into();
        self
    }

    pub fn animation(mut self, animation: SkeletonAnimation) -> Self {
        self.animation = animation;
        self
    }
}

impl Render for SkeletonText {
    fn render(&self) -> Markup {
        html! {
            div class="sh-skeleton-text" style=(format!("--skeleton-line-height: {}", self.line_height)) {
                @for i in 0..self.lines {
                    @let width = if i == self.lines - 1 { self.last_line_width.clone() } else { "100%".to_string() };
                    (Skeleton::new()
                        .width(width)
                        .height(&self.line_height)
                        .animation(self.animation)
                        .render())
                }
            }
        }
    }
}

/// Skeleton card preset
pub struct SkeletonCard {
    has_image: bool,
    image_height: String,
    title_width: String,
    lines: u8,
}

impl SkeletonCard {
    pub fn new() -> Self {
        Self {
            has_image: true,
            image_height: "200px".to_string(),
            title_width: "70%".to_string(),
            lines: 3,
        }
    }

    pub fn has_image(mut self, has: bool) -> Self {
        self.has_image = has;
        self
    }

    pub fn image_height(mut self, height: impl Into<String>) -> Self {
        self.image_height = height.into();
        self
    }

    pub fn title_width(mut self, width: impl Into<String>) -> Self {
        self.title_width = width.into();
        self
    }

    pub fn lines(mut self, lines: u8) -> Self {
        self.lines = lines;
        self
    }
}

impl Default for SkeletonCard {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SkeletonCard {
    fn render(&self) -> Markup {
        html! {
            div class="sh-skeleton-card" {
                @if self.has_image {
                    (Skeleton::new()
                        .width("100%")
                        .height(&self.image_height)
                        .border_radius("var(--sh-radius-lg) var(--sh-radius-lg) 0 0")
                        .render())
                }
                div class="sh-skeleton-card__content" {
                    (Skeleton::new()
                        .width(&self.title_width)
                        .height("1.5rem")
                        .render())
                    (SkeletonText::new(self.lines).render())
                }
            }
        }
    }
}

/// Skeleton avatar + text preset (for lists)
pub struct SkeletonAvatarText {
    avatar_size: String,
    lines: u8,
}

impl SkeletonAvatarText {
    pub fn new() -> Self {
        Self {
            avatar_size: "3rem".to_string(),
            lines: 2,
        }
    }

    pub fn avatar_size(mut self, size: impl Into<String>) -> Self {
        self.avatar_size = size.into();
        self
    }

    pub fn lines(mut self, lines: u8) -> Self {
        self.lines = lines;
        self
    }
}

impl Default for SkeletonAvatarText {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SkeletonAvatarText {
    fn render(&self) -> Markup {
        html! {
            div class="sh-skeleton-avatar-text" {
                (Skeleton::new().circle(&self.avatar_size).render())
                div class="sh-skeleton-avatar-text__lines" {
                    (SkeletonText::new(self.lines).render())
                }
            }
        }
    }
}

/// Generate CSS for skeleton components
pub fn skeleton_css() -> String {
    r#"
/* Skeleton Base */
.sh-skeleton {
  display: block;
  background: var(--sh-surface-2);
  position: relative;
  overflow: hidden;
}

/* Pulse animation */
.sh-skeleton--pulse {
  animation: skeleton-pulse 2s ease-in-out infinite;
}

@keyframes skeleton-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

/* Shimmer animation */
.sh-skeleton--shimmer::after {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(
    90deg,
    transparent 0%,
    color-mix(in srgb, var(--sh-surface) 50%, transparent) 50%,
    transparent 100%
  );
  animation: skeleton-shimmer 1.5s infinite;
}

@keyframes skeleton-shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}

/* Wave animation */
.sh-skeleton--wave {
  background: linear-gradient(
    90deg,
    var(--sh-surface-2) 0%,
    color-mix(in srgb, var(--sh-surface) 80%, var(--sh-surface-2)) 50%,
    var(--sh-surface-2) 100%
  );
  background-size: 200% 100%;
  animation: skeleton-wave 1.5s ease-in-out infinite;
}

@keyframes skeleton-wave {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

/* Static (no animation) */
.sh-skeleton--static {
  opacity: 0.7;
}

/* Skeleton Text Container */
.sh-skeleton-text {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

/* Skeleton Card */
.sh-skeleton-card {
  background: var(--sh-surface);
  border-radius: var(--sh-radius-lg);
  border: 1px solid var(--sh-border);
  overflow: hidden;
}

.sh-skeleton-card__content {
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

/* Skeleton Avatar + Text */
.sh-skeleton-avatar-text {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.sh-skeleton-avatar-text__lines {
  flex: 1;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .sh-skeleton {
    background: color-mix(in srgb, var(--sh-surface-2) 50%, var(--sh-surface));
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .sh-skeleton--pulse,
  .sh-skeleton--shimmer::after,
  .sh-skeleton--wave {
    animation: none;
  }
  
  .sh-skeleton--pulse {
    opacity: 0.7;
  }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_creation() {
        let skeleton = Skeleton::new()
            .width("200px")
            .height("100px")
            .border_radius("8px")
            .animation(SkeletonAnimation::Shimmer);

        assert_eq!(skeleton.width, "200px");
        assert_eq!(skeleton.height, "100px");
        assert!(matches!(skeleton.animation, SkeletonAnimation::Shimmer));
    }

    #[test]
    fn test_skeleton_circle() {
        let skeleton = Skeleton::new().circle("48px");
        assert_eq!(skeleton.width, "48px");
        assert_eq!(skeleton.height, "48px");
        assert_eq!(skeleton.border_radius, "50%");
    }

    #[test]
    fn test_skeleton_text() {
        let text = SkeletonText::new(4)
            .line_height("1.5rem")
            .last_line_width("40%");

        assert_eq!(text.lines, 4);
        assert_eq!(text.line_height, "1.5rem");
        assert_eq!(text.last_line_width, "40%");
    }

    #[test]
    fn test_skeleton_card() {
        let card = SkeletonCard::new()
            .has_image(true)
            .image_height("150px")
            .lines(5);

        assert!(card.has_image);
        assert_eq!(card.image_height, "150px");
        assert_eq!(card.lines, 5);
    }
}
