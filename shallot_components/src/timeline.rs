use maud::{html, Markup};

pub struct TimelineItem<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> TimelineItem<'a> {
    pub fn new(title: &'a str, body: &'a str) -> Self {
        Self { title, body }
    }
}

pub struct Timeline<'a> {
    pub items: Vec<TimelineItem<'a>>,
}

impl<'a> Timeline<'a> {
    pub fn new(items: Vec<TimelineItem<'a>>) -> Self {
        Self { items }
    }

    pub fn render(self) -> Markup {
        html! {
            ol class="sh-timeline" role="list" aria-label="Timeline" {
                @for it in self.items {
                    li class="sh-timeline__item" {
                        div class="sh-timeline__marker" aria-hidden="true" {}
                        div class="sh-timeline__content" {
                            div class="sh-timeline__title" { (it.title) }
                            div class="sh-timeline__body" { (it.body) }
                        }
                    }
                }
            }
        }
    }
}

/// Generate CSS for timeline component
pub fn timeline_css() -> String {
    r#"
.sh-timeline {
    position: relative;
    list-style: none;
    padding: 0;
    margin: 0;
    padding-left: 2rem;
}

.sh-timeline::before {
    content: '';
    position: absolute;
    left: 0.5rem;
    top: 0;
    bottom: 0;
    width: 2px;
    background: var(--sh-border, #e5e7eb);
}

.sh-timeline__item {
    position: relative;
    padding-bottom: 1.5rem;
}

.sh-timeline__marker {
    position: absolute;
    left: -2rem;
    top: 0.25rem;
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
    background: var(--sh-primary, #3b82f6);
    border: 2px solid var(--sh-surface, #fff);
    box-shadow: 0 0 0 2px var(--sh-primary, #3b82f6);
}

.sh-timeline__content {
    background: var(--sh-surface, #fff);
    padding: 1rem;
    border-radius: var(--sh-radius-md, 0.375rem);
    border: 1px solid var(--sh-border, #e5e7eb);
}

.sh-timeline__title {
    font-weight: 600;
    color: var(--sh-text, #1f2937);
    margin-bottom: 0.5rem;
}

.sh-timeline__body {
    font-size: 0.875rem;
    color: var(--sh-text-secondary, #4b5563);
    line-height: 1.5;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_item_creation() {
        let item = TimelineItem::new("Event", "Description");
        assert_eq!(item.title, "Event");
        assert_eq!(item.body, "Description");
    }

    #[test]
    fn test_timeline_creation() {
        let items = vec![TimelineItem::new("Step 1", "First step")];
        let timeline = Timeline::new(items);
        assert_eq!(timeline.items.len(), 1);
    }

    #[test]
    fn test_timeline_css() {
        let css = timeline_css();
        assert!(css.contains(".sh-timeline"));
        assert!(css.contains(".sh-timeline__marker"));
    }
}
