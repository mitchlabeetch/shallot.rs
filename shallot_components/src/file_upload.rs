//! File Upload Component
//!
//! A drag-and-drop file upload component with preview and progress tracking.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

/// FileUpload - Drag and drop file upload component
pub struct FileUpload<'a> {
    name: &'a str,
    label: Option<&'a str>,
    placeholder: Option<&'a str>,
    accept: Option<&'a str>,
    multiple: bool,
    disabled: bool,
    required: bool,
    size: ComponentSize,
}

impl<'a> FileUpload<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            label: None,
            placeholder: None,
            accept: None,
            multiple: false,
            disabled: false,
            required: false,
            size: ComponentSize::Md,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn accept(mut self, accept: &'a str) -> Self {
        self.accept = Some(accept);
        self
    }

    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Render for FileUpload<'a> {
    fn render(&self) -> Markup {
        let placeholder_text = self
            .placeholder
            .unwrap_or("Drag and drop or click to upload");

        let size_class = match self.size {
            ComponentSize::Xs => "sh-upload--xs",
            ComponentSize::Sm => "sh-upload--sm",
            ComponentSize::Md => "sh-upload--md",
            ComponentSize::Lg => "sh-upload--lg",
            ComponentSize::Xl => "sh-upload--xl",
        };

        let aria_label = self.label.unwrap_or("File upload");
        let accept_value = self.accept.unwrap_or("");

        html! {
            div class="sh-upload-wrapper" {
                @if let Some(label) = self.label {
                    label class="sh-upload-label" for=(self.name) {
                        (label)
                        @if self.required {
                            span class="sh-required" aria-hidden="true" { "*" }
                        }
                    }
                }

                label class=(format!("sh-upload {}", size_class)) for=(self.name) {
                    input
                        type="file"
                        id=(self.name)
                        name=(self.name)
                        class="sh-upload-input"
                        accept=(accept_value)
                        multiple?[self.multiple]
                        disabled?[self.disabled]
                        required?[self.required]
                        aria-label=(aria_label);

                    div class="sh-upload-content" {
                        div class="sh-upload-icon" aria-hidden="true" {
                            svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4";
                                polyline points="17 8 12 3 7 8";
                                line x1="12" y1="3" x2="12" y2="15";
                            }
                        }
                        span class="sh-upload-placeholder" { (placeholder_text) }
                    }
                }
            }
        }
    }
}

/// FilePreview - Preview component for uploaded files
pub struct FilePreview<'a> {
    filename: &'a str,
    filesize: Option<&'a str>,
}

impl<'a> FilePreview<'a> {
    pub fn new(filename: &'a str) -> Self {
        Self {
            filename,
            filesize: None,
        }
    }

    pub fn filesize(mut self, size: &'a str) -> Self {
        self.filesize = Some(size);
        self
    }
}

impl<'a> Render for FilePreview<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-file-preview" role="listitem" {
                div class="sh-file-preview-info" {
                    span class="sh-file-preview-name" { (self.filename) }
                    @if let Some(size) = self.filesize {
                        span class="sh-file-preview-size" { (size) }
                    }
                }
            }
        }
    }
}

/// Generate file upload CSS
pub fn file_upload_css() -> String {
    r#"
.sh-upload-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
}

.sh-upload-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
}

.sh-required {
    color: var(--sh-error, #ef4444);
    margin-left: 0.125rem;
}

.sh-upload {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    border: 2px dashed var(--sh-border, #d1d5db);
    border-radius: var(--sh-radius-md, 0.5rem);
    background: var(--sh-surface, #f9fafb);
    cursor: pointer;
    transition: all 0.2s ease;
}

.sh-upload--sm {
    padding: 1rem;
}

.sh-upload--lg {
    padding: 3rem;
}

.sh-upload:hover {
    border-color: var(--sh-primary, #3b82f6);
}

.sh-upload-input {
    display: none;
}

.sh-upload-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
}

.sh-upload-icon {
    color: var(--sh-primary, #3b82f6);
}

.sh-upload-placeholder {
    font-size: 0.9375rem;
    color: var(--sh-text-secondary, #6b7280);
}

.sh-file-preview {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: var(--sh-surface, #f9fafb);
    border: 1px solid var(--sh-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.5rem);
}

.sh-file-preview-name {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text, #1f2937);
}

.sh-file-preview-size {
    font-size: 0.75rem;
    color: var(--sh-text-secondary, #6b7280);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_upload_default() {
        let upload = FileUpload::new("file");
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("sh-upload"));
    }

    #[test]
    fn test_file_upload_with_label() {
        let upload = FileUpload::new("file").label("Upload Document");
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("Upload Document"));
        assert!(rendered.0.as_str().contains("sh-upload-label"));
    }

    #[test]
    fn test_file_upload_with_placeholder() {
        let upload = FileUpload::new("file").placeholder("Drop files here");
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("Drop files here"));
    }

    #[test]
    fn test_file_upload_with_accept() {
        let upload = FileUpload::new("file").accept(".pdf,.doc");
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("accept=\".pdf,.doc\""));
    }

    #[test]
    fn test_file_upload_multiple() {
        let upload = FileUpload::new("file").multiple();
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("multiple"));
    }

    #[test]
    fn test_file_upload_disabled() {
        let upload = FileUpload::new("file").disabled();
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("disabled"));
    }

    #[test]
    fn test_file_upload_required() {
        let upload = FileUpload::new("file").label("File").required();
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("required"));
        assert!(rendered.0.as_str().contains("sh-required"));
    }

    #[test]
    fn test_file_upload_size_sm() {
        let upload = FileUpload::new("file").size(ComponentSize::Sm);
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("sh-upload--sm"));
    }

    #[test]
    fn test_file_upload_size_lg() {
        let upload = FileUpload::new("file").size(ComponentSize::Lg);
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("sh-upload--lg"));
    }

    #[test]
    fn test_file_upload_a11y() {
        let upload = FileUpload::new("file").label("Document");
        let rendered = upload.render();
        assert!(rendered.0.as_str().contains("aria-label=\"Document\""));
    }

    #[test]
    fn test_file_preview_default() {
        let preview = FilePreview::new("document.pdf");
        let rendered = preview.render();
        assert!(rendered.0.as_str().contains("sh-file-preview"));
        assert!(rendered.0.as_str().contains("document.pdf"));
    }

    #[test]
    fn test_file_preview_with_size() {
        let preview = FilePreview::new("document.pdf").filesize("2.5 MB");
        let rendered = preview.render();
        assert!(rendered.0.as_str().contains("2.5 MB"));
    }

    #[test]
    fn test_file_preview_a11y() {
        let preview = FilePreview::new("document.pdf");
        let rendered = preview.render();
        assert!(rendered.0.as_str().contains("role=\"listitem\""));
    }

    #[test]
    fn test_file_upload_css_function() {
        let css = file_upload_css();
        assert!(css.contains(".sh-upload"));
        assert!(css.contains(".sh-file-preview"));
    }
}
