# Product Guidelines

## Prose Style

### Voice and Tone
- **Confident but Humble**: We believe in our approach (Zero-JS) but acknowledge it's a choice, not a mandate
- **Technical Precision**: Use exact terms (FSM, Phantom Types, Maud templates) without unnecessary jargon
- **Manifesto Energy**: We're proving something important about the web, but we're building, not preaching

### Writing Principles
1. **Show, Don't Tell**: Demonstrate capabilities through working code, not claims
2. **Explain the Why**: Every technical decision should include the reasoning (e.g., "Zero JS for TOR compatibility")
3. **Fractal Documentation**: Each layer (Bulb, Skin, Aura) has its own documentation that connects to the whole

## Branding Guidelines

### Visual Identity
- **Iron & Glass**: The dual nature of our brand - strong logic, beautiful presentation
- **The Shallot Metaphor**: Layered architecture (Allium model) - use consistently
- **Color Philosophy**: Primary colors should feel "heavy" and "real" - avoid plastic/bright aesthetics

### Messaging Pillars
1. **Post-JS Movement**: We're part of a broader shift back to server-side rendering
2. **Compiler as Guardian**: Type safety prevents runtime errors
3. **Performance Ceiling**: 0ms TBT, 0.00 CLS - measurable claims
4. **Privacy First**: TOR-compatible, NoScript-friendly

## UX Principles

### Accessibility First
- **WCAG 2.1 AA by Default**: Every component must pass without configuration
- **Screen Reader Parity**: The "Two-Lens View" - screen readers see clean semantics, sighted users see refractive beauty
- **Keyboard Navigation**: All interactive elements must be keyboard-accessible without JavaScript

### Progressive Enhancement
- **Core First**: Basic functionality works without CSS (rare, but possible)
- **Enhanced Second**: Glass effects, animations, and advanced features layer on top
- **Graceful Degradation**: Older browsers see simplified but functional versions

### Performance as UX
- **Sub-100ms First Paint**: Perceived performance matters more than benchmarks
- **Zero Layout Shift**: Pre-calculated aspect ratios prevent jarring shifts
- **Hardware Acceleration**: CSS transforms over layout changes

## Code Aesthetics

### Rust Patterns
- **Fluent Builders**: Components use builder pattern for compile-time validation
- **Phantom Types for State**: State machines enforced by the type system
- **No unwrap() in Production**: Graceful error handling throughout

### CSS Philosophy
- **Custom Properties for Theming**: No magic strings, compile-time validation
- **Hardware-Accelerated Animations**: `transform` and `opacity` over layout properties
- **Reduced Motion Support**: `@media (prefers-reduced-motion)` respected everywhere

### Documentation Standards
- **Public APIs Documented**: Every `pub fn` has doc comments with examples
- **Architecture Decision Records**: Major decisions include ADR explaining the why
- **Component Cards**: Each component has: Overview, Usage, Variants, Accessibility Notes
