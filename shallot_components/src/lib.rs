//! Shallot Components - Zero-JS Rust UI Component Library
//!
//! This crate provides a comprehensive set of UI components built with:
//! - Maud for type-safe HTML templating
//! - Shallot Foundation for design tokens and utilities
//! - Zero JavaScript philosophy

// Core component system
pub mod component;

// Layout components
pub mod r#box;
pub mod layout;
pub mod typography;

// Form components
pub mod form_validation;
pub mod input;

// Content components
pub mod card;
pub mod code_block;
pub mod product_card;

// Data display components
pub mod avatar;
pub mod badge;
pub mod skeleton;
pub mod stats;
// Skeleton module declared above

// Basic components
pub mod button;

// Enhanced components
pub mod enhanced_button;
pub mod enhanced_modal;

// Navigation
pub mod breadcrumbs;
pub mod dock;
pub mod navbar;
pub mod navigation;
pub mod sidebar;
pub mod tabs;

// Data display
pub mod bento;
pub mod calendar;
pub mod carousel;
pub mod charts;
pub mod data_display;
pub mod timeline;

// Feedback
pub mod alert;
pub mod feedback;
pub mod progress;
pub mod rating;
pub mod toast;

// Effects and animations
pub mod animated_beam;
pub mod animated_text;
pub mod border_beam;
pub mod confetti;
pub mod fx;
pub mod glass_card;
pub mod glitch_text;
pub mod magic_card;
pub mod magnetic_button;
pub mod marquee;
pub mod orbiting_circles;
pub mod parallax_section;
pub mod scroll_reveal;
pub mod text_reveal;
pub mod typing_animation;

// Overlay
pub mod drawer;
pub mod dropdown;
pub mod popover;

// Utilities
pub mod collapsible;
pub mod footer;
pub mod form;
pub mod icon;
pub mod join;
pub mod pattern;
pub mod scroll_area;
pub mod section;
pub mod slider;
pub mod steps;
pub mod surface;
pub mod util;

// New form components
pub mod copy_button;
pub mod file_list;
pub mod file_upload;
// Date/time form components
pub mod date_picker;
pub mod otp_input;
pub mod range_slider;
pub mod time_picker;
// Additional form components
pub mod credit_card_input;
pub mod glass_select;
pub mod image_upload;
pub mod multi_select;
pub mod rich_text;

// State and feedback
pub mod empty_state;
pub mod notification_stack;

// Data display
pub mod key_value_list;
pub mod site_map;
pub mod table_of_contents;
pub mod tree_view;
pub mod video_player;

// Effects
pub mod gradient_text;
pub mod pulse_ring;
pub mod shimmer;
pub mod spotlight;

// Overlays
pub mod bottom_sheet;
pub mod command_palette;
// Layout
pub mod aspect_ratio;
pub mod feed_layout;
pub mod masonry;
pub mod split_pane;
pub mod z_stack;

// New data display components
pub mod list;

// Re-added components (previously failed)
pub mod accordion;
pub mod avatar_group;
pub mod color_picker;
pub mod counter;
pub mod dialog;
pub mod menu;
pub mod pagination;
pub mod search_input;
pub mod table;
pub mod tag_input;
pub mod toggle_group;
pub mod tooltip;

// Form grouping components
pub mod checkbox_group;
pub mod radio_group;
pub mod rating_input;

// Data display components
pub mod description_list;
pub mod divider;
pub mod progress_circle;

// Signature animated components
pub mod capdrop;
pub mod liquid_button;
pub mod masked_image;
pub mod mesh_gradient;
pub mod refractive_gauge;
pub mod shadow_elevator;

// Re-exports for convenience
pub use component::{
    component_base_css, AriaAttrs, AriaLive, AriaPopup, AriaRelevance, Component, ComponentColor,
    ComponentShape, ComponentSize, ComponentState, ComponentVariant, IconConfig, IconPosition,
    Spacing,
};

pub use layout::{
    layout_css, AlignItems, AspectRatio, Container, Divider, Grid, JustifyContent, Section, Spacer,
    Stack,
};

pub use r#box::{
    box_css, BorderStyle, Box, BoxSizing, Center, Cursor, Display, Overflow, Position, SizeValue,
    SpacingValue,
};

pub use typography::{
    typography_css, Code, FontFamily, FontWeight, Heading, List, ListItem, ListVariant,
    MarkerStyle, Quote, QuoteVariant, Text, TextAlign, TextColor, TextDecoration, TextSize,
    TextTransform,
};

pub use input::{
    input_css, Checkbox, Input, InputType, InputVariant, Radio, Select, SelectOption, Switch,
    Textarea, TextareaResize,
};

pub use form_validation::{
    presets as validation_presets, AnyFormField, Dirty, FormContext, FormField, Invalid, Pristine,
    Valid, ValidationError, ValidationRule,
};

pub use card::{card_css, Card, CardSection, CardVariant, Elevation};

pub use calendar::{calendar_css, Calendar, CalendarSize, CalendarVariant};

pub use code_block::{
    code_block_css, CodeBlock, CodeBlockVariant, CodeTheme, CommandItem, InlineCode, Language,
    Platform,
};

pub use product_card::{
    product_card_css, BadgeStyle, CartItem, CartSummary, ProductCard, ProductCardVariant,
    ProductPrice, QuantityStepper, Rating, RatingSize,
};

pub use badge::{badge_css, Badge, BadgeVariant, CountBadge, StatusDot};

pub use avatar::{avatar_css, Avatar, AvatarBorder, AvatarGroup, AvatarShape, AvatarStatus};

pub use stats::{stats_css, MetricCard, Stat, StatTrend, Stats, StatsLayout, TrendDirection};

pub use magic_card::{magic_card_css, MagicCard};

pub use animated_text::{
    animated_text_css, AnimatedGradientText, NumberTicker, ShimmerText, WordRotate,
};

pub use skeleton::{
    skeleton_css, Skeleton, SkeletonAnimation, SkeletonAvatarText, SkeletonCard, SkeletonText,
};

pub use animated_beam::{animated_beam_css, beam_presets, AnimatedBeam, AnimatedBeamGroup};

pub use button::{button_css, Button, ButtonSize, ButtonVariant};

pub use alert::{alert_css, Alert, AlertKind};

pub use fx::{fx_css, GlowCard, ShimmerButton, ShinyButton};

pub use enhanced_button::{
    enhanced_button_css, AnimationType, AriaConfig, ButtonShape, EnhancedButton,
};

pub use enhanced_modal::{
    enhanced_modal_css, AriaModal, BackdropStyle, EnhancedModal, ModalConfig, ModalPosition,
    ModalSize,
};

pub use glass_card::{glass_css, GlassCard, GlassIntensity, GlassPanel, GlowPosition};

pub use scroll_reveal::{
    scroll_reveal_css, Easing, ParallaxContainer, RevealAnimation, RevealThreshold, ScrollReveal,
    StaggerContainer,
};

pub use charts::{charts_css, BarChart, ChartColor, CurveType, Gauge, Sparkline};

pub use util::css;

pub use form::{
    form_css, validators, EmailValidator, FieldValidation, Form, FormField as FormUiField,
    FormGroup, FormGroupLayout, FormSchema, FormSize, FormState, FormVariant, MaxLengthValidator,
    MinLengthValidator, PatternValidator, RangeValidator, RequiredValidator, UrlValidator,
    ValidationResult, Validator,
};

pub use collapsible::{collapsible_css, Collapsible};
pub use footer::{footer_css, Footer, FooterColumn};
pub use join::{join_css, Join, JoinGap, JoinVariant};
pub use scroll_area::{scroll_area_css, ScrollArea};
pub use slider::{slider_css, Slider, SliderSize, SliderVariant};
pub use steps::{steps_css, Steps};

// Re-added component exports
pub use accordion::{accordion_css, Accordion, AccordionItem, AccordionVariant};
pub use avatar_group::{
    avatar_group_css, AvatarGroup as StackedAvatarGroup, AvatarGroupVariant, AvatarItem,
};
pub use color_picker::{color_picker_css, ColorPicker, ColorSwatch, ColorSwatches};
pub use counter::{counter_css, Counter, CounterGroup, CounterLayout, CounterVariant, Statistic};
pub use dialog::{dialog_css, ConfirmDialog, Dialog, DialogSize, DialogTrigger, DialogVariant};
pub use drawer::{drawer_css, Drawer};
pub use dropdown::{dropdown_css, Dropdown};
pub use menu::{
    menu_css, DropdownMenu, DropdownPosition, Menu, MenuDivider, MenuItem, MenuVariant,
};
pub use pagination::{
    pagination_css, Pagination, PaginationInfo, PaginationVariant, SimplePagination,
};
pub use search_input::{
    search_input_css, SearchInput, SearchResult, SearchVariant, SearchWithResults,
};

pub use dock::{dock_css, Dock};
pub use navbar::{navbar_css, Navbar};
pub use navigation::navigation_css;
pub use sidebar::{sidebar_css, Sidebar};
pub use table::{table_css, ColumnAlign, SortDir, Table, TableColumn, TableRow, TableVariant};
pub use tag_input::{tag_input_css, Tag, TagInput, TagList, TagVariant};
pub use toggle_group::{
    toggle_group_css, IconButton, IconButtonGroup, ToggleGroup, ToggleItem, ToggleVariant,
};
pub use tooltip::{
    tooltip_css, IconTooltip, RichTooltip, Tooltip, TooltipDelay, TooltipPosition, TooltipVariant,
};

pub use popover::{popover_css, Popover, PopoverPosition, PopoverSize, PopoverVariant};

pub use magnetic_button::{
    magnetic_button_css, MagneticButton, MagneticSize, MagneticStrength, MagneticVariant,
};

pub use glitch_text::{glitch_text_css, GlitchColor, GlitchIntensity, GlitchText};

pub use capdrop::{capdrop_css, CapDrop, CapDropSize, CapDropStyle};

pub use parallax_section::{
    parallax_section_css, ParallaxDirection, ParallaxLayer, ParallaxSection, ParallaxSpeed,
};
pub use site_map::{site_map_css, SiteMap};
pub use table_of_contents::{table_of_contents_css, TableOfContents};
pub use video_player::{video_player_css, VideoPlayer};

pub use tabs::{tabs_css, AnimatedTabs, Tab, TabAnimation, TabSize, TabVariant, Tabs};
// Effects - Animation components
pub use border_beam::{border_beam_css, BorderBeam, BorderBeamSize, BorderBeamVariant};
pub use confetti::{confetti_css, Confetti, ConfettiShape, ConfettiSize, ConfettiVariant};
pub use orbiting_circles::{
    orbiting_circles_css, OrbitDirection, OrbitingCircle, OrbitingCircles, OrbitingCirclesSize,
    OrbitingCirclesVariant,
};
pub use pattern::{pattern_css, Pattern, PatternKind, PatternOpacity, PatternSize};
pub use text_reveal::{text_reveal_css, TextReveal, TextRevealSpeed, TextRevealVariant};
pub use typing_animation::{
    typing_animation_css, TypingAnimation, TypingAnimationVariant, TypingSpeed,
};

pub use bento::{bento_css, BentoCard, BentoCardSize, BentoGrid, BentoVariant};
pub use breadcrumbs::{breadcrumbs_css, Breadcrumbs};
pub use carousel::{carousel_css, Carousel};
pub use timeline::{timeline_css, Timeline, TimelineItem};

pub use liquid_button::{liquid_button_css, LiquidButton, LiquidButtonSize};
pub use masked_image::{masked_image_css, MaskShape, MaskedImage};
pub use mesh_gradient::{mesh_gradient_css, MeshGradientBackground};
pub use refractive_gauge::{refractive_gauge_css, RefractiveGauge};
pub use shadow_elevator::{shadow_elevator_css, ShadowElevator};

pub use copy_button::{copy_button_css, CopyButton};
pub use credit_card_input::{credit_card_input_css, CardType, CreditCardInput};
pub use data_display::{data_display_css, Chip};
pub use file_list::{file_list_css, FileList};
pub use glass_select::{glass_select_css, GlassSelect};
pub use icon::{icon_css, Icon, IconButton};
pub use image_upload::{image_upload_css, ImageUpload};
pub use multi_select::{multi_select_css, MultiSelect};
pub use rich_text::{rich_text_css, RichText};
pub use z_stack::{z_stack_css, ZStack};

pub use feedback::{feedback_css, Spinner};
pub use progress::{progress_css, ProgressBar};
pub use progress_circle::{progress_circle_css, ProgressCircle};
pub use rating::{rating_css, Rating as StarRating};
pub use toast::{toast_css, Toast, ToastContainer};

/// Generate all component CSS
pub fn all_component_css() -> String {
    let mut css = String::new();

    // Base component styles
    css.push_str(&component_base_css());
    css.push('\n');

    // Layout
    css.push_str(&layout_css());
    css.push('\n');

    // Navigation
    css.push_str(&navbar_css());
    css.push('\n');
    css.push_str(&sidebar_css());
    css.push('\n');
    css.push_str(&dock_css());
    css.push('\n');
    css.push_str(&navigation_css());
    css.push('\n');

    // Alert
    css.push_str(&alert_css());
    css.push('\n');

    // Button
    css.push_str(&button_css());
    css.push('\n');

    // FX
    css.push_str(&fx_css());
    css.push('\n');

    // Input
    css.push_str(&input_css());
    css.push('\n');

    // Card
    css.push_str(&card_css());
    css.push('\n');

    // Badge
    css.push_str(&badge_css());
    css.push('\n');

    // Avatar
    css.push_str(&avatar_css());
    css.push('\n');

    // Stats
    css.push_str(&stats_css());
    css.push('\n');

    // Magic Card
    css.push_str(&magic_card_css());
    css.push('\n');

    // Animated Text
    css.push_str(&animated_text_css());
    css.push('\n');

    // Skeleton
    css.push_str(&skeleton_css());
    css.push('\n');

    // Animated Beam
    css.push_str(&animated_beam_css());
    css.push('\n');

    // Enhanced button
    css.push_str(&enhanced_button_css());
    css.push('\n');

    // Enhanced modal
    css.push_str(&enhanced_modal_css());
    css.push('\n');

    // Box primitives
    css.push_str(&box_css());
    css.push('\n');

    // Typography system
    css.push_str(&typography_css());
    css.push('\n');

    // Glass card
    css.push_str(&glass_css());
    css.push('\n');

    // Scroll reveal
    css.push_str(&scroll_reveal_css());
    css.push('\n');

    // Charts
    css.push_str(&charts_css());
    css.push('\n');

    // Calendar
    css.push_str(&calendar_css());
    css.push('\n');

    // Code Block
    css.push_str(&code_block_css());
    css.push('\n');

    // Product Card
    css.push_str(&product_card_css());
    css.push('\n');

    // Re-added components
    css.push_str(&table_css());
    css.push('\n');

    css.push_str(&accordion_css());
    css.push('\n');

    css.push_str(&dialog_css());
    css.push('\n');

    css.push_str(&tooltip_css());
    css.push('\n');

    css.push_str(&popover_css());
    css.push('\n');

    css.push_str(&menu_css());
    css.push('\n');

    css.push_str(&drawer_css());
    css.push('\n');

    css.push_str(&dropdown_css());
    css.push('\n');

    css.push_str(&pagination_css());
    css.push('\n');

    css.push_str(&avatar_group_css());
    css.push('\n');

    css.push_str(&toggle_group_css());
    css.push('\n');

    css.push_str(&search_input_css());
    css.push('\n');

    css.push_str(&tag_input_css());
    css.push('\n');

    css.push_str(&color_picker_css());
    css.push('\n');

    css.push_str(&counter_css());
    css.push('\n');

    // List components
    css.push_str(&list::list_css());
    css.push('\n');

    // File upload
    css.push_str(&file_upload::file_upload_css());
    css.push('\n');

    // New components - Form
    css.push_str(&date_picker::date_picker_css());
    css.push('\n');

    css.push_str(&time_picker::time_picker_css());
    css.push('\n');

    css.push_str(&otp_input::otp_input_css());
    css.push('\n');

    css.push_str(&range_slider::range_slider_css());
    css.push('\n');

    // New components - State/Feedback
    css.push_str(&empty_state::empty_state_css());
    css.push('\n');

    css.push_str(&notification_stack::notification_stack_css());
    css.push('\n');

    // New components - Data display
    css.push_str(&tree_view::tree_view_css());
    css.push('\n');

    css.push_str(&key_value_list::key_value_list_css());
    css.push('\n');

    // New components - Effects
    css.push_str(&gradient_text::gradient_text_css());
    css.push('\n');

    css.push_str(&spotlight::spotlight_css());
    css.push('\n');

    css.push_str(&shimmer::shimmer_css());
    css.push('\n');

    css.push_str(&pulse_ring::pulse_ring_css());
    css.push('\n');

    // New components - Overlays
    css.push_str(&bottom_sheet::bottom_sheet_css());
    css.push('\n');

    css.push_str(&command_palette::command_palette_css());
    css.push('\n');

    // Credit Card Input
    css.push_str(&credit_card_input_css());
    css.push('\n');

    // Glass Select
    css.push_str(&glass_select_css());
    css.push('\n');

    // Image Upload
    css.push_str(&image_upload_css());
    css.push('\n');

    // Multi Select
    css.push_str(&multi_select_css());
    css.push('\n');

    // Rich Text
    css.push_str(&rich_text_css());
    css.push('\n');

    // Copy Button
    css.push_str(&copy_button_css());
    css.push('\n');

    // File List
    css.push_str(&file_list_css());
    css.push('\n');

    // New components - Layout
    css.push_str(&aspect_ratio::aspect_ratio_css());
    css.push('\n');

    css.push_str(&masonry::masonry_css());
    css.push('\n');

    css.push_str(&split_pane::split_pane_css());
    css.push('\n');

    css.push_str(&section::section_css());
    css.push('\n');

    css.push_str(&surface::surface_css());
    css.push('\n');

    css.push_str(&marquee::marquee_css());
    css.push('\n');

    css.push_str(&bento::bento_css());
    css.push('\n');

    css.push_str(&slider::slider_css());
    css.push('\n');

    css.push_str(&join::join_css());
    css.push('\n');
    css.push_str(&feed_layout::feed_layout_css());
    css.push('\n');

    // New components - Form groups
    css.push_str(&checkbox_group::checkbox_group_css());
    css.push('\n');

    css.push_str(&radio_group::radio_group_css());
    css.push('\n');

    css.push_str(&rating_input::rating_input_css());
    css.push('\n');

    // New components - Data display
    css.push_str(&description_list::description_list_css());
    css.push('\n');

    css.push_str(&progress_circle::progress_circle_css());
    css.push('\n');

    css.push_str(&divider::divider_css());
    css.push('\n');

    // Signature animated components
    css.push_str(&magnetic_button_css());
    css.push('\n');

    css.push_str(&liquid_button_css());
    css.push('\n');

    css.push_str(&masked_image_css());
    css.push('\n');

    css.push_str(&mesh_gradient_css());
    css.push('\n');

    css.push_str(&refractive_gauge_css());
    css.push('\n');

    css.push_str(&shadow_elevator_css());
    css.push('\n');

    css.push_str(&glitch_text_css());
    css.push('\n');

    css.push_str(&parallax_section_css());
    css.push('\n');

    css.push_str(&tabs_css());
    css.push('\n');

    // Form components
    css.push_str(&form_css());
    css.push('\n');

    // Footer
    css.push_str(&footer_css());
    css.push('\n');

    // Scroll Area
    css.push_str(&scroll_area_css());
    css.push('\n');

    // Breadcrumbs
    css.push_str(&breadcrumbs_css());
    css.push('\n');

    // Carousel
    css.push_str(&carousel_css());
    css.push('\n');

    // Timeline
    css.push_str(&timeline_css());
    css.push('\n');

    // Feedback & Notifications
    css.push_str(&feedback_css());
    css.push('\n');
    css.push_str(&progress_css());
    css.push('\n');
    css.push_str(&toast_css());
    css.push('\n');
    css.push_str(&rating_css());
    css.push('\n');

    // Specialized Content
    css.push_str(&capdrop_css());
    css.push('\n');
    css.push_str(&site_map_css());
    css.push('\n');
    css.push_str(&table_of_contents_css());
    css.push('\n');
    css.push_str(&video_player_css());
    css.push('\n');

    // Primitives & Specialized Content
    css.push_str(&icon_css());
    css.push('\n');
    css.push_str(&data_display_css());
    css.push('\n');
    css.push_str(&z_stack_css());
    css.push('\n');
    css.push_str(&collapsible_css());
    css.push('\n');
    css.push_str(&steps_css());
    css.push('\n');
    css.push_str(&progress_circle_css());
    css.push('\n');

    // BorderBeam
    css.push_str(&border_beam::border_beam_css());
    css.push('\n');

    // Confetti
    css.push_str(&confetti::confetti_css());
    css.push('\n');

    // OrbitingCircles
    css.push_str(&orbiting_circles::orbiting_circles_css());
    css.push('\n');

    // Pattern
    css.push_str(&pattern::pattern_css());
    css.push('\n');

    // TextReveal
    css.push_str(&text_reveal::text_reveal_css());
    css.push('\n');

    // TypingAnimation
    css.push_str(&typing_animation::typing_animation_css());
    css.push('\n');

    css
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get library information
pub fn library_info() -> &'static str {
    "Shallot Components - Zero-JS Rust UI Component Library"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_css_generation() {
        let css = all_component_css();
        assert!(!css.is_empty());
        assert!(css.contains(".sh-"));
    }
}
