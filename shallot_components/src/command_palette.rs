//! Command Palette Component - Keyboard-accessible command menu
//! CSS-only using details/summary and :focus-within

use maud::{html, Markup, Render};

/// Command item
#[derive(Debug, Clone)]
pub struct CommandItem<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub shortcut: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub category: Option<&'a str>,
    pub disabled: bool,
}

impl<'a> CommandItem<'a> {
    pub fn new(id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            label,
            shortcut: None,
            icon: None,
            category: None,
            disabled: false,
        }
    }

    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn category(mut self, category: &'a str) -> Self {
        self.category = Some(category);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn render(&self) -> Markup {
        let disabled_class = if self.disabled {
            " sh-command-item--disabled"
        } else {
            ""
        };

        html! {
            button
                type="button"
                class=(format!("sh-command-item{}", disabled_class))
                disabled?[self.disabled]
                role="option"
            {
                @if let Some(icon) = self.icon {
                    span class=(format!("sh-command-item__icon sh-icon--{}", icon)) {}
                }
                span class="sh-command-item__label" {
                    (self.label)
                }
                @if let Some(shortcut) = self.shortcut {
                    span class="sh-command-item__shortcut" {
                        (shortcut)
                    }
                }
            }
        }
    }
}

/// Command group
#[derive(Debug, Clone)]
pub struct CommandGroup<'a> {
    pub label: &'a str,
    pub items: Vec<CommandItem<'a>>,
}

impl<'a> CommandGroup<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            items: Vec::new(),
        }
    }

    pub fn add(mut self, item: CommandItem<'a>) -> Self {
        self.items.push(item);
        self
    }

    pub fn render(&self) -> Markup {
        html! {
            div class="sh-command-group" role="group" aria-label=(self.label) {
                div class="sh-command-group__label" {
                    (self.label)
                }
                div class="sh-command-group__items" role="listbox" {
                    @for item in &self.items {
                        (item.render())
                    }
                }
            }
        }
    }
}

/// Command Palette component
#[derive(Debug, Clone)]
pub struct CommandPalette<'a> {
    pub id: &'a str,
    pub placeholder: &'a str,
    pub groups: Vec<CommandGroup<'a>>,
    pub open: bool,
}

impl<'a> CommandPalette<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            placeholder: "Search commands...",
            groups: Vec::new(),
            open: false,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn groups(mut self, groups: Vec<CommandGroup<'a>>) -> Self {
        self.groups = groups;
        self
    }

    pub fn add_group(mut self, group: CommandGroup<'a>) -> Self {
        self.groups.push(group);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl<'a> Render for CommandPalette<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-command-palette" role="dialog" aria-modal="true" aria-label="Command palette" {
                div class="sh-command-palette__input-wrapper" {
                    span class="sh-command-palette__search-icon" {
                        "⌘"
                    }
                    input
                        type="text"
                        class="sh-command-palette__input"
                        placeholder=(self.placeholder)
                        aria-label="Search commands"
                        autocomplete="off";
                }

                div class="sh-command-palette__content" {
                    @for group in &self.groups {
                        (group.render())
                    }

                    @if self.groups.is_empty() {
                        div class="sh-command-palette__empty" {
                            "No commands found"
                        }
                    }
                }

                div class="sh-command-palette__footer" {
                    span class="sh-command-palette__hint" {
                        "↑↓ to navigate"
                    }
                    span class="sh-command-palette__hint" {
                        "↵ to select"
                    }
                    span class="sh-command-palette__hint" {
                        "esc to close"
                    }
                }
            }
        }
    }
}

pub fn command_palette_css() -> String {
    r#"
.sh-command-palette {
    width: 100%;
    max-width: 560px;
    background: var(--sh-color-surface, #ffffff);
    border-radius: var(--sh-radius-lg, 0.5rem);
    box-shadow: var(--sh-shadow-2xl, 0 25px 50px -12px rgba(0, 0, 0, 0.25));
    overflow: hidden;
}

.sh-command-palette__input-wrapper {
    display: flex;
    align-items: center;
    gap: var(--sh-spacing-sm, 0.5rem);
    padding: var(--sh-spacing-md, 1rem);
    border-bottom: 1px solid var(--sh-color-border, #e5e5e5);
}

.sh-command-palette__search-icon {
    color: var(--sh-color-text-muted, #666);
    font-size: 1.25rem;
}

.sh-command-palette__input {
    flex: 1;
    border: none;
    outline: none;
    font-size: var(--sh-font-size-md, 1rem);
    background: transparent;
    color: var(--sh-color-text, #1a1a1a);
}

.sh-command-palette__input::placeholder {
    color: var(--sh-color-text-muted, #666);
}

.sh-command-palette__content {
    max-height: 400px;
    overflow-y: auto;
    padding: var(--sh-spacing-sm, 0.5rem);
}

.sh-command-palette__empty {
    padding: var(--sh-spacing-xl, 2rem);
    text-align: center;
    color: var(--sh-color-text-muted, #666);
}

.sh-command-palette__footer {
    display: flex;
    gap: var(--sh-spacing-md, 1rem);
    padding: var(--sh-spacing-sm, 0.5rem) var(--sh-spacing-md, 1rem);
    border-top: 1px solid var(--sh-color-border, #e5e5e5);
    background: var(--sh-color-surface-muted, #f5f5f5);
}

.sh-command-palette__hint {
    font-size: var(--sh-font-size-xs, 0.75rem);
    color: var(--sh-color-text-muted, #666);
}

.sh-command-group {
    margin-bottom: var(--sh-spacing-sm, 0.5rem);
}

.sh-command-group__label {
    padding: var(--sh-spacing-xs, 0.25rem) var(--sh-spacing-sm, 0.5rem);
    font-size: var(--sh-font-size-xs, 0.75rem);
    font-weight: var(--sh-font-weight-semibold, 600);
    color: var(--sh-color-text-muted, #666);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.sh-command-group__items {
    display: flex;
    flex-direction: column;
    gap: var(--sh-spacing-xs, 0.25rem);
}

.sh-command-item {
    display: flex;
    align-items: center;
    gap: var(--sh-spacing-sm, 0.5rem);
    width: 100%;
    padding: var(--sh-spacing-sm, 0.5rem) var(--sh-spacing-md, 1rem);
    border: none;
    background: transparent;
    border-radius: var(--sh-radius-md, 0.375rem);
    cursor: pointer;
    text-align: left;
    transition: background-color 0.15s ease;
}

.sh-command-item:hover,
.sh-command-item:focus {
    background: var(--sh-color-surface-hover, #f0f0f0);
    outline: none;
}

.sh-command-item:focus-visible {
    box-shadow: inset 0 0 0 2px var(--sh-color-primary, #3b82f6);
}

.sh-command-item--disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

.sh-command-item__icon {
    flex-shrink: 0;
    color: var(--sh-color-text-muted, #666);
}

.sh-command-item__label {
    flex: 1;
    font-size: var(--sh-font-size-md, 1rem);
    color: var(--sh-color-text, #1a1a1a);
}

.sh-command-item__shortcut {
    font-size: var(--sh-font-size-xs, 0.75rem);
    color: var(--sh-color-text-muted, #666);
    font-family: monospace;
    padding: var(--sh-spacing-xs, 0.25rem) var(--sh-spacing-sm, 0.5rem);
    background: var(--sh-color-surface-muted, #e5e5e5);
    border-radius: var(--sh-radius-sm, 0.25rem);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_item_creation() {
        let item = CommandItem::new("save", "Save File")
            .shortcut("⌘S")
            .icon("save");

        assert_eq!(item.id, "save");
        assert_eq!(item.label, "Save File");
        assert_eq!(item.shortcut, Some("⌘S"));
    }

    #[test]
    fn test_command_item_render() {
        let item = CommandItem::new("test", "Test Command");
        let html = item.render().into_string();

        assert!(html.contains("sh-command-item"));
        assert!(html.contains("Test Command"));
    }

    #[test]
    fn test_command_group() {
        let group = CommandGroup::new("File")
            .add(CommandItem::new("new", "New File"))
            .add(CommandItem::new("open", "Open File"));

        assert_eq!(group.items.len(), 2);
    }

    #[test]
    fn test_command_palette_creation() {
        let palette = CommandPalette::new("cmd").placeholder("Type a command...");

        assert_eq!(palette.id, "cmd");
        assert_eq!(palette.placeholder, "Type a command...");
    }

    #[test]
    fn test_command_palette_render() {
        let palette = CommandPalette::new("palette")
            .add_group(CommandGroup::new("Actions").add(CommandItem::new("a1", "Action 1")));

        let html = palette.render().into_string();

        assert!(html.contains("sh-command-palette"));
        assert!(html.contains("Actions"));
    }

    #[test]
    fn test_css_generation() {
        let css = command_palette_css();
        assert!(css.contains(".sh-command-palette"));
        assert!(css.contains(".sh-command-item"));
    }
}
