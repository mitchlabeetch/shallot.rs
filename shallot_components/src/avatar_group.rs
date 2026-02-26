//! Avatar Group Component
//!
//! Display multiple avatars in a stacked or grid layout.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct AvatarItem<'a> {
    pub src: &'a str,
    pub alt: &'a str,
    pub href: Option<&'a str>,
}

pub struct AvatarGroup<'a> {
    avatars: Vec<AvatarItem<'a>>,
    size: ComponentSize,
    max: usize,
    variant: AvatarGroupVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarGroupVariant {
    #[default]
    Stacked,
    Grid,
    Row,
}

impl<'a> AvatarGroup<'a> {
    pub fn new(avatars: Vec<AvatarItem<'a>>) -> Self {
        Self {
            avatars,
            size: ComponentSize::Md,
            max: 4,
            variant: AvatarGroupVariant::Stacked,
        }
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }

    pub fn variant(mut self, variant: AvatarGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    #[allow(dead_code)] // Reserved for inline size styling
    fn size_px(&self) -> u32 {
        match self.size {
            ComponentSize::Xs => 24,
            ComponentSize::Sm => 32,
            ComponentSize::Md => 40,
            ComponentSize::Lg => 48,
            ComponentSize::Xl => 56,
        }
    }

    fn variant_class(&self) -> &'static str {
        match self.variant {
            AvatarGroupVariant::Stacked => "sh-avatar-group--stacked",
            AvatarGroupVariant::Grid => "sh-avatar-group--grid",
            AvatarGroupVariant::Row => "sh-avatar-group--row",
        }
    }
}

impl<'a> Render for AvatarGroup<'a> {
    fn render(&self) -> Markup {
        let size_class = format!("sh-avatar-group--{}", self.size.class_suffix());
        let visible_count = self.avatars.len().min(self.max);
        let remaining = if self.avatars.len() > self.max {
            self.avatars.len() - self.max
        } else {
            0
        };

        html! {
            div
                class={(format!("sh-avatar-group {} {}", self.variant_class(), size_class))}
                role="group"
                aria-label="Avatar group"
                aria-description="A group of user avatars"
            {
                @for (i, avatar) in self.avatars.iter().take(visible_count).enumerate() {
                    @let z_index = visible_count - i;
                    @let style = if self.variant == AvatarGroupVariant::Stacked && i > 0 {
                        format!("z-index: {}; margin-left: -8px", z_index)
                    } else {
                        format!("z-index: {}", z_index)
                    };

                    div class="sh-avatar-group__item" style=(style) {
                        @if let Some(href) = avatar.href {
                            a href=(href) class="sh-avatar-group__link" aria-label=(avatar.alt) {
                                img src=(avatar.src) alt=(avatar.alt) class="sh-avatar-group__img";
                            }
                        } @else {
                            img src=(avatar.src) alt=(avatar.alt) class="sh-avatar-group__img";
                        }
                    }
                }

                @if remaining > 0 {
                    div class="sh-avatar-group__item" {
                        span class="sh-avatar-group__overflow" aria-label=(format!("{} more avatars", remaining)) {
                            "+" (remaining)
                        }
                    }
                }
            }
        }
    }
}

pub fn avatar_group_css() -> String {
    r#"
.sh-avatar-group {
    display: flex;
    align-items: center;
}

.sh-avatar-group--stacked {
    flex-direction: row;
}

.sh-avatar-group--row {
    flex-direction: row;
    gap: 0.5rem;
}

.sh-avatar-group--grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(2.5rem, 1fr));
    gap: 0.5rem;
}

.sh-avatar-group__item {
    position: relative;
    flex-shrink: 0;
}

.sh-avatar-group__link {
    display: block;
}

.sh-avatar-group__img {
    border-radius: 50%;
    object-fit: cover;
    border: 2px solid var(--sh-surface, #fff);
    background: var(--sh-surface-2, #f3f4f6);
}

.sh-avatar-group__overflow {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--sh-surface-2, #f3f4f6);
    color: var(--sh-text-muted, #6b7280);
    font-size: 0.75rem;
    font-weight: 500;
    border: 2px solid var(--sh-surface, #fff);
}

/* Size variants */
.sh-avatar-group--xs .sh-avatar-group__img,
.sh-avatar-group--xs .sh-avatar-group__overflow {
    width: 1.5rem;
    height: 1.5rem;
    font-size: 0.625rem;
}

.sh-avatar-group--sm .sh-avatar-group__img,
.sh-avatar-group--sm .sh-avatar-group__overflow {
    width: 2rem;
    height: 2rem;
    font-size: 0.6875rem;
}

.sh-avatar-group--md .sh-avatar-group__img,
.sh-avatar-group--md .sh-avatar-group__overflow {
    width: 2.5rem;
    height: 2.5rem;
    font-size: 0.75rem;
}

.sh-avatar-group--lg .sh-avatar-group__img,
.sh-avatar-group--lg .sh-avatar-group__overflow {
    width: 3rem;
    height: 3rem;
    font-size: 0.8125rem;
}

.sh-avatar-group--xl .sh-avatar-group__img,
.sh-avatar-group--xl .sh-avatar-group__overflow {
    width: 3.5rem;
    height: 3.5rem;
    font-size: 0.875rem;
}

/* Stacked hover effect */
.sh-avatar-group--stacked:hover .sh-avatar-group__item {
    margin-left: 0.25rem !important;
    transition: margin 0.15s ease;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_group_creation() {
        let avatars = vec![
            AvatarItem {
                src: "/1.jpg",
                alt: "User 1",
                href: None,
            },
            AvatarItem {
                src: "/2.jpg",
                alt: "User 2",
                href: Some("/user/2"),
            },
        ];

        let group = AvatarGroup::new(avatars)
            .max(3)
            .variant(AvatarGroupVariant::Stacked);

        assert_eq!(group.avatars.len(), 2);
        assert_eq!(group.max, 3);
    }
}
