# Technology Stack

## Core Language
- **Rust (Edition 2021)**: Primary language for all components and backend

## Templating & Rendering
- **Maud 0.26**: Type-safe HTML templating with compile-time validation
- **Markup Type**: Maud's `Markup` type for all component rendering

## Web Framework
- **Axum 0.7**: Server-side routing and HTTP handling
- **Tower HTTP 0.5**: Middleware and static file serving

## Async Runtime
- **Tokio 1.x**: Async runtime for all asynchronous operations

## Build System
- **Cargo Workspaces**: Multi-crate workspace structure
- **Custom Build Scripts**: `shallot_build` for CSS generation and asset compilation

## Testing
- **cargo-test**: Built-in Rust testing framework
- **shallot_testing**: Custom testing utilities for component validation

## Project Structure (Workspace Members)
1. **shallot**: Umbrella crate
2. **shallot_components**: UI components library (129+ modules)
3. **shallot_foundation**: Design tokens, theming, animations
4. **shallot_testing**: Testing framework and utilities
5. **shallot_build**: Build scripts and code generation
6. **shallot_website**: Static site generation for documentation

## Key Dependencies
```toml
axum = "0.7"           # Web framework
maud = "0.26"          # HTML templating
tokio = "1"            # Async runtime
tower-http = "0.5"     # HTTP utilities
serde = "1.0"          # Serialization
thiserror = "1.0"      # Error handling
walkdir = "2.4"        # File walking (build)
```

## Deployment
- **Vercel**: Static site deployment
- **Output Directory**: `output/` containing generated HTML and CSS

## Development Tools
- **rustfmt**: Code formatting
- **clippy**: Linting
- **cargo-doc**: Documentation generation

## Browser Compatibility
- All modern browsers (Chrome, Firefox, Safari, Edge)
- TOR Browser (High Security mode)
- Graceful degradation for older browsers
