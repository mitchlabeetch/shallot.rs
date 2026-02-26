//! Code Block Component - Syntax Highlighted Code Display
//!
//! A comprehensive code block component with syntax highlighting hooks,
//! copy functionality (zero-JS via server-side), line numbers, and file name display.
//!
//! Research Reference: Section 5.7.2 - Content Publishing
//!
//! # Example
//! ```
//! use shallot_components::code_block::{CodeBlock, Language};
//!
//! let code = CodeBlock::new(r#"fn main() {
//!     println!("Hello, world!");
//! }"#)
//! .language(Language::Rust)
//! .filename("main.rs")
//! .with_line_numbers()
//! .highlight_lines(vec![2]);
//! ```

use maud::{html, Markup};

/// Supported programming languages for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Html,
    Css,
    Scss,
    Json,
    Yaml,
    Markdown,
    Sql,
    Bash,
    Go,
    Java,
    C,
    Cpp,
    CSharp,
    Php,
    Ruby,
    Swift,
    Kotlin,
    Dart,
    Lua,
    R,
    Perl,
    Scala,
    Groovy,
    VbNet,
    PowerShell,
    Docker,
    Nginx,
    Graphql,
    Regex,
    PlainText,
    Custom(&'static str),
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Html => "html",
            Self::Css => "css",
            Self::Scss => "scss",
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Markdown => "markdown",
            Self::Sql => "sql",
            Self::Bash => "bash",
            Self::Go => "go",
            Self::Java => "java",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::CSharp => "csharp",
            Self::Php => "php",
            Self::Ruby => "ruby",
            Self::Swift => "swift",
            Self::Kotlin => "kotlin",
            Self::Dart => "dart",
            Self::Lua => "lua",
            Self::R => "r",
            Self::Perl => "perl",
            Self::Scala => "scala",
            Self::Groovy => "groovy",
            Self::VbNet => "vbnet",
            Self::PowerShell => "powershell",
            Self::Docker => "docker",
            Self::Nginx => "nginx",
            Self::Graphql => "graphql",
            Self::Regex => "regex",
            Self::PlainText => "text",
            Self::Custom(s) => s,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Scss => "SCSS",
            Self::Json => "JSON",
            Self::Yaml => "YAML",
            Self::Markdown => "Markdown",
            Self::Sql => "SQL",
            Self::Bash => "Bash",
            Self::Go => "Go",
            Self::Java => "Java",
            Self::C => "C",
            Self::Cpp => "C++",
            Self::CSharp => "C#",
            Self::Php => "PHP",
            Self::Ruby => "Ruby",
            Self::Swift => "Swift",
            Self::Kotlin => "Kotlin",
            Self::Dart => "Dart",
            Self::Lua => "Lua",
            Self::R => "R",
            Self::Perl => "Perl",
            Self::Scala => "Scala",
            Self::Groovy => "Groovy",
            Self::VbNet => "VB.NET",
            Self::PowerShell => "PowerShell",
            Self::Docker => "Docker",
            Self::Nginx => "Nginx",
            Self::Graphql => "GraphQL",
            Self::Regex => "Regex",
            Self::PlainText => "Plain Text",
            Self::Custom(s) => s,
        }
    }

    /// Get the appropriate CSS class for syntax highlighting
    pub fn css_class(&self) -> String {
        format!("language-{}", self.as_str())
    }
}

/// Code block theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeTheme {
    Default,
    Dark,
    Light,
    HighContrast,
}

/// Code block variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockVariant {
    Default,
    Inline,  // For inline code displays
    Card,    // Card-styled container
    Minimal, // Minimal styling
}

/// Code block component
#[derive(Debug, Clone)]
pub struct CodeBlock {
    code: String,
    language: Option<Language>,
    filename: Option<String>,
    show_line_numbers: bool,
    highlight_lines: Vec<u32>,
    starting_line_number: u32,
    show_copy_button: bool,
    wrap_lines: bool,
    max_height: Option<u16>,
    variant: CodeBlockVariant,
    theme: CodeTheme,
    class: Option<String>,
    id: Option<String>,
}

impl CodeBlock {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: None,
            filename: None,
            show_line_numbers: false,
            highlight_lines: Vec::new(),
            starting_line_number: 1,
            show_copy_button: true,
            wrap_lines: false,
            max_height: None,
            variant: CodeBlockVariant::Default,
            theme: CodeTheme::Default,
            class: None,
            id: None,
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    pub fn with_line_numbers(mut self) -> Self {
        self.show_line_numbers = true;
        self
    }

    pub fn starting_line_number(mut self, number: u32) -> Self {
        self.starting_line_number = number;
        self
    }

    pub fn highlight_lines(mut self, lines: Vec<u32>) -> Self {
        self.highlight_lines = lines;
        self
    }

    pub fn show_copy_button(mut self, show: bool) -> Self {
        self.show_copy_button = show;
        self
    }

    pub fn wrap_lines(mut self, wrap: bool) -> Self {
        self.wrap_lines = wrap;
        self
    }

    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn variant(mut self, variant: CodeBlockVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Escape HTML special characters
    fn escape_html_static(content: &str) -> String {
        content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    /// Public method to escape HTML special characters
    pub fn escape_html(&self, content: &str) -> String {
        Self::escape_html_static(content)
    }

    /// Get the code content as lines
    fn code_lines(&self) -> Vec<&str> {
        self.code.lines().collect()
    }

    /// Generate the CSS class for the container
    fn container_class(&self) -> String {
        let mut classes = vec!["sh-code-block-container".to_string()];

        match self.variant {
            CodeBlockVariant::Inline => classes.push("sh-code-block--inline".to_string()),
            CodeBlockVariant::Card => classes.push("sh-code-block--card".to_string()),
            CodeBlockVariant::Minimal => classes.push("sh-code-block--minimal".to_string()),
            _ => {}
        }

        match self.theme {
            CodeTheme::Dark => classes.push("sh-code-block--dark".to_string()),
            CodeTheme::Light => classes.push("sh-code-block--light".to_string()),
            CodeTheme::HighContrast => classes.push("sh-code-block--high-contrast".to_string()),
            _ => {}
        }

        if self.wrap_lines {
            classes.push("sh-code-block--wrap".to_string());
        }

        if let Some(ref c) = self.class {
            classes.push(c.clone());
        }

        classes.join(" ")
    }

    fn build_styles(&self) -> String {
        let mut styles = String::new();

        if let Some(height) = self.max_height {
            styles.push_str(&format!("max-height:{}px;", height));
        }

        styles
    }

    pub fn render(self) -> Markup {
        let lines = self.code_lines();
        let line_count = lines.len();
        let escaped_code = Self::escape_html_static(&self.code);
        let container_class = self.container_class();
        let styles = self.build_styles();
        let id = self.id.clone();
        let starting_line = self.starting_line_number;

        let language_class = self
            .language
            .as_ref()
            .map(|l| l.css_class())
            .unwrap_or_default();

        let language_display = self
            .language
            .as_ref()
            .map(|l| l.display_name())
            .unwrap_or("Code");

        html! {
            figure
                class=(container_class)
                style=(styles)
                id=[id]
            {
                // Header with filename and language
                @if self.filename.is_some() || self.language.is_some() || self.show_copy_button {
                    figcaption class="sh-code-header" {
                        div class="sh-code-header-left" {
                            @if let Some(ref filename) = self.filename {
                                span class="sh-code-filename" {
                                    (filename)
                                }
                            }
                        }

                        div class="sh-code-header-right" {
                            @if self.language.is_some() {
                                span class="sh-code-language" {
                                    (language_display)
                                }
                            }

                            @if self.show_copy_button {
                                button
                                    class="sh-code-copy-btn"
                                    type="button"
                                    title="Copy to clipboard"
                                    aria-label="Copy code to clipboard"
                                    data-code=(escaped_code)
                                {
                                    span class="sh-code-copy-icon" {
                                        svg width="16" height="16" viewBox="0 0 16 16" fill="none" {
                                            path d="M5 2h6v10H5V2z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" {}
                                            path d="M3 6h8v8H3V6z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" {}
                                        }
                                    }
                                    span class="sh-code-copy-text" { "Copy" }
                                }
                            }
                        }
                    }
                }

                // Code content
                div class="sh-code-content" {
                    pre class=(format!("sh-pre {}", language_class)) {
                        @if self.show_line_numbers {
                            div class="sh-line-numbers" aria-hidden="true" {
                                @for i in starting_line..(starting_line + line_count as u32) {
                                    span class="sh-line-number" { (i) }
                                }
                            }
                        }

                        code class=(format!("sh-block-code {}", language_class)) {
                            @for (i, line) in lines.iter().enumerate() {
                                @let line_num = starting_line + i as u32;
                                @let is_highlighted = self.highlight_lines.contains(&line_num);
                                @let escaped_line = Self::escape_html_static(line);

                                span
                                    class=(if is_highlighted { "sh-line sh-line--highlighted" } else { "sh-line" })
                                    data-line-number=(line_num)
                                {
                                    (maud::PreEscaped(escaped_line))
                                    @if i < lines.len() - 1 {
                                        "\n"
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

/// Inline code component
#[derive(Debug, Clone)]
pub struct InlineCode {
    code: String,
    language: Option<Language>,
}

impl InlineCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: None,
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    fn escape_html(&self) -> String {
        self.code
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    pub fn render(self) -> Markup {
        let class = self
            .language
            .map(|l| format!("sh-inline-code {}", l.css_class()))
            .unwrap_or_else(|| "sh-inline-code".to_string());

        html! {
            code class=(class) {
                (self.escape_html())
            }
        }
    }
}

/// Command palette item for displaying CLI commands
#[derive(Debug, Clone)]
pub struct CommandItem {
    command: String,
    description: Option<String>,
    platform: Option<Platform>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
    Universal,
}

impl Platform {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::MacOs => "macOS",
            Self::Linux => "Linux",
            Self::Universal => "All Platforms",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Windows => "🪟",
            Self::MacOs => "🍎",
            Self::Linux => "🐧",
            Self::Universal => "🌐",
        }
    }
}

impl CommandItem {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            description: None,
            platform: None,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn render(&self) -> Markup {
        html! {
            div class="sh-command-item" {
                div class="sh-command-header" {
                    code class="sh-command-code" {
                        (&self.command)
                    }
                    @if let Some(ref platform) = self.platform {
                        span class="sh-command-platform" title=(platform.display()) {
                            (platform.icon())
                        }
                    }
                }
                @if let Some(ref desc) = self.description {
                    p class="sh-command-description" {
                        (desc)
                    }
                }
            }
        }
    }
}

/// Generate CSS for code block components
pub fn code_block_css() -> String {
    r#"
/* Code Block Component Styles */

.sh-code-block-container {
    margin: 1.5rem 0;
    border-radius: 0.5rem;
    overflow: hidden;
    background: var(--sh-surface-alt);
}

.sh-code-block--card {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    border: 1px solid var(--sh-border);
}

.sh-code-block--minimal {
    background: transparent;
    border-radius: 0;
}

.sh-code-block--minimal .sh-code-header {
    display: none;
}

.sh-code-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    background: var(--sh-surface);
    border-bottom: 1px solid var(--sh-border);
}

.sh-code-header-left,
.sh-code-header-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
}

.sh-code-filename {
    font-family: var(--sh-font-mono);
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--sh-text);
}

.sh-code-language {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--sh-text-muted);
    background: var(--sh-surface-alt);
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
}

.sh-code-copy-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
    color: var(--sh-text-muted);
    background: var(--sh-surface-alt);
    border: 1px solid var(--sh-border);
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.2s;
}

.sh-code-copy-btn:hover {
    background: var(--sh-surface);
    color: var(--sh-text);
}

.sh-code-content {
    overflow-x: auto;
    overflow-y: auto;
}

.sh-code-block--wrap .sh-pre {
    white-space: pre-wrap;
    word-break: break-all;
}

.sh-pre {
    display: flex;
    margin: 0;
    padding: 1rem;
    font-family: var(--sh-font-mono);
    font-size: 0.875rem;
    line-height: 1.6;
    background: var(--sh-surface-alt);
}

.sh-line-numbers {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    padding-right: 1rem;
    margin-right: 1rem;
    border-right: 1px solid var(--sh-border);
    color: var(--sh-text-muted);
    text-align: right;
    user-select: none;
}

.sh-line-number {
    display: block;
    padding: 0 0.5rem;
}

.sh-block-code {
    display: block;
    flex: 1;
    min-width: 0;
}

.sh-line {
    display: block;
    padding: 0 0.25rem;
    border-radius: 0.125rem;
}

.sh-line--highlighted {
    background: var(--sh-accent-muted);
}

/* Inline Code */
.sh-inline-code {
    font-family: var(--sh-font-mono);
    font-size: 0.875em;
    padding: 0.125rem 0.375rem;
    background: var(--sh-surface-alt);
    border-radius: 0.25rem;
    color: var(--sh-accent);
}

/* Command Item */
.sh-command-item {
    padding: 0.75rem 1rem;
    border: 1px solid var(--sh-border);
    border-radius: 0.5rem;
    background: var(--sh-surface);
}

.sh-command-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
}

.sh-command-code {
    font-family: var(--sh-font-mono);
    font-size: 0.875rem;
    color: var(--sh-text);
}

.sh-command-platform {
    font-size: 0.875rem;
}

.sh-command-description {
    margin: 0.5rem 0 0;
    font-size: 0.875rem;
    color: var(--sh-text-muted);
}

/* Syntax Highlighting Colors (Basic) */
.sh-block-code .keyword { color: var(--sh-code-keyword, #c678dd); }
.sh-block-code .string { color: var(--sh-code-string, #98c379); }
.sh-block-code .comment { color: var(--sh-code-comment, #5c6370); font-style: italic; }
.sh-block-code .number { color: var(--sh-code-number, #d19a66); }
.sh-block-code .function { color: var(--sh-code-function, #61afef); }
.sh-block-code .operator { color: var(--sh-code-operator, #56b6c2); }

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
    .sh-code-copy-btn {
        transition: none;
    }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_block_basic() {
        let block = CodeBlock::new("fn main() {}")
            .language(Language::Rust)
            .filename("main.rs")
            .with_line_numbers();

        assert_eq!(block.language, Some(Language::Rust));
        assert_eq!(block.filename, Some("main.rs".to_string()));
        assert!(block.show_line_numbers);
    }

    #[test]
    fn test_code_block_highlight_lines() {
        let block = CodeBlock::new("line1\nline2\nline3")
            .highlight_lines(vec![1, 3])
            .starting_line_number(1);

        assert_eq!(block.highlight_lines, vec![1, 3]);
        assert_eq!(block.starting_line_number, 1);
    }

    #[test]
    fn test_language_display_names() {
        assert_eq!(Language::Rust.display_name(), "Rust");
        assert_eq!(Language::JavaScript.display_name(), "JavaScript");
        assert_eq!(Language::Cpp.display_name(), "C++");
        assert_eq!(Language::PlainText.display_name(), "Plain Text");
    }

    #[test]
    fn test_language_css_classes() {
        assert_eq!(Language::Rust.css_class(), "language-rust");
        assert_eq!(Language::TypeScript.css_class(), "language-typescript");
    }

    #[test]
    fn test_inline_code() {
        let code = InlineCode::new("let x = 5").language(Language::Rust);
        assert_eq!(code.code, "let x = 5");
        assert_eq!(code.language, Some(Language::Rust));
    }

    #[test]
    fn test_escape_html() {
        let block = CodeBlock::new("<div>Test & Demo</div>");
        let escaped = block.escape_html("<div>Test & Demo</div>");
        assert_eq!(escaped, "&lt;div&gt;Test &amp; Demo&lt;/div&gt;");
    }

    #[test]
    fn test_command_item() {
        let cmd = CommandItem::new("cargo build")
            .description("Build the project")
            .platform(Platform::Universal);

        assert_eq!(cmd.command, "cargo build");
        assert_eq!(cmd.description, Some("Build the project".to_string()));
        assert_eq!(cmd.platform, Some(Platform::Universal));
    }
}
