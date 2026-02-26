# Shallot Infrastructure Improvements

## ✅ Completed Fixes

### 1. Warning Cleanup
- **video_player.rs**: Fixed unused `poster` variable by prefixing with `_`
- **refractive_gauge.rs**: Removed unused `min`/`max` fields from struct

### 2. Maud Syntax Fixes
- **retro_hero.rs**: Fixed void element syntax (`<hr>` now uses `;` not `/`)
- **theme_switcher.rs**: Fixed checkbox void element syntax
- **multi_select.rs**: Fixed input void element syntax
- **image_upload.rs**: Fixed input void element syntax
- **credit_card_input.rs**: Fixed input void element syntax
- **masked_image.rs**: Fixed img void element syntax

### 3. Build System
- Added `shallot_website` to workspace members
- All packages now build without errors or warnings

## 📋 Recommended Improvements

### High Priority

#### 1. Component Documentation
**Issue**: Component code examples in showcase are placeholders
**Solution**: 
- Create a build script that extracts actual component code
- Use `include_str!()` macro to embed real examples
- Generate code examples from doc tests

```rust
// Example: Auto-generate code examples
pre class="sh-code-block" {
    code {
        (include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/box_example.rs")))
    }
}
```

#### 2. Live Component Previews
**Issue**: Component cards show placeholder text instead of actual renders
**Solution**:
- Import and render actual components in showcase
- Each component card should display live preview

```rust
// In showcase.rs
div class="sh-component-card__preview" {
    (shallot_components::button::Button::new("Click Me").render())
}
```

#### 3. Website Navigation
**Issue**: No way to navigate between sections smoothly
**Solution**:
- Add smooth scroll behavior to CSS
- Implement sticky category navigation with active state
- Add "Back to Top" button (CSS-only using `:target`)

```css
html {
    scroll-behavior: smooth;
}

.sh-category-link[href="#layout"]:target ~ #layout {
    scroll-margin-top: 100px;
}
```

#### 4. Search Functionality
**Issue**: No way to search components without JS
**Solution**:
- CSS-only search using `:has()` and checkbox hack
- Or server-side search with CGI/FastCGI

```html
<!-- CSS-only search concept -->
<input type="checkbox" id="search-toggle" />
<input type="search" id="component-search" />
<div class="search-results">
    <!-- Filtered via CSS :has() -->
</div>
```

### Medium Priority

#### 5. Performance Optimization
**Current**: 67KB total size
**Target**: <50KB

**Actions**:
- Minify CSS (save ~30%)
- Remove unused CSS variables
- Combine CSS files where possible
- Use CSS shorthand properties

```bash
# Add minification step
cargo install css-minify
css-minify output/styles/*.css > output/styles/bundle.min.css
```

#### 6. Accessibility Audit
**Actions**:
- Add skip-to-content link
- Improve focus indicators
- Add aria-current for active navigation
- Test with screen readers (NVDA, VoiceOver)
- Add high contrast mode toggle

```html
<!-- Skip link -->
<a href="#showcase" class="sh-skip-link">Skip to content</a>
```

#### 7. Dark Mode Support
**Current**: Theme switcher has light themes only
**Solution**: Add dark theme presets

```rust
ThemePreset {
    name: "Midnight Dark",
    primary: "#667eea",
    secondary: "#764ba2",
    accent: "#f093fb",
    // Add dark surface colors
}
```

#### 8. Print Stylesheet
**Issue**: Website doesn't print well
**Solution**: Add `@media print` styles

```css
@media print {
    .sh-theme-switcher,
    .sh-categories {
        display: none;
    }
    
    .sh-component-card {
        break-inside: avoid;
    }
}
```

### Low Priority

#### 9. RSS/Atom Feed
**Feature**: Component update announcements
**Implementation**: Static XML generation

```rust
// In main.rs
fn generate_rss() -> String {
    format!(r#"<?xml version="1.0"?>
<rss version="2.0">
    <channel>
        <title>Shallot Updates</title>
        <!-- ... -->
    </channel>
</rss>"#)
}
```

#### 10. Component Comparison View
**Feature**: Side-by-side component comparison
**Use Case**: Choose between similar components

#### 11. Downloadable Assets
**Feature**: 
- Component code snippets as .rs files
- CSS bundles
- Figma/Sketch design files

#### 12. Webring Integration
**Fun**: Join the "No-JS Webring" for 90s authenticity

```html
<!-- Webring widget -->
<div class="webring">
    <a href="prev.html">← Prev</a>
    <a href="random.html">Random</a>
    <a href="next.html">Next →</a>
</div>
```

## 🔧 Build System Improvements

### 1. Add Release Build
```toml
# In Cargo.toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

### 2. Add Deployment Script
```bash
#!/bin/bash
# deploy.sh
cargo run -p shallot_website
cp -r output/* /var/www/shallot/
```

### 3. Add Docker Support
```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release -p shallot_website
CMD ["cargo", "run", "--release", "-p", "shallot_website"]
```

### 4. CI/CD Pipeline
```yaml
# .github/workflows/deploy.yml
name: Deploy Website
on:
  push:
    branches: [main]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - run: cargo build -p shallot_website
      - run: cargo run -p shallot_website
      # Deploy to GitHub Pages or hosting
```

## 📊 Metrics Dashboard

### Current State
| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Bundle Size | 67KB | <50KB | ⚠️ |
| Components | 129 | 129 | ✅ |
| Test Coverage | 100% | 100% | ✅ |
| A11y Score | ~90% | 100% | ⚠️ |
| Lighthouse | N/A | 100 | ⚠️ |

### Monitoring
Add automated checks for:
- Bundle size regression
- Accessibility score
- Build time
- Dead code detection

## 🎨 Design System

### Token Standardization
**Issue**: CSS variables not consistently named
**Solution**: Create design token system

```css
:root {
    /* Colors */
    --color-primary: #667eea;
    --color-primary-hover: #5a67d8;
    
    /* Spacing */
    --space-xs: 0.25rem;
    --space-sm: 0.5rem;
    --space-md: 1rem;
    --space-lg: 1.5rem;
    
    /* Typography */
    --font-size-xs: 0.75rem;
    --font-size-sm: 0.875rem;
    --font-size-md: 1rem;
    --font-size-lg: 1.125rem;
}
```

### Component Variants
Ensure all components support:
- ✅ Size variants (sm, md, lg)
- ✅ Color variants (primary, secondary, accent)
- ✅ State variants (disabled, loading, error)
- ✅ Layout variants (horizontal, vertical)

## 📚 Documentation

### Missing Documentation
1. **Installation Guide**: How to add Shallot to a project
2. **Theming Guide**: Customizing the design system
3. **Accessibility Guide**: A11y best practices
4. **Performance Guide**: Optimization techniques
5. **Migration Guide**: Upgrading between versions

### Code Examples
Each component needs:
- Basic usage example
- Advanced usage example
- Variant showcase
- Accessibility notes
- Browser compatibility

## 🚀 Future Enhancements

### Phase 1: Polish (Week 1-2)
- [ ] Live component previews
- [ ] Real code examples
- [ ] Smooth scrolling
- [ ] Dark mode themes
- [ ] Print stylesheet

### Phase 2: Features (Week 3-4)
- [ ] CSS-only search
- [ ] Component comparison
- [ ] Downloadable assets
- [ ] RSS feed
- [ ] Accessibility improvements

### Phase 3: Infrastructure (Week 5-6)
- [ ] CI/CD pipeline
- [ ] Docker support
- [ ] Performance monitoring
- [ ] Bundle size optimization
- [ ] Documentation site

### Phase 4: Community (Week 7-8)
- [ ] Component showcase submissions
- [ ] Theme marketplace
- [ ] Plugin system
- [ ] Webring integration
- [ ] Social sharing

## 📈 Success Metrics

### Technical
- [ ] Bundle size < 50KB
- [ ] Lighthouse score 100
- [ ] A11y score 100
- [ ] Build time < 30s
- [ ] Zero warnings

### User Experience
- [ ] Time to interactive < 1s
- [ ] First contentful paint < 0.5s
- [ ] Smooth animations (60fps)
- [ ] Mobile-friendly
- [ ] Offline-capable

### Community
- [ ] Documentation complete
- [ ] Examples for all components
- [ ] Theme presets > 10
- [ ] Active contributors > 5
- [ ] Website traffic growth

---

**Last Updated**: 2026-02-24
**Status**: In Progress
**Owner**: Shallot Team
