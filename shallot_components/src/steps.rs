use maud::{html, Markup};

pub struct Steps<'a> {
    pub items: Vec<&'a str>,
    pub completed: usize,
}

impl<'a> Steps<'a> {
    pub fn new(items: Vec<&'a str>) -> Self {
        Self {
            items,
            completed: 0,
        }
    }

    pub fn completed(mut self, completed: usize) -> Self {
        self.completed = completed;
        self
    }

    pub fn render(self) -> Markup {
        html! {
            ol class="sh-steps" role="list" aria-label="Progress steps" {
                @for (idx, label) in self.items.iter().enumerate() {
                    li
                        class={(if idx < self.completed {"sh-step sh-step--done"} else {"sh-step"})}
                        aria-current={(if idx == self.completed { "step" } else { "" })}
                    {
                        span class="sh-step__dot" aria-hidden="true" {}
                        span class="sh-step__label" { (label) }
                    }
                }
            }
        }
    }
}

/// Generate CSS for steps component
pub fn steps_css() -> String {
    r#"
.sh-steps {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0;
    gap: 1rem;
}

.sh-step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    position: relative;
    flex: 1;
}

.sh-step__dot {
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    background: var(--sh-surface-2, #f3f4f6);
    border: 2px solid var(--sh-border, #e5e7eb);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--sh-text-muted, #6b7280);
    transition: all 0.2s ease;
}

.sh-step--done .sh-step__dot {
    background: var(--sh-primary, #3b82f6);
    border-color: var(--sh-primary, #3b82f6);
    color: white;
}

.sh-step__label {
    font-size: 0.875rem;
    color: var(--sh-text, #1f2937);
    text-align: center;
}

.sh-step--done .sh-step__label {
    color: var(--sh-primary, #3b82f6);
    font-weight: 500;
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps_creation() {
        let steps = Steps::new(vec!["Step 1", "Step 2", "Step 3"]);
        assert_eq!(steps.items.len(), 3);
        assert_eq!(steps.completed, 0);
    }

    #[test]
    fn test_steps_completed() {
        let steps = Steps::new(vec!["Step 1", "Step 2"]).completed(2);
        assert_eq!(steps.completed, 2);
    }

    #[test]
    fn test_steps_css() {
        let css = steps_css();
        assert!(css.contains(".sh-steps"));
        assert!(css.contains(".sh-step__dot"));
    }
}
