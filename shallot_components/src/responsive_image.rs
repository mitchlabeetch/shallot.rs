//! Responsive Image Component - Image with responsive sizing
//! CSS-only responsive image with aspect ratio support

use maud::{html, Markup, Render};

/// Image fit mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFit {
    Cover,
    Contain,
    #[default]
    Fill,
    None,
}

/// Image radius
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageRadius {
    None,
    #[default]
    Md,
    Lg,
    Full,
}

/// Responsive image component
#[derive(Debug, Clone)]
pub struct ResponsiveImage<'a> {
    pub src: &'a str,
    pub alt: &'a str,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fit: ImageFit,
    pub radius: ImageRadius,
    pub lazy: bool,
    pub sources: Vec<ImageSource<'a>>,
}

/// Image source for srcset
#[derive(Debug, Clone)]
pub struct ImageSource<'a> {
    pub srcset: &'a str,
    pub media: Option<&'a str>,
    pub r#type: Option<&'a str>,
}

impl<'a> ResponsiveImage<'a> {
    /// Create a new responsive image
    pub fn new(src: &'a str, alt: &'a str) -> Self {
        Self {
            src,
            alt,
            width: None,
            height: None,
            fit: ImageFit::default(),
            radius: ImageRadius::default(),
            lazy: true,
            sources: Vec::new(),
        }
    }

    /// Set the width
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the fit mode
    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    /// Set the border radius
    pub fn radius(mut self, radius: ImageRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Set lazy loading
    pub fn lazy(mut self, lazy: bool) -> Self {
        self.lazy = lazy;
        self
    }

    /// Add a source for srcset
    pub fn source(mut self, srcset: &'a str) -> Self {
        self.sources.push(ImageSource {
            srcset,
            media: None,
            r#type: None,
        });
        self
    }

    /// Add a source with media query
    pub fn source_with_media(mut self, srcset: &'a str, media: &'a str) -> Self {
        self.sources.push(ImageSource {
            srcset,
            media: Some(media),
            r#type: None,
        });
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-image".to_string()];

        let fit_class = match self.fit {
            ImageFit::Cover => "sh-image--cover",
            ImageFit::Contain => "sh-image--contain",
            ImageFit::Fill => "sh-image--fill",
            ImageFit::None => "sh-image--none",
        };
        classes.push(fit_class.to_string());

        let radius_class = match self.radius {
            ImageRadius::None => "sh-image--radius-none",
            ImageRadius::Md => "sh-image--radius-md",
            ImageRadius::Lg => "sh-image--radius-lg",
            ImageRadius::Full => "sh-image--radius-full",
        };
        classes.push(radius_class.to_string());

        classes.join(" ")
    }
}

impl<'a> Render for ResponsiveImage<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            picture
                class="sh-image__container"
                role="img"
                aria-label=(self.alt)
            {
                @for source in &self.sources {
                    source
                        srcset=(source.srcset)
                        media?=(source.media)
                        type?=(source.r#type);
                }
                img
                    src=(self.src)
                    alt=(self.alt)
                    class=(classes)
                    width?=(self.width)
                    height?=(self.height)
                    loading?={(if self.lazy { Some("lazy") } else { None })}
                    decoding="async";
            }
        }
    }
}

/// Generate CSS for responsive image component
pub fn responsive_image_css() -> String {
    r#"
.sh-image__container {
    display: inline-block;
    max-width: 100%;
}

.sh-image {
    max-width: 100%;
    height: auto;
    display: block;
}

.sh-image--cover {
    object-fit: cover;
}

.sh-image--contain {
    object-fit: contain;
}

.sh-image--fill {
    object-fit: fill;
}

.sh-image--none {
    object-fit: none;
}

.sh-image--radius-none {
    border-radius: 0;
}

.sh-image--radius-md {
    border-radius: var(--sh-radius-md, 0.375rem);
}

.sh-image--radius-lg {
    border-radius: var(--sh-radius-lg, 0.5rem);
}

.sh-image--radius-full {
    border-radius: var(--sh-radius-full, 9999px);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_image_creation() {
        let img = ResponsiveImage::new("/img.jpg", "Test image");

        assert_eq!(img.src, "/img.jpg");
        assert_eq!(img.alt, "Test image");
        assert_eq!(img.fit, ImageFit::Fill);
        assert!(img.lazy);
    }

    #[test]
    fn test_responsive_image_render() {
        let img = ResponsiveImage::new("/test.jpg", "Test");
        let html = img.render().into_string();

        assert!(html.contains("sh-image"));
        assert!(html.contains("src=\"/test.jpg\""));
        assert!(html.contains("alt=\"Test\""));
    }

    #[test]
    fn test_responsive_image_fit_modes() {
        let cover = ResponsiveImage::new("/", "test").fit(ImageFit::Cover);
        let contain = ResponsiveImage::new("/", "test").fit(ImageFit::Contain);

        assert!(cover.render().into_string().contains("sh-image--cover"));
        assert!(contain.render().into_string().contains("sh-image--contain"));
    }

    #[test]
    fn test_responsive_image_radius() {
        let none = ResponsiveImage::new("/", "test").radius(ImageRadius::None);
        let full = ResponsiveImage::new("/", "test").radius(ImageRadius::Full);

        assert!(none
            .render()
            .into_string()
            .contains("sh-image--radius-none"));
        assert!(full
            .render()
            .into_string()
            .contains("sh-image--radius-full"));
    }

    #[test]
    fn test_responsive_image_dimensions() {
        let img = ResponsiveImage::new("/", "test").width(200).height(150);
        let html = img.render().into_string();

        assert!(html.contains("width=\"200\""));
        assert!(html.contains("height=\"150\""));
    }

    #[test]
    fn test_responsive_image_lazy() {
        let lazy = ResponsiveImage::new("/", "test").lazy(true);
        let eager = ResponsiveImage::new("/", "test").lazy(false);

        assert!(lazy.render().into_string().contains("loading=\"lazy\""));
        assert!(!eager.render().into_string().contains("loading=\"lazy\""));
    }

    #[test]
    fn test_responsive_image_sources() {
        let img = ResponsiveImage::new("/", "test")
            .source("/img-400.jpg 400w")
            .source_with_media("/img-800.jpg 800w", "(min-width: 600px)");
        let html = img.render().into_string();

        assert!(html.contains("<source"));
        assert!(html.contains("srcset"));
        assert!(html.contains("media"));
    }

    #[test]
    fn test_responsive_image_css() {
        let css = responsive_image_css();
        assert!(css.contains(".sh-image"));
        assert!(css.contains("object-fit"));
    }
}
