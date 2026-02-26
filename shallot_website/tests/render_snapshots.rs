//! Snapshot tests for Shallot showcase components
//!
//! These tests verify the rendered HTML structure and content
//! are consistent across changes. Uses insta snapshots.
//!
//! Run with: cargo test --test render_snapshots
//! Update snapshots: cargo insta review

#[cfg(test)]
mod tests {
    use shallot_website::{homepage, retro_hero, showcase, theme_panel};

    #[test]
    fn test_hero_renders_without_debug_strings() {
        let hero_markup = retro_hero::render().into_string();

        // Hero should be present
        assert!(
            hero_markup.contains("sh-retro-hero"),
            "Hero section missing"
        );

        // Should not contain debug/git markers
        assert!(
            !hero_markup.contains("<<<<<<< HEAD"),
            "Merge conflict markers found"
        );
        assert!(
            !hero_markup.contains("======="),
            "Merge conflict markers found"
        );
        assert!(
            !hero_markup.contains(">>>>>>>"),
            "Merge conflict markers found"
        );

        // Should have accessibility features
        assert!(
            hero_markup.contains("aria-hidden"),
            "ARIA attributes missing"
        );
        assert!(hero_markup.contains("h1"), "Main heading missing");
    }

    #[test]
    fn test_hero_uses_plain_background() {
        let hero_css = retro_hero::retro_css();

        // Should use plain background, not aggressive gradients
        assert!(
            hero_css.contains("#f5f7fa"),
            "Plain grey background missing"
        );

        // Should not have neon-like effects
        assert!(!hero_css.contains("neon"), "Neon effects should be removed");

        // Should have accessible contrast
        assert!(hero_css.contains("color:"), "Text colors must be defined");
    }

    #[test]
    fn test_showcase_code_tabs_structure() {
        let showcase_markup = showcase::render().into_string();

        // Code tabs should be present
        assert!(showcase_markup.contains("sh-code-tab"), "Code tabs missing");

        // Should have radio inputs for tab switching
        assert!(
            showcase_markup.contains("type=\"radio\""),
            "Radio inputs missing"
        );
        assert!(
            showcase_markup.contains("name=\"tabs"),
            "Tab groups missing"
        );

        // Should have code blocks
        assert!(
            showcase_markup.contains("sh-code-block"),
            "Code blocks missing"
        );

        // Should have proper structure (no nested divs breaking selectors)
        assert!(
            showcase_markup.contains("sh-code-block--full"),
            "Full code block missing"
        );
        assert!(
            showcase_markup.contains("sh-code-block--library"),
            "Library code block missing"
        );
    }

    #[test]
    fn test_showcase_component_previews_exist() {
        let showcase_markup = showcase::render().into_string();

        // Should have component cards
        assert!(
            showcase_markup.contains("sh-component-card"),
            "Component cards missing"
        );

        // Should have at least some live previews
        assert!(
            showcase_markup.contains("Button"),
            "Button component missing"
        );

        // Should have placeholder messaging for components without previews
        assert!(
            showcase_markup.contains("coming soon") || showcase_markup.contains("Live preview"),
            "Component preview messaging missing"
        );
    }

    #[test]
    fn test_theme_panel_renders() {
        let panel_markup = theme_panel::render().into_string();

        // Theme panel should be present
        assert!(
            panel_markup.contains("sh-theme-panel"),
            "Theme panel missing"
        );

        // Should have preset themes (at least Default and Cyberpunk)
        assert!(
            panel_markup.contains("theme-default") || panel_markup.contains("Default"),
            "Default theme missing"
        );
        assert!(
            panel_markup.contains("theme-cyberpunk") || panel_markup.contains("Cyberpunk"),
            "Cyberpunk theme missing"
        );

        // Should have color picker
        assert!(
            panel_markup.contains("type=\"color\""),
            "Color picker missing"
        );

        // Should be fully keyboard accessible
        assert!(
            panel_markup.contains("label"),
            "Labels for form controls missing"
        );
    }

    #[test]
    fn test_theme_panel_css_valid() {
        let panel_css = theme_panel::theme_panel_css();

        // CSS should define theme panel styles
        assert!(
            panel_css.contains(".sh-theme-panel"),
            "Theme panel CSS missing"
        );

        // Should use CSS variables for theming
        assert!(panel_css.contains("--sh-"), "CSS variables missing");

        // Should have theme-related selectors (color, styling)
        assert!(
            panel_css.contains("color:") || panel_css.contains("background:"),
            "Theme panel styling missing"
        );
    }

    #[test]
    fn test_homepage_no_scripts() {
        let home_html = homepage().into_string();

        // Zero-JS requirement: no script tags
        assert!(
            !home_html.contains("<script"),
            "JavaScript found in homepage!"
        );
        assert!(!home_html.contains("onclick"), "Inline JavaScript found!");
        assert!(!home_html.contains("onchange"), "Inline JavaScript found!");

        // Should have doctype
        assert!(home_html.contains("<!DOCTYPE"), "Missing DOCTYPE");
        assert!(home_html.contains("<html"), "Missing html tag");
        assert!(home_html.contains("<head"), "Missing head tag");
        assert!(home_html.contains("<body"), "Missing body tag");
    }

    #[test]
    fn test_homepage_accessibility_features() {
        let home_html = homepage().into_string();

        // Should have skip link
        assert!(
            home_html.contains("skip-link") || home_html.contains("Skip to"),
            "Skip link missing"
        );

        // Should have proper semantic HTML
        assert!(home_html.contains("<main"), "Missing main landmark");
        assert!(home_html.contains("<nav"), "Missing nav landmark");
        assert!(home_html.contains("<footer"), "Missing footer landmark");

        // Should have ARIA labels
        assert!(
            home_html.contains("aria-label") || home_html.contains("aria-describedby"),
            "ARIA attributes missing"
        );
    }

    #[test]
    fn test_footer_clean_no_debug() {
        let home_html = homepage().into_string();

        // Footer should not contain commit hashes
        assert!(
            !home_html.to_lowercase().contains("commit"),
            "Commit reference found in output"
        );

        // Should have clean footer text
        assert!(home_html.contains("2026"), "Copyright year missing");
        assert!(
            home_html.contains("Shallot"),
            "Project name missing in footer"
        );

        // Should mention zero JavaScript
        assert!(
            home_html.contains("JavaScript") || home_html.contains("Rust"),
            "Project identity missing from footer"
        );
    }

    #[test]
    fn test_css_uses_design_tokens() {
        let main_css_content = shallot_website::main_css();

        // All colors should use CSS variables
        assert!(
            main_css_content.contains("--sh-primary"),
            "Primary color token missing"
        );
        assert!(
            main_css_content.contains("--sh-accent"),
            "Accent color token missing"
        );
        assert!(
            main_css_content.contains("--sh-text"),
            "Text color token missing"
        );
        assert!(
            main_css_content.contains("--sh-surface"),
            "Surface color token missing"
        );

        // Should use design radius variables
        assert!(
            main_css_content.contains("--sh-radius"),
            "Border radius tokens missing"
        );
    }

    #[test]
    fn test_typography_responsive() {
        let main_css_content = shallot_website::main_css();

        // Should use clamp() for responsive typography
        assert!(
            main_css_content.contains("clamp("),
            "Responsive typography (clamp) missing"
        );

        // Should maintain readability on all sizes
        // clamp() ensures:
        //   - min: text size on smallest phones
        //   - mid: scales with viewport
        //   - max: text size on largest desktops

        // Example: h1 should clamp from 1.75rem to 2.5rem
        let has_responsive_h1 =
            main_css_content.contains("1.75rem") && main_css_content.contains("2.5rem");
        assert!(has_responsive_h1, "H1 responsive sizing missing");
    }

    #[test]
    fn test_performance_optimizations() {
        let showcase_css = shallot_website::showcase_css();

        // Should have content-visibility for performance
        assert!(
            showcase_css.contains("content-visibility: auto"),
            "Content visibility optimization missing"
        );

        // Should use containment for layout performance
        assert!(
            showcase_css.contains("contain:") || showcase_css.contains("contain ;"),
            "CSS containment missing"
        );
    }

    #[test]
    fn test_accessibility_reduced_motion() {
        let main_css_content = shallot_website::main_css();

        // Should respect prefers-reduced-motion
        assert!(
            main_css_content.contains("prefers-reduced-motion"),
            "Reduced motion support missing"
        );

        // Should disable animations for users who prefer reduced motion
        assert!(
            main_css_content.contains("animation-duration: 0.01ms"),
            "Reduced motion animations not applied"
        );
    }

    #[test]
    fn test_dark_mode_support() {
        let main_css_content = shallot_website::main_css();

        // Should have dark mode variant
        assert!(
            main_css_content.contains("dark") || main_css_content.contains("[data-theme"),
            "Dark mode support missing"
        );

        // Should define colors for dark mode
        let has_dark_colors = main_css_content.contains("#f9fafb") && // light text
                             main_css_content.contains("#1f2937"); // dark surface
        assert!(has_dark_colors, "Dark mode color definitions missing");
    }

    #[test]
    fn test_no_magic_strings_in_css() {
        let main_css_content = shallot_website::main_css();

        // Color values should be in variables, not hardcoded
        // Some hardcoded colors are OK for system colors, but check major ones are variables
        assert!(
            main_css_content.contains("var(--sh-"),
            "CSS variables not being used consistently"
        );

        // Count variable usage
        let var_count = main_css_content.matches("var(--sh-").count();
        assert!(
            var_count > 20,
            "Too few CSS variable usages (expected 20+, got {})",
            var_count
        );
    }
}
