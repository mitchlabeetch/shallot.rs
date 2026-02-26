//! ImageUpload Component - Drag-and-Drop Image Upload Interface
//!
//! A beautiful drag-and-drop image upload interface with preview.
//! Uses pure CSS for hover states and visual feedback.

use maud::{html, Markup, Render};

/// ImageUpload size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageUploadSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ImageUploadSize {
    fn css_class(&self) -> &'static str {
        match self {
            ImageUploadSize::Small => "sh-imageupload--sm",
            ImageUploadSize::Medium => "sh-imageupload--md",
            ImageUploadSize::Large => "sh-imageupload--lg",
        }
    }
}

/// ImageUpload component
pub struct ImageUpload<'a> {
    name: &'a str,
    accept: &'a str,
    multiple: bool,
    preview_url: Option<&'a str>,
    size: ImageUploadSize,
    disabled: bool,
    required: bool,
    class: Option<&'a str>,
}

impl<'a> ImageUpload<'a> {
    /// Create a new ImageUpload
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            accept: "image/*",
            multiple: false,
            preview_url: None,
            size: ImageUploadSize::default(),
            disabled: false,
            required: false,
            class: None,
        }
    }

    /// Set accepted file types
    pub fn accept(mut self, accept: &'a str) -> Self {
        self.accept = accept;
        self
    }

    /// Allow multiple files
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Set preview image URL
    pub fn preview(mut self, url: &'a str) -> Self {
        self.preview_url = Some(url);
        self
    }

    /// Set size
    pub fn size(mut self, size: ImageUploadSize) -> Self {
        self.size = size;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set required state
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Add custom class
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-imageupload".to_string()];
        classes.push(self.size.css_class().to_string());
        if self.disabled {
            classes.push("sh-imageupload--disabled".to_string());
        }
        if let Some(custom) = self.class {
            classes.push(custom.to_string());
        }
        classes.join(" ")
    }
}

impl<'a> Render for ImageUpload<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();
        let input_id = format!("sh-imageupload-{}", self.name);

        html! {
            div class=(classes) {
                label class="sh-imageupload__dropzone" for=(input_id) {
                    input
                        type="file"
                        id=(input_id)
                        name=(self.name)
                        accept=(self.accept)
                        multiple?[self.multiple]
                        disabled?[self.disabled]
                        required?[self.required]
                        class="sh-imageupload__input"
                        aria-label="Upload image"
                    ;

                    @if let Some(preview) = self.preview_url {
                        div class="sh-imageupload__preview" {
                            img src=(preview) alt="Preview" class="sh-imageupload__preview-img";
                        }
                    } @else {
                        div class="sh-imageupload__icon" aria-hidden="true" {
                            svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" {
                                path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4";
                                polyline points="17 8 12 3 7 8";
                                line x1="12" y1="3" x2="12" y2="15";
                            }
                        }
                        div class="sh-imageupload__text" {
                            span class="sh-imageupload__title" {
                                "Drop image here or click to upload"
                            }
                            span class="sh-imageupload__subtitle" {
                                "PNG, JPG, GIF up to 10MB"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for ImageUpload component
pub fn image_upload_css() -> String {
    r#"
.sh-imageupload {
    width: 100%;
}

.sh-imageupload__dropzone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    border: 2px dashed var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-lg, 0.5rem);
    background: var(--sh-surface-2, #f9fafb);
    cursor: pointer;
    transition: all 0.2s ease;
    min-height: 12rem;
    position: relative;
}

.sh-imageupload__dropzone:hover {
    border-color: var(--sh-primary, #3b82f6);
    background: var(--sh-primary-bg, rgba(59, 130, 246, 0.05));
}

.sh-imageupload__dropzone:has(.sh-imageupload__input:focus) {
    border-color: var(--sh-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.sh-imageupload__input {
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
}

.sh-imageupload__icon {
    color: var(--sh-text-muted, #9ca3af);
    margin-bottom: 1rem;
}

.sh-imageupload__text {
    text-align: center;
}

.sh-imageupload__title {
    display: block;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
    margin-bottom: 0.25rem;
}

.sh-imageupload__subtitle {
    display: block;
    font-size: 0.875rem;
    color: var(--sh-text-muted, #6b7280);
}

.sh-imageupload__preview {
    position: relative;
    max-width: 100%;
    max-height: 12rem;
}

.sh-imageupload__preview-img {
    max-width: 100%;
    max-height: 12rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    object-fit: contain;
}

/* Size variants */
.sh-imageupload--sm .sh-imageupload__dropzone {
    min-height: 8rem;
    padding: 1rem;
}

.sh-imageupload--md .sh-imageupload__dropzone {
    min-height: 12rem;
    padding: 2rem;
}

.sh-imageupload--lg .sh-imageupload__dropzone {
    min-height: 16rem;
    padding: 3rem;
}

/* Disabled state */
.sh-imageupload--disabled .sh-imageupload__dropzone {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imageupload_creation() {
        let upload = ImageUpload::new("avatar");
        assert_eq!(upload.name, "avatar");
        assert_eq!(upload.accept, "image/*");
        assert!(!upload.multiple);
    }

    #[test]
    fn test_imageupload_accept() {
        let upload = ImageUpload::new("file").accept(".png,.jpg");
        assert_eq!(upload.accept, ".png,.jpg");
    }

    #[test]
    fn test_imageupload_multiple() {
        let upload = ImageUpload::new("gallery").multiple(true);
        assert!(upload.multiple);
    }

    #[test]
    fn test_imageupload_preview() {
        let upload = ImageUpload::new("avatar").preview("/preview.jpg");
        assert_eq!(upload.preview_url, Some("/preview.jpg"));
    }

    #[test]
    fn test_imageupload_size() {
        let upload = ImageUpload::new("file").size(ImageUploadSize::Large);
        assert_eq!(upload.size, ImageUploadSize::Large);
    }

    #[test]
    fn test_imageupload_css() {
        let css = image_upload_css();
        assert!(css.contains(".sh-imageupload"));
        assert!(css.contains(".sh-imageupload__dropzone"));
    }
}
