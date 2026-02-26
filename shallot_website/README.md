# Shallot Website

🌐 **Zero-JS, Tor-Compatible Component Showcase**

A fully interactive, themeable website showcasing the Shallot component library - built with **pure HTML/CSS** and **zero JavaScript**.

## 🚀 Quick Start

```bash
# Generate the static website
cargo run -p shallot_website

# The website will be generated in the `output/` directory
# Open output/index.html in your browser
```

## 📁 Generated Files

```
output/
├── index.html          # Main homepage (55KB)
└── styles/
    ├── main.css        # Core styles (2.7KB)
    ├── retro.css       # 90s hero section styles (5.8KB)
    └── showcase.css    # Component showcase styles (5.8KB)
```

## ✨ Features

### 🎨 Retro Hero Section
A humorous homage to the 90s web era:
- CRT scanlines and flicker effects (pure CSS animations)
- Marquee scrolling text
- "Under Construction" GIF
- Comic Sans typography with rainbow animation
- Retro table layout
- Badge row (No JS, Rust, 0kb, Tor Compatible)
- Punchline: *"No JavaScript doesn't have to look like this"*

### 🎯 Modern Showcase
A beautiful, interactive component gallery:
- **7 Category Tabs**: Layout, Typography, Forms, Navigation, Overlays, Data, Animated
- **Component Cards**: Preview, description, and expandable code view
- **Dual Code View**: "Full Code" vs "In Library" usage examples
- **Smooth Animations**: Hover effects and transitions

### 🌈 Theme Switcher
Floating customization panel with:
- **7 Beautiful Presets**: Ocean, Forest, Sunset, Midnight, Cherry, Teal, Amber
- **Custom Color Picker**: Select any primary color
- **Toggle Options**: Rounded corners, Shadows, Animations
- **Mobile-Friendly**: Slides up from bottom on small screens
- **Fully Accessible**: Keyboard navigation, ARIA labels

## 🔧 Technical Highlights

### Zero JavaScript
All interactivity achieved through CSS techniques:
- ✅ Checkbox hack for dropdowns
- ✅ Radio buttons for tabs
- ✅ `:has()` selector for theme switching
- ✅ CSS custom properties for theming
- ✅ `:target` for navigation

### Tor-Compatible
- ✅ No external dependencies
- ✅ All CSS inline or local
- ✅ No tracking/analytics
- ✅ No WebAssembly
- ✅ Minimal bundle size (67KB total)

### Accessibility
- ✅ Semantic HTML5 structure
- ✅ ARIA labels and roles
- ✅ Keyboard navigation support
- ✅ Focus indicators
- ✅ Reduced motion support
- ✅ High contrast ratios

### Responsive Design
- ✅ Mobile-first approach
- ✅ Fluid typography
- ✅ Flexible grids
- ✅ Touch-friendly targets

## 📊 Performance

| Metric | Value |
|--------|-------|
| Total Size | 67KB |
| JavaScript | 0KB |
| External Requests | 0 |
| First Paint | Instant |
| Time to Interactive | Instant |

## 🎯 Browser Compatibility

Works in all modern browsers:
- ✅ Chrome/Edge 88+
- ✅ Firefox 85+
- ✅ Safari 14+
- ✅ Tor Browser

Graceful degradation in older browsers.

## 🛠️ Development

### Project Structure

```
shallot_website/
├── src/
│   ├── lib.rs              # Main library exports
│   ├── main.rs             # Binary entry point
│   ├── retro_hero.rs       # 90s hero section
│   ├── showcase.rs         # Component showcase
│   ├── theme_switcher.rs   # Theme customization
│   └── component_docs.rs   # Component documentation
├── Cargo.toml
└── output/                 # Generated static files
```

### Adding New Components

1. Add component to `SAMPLE_COMPONENTS` in `showcase.rs`
2. Ensure component category matches
3. Rebuild: `cargo run -p shallot_website`

### Customizing Themes

Edit `THEME_PRESETS` in `theme_switcher.rs`:

```rust
ThemePreset {
    name: "Custom",
    primary: "#yourcolor",
    secondary: "#yoursecondary",
    accent: "#youraccent",
}
```

## 📝 License

MIT License - See main project LICENSE

## 🤝 Contributing

Contributions welcome! Please ensure:
- No JavaScript additions
- Maintain Tor compatibility
- Test accessibility
- Keep bundle size minimal

## 🎉 Fun Facts

- The entire website is smaller than most JavaScript bundles
- All animations run at 60fps using CSS compositing
- The retro hero section uses actual 90s web design patterns
- Theme switching works without any JavaScript state management

---

**Built with ❤️ and zero JavaScript by the Shallot Team**

*Iron logic. Glass aesthetics. Pure Rust.*
