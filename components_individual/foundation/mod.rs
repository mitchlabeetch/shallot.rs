mod animations;
mod hsl;
mod theme;

pub use animations::{all_animations, animation_classes, Animation};
pub use hsl::Hsl;
pub use theme::{ColorMode, Theme, ThemeName};

pub fn foundation_css() -> String {
    let mut css = String::new();
    css.push_str(&Theme::default().to_css());
    css.push_str("\n");
    css.push_str(&all_animations());
    css.push_str("\n");
    css.push_str(&animation_classes());
    css
}
