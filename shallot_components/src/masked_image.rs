//! MaskedImage Component - Creative Shape Image Masks
//!
//! Displays images with creative CSS mask shapes.
//! No JavaScript required for the masking effect.

use maud::{html, Markup, Render};

/// Mask shape variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaskShape {
    #[default]
    Circle,
    RoundedSquare,
    Blob,
    Star,
    Heart,
    Diamond,
    Hexagon,
    Triangle,
}

impl MaskShape {
    fn css_class(&self) -> &'static str {
        match self {
            MaskShape::Circle => "sh-maskedimg--circle",
            MaskShape::RoundedSquare => "sh-maskedimg--rounded",
            MaskShape::Blob => "sh-maskedimg--blob",
            MaskShape::Star => "sh-maskedimg--star",
            MaskShape::Heart => "sh-maskedimg--heart",
            MaskShape::Diamond => "sh-maskedimg--diamond",
            MaskShape::Hexagon => "sh-maskedimg--hexagon",
            MaskShape::Triangle => "sh-maskedimg--triangle",
        }
    }
}

/// MaskedImage component
pub struct MaskedImage<'a> {
    src: &'a str,
    alt: &'a str,
    shape: MaskShape,
    size: Option<&'a str>,
    class: Option<&'a str>,
}

impl<'a> MaskedImage<'a> {
    /// Create a new MaskedImage
    pub fn new(src: &'a str, alt: &'a str) -> Self {
        Self {
            src,
            alt,
            shape: MaskShape::default(),
            size: None,
            class: None,
        }
    }

    /// Set the mask shape
    pub fn shape(mut self, shape: MaskShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set explicit size (e.g., "100px", "50%")
    pub fn size(mut self, size: &'a str) -> Self {
        self.size = Some(size);
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-maskedimg".to_string()];
        classes.push(self.shape.css_class().to_string());
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for MaskedImage<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            figure class=(classes) {
                img
                    class="sh-maskedimg__image"
                    src=(self.src)
                    alt=(self.alt)
                    loading="lazy"
                    style=[self.size.map(|s| format!("width: {}; height: {}; object-fit: cover;", s, s))]
                ;
            }
        }
    }
}

/// Generate CSS for MaskedImage component
pub fn masked_image_css() -> String {
    r#"
.sh-maskedimg {
    display: inline-block;
    margin: 0;
    overflow: hidden;
}

.sh-maskedimg__image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s ease;
}

.sh-maskedimg:hover .sh-maskedimg__image {
    transform: scale(1.05);
}

/* Circle mask */
.sh-maskedimg--circle {
    clip-path: circle(50% at 50% 50%);
}

.sh-maskedimg--circle .sh-maskedimg__image {
    aspect-ratio: 1;
}

/* Rounded square */
.sh-maskedimg--rounded {
    clip-path: inset(0 0 0 0 round 20%);
}

/* Blob shape */
.sh-maskedimg--blob {
    clip-path: path("M42.7,-62.9C54.4,-52.5,62.3,-38.4,66.1,-23.3C69.9,-8.2,69.6,7.9,64.4,21.8C59.2,35.7,49.1,47.4,37.5,55.8C25.9,64.2,12.8,69.3,-0.6,70.7C-14,72.1,-27.7,69.8,-39.8,62.8C-51.9,55.8,-62.4,44.1,-69.3,30.7C-76.2,17.3,-79.5,2.2,-76.6,-11.8C-73.7,-25.8,-64.6,-38.7,-53.4,-48.4C-42.2,-58.1,-28.9,-64.6,-15.8,-65.8C-2.7,-67,10.4,-62.9,21.6,-56.4L42.7,-62.9Z");
    transform: scale(0.8);
}

/* Star shape */
.sh-maskedimg--star {
    clip-path: polygon(50% 0%, 61% 35%, 98% 35%, 68% 57%, 79% 91%, 50% 70%, 21% 91%, 32% 57%, 2% 35%, 39% 35%);
}

.sh-maskedimg--star .sh-maskedimg__image {
    aspect-ratio: 1;
}

/* Heart shape */
.sh-maskedimg--heart {
    clip-path: path("M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z");
    transform: scale(0.8);
}

.sh-maskedimg--heart .sh-maskedimg__image {
    aspect-ratio: 1;
}

/* Diamond shape */
.sh-maskedimg--diamond {
    clip-path: polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%);
}

.sh-maskedimg--diamond .sh-maskedimg__image {
    aspect-ratio: 1;
}

/* Hexagon shape */
.sh-maskedimg--hexagon {
    clip-path: polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%);
}

.sh-maskedimg--hexagon .sh-maskedimg__image {
    aspect-ratio: 1;
}

/* Triangle shape */
.sh-maskedimg--triangle {
    clip-path: polygon(50% 0%, 0% 100%, 100% 100%);
}

.sh-maskedimg--triangle .sh-maskedimg__image {
    aspect-ratio: 1;
}

/* Animation on hover */
@keyframes sh-masked-pulse {
    0%, 100% {
        transform: scale(1);
    }
    50% {
        transform: scale(1.05);
    }
}

.sh-maskedimg:hover .sh-maskedimg__image {
    animation: sh-masked-pulse 0.6s ease-in-out;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maskedimg_creation() {
        let img = MaskedImage::new("/photo.jpg", "A photo");
        assert_eq!(img.src, "/photo.jpg");
        assert_eq!(img.shape, MaskShape::Circle);
    }

    #[test]
    fn test_maskedimg_shape() {
        let img = MaskedImage::new("/photo.jpg", "Photo").shape(MaskShape::Star);
        assert_eq!(img.shape, MaskShape::Star);
    }

    #[test]
    fn test_maskedimg_size() {
        let img = MaskedImage::new("/photo.jpg", "Photo").size("200px");
        assert_eq!(img.size, Some("200px"));
    }

    #[test]
    fn test_maskedimg_css() {
        let css = masked_image_css();
        assert!(css.contains(".sh-maskedimg"));
        assert!(css.contains(".sh-maskedimg--circle"));
        assert!(css.contains(".sh-maskedimg--star"));
    }
}
