use maud::{html, Markup, Render};

pub struct Avatar<'a> {
    pub src: Option<&'a str>,
    pub alt: &'a str,
    pub initials: &'a str,
}

impl<'a> Render for Avatar<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-avatar" {
                @if let Some(src) = self.src {
                    img src=(src) alt=(self.alt) loading="lazy";
                } @else {
                    (self.initials)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Primary,
    Secondary,
    Outline,
}

pub struct Badge<'a> {
    pub label: &'a str,
    pub variant: BadgeVariant,
}

impl<'a> Render for Badge<'a> {
    fn render(&self) -> Markup {
        let class = match self.variant {
            BadgeVariant::Primary => "sh-badge sh-badge--primary",
            BadgeVariant::Secondary => "sh-badge sh-badge--secondary",
            BadgeVariant::Outline => "sh-badge sh-badge--outline",
        };
        html! {
            span class=(class) { (self.label) }
        }
    }
}

pub struct Chip<'a> {
    pub label: &'a str,
    pub filled: bool,
    pub on_click: Option<&'a str>, // Just a href for now or JS handler string if we allowed JS
}

impl<'a> Render for Chip<'a> {
    fn render(&self) -> Markup {
        let class = if self.filled {
            "sh-chip sh-chip--filled"
        } else {
            "sh-chip"
        };
        html! {
            button type="button" class=(class) { (self.label) }
        }
    }
}

pub struct Table<'a> {
    pub headers: Vec<&'a str>,
    pub rows: Vec<Vec<Markup>>,
}

impl<'a> Render for Table<'a> {
    fn render(&self) -> Markup {
        html! {
            div class="sh-table-wrapper" {
                table class="sh-table" {
                    thead {
                        tr {
                            @for h in &self.headers {
                                th { (h) }
                            }
                        }
                    }
                    tbody {
                        @for row in &self.rows {
                            tr {
                                @for cell in row {
                                    td { (cell) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct Typography<'a> {
    pub text: &'a str,
    pub variant: TypographyVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Body1,
    Body2,
    Caption,
}

impl<'a> Render for Typography<'a> {
    fn render(&self) -> Markup {
        match self.variant {
            TypographyVariant::H1 => html! { h1 class="sh-h1" { (self.text) } },
            TypographyVariant::H2 => html! { h2 class="sh-h2" { (self.text) } },
            TypographyVariant::H3 => html! { h3 class="sh-h3" { (self.text) } },
            TypographyVariant::H4 => html! { h4 class="sh-h4" { (self.text) } },
            TypographyVariant::H5 => html! { h5 class="sh-h5" { (self.text) } },
            TypographyVariant::H6 => html! { h6 class="sh-h6" { (self.text) } },
            TypographyVariant::Body1 => html! { p class="sh-body1" { (self.text) } },
            TypographyVariant::Body2 => html! { p class="sh-body2" { (self.text) } },
            TypographyVariant::Caption => html! { span class="sh-caption" { (self.text) } },
        }
    }
}

/// Generate CSS for data display components
pub fn data_display_css() -> String {
    r#"
/* Avatar */
.sh-avatar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    background: var(--sh-surface-2, #f3f4f6);
    font-weight: 500;
    font-size: 0.875rem;
    overflow: hidden;
}

.sh-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

/* Badge */
.sh-badge {
    display: inline-block;
    padding: 0.25rem 0.625rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 500;
}

.sh-badge--primary {
    background: var(--sh-primary, #3b82f6);
    color: white;
}

.sh-badge--secondary {
    background: var(--sh-surface-2, #f3f4f6);
    color: var(--sh-text, #1f2937);
}

.sh-badge--outline {
    border: 1px solid var(--sh-border, #e5e7eb);
    color: var(--sh-text, #1f2937);
}

/* Chip */
.sh-chip {
    display: inline-flex;
    align-items: center;
    padding: 0.375rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.875rem;
    background: var(--sh-surface-2, #f3f4f6);
    border: 1px solid var(--sh-border, #e5e7eb);
    cursor: pointer;
    transition: all 0.2s ease;
}

.sh-chip--filled {
    background: var(--sh-primary, #3b82f6);
    color: white;
    border-color: var(--sh-primary, #3b82f6);
}

.sh-chip:hover {
    opacity: 0.9;
}

/* Table */
.sh-table-wrapper {
    overflow-x: auto;
}

.sh-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
}

.sh-table th,
.sh-table td {
    padding: 0.75rem 1rem;
    text-align: left;
    border-bottom: 1px solid var(--sh-border, #e5e7eb);
}

.sh-table th {
    font-weight: 600;
    background: var(--sh-surface-2, #f9fafb);
}

.sh-table tbody tr:hover {
    background: var(--sh-surface-hover, #f3f4f6);
}

/* Typography */
.sh-h1 { font-size: 2.25rem; font-weight: 700; line-height: 1.2; }
.sh-h2 { font-size: 1.875rem; font-weight: 600; line-height: 1.3; }
.sh-h3 { font-size: 1.5rem; font-weight: 600; line-height: 1.4; }
.sh-h4 { font-size: 1.25rem; font-weight: 600; line-height: 1.4; }
.sh-h5 { font-size: 1.125rem; font-weight: 500; line-height: 1.5; }
.sh-h6 { font-size: 1rem; font-weight: 500; line-height: 1.5; }
.sh-body1 { font-size: 1rem; line-height: 1.6; }
.sh-body2 { font-size: 0.875rem; line-height: 1.5; }
.sh-caption { font-size: 0.75rem; color: var(--sh-text-muted, #6b7280); }
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_creation() {
        let avatar = Avatar {
            src: None,
            alt: "User",
            initials: "JD",
        };
        assert_eq!(avatar.alt, "User");
        assert_eq!(avatar.initials, "JD");
    }

    #[test]
    fn test_badge_variants() {
        let badge = Badge {
            label: "New",
            variant: BadgeVariant::Primary,
        };
        assert_eq!(badge.variant, BadgeVariant::Primary);
    }

    #[test]
    fn test_chip_filled() {
        let chip = Chip {
            label: "Filter",
            filled: true,
            on_click: None,
        };
        assert!(chip.filled);
    }

    #[test]
    fn test_typography_variants() {
        let h1 = Typography {
            text: "Title",
            variant: TypographyVariant::H1,
        };
        assert_eq!(h1.variant, TypographyVariant::H1);
    }

    #[test]
    fn test_data_display_css() {
        let css = data_display_css();
        assert!(css.contains(".sh-avatar"));
        assert!(css.contains(".sh-badge"));
        assert!(css.contains(".sh-chip"));
    }
}
