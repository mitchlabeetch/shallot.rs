//! File List Component - List of files with actions
//! CSS-only styling with hover states

use maud::{html, Markup, Render};

/// File item for the file list
#[derive(Debug, Clone)]
pub struct FileItem<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub size: Option<&'a str>,
    pub file_type: Option<&'a str>,
    pub thumbnail: Option<&'a str>,
    pub removable: bool,
    pub downloadable: bool,
}

impl<'a> FileItem<'a> {
    /// Create a new file item
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            size: None,
            file_type: None,
            thumbnail: None,
            removable: true,
            downloadable: false,
        }
    }

    /// Set the file size
    pub fn size(mut self, size: &'a str) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the file type
    pub fn file_type(mut self, file_type: &'a str) -> Self {
        self.file_type = Some(file_type);
        self
    }

    /// Set the thumbnail URL
    pub fn thumbnail(mut self, thumbnail: &'a str) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    /// Set if the file can be removed
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    /// Set if the file can be downloaded
    pub fn downloadable(mut self, downloadable: bool) -> Self {
        self.downloadable = downloadable;
        self
    }

    fn get_icon(&self) -> &'static str {
        match self.file_type.map(|s| s.to_lowercase()).as_deref() {
            Some("pdf") => "pdf",
            Some("image") | Some("jpg") | Some("jpeg") | Some("png") | Some("gif")
            | Some("webp") => "image",
            Some("video") | Some("mp4") | Some("webm") => "video",
            Some("audio") | Some("mp3") | Some("wav") => "audio",
            Some("zip") | Some("archive") => "archive",
            Some("doc") | Some("docx") => "doc",
            Some("xls") | Some("xlsx") => "xls",
            Some("code") | Some("js") | Some("ts") | Some("rs") | Some("py") => "code",
            _ => "file",
        }
    }
}

/// File list component
#[derive(Debug, Clone)]
pub struct FileList<'a> {
    pub files: Vec<FileItem<'a>>,
    pub compact: bool,
}

impl<'a> FileList<'a> {
    /// Create a new file list
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            compact: false,
        }
    }

    /// Add a file to the list
    pub fn file(mut self, file: FileItem<'a>) -> Self {
        self.files.push(file);
        self
    }

    /// Set files from a vector
    pub fn files(mut self, files: Vec<FileItem<'a>>) -> Self {
        self.files = files;
        self
    }

    /// Set compact mode
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}

impl<'a> Default for FileList<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for FileList<'a> {
    fn render(&self) -> Markup {
        let container_class = if self.compact {
            "sh-file-list sh-file-list--compact"
        } else {
            "sh-file-list"
        };

        html! {
            ul class=(container_class) role="list" {
                @for file in &self.files {
                    li class="sh-file-list__item" {
                        @if let Some(thumb) = file.thumbnail {
                            img
                                src=(thumb)
                                alt={"Preview of " (file.name)}
                                class="sh-file-list__thumbnail";
                        } @else {
                            div class={"sh-file-list__icon sh-file-list__icon--" (file.get_icon())} {
                                // File icon
                                svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round" {
                                    path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" {}
                                    polyline points="13 2 13 9 20 9" {}
                                }
                            }
                        }
                        div class="sh-file-list__info" {
                            span class="sh-file-list__name" {
                                (file.name)
                            }
                            @if let Some(size) = file.size {
                                span class="sh-file-list__size" {
                                    (size)
                                }
                            }
                        }
                        div class="sh-file-list__actions" {
                            @if file.downloadable {
                                button
                                    type="button"
                                    class="sh-file-list__action sh-file-list__action--download"
                                    aria-label={"Download " (file.name)}
                                    data-file-id=(file.id) {
                                    svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round" {
                                        path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" {}
                                        polyline points="7 10 12 15 17 10" {}
                                        line x1="12" y1="15" x2="12" y2="3" {}
                                    }
                                }
                            }
                            @if file.removable {
                                button
                                    type="button"
                                    class="sh-file-list__action sh-file-list__action--remove"
                                    aria-label={"Remove " (file.name)}
                                    data-file-id=(file.id) {
                                    svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round" {
                                        line x1="18" y1="6" x2="6" y2="18" {}
                                        line x1="6" y1="6" x2="18" y2="18" {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for file list component
pub fn file_list_css() -> String {
    r#"
.sh-file-list {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-2, 0.5rem);
    list-style: none;
    padding: 0;
    margin: 0;
}

.sh-file-list--compact {
    gap: var(--sh-spacing-1, 0.25rem);
}

.sh-file-list__item {
    display: flex;
    align-items: center;
    gap: var(--sh-spacing-3, 0.75rem);
    padding: var(--sh-spacing-3, 0.75rem);
    background-color: var(--sh-color-background, #ffffff);
    border: 1px solid var(--sh-color-border, #e5e7eb);
    border-radius: var(--sh-radius-md, 0.375rem);
    transition: background-color 0.15s ease;
}

.sh-file-list--compact .sh-file-list__item {
    padding: var(--sh-spacing-2, 0.5rem);
}

.sh-file-list__item:hover {
    background-color: var(--sh-color-muted, #f3f4f6);
}

.sh-file-list__thumbnail {
    width: 40px;
    height: 40px;
    border-radius: var(--sh-radius-sm, 0.25rem);
    object-fit: cover;
    flex-shrink: 0;
}

.sh-file-list--compact .sh-file-list__thumbnail {
    width: 32px;
    height: 32px;
}

.sh-file-list__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: var(--sh-radius-sm, 0.25rem);
    background-color: var(--sh-color-muted, #f3f4f6);
    color: var(--sh-color-muted-foreground, #6b7280);
    flex-shrink: 0;
}

.sh-file-list__icon--pdf {
    background-color: #fef2f2;
    color: #ef4444;
}

.sh-file-list__icon--image {
    background-color: #ecfdf5;
    color: #10b981;
}

.sh-file-list__icon--video {
    background-color: #eff6ff;
    color: #3b82f6;
}

.sh-file-list__icon--audio {
    background-color: #fdf4ff;
    color: #a855f7;
}

.sh-file-list__icon--archive {
    background-color: #fefce8;
    color: #eab308;
}

.sh-file-list__icon--doc {
    background-color: #eff6ff;
    color: #3b82f6;
}

.sh-file-list__icon--code {
    background-color: #1f2937;
    color: #10b981;
}

.sh-file-list__info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-1, 0.25rem);
}

.sh-file-list__name {
    font-size: var(--sh-font-size-sm, 0.875rem);
    font-weight: var(--sh-font-weight-medium, 500);
    color: var(--sh-color-foreground, #1f2937);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.sh-file-list__size {
    font-size: var(--sh-font-size-xs, 0.75rem);
    color: var(--sh-color-muted-foreground, #6b7280);
}

.sh-file-list__actions {
    display: flex;
    gap: var(--sh-spacing-1, 0.25rem);
    opacity: 0;
    transition: opacity 0.15s ease;
}

.sh-file-list__item:hover .sh-file-list__actions {
    opacity: 1;
}

.sh-file-list__action {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    border: none;
    background-color: transparent;
    color: var(--sh-color-muted-foreground, #6b7280);
    border-radius: var(--sh-radius-sm, 0.25rem);
    cursor: pointer;
    transition: all 0.15s ease;
}

.sh-file-list__action:hover {
    background-color: var(--sh-color-muted, #f3f4f6);
    color: var(--sh-color-foreground, #1f2937);
}

.sh-file-list__action--remove:hover {
    background-color: #fef2f2;
    color: #ef4444;
}

.sh-file-list__action:focus-visible {
    outline: 2px solid var(--sh-color-primary, #3b82f6);
    outline-offset: 2px;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_item_creation() {
        let file = FileItem::new("1", "document.pdf");

        assert_eq!(file.id, "1");
        assert_eq!(file.name, "document.pdf");
        assert!(file.removable);
        assert!(!file.downloadable);
    }

    #[test]
    fn test_file_list_creation() {
        let list = FileList::new();

        assert!(list.files.is_empty());
        assert!(!list.compact);
    }

    #[test]
    fn test_file_list_render() {
        let list = FileList::new().file(FileItem::new("1", "test.pdf").size("1.2 MB"));

        let html = list.render().into_string();

        assert!(html.contains("sh-file-list"));
        assert!(html.contains("test.pdf"));
        assert!(html.contains("1.2 MB"));
    }

    #[test]
    fn test_file_list_compact() {
        let list = FileList::new().compact(true);
        let html = list.render().into_string();

        assert!(html.contains("sh-file-list--compact"));
    }

    #[test]
    fn test_file_item_removable() {
        let file = FileItem::new("1", "test.pdf").removable(true);
        let list = FileList::new().file(file);
        let html = list.render().into_string();

        assert!(html.contains("sh-file-list__action--remove"));
    }

    #[test]
    fn test_file_item_downloadable() {
        let file = FileItem::new("1", "test.pdf").downloadable(true);
        let list = FileList::new().file(file);
        let html = list.render().into_string();

        assert!(html.contains("sh-file-list__action--download"));
    }

    #[test]
    fn test_file_list_css() {
        let css = file_list_css();
        assert!(css.contains(".sh-file-list"));
        assert!(css.contains(".sh-file-list__item"));
    }
}
