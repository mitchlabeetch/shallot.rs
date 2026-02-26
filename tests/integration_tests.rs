#[cfg(test)]
mod tests {
    use shallot_components::{
        enhanced_button::*, enhanced_modal::*,
        button::Button, button::ButtonSize, button::ButtonVariant,
    };
    use shallot_foundation::{
        DesignTokens, HSLColor, ColorScheme, Breakpoint,
        fade_in, slide_in_up, bounce_in,
    };
    use shallot_testing::{
        TestCase, TestSuite, TestRunner,
        accessibility, performance, visual,
    };
    use maud::html;

    #[test]
    fn test_enhanced_button_creation() {
        let button = EnhancedButton::new("Test Button")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Md)
            .shape(ButtonShape::Pill);

        // Test that button is created successfully
        assert_eq!(button.label, "Test Button");
        assert_eq!(button.variant, ButtonVariant::Primary);
        assert_eq!(button.size, ButtonSize::Md);
        assert_eq!(button.shape, ButtonShape::Pill);
    }

    #[test]
    fn test_design_tokens_generation() {
        let tokens = DesignTokens::new(
            HSLColor::new(250.0, 60.0, 50.0),
            ColorScheme::Monochromatic,
        );

        let css_vars = tokens.to_css_variables();
        
        // Test that essential CSS variables are generated
        assert!(css_vars.contains_key("--sh-color-primary"));
        assert!(css_vars.contains_key("--sh-color-secondary"));
        assert!(css_vars.contains_key("--sh-color-accent"));
        assert!(css_vars.contains_key("--sh-gradient-primary"));
    }

    #[test]
    fn test_animation_generation() {
        let fade_animation = fade_in();
        let css = fade_animation.to_css();
        
        // Test that CSS contains keyframe definition
        assert!(css.contains("@keyframes sh-fade-in"));
        assert!(css.contains("opacity: 0"));
        assert!(css.contains("opacity: 1"));
    }

    #[test]
    fn test_breakpoint_system() {
        let mobile = Breakpoint::Xs;
        let tablet = Breakpoint::Md;
        let desktop = Breakpoint::Lg;

        assert_eq!(mobile.min_width(), 0);
        assert_eq!(tablet.min_width(), 768);
        assert_eq!(desktop.min_width(), 1024);

        let media_query = tablet.media_query();
        assert!(media_query.contains("(min-width: 768px)"));
    }

    #[test]
    fn test_color_palette_generation() {
        let primary = HSLColor::new(200.0, 50.0, 50.0);
        let palette = ColorPalette::from_primary(primary, ColorScheme::Complementary);

        // Test that secondary color is different from primary
        assert_ne!(palette.primary, palette.secondary);
        
        // Test that colors are properly generated
        assert!(palette.success.l > 30.0); // Success should be reasonably light
        assert!(palette.error.l < 60.0);  // Error should be reasonably dark
    }

    #[test]
    fn test_enhanced_modal_creation() {
        let content = html! { p { "Modal content" } };
        let modal = EnhancedModal::new("test-modal", content)
            .title("Test Modal")
            .size(ModalSize::Md)
            .position(ModalPosition::Center)
            .backdrop(BackdropStyle::Blur);

        assert_eq!(modal.id, "test-modal");
        assert_eq!(modal.size, ModalSize::Md);
        assert_eq!(modal.position, ModalPosition::Center);
        assert_eq!(modal.backdrop, BackdropStyle::Blur);
    }

    #[test]
    fn test_accessibility_compliance() {
        // Test color contrast
        let contrast_result = accessibility::test_color_contrast("#000000", "#ffffff", 4.5);
        assert_eq!(contrast_result, TestResult::Pass);

        // Test ARIA label presence
        let aria_result = accessibility::test_aria_label("button-id", "Submit Form");
        assert_eq!(aria_result, TestResult::Pass);

        // Test empty ARIA label
        let empty_aria_result = accessibility::test_aria_label("button-id", "");
        match empty_aria_result {
            TestResult::Fail(_) => assert!(true),
            _ => panic!("Expected failure for empty ARIA label"),
        }
    }

    #[test]
    fn test_performance_metrics() {
        // Test render time measurement
        let (result, duration) = performance::measure_render_time(|| {
            let button = EnhancedButton::new("Performance Test").render();
            let _html = button.into_string();
        });

        assert_eq!(result, TestResult::Pass);
        assert!(duration > 0); // Should measure some time

        // Test CSS size limits
        let large_css = "a".repeat(60000); // 60KB
        let size_result = performance::test_css_size(&large_css, 50000);
        match size_result {
            TestResult::Fail(_) => assert!(true),
            _ => panic!("Expected failure for oversized CSS"),
        }
    }

    #[test]
    fn test_integration_suite() {
        let mut runner = TestRunner::new();
        
        // Create component test suite
        let component_suite = TestSuite::new("Component Integration Tests", "Test component integration")
            .add_test(TestCase::new("Button Component", "Test button rendering")
                .with_tags(vec!["component", "button"]))
            .add_test(TestCase::new("Modal Component", "Test modal functionality")
                .with_tags(vec!["component", "modal"]))
            .add_test(TestCase::new("Animation System", "Test animation generation")
                .with_tags(vec!["animation", "css"]));

        runner = runner.add_suite(component_suite);

        // Run tests
        let results = runner.run();
        assert!(!results.is_empty());
        
        for result in &results {
            println!("{}", result.summary());
        }
    }

    #[test]
    fn test_responsive_value_system() {
        use shallot_foundation::ResponsiveValue;
        
        let responsive = ResponsiveValue::new(100)
            .with_sm(150)
            .with_md(200)
            .with_lg(300);

        assert_eq!(responsive.get(Breakpoint::Xs), Some(&100));
        assert_eq!(responsive.get(Breakpoint::Sm), Some(&150));
        assert_eq!(responsive.get(Breakpoint::Md), Some(&200));
        assert_eq!(responsive.get(Breakpoint::Lg), Some(&300));
        
        // Test fallback behavior
        assert_eq!(responsive.get(Breakpoint::Xl), Some(&300)); // Falls back to lg
    }

    #[test]
    fn test_css_variable_generation() {
        let tokens = DesignTokens::default();
        let css_string = tokens.to_css_string();
        
        // Test that CSS is properly formatted
        assert!(css_string.starts_with(":root {"));
        assert!(css_string.ends_with("}"));
        
        // Test that it contains expected variables
        assert!(css_string.contains("--sh-color-primary"));
        assert!(css_string.contains("--sh-font-family-base"));
        assert!(css_string.contains("--sh-radius-sm"));
    }

    #[test]
    fn test_hsl_color_operations() {
        let color = HSLColor::new(180.0, 50.0, 50.0);
        
        // Test lightening
        let lighter = color.lighten(20.0);
        assert_eq!(lighter.l, 70.0);
        
        // Test darkening
        let darker = color.darken(20.0);
        assert_eq!(darker.l, 30.0);
        
        // Test saturation
        let more_saturated = color.saturate(20.0);
        assert_eq!(more_saturated.s, 70.0);
        
        // Test desaturation
        let less_saturated = color.desaturate(20.0);
        assert_eq!(less_saturated.s, 30.0);
        
        // Test hue rotation
        let rotated = color.rotate(90.0);
        assert_eq!(rotated.h, 270.0);
    }

    #[test]
    fn test_component_composition() {
        // Test that components can be composed together
        let header = html! {
            div class="header" {
                h1 { "Test Header" }
            }
        };
        
        let button = EnhancedButton::new("Action Button")
            .variant(ButtonVariant::Primary)
            .render();
        
        let combined = html! {
            div class="container" {
                (header)
                (button)
            }
        };
        
        let html_string = combined.into_string();
        assert!(html_string.contains("Test Header"));
        assert!(html_string.contains("Action Button"));
        assert!(html_string.contains("sh-btn--primary"));
    }

    #[test] 
    fn test_theme_consistency() {
        // Test that themes generate consistent CSS variables
        let theme1 = DesignTokens::default();
        let theme2 = DesignTokens::default();
        
        let css1 = theme1.to_css_string();
        let css2 = theme2.to_css_string();
        
        // Default themes should be identical
        assert_eq!(css1, css2);
        
        // Test custom theme
        let custom_theme = DesignTokens::new(
            HSLColor::new(300.0, 70.0, 50.0),
            ColorScheme::Complementary,
        );
        
        let custom_css = custom_theme.to_css_string();
        assert_ne!(custom_css, css1); // Should be different from default
        assert!(custom_css.contains("--sh-color-primary"));
    }
}