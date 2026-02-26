//! Table Component
//!
//! Data tables with sorting, selection, and responsive design.

use crate::component::ComponentSize;
use maud::{html, Markup, Render};

pub struct TableColumn<'a> {
    pub key: &'a str,
    pub label: &'a str,
    pub sortable: bool,
    pub width: Option<&'a str>,
    pub align: ColumnAlign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl ColumnAlign {
    fn class(&self) -> &'static str {
        match self {
            ColumnAlign::Left => "sh-table__cell--left",
            ColumnAlign::Center => "sh-table__cell--center",
            ColumnAlign::Right => "sh-table__cell--right",
        }
    }
}

pub struct TableRow<'a> {
    pub cells: Vec<Markup>,
    pub id: Option<&'a str>,
    pub selected: bool,
    pub disabled: bool,
    pub href: Option<&'a str>,
}

pub struct Table<'a> {
    columns: Vec<TableColumn<'a>>,
    rows: Vec<TableRow<'a>>,
    caption: Option<&'a str>,
    size: ComponentSize,
    variant: TableVariant,
    striped: bool,
    hoverable: bool,
    bordered: bool,
    compact: bool,
    sticky_header: bool,
    sort_key: Option<&'a str>,
    sort_dir: Option<SortDir>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableVariant {
    #[default]
    Default,
    Bordered,
    Borderless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

impl<'a> Table<'a> {
    pub fn new(columns: Vec<TableColumn<'a>>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
            caption: None,
            size: ComponentSize::Md,
            variant: TableVariant::Default,
            striped: false,
            hoverable: true,
            bordered: false,
            compact: false,
            sticky_header: false,
            sort_key: None,
            sort_dir: None,
        }
    }

    pub fn rows(mut self, rows: Vec<TableRow<'a>>) -> Self {
        self.rows = rows;
        self
    }

    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: TableVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn sticky_header(mut self, sticky: bool) -> Self {
        self.sticky_header = sticky;
        self
    }

    pub fn sorted(mut self, key: &'a str, dir: SortDir) -> Self {
        self.sort_key = Some(key);
        self.sort_dir = Some(dir);
        self
    }

    fn build_classes(&self) -> String {
        let mut classes = vec!["sh-table".to_string()];

        classes.push(format!("sh-table--{}", self.size.class_suffix()));

        if self.striped {
            classes.push("sh-table--striped".to_string());
        }
        if self.hoverable {
            classes.push("sh-table--hover".to_string());
        }
        if self.bordered {
            classes.push("sh-table--bordered".to_string());
        }
        if self.compact {
            classes.push("sh-table--compact".to_string());
        }
        if self.sticky_header {
            classes.push("sh-table--sticky".to_string());
        }

        classes.join(" ")
    }
}

impl<'a> Render for Table<'a> {
    fn render(&self) -> Markup {
        let class = self.build_classes();

        html! {
            div class="sh-table-wrapper" role="region" aria-label=(self.caption.unwrap_or("Data table")) {
                table
                    class=(class)
                    role="table"
                    aria-label=(self.caption.unwrap_or("Data table"))
                {
                    @if let Some(caption) = self.caption {
                        caption class="sh-table__caption" { (caption) }
                    }

                    thead class="sh-table__head" {
                        tr {
                            @for col in &self.columns {
                                @let header_class = self.build_header_class(col);
                                th
                                    class=(header_class)
                                    data-key=(col.key)
                                    data-sortable=[if col.sortable { Some("true") } else { None }]
                                    style=[col.width.map(|w| format!("width: {}", w))]
                                    scope="col"
                                    aria-sort=[if self.sort_key == Some(col.key) {
                                        Some(match self.sort_dir {
                                            Some(SortDir::Asc) => "ascending",
                                            Some(SortDir::Desc) => "descending",
                                            None => "none",
                                        })
                                    } else { None }]
                                {
                                    (col.label)
                                    @if self.sort_key == Some(col.key) {
                                        span class="sh-table__sort-icon" aria-hidden="true" {
                                            @match self.sort_dir {
                                                Some(SortDir::Asc) => " \u{2191}",
                                                Some(SortDir::Desc) => " \u{2193}",
                                                None => "",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    tbody class="sh-table__body" {
                        @for row in &self.rows {
                            @let row_classes = self.build_row_class(row);
                            tr
                                class=(row_classes)
                                data-id=[row.id]
                            {
                                @for cell in &row.cells {
                                    td class="sh-table__cell" {
                                        (cell.clone())
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

impl<'a> Table<'a> {
    fn build_header_class(&self, col: &TableColumn<'a>) -> String {
        let mut cls = vec![
            "sh-table__header".to_string(),
            col.align.class().to_string(),
        ];
        if col.sortable {
            cls.push("sh-table__header--sortable".to_string());
        }
        if self.sort_key == Some(col.key) {
            cls.push("sh-table__header--sorted".to_string());
        }
        cls.join(" ")
    }

    fn build_row_class(&self, row: &TableRow<'a>) -> String {
        let mut cls = vec!["sh-table__row".to_string()];
        if row.selected {
            cls.push("sh-table__row--selected".to_string());
        }
        if row.disabled {
            cls.push("sh-table__row--disabled".to_string());
        }
        if row.href.is_some() {
            cls.push("sh-table__row--clickable".to_string());
        }
        cls.join(" ")
    }
}

pub struct TableFooter {
    pub content: Markup,
    pub colspan: u32,
}

impl Render for TableFooter {
    fn render(&self) -> Markup {
        html! {
            tfoot class="sh-table__foot" {
                tr {
                    td colspan=(self.colspan) class="sh-table__footer-cell" {
                        (self.content.clone())
                    }
                }
            }
        }
    }
}

pub fn table_css() -> String {
    r#"
.sh-table-wrapper {
    width: 100%;
    overflow-x: auto;
    border-radius: var(--sh-radius-md, 0.5rem);
}

.sh-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
    background: var(--sh-surface, #fff);
}

.sh-table__caption {
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    font-weight: 600;
    text-align: left;
    color: var(--sh-text, #1f2937);
    caption-side: top;
}

.sh-table__head {
    background: var(--sh-surface-2, #f9fafb);
}

.sh-table__header {
    padding: 0.75rem 1rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--sh-text-muted, #6b7280);
    text-align: left;
    border-bottom: 2px solid var(--sh-border, #e5e7eb);
}

.sh-table__header--sortable {
    cursor: pointer;
    user-select: none;
}

.sh-table__header--sortable:hover {
    color: var(--sh-text, #1f2937);
}

.sh-table__header--sorted {
    color: var(--sh-accent, #3b82f6);
}

.sh-table__cell {
    padding: 0.75rem 1rem;
    color: var(--sh-text, #1f2937);
    border-bottom: 1px solid var(--sh-border, #e5e7eb);
    vertical-align: middle;
}

.sh-table__cell--center {
    text-align: center;
}

.sh-table__cell--right {
    text-align: right;
}

.sh-table__row:hover {
    background: var(--sh-surface-hover, #f3f4f6);
}

.sh-table__row--selected {
    background: color-mix(in srgb, var(--sh-accent, #3b82f6) 10%, transparent);
}

.sh-table__row--disabled {
    opacity: 0.5;
    pointer-events: none;
}

.sh-table__row--clickable {
    cursor: pointer;
}

/* Variants */
.sh-table--striped .sh-table__row:nth-child(even) {
    background: var(--sh-surface-2, #f9fafb);
}

.sh-table--bordered {
    border: 1px solid var(--sh-border, #e5e7eb);
}

.sh-table--bordered .sh-table__cell,
.sh-table--bordered .sh-table__header {
    border: 1px solid var(--sh-border, #e5e7eb);
}

.sh-table--compact .sh-table__cell,
.sh-table--compact .sh-table__header {
    padding: 0.5rem 0.75rem;
}

.sh-table--sticky .sh-table__head {
    position: sticky;
    top: 0;
    z-index: 10;
}

/* Size variants */
.sh-table--sm .sh-table__cell,
.sh-table--sm .sh-table__header {
    padding: 0.5rem 0.75rem;
    font-size: 0.8125rem;
}

.sh-table--lg .sh-table__cell,
.sh-table--lg .sh-table__header {
    padding: 1rem 1.25rem;
    font-size: 1rem;
}

/* Responsive */
@media (max-width: 640px) {
    .sh-table__header,
    .sh-table__cell {
        padding: 0.5rem 0.75rem;
    }
}

.sh-table__sort-icon {
    font-size: 0.75rem;
    margin-left: 0.25rem;
}

.sh-table__footer-cell {
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    font-weight: 500;
    background: var(--sh-surface-2, #f9fafb);
    border-top: 2px solid var(--sh-border, #e5e7eb);
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let columns = vec![
            TableColumn {
                key: "name",
                label: "Name",
                sortable: true,
                width: None,
                align: ColumnAlign::Left,
            },
            TableColumn {
                key: "email",
                label: "Email",
                sortable: false,
                width: None,
                align: ColumnAlign::Left,
            },
        ];

        let table = Table::new(columns).caption("Users").striped(true);

        assert_eq!(table.columns.len(), 2);
        assert!(table.striped);
    }

    #[test]
    fn test_table_classes() {
        let columns = vec![TableColumn {
            key: "id",
            label: "ID",
            sortable: false,
            width: None,
            align: ColumnAlign::Left,
        }];

        let table = Table::new(columns)
            .striped(true)
            .hoverable(true)
            .compact(true);

        let classes = table.build_classes();
        assert!(classes.contains("sh-table--striped"));
        assert!(classes.contains("sh-table--hover"));
        assert!(classes.contains("sh-table--compact"));
    }
}
