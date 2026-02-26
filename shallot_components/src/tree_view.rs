//! Tree View Component - Collapsible hierarchical tree structure
//! Uses CSS-only details/summary for expand/collapse

use maud::{html, Markup, Render};

/// A node in the tree view
#[derive(Debug, Clone)]
pub struct TreeNode<'a> {
    /// Node label
    pub label: &'a str,
    /// Node value/ID
    pub value: &'a str,
    /// Child nodes
    pub children: Vec<TreeNode<'a>>,
    /// Is expanded by default
    pub expanded: bool,
    /// Is selected
    pub selected: bool,
    /// Icon name
    pub icon: Option<&'a str>,
}

impl<'a> TreeNode<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            children: Vec::new(),
            expanded: false,
            selected: false,
            icon: None,
        }
    }

    pub fn children(mut self, children: Vec<TreeNode<'a>>) -> Self {
        self.children = children;
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn render(&self) -> Markup {
        let has_children = !self.children.is_empty();
        let selected_class = if self.selected {
            " sh-tree-node--selected"
        } else {
            ""
        };

        if has_children {
            html! {
                li class=(format!("sh-tree-node{}", selected_class)) {
                    details open?[self.expanded] {
                        summary class="sh-tree-node__header" {
                            span class="sh-tree-node__toggle" {}
                            @if let Some(icon) = self.icon {
                                span class=(format!("sh-tree-node__icon sh-icon--{}", icon)) {}
                            }
                            span class="sh-tree-node__label" {
                                (self.label)
                            }
                        }
                        ul class="sh-tree-node__children" role="group" {
                            @for child in &self.children {
                                (child.render())
                            }
                        }
                    }
                }
            }
        } else {
            html! {
                li class=(format!("sh-tree-node sh-tree-node--leaf{}", selected_class)) {
                    div class="sh-tree-node__header" {
                        span class="sh-tree-node__spacer" {}
                        @if let Some(icon) = self.icon {
                            span class=(format!("sh-tree-node__icon sh-icon--{}", icon)) {}
                        }
                        span class="sh-tree-node__label" {
                            (self.label)
                        }
                    }
                }
            }
        }
    }
}

/// Tree view size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeViewSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Tree view component
#[derive(Debug, Clone)]
pub struct TreeView<'a> {
    /// Tree nodes
    pub nodes: Vec<TreeNode<'a>>,
    /// Size variant
    pub size: TreeViewSize,
    /// Accessibility label
    pub label: &'a str,
}

impl<'a> TreeView<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            nodes: Vec::new(),
            size: TreeViewSize::default(),
            label,
        }
    }

    pub fn nodes(mut self, nodes: Vec<TreeNode<'a>>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn size(mut self, size: TreeViewSize) -> Self {
        self.size = size;
        self
    }

    fn build_classes(&self) -> String {
        let size_class = match self.size {
            TreeViewSize::Sm => "sh-tree-view--sm",
            TreeViewSize::Md => "sh-tree-view--md",
            TreeViewSize::Lg => "sh-tree-view--lg",
        };

        format!("sh-tree-view {}", size_class)
    }
}

impl<'a> Render for TreeView<'a> {
    fn render(&self) -> Markup {
        let classes = self.build_classes();

        html! {
            nav class=(classes) role="tree" aria-label=(self.label) {
                ul class="sh-tree-view__root" role="group" {
                    @for node in &self.nodes {
                        (node.render())
                    }
                }
            }
        }
    }
}

pub fn tree_view_css() -> String {
    r#"
.sh-tree-view {
    font-size: var(--sh-font-size-md, 1rem);
    line-height: 1.5;
}

.sh-tree-view--sm {
    font-size: var(--sh-font-size-sm, 0.875rem);
}

.sh-tree-view--lg {
    font-size: var(--sh-font-size-lg, 1.125rem);
}

.sh-tree-view__root,
.sh-tree-node__children {
    list-style: none;
    margin: 0;
    padding: 0;
}

.sh-tree-node__children {
    padding-left: var(--sh-spacing-lg, 1.5rem);
}

.sh-tree-node__header {
    display: flex;
    align-items: center;
    gap: var(--sh-spacing-xs, 0.25rem);
    padding: var(--sh-spacing-xs, 0.25rem) var(--sh-spacing-sm, 0.5rem);
    border-radius: var(--sh-radius-sm, 0.25rem);
    cursor: pointer;
    transition: background-color 0.15s ease;
}

.sh-tree-node__header:hover {
    background: var(--sh-color-surface-hover, #f0f0f0);
}

.sh-tree-node--selected > .sh-tree-node__header {
    background: var(--sh-color-primary-alpha, rgba(59, 130, 246, 0.1));
    color: var(--sh-color-primary, #3b82f6);
}

.sh-tree-node__toggle {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
}

.sh-tree-node__toggle::before {
    content: "▶";
    font-size: 0.625rem;
    transition: transform 0.15s ease;
}

details[open] > .sh-tree-node__header .sh-tree-node__toggle::before {
    transform: rotate(90deg);
}

.sh-tree-node--leaf .sh-tree-node__spacer {
    width: 16px;
}

.sh-tree-node__icon {
    flex-shrink: 0;
    color: var(--sh-color-text-muted, #666);
}

.sh-tree-node__label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.sh-tree-node details {
    margin: 0;
}

.sh-tree-node details summary {
    list-style: none;
}

.sh-tree-node details summary::-webkit-details-marker {
    display: none;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_creation() {
        let node = TreeNode::new("Root", "root").expanded(true).selected(false);

        assert_eq!(node.label, "Root");
        assert_eq!(node.value, "root");
        assert!(node.expanded);
    }

    #[test]
    fn test_tree_node_with_children() {
        let child = TreeNode::new("Child", "child");
        let parent = TreeNode::new("Parent", "parent").children(vec![child]);

        assert_eq!(parent.children.len(), 1);
    }

    #[test]
    fn test_tree_view_creation() {
        let tree = TreeView::new("File browser").size(TreeViewSize::Sm);

        assert_eq!(tree.label, "File browser");
        assert_eq!(tree.size, TreeViewSize::Sm);
    }

    #[test]
    fn test_tree_view_render() {
        let tree = TreeView::new("Test tree").nodes(vec![TreeNode::new("Root", "root")]);

        let rendered = tree.render();
        let html = rendered.into_string();

        assert!(html.contains("sh-tree-view"));
        assert!(html.contains("Root"));
    }

    #[test]
    fn test_css_generation() {
        let css = tree_view_css();
        assert!(css.contains(".sh-tree-view"));
        assert!(css.contains(".sh-tree-node"));
    }
}
