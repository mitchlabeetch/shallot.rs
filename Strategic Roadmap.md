# **Shallot.rs: Strategic Roadmap & System Requirements**

## **1\. Functional Requirements: The Allium Architecture**

* **150+ Components (The Petal Expansion)**: We are committed to a comprehensive library covering 7 distinct "Petals." This includes:  
  * **Core Primitives**: Type-safe layout tools (Box, Stack, Z-Stack) that enforce spacing logic at compile time.  
  * **Refractive Forms**: A complete set of inputs (FloatingInput, GlassSelect, MultiStepForm) featuring zero-JS server-side validation and native CSS state feedback.  
  * **Advanced Data Displays**: High-performance components (DataGrid, RefractiveTimeline, BionicTable) utilizing content-visibility for massive datasets without scrolling lag.  
  * **Navigation & Wayfinding**: Interactive components (MegaMenu, CommandPalette, BreadcrumbTrail) using the :target and :focus-within patterns for depth.  
* **Theming Engine (The Aura Core)**: A dynamic engine that translates Rust Theme structs into a high-performance CSS variable block.  
  * *Requirement*: Support for runtime theme swapping (e.g., Light to Dark) via the "Checkbox Hack" selector, ensuring 0ms latency for user-driven aesthetic shifts.  
  * *Requirement*: Programmatic derivation of "Glass" variants (automatically calculating optimal blur and transparency values based on the primary hue).  
* **Accessibility Suite (Iron Integrity)**: Automated generation of WCAG 2.1 AA compliant markup.  
  * *Requirement*: Every interactive component must automatically generate the correct ARIA roles and keyboard focus traps (especially for Modals and Drawers).  
  * *Requirement*: High-contrast fallback mode triggered by system preferences without additional CSS files.  
* **Build Adapters & Ecosystem Integration**:  
  * **SSR Adapters**: Seamless wrappers for Axum and Actix-Web that handle the injection of the Aura CSS block into the HTML head.  
  * **Static Site Generators**: Support for Zola and Cobalt, allowing developers to build lightning-fast, zero-JS documentation and marketing sites that feel like high-end SPAs.

## **2\. Non-Functional Requirements: The Iron & Glass Standard**

* **Zero-JS Mandate (Uncompromising Purity)**:  
  * Absolutely no client-side JavaScript or TypeScript. This means no wasm-bindgen glue code for baseline interactivity and no external script tags.  
  * *Implication*: Interactivity is managed through CSS Finite State Machines (FSM). This eliminates XSS vectors and ensures the UI is interactive the moment the first byte of HTML is rendered.  
* **Performance Profiles**:  
  * **Lighthouse Excellence**: 95+ score across Performance, Accessibility, and Best Practices.  
  * **First Paint Target**: Under 100ms on standard 4G connections.  
  * **Zero CLS (Cumulative Layout Shift)**: Forced use of aspect-ratio and pre-calculated layout logic in the Rust Bulb to prevent visual jumping.  
* **Binary Size & Resource Management**:  
  * **Core Runtime**: Under 200KB for the server-side logic.  
  * **CSS Purging**: Automated compile-time purging of unused component styles to keep the "Aura" block under 50KB for typical applications.  
  * **Memory Efficiency**: Zero unsafe blocks in component logic, utilizing Rust's ownership model to prevent memory leaks in long-running SSR processes.  
* **Universal Compatibility (The Tor/NoScript Standard)**:  
  * The library must be fully functional on TOR Browser (High Security level) and browsers with NoScript or UBlock Origin set to "Block all Scripts."  
  * Support for legacy browsers through graceful degradation of "Glass" effects (blurs), while maintaining "Iron" logic (functionality).

## **3\. Success Metrics: The Golden Shallot**

* **Total Transparency**: A final build audit must confirm 0 bytes of JS/TS are sent to the client.  
* **Safety Audit**: 100% of core logic must pass cargo clippy and cargo audit, with zero reliance on unsafe logic for DOM manipulation or state management.  
* **Versatility Baseline**: Delivery of 10+ pre-built "Glass" themes (e.g., "Obsidian," "Frost," "Cyberpunk," "Minimalist") available as Rust crate features.  
* **Developer Experience (DX)**: A "Zero Magic String" policy. 100% of API surfaces must be typed. If the project compiles, the design system is intact.

## **4\. Phased Roadmap**

### **Phase 1: The Foundation (Current)**

* Finalize the foundation crate (Hsl logic, Spacing, Typography scales).  
* Complete the first 10 "Primitive" components.  
* Establish the Maud rendering standard for the "Skin."

### **Phase 2: The Interactive Petals**

* Implement the Zero-JS Form System (Logic-driven CSS validation).  
* Launch the Overlay Petal (Modals and Drawers using the :target hack).  
* Initial release of the Aura Theming Engine.

### **Phase 3: The Refractive Experience**

* Implement Signature Animated Experiences (Magnetic Buttons, Spotlight Cards).  
* Release the Data Visualization Petal (Pure SVG/CSS charts).  
* Finalize Build Adapters for Axum and Actix.

### **Phase 4: Enterprise Scale**

* Expand to 150+ components.  
* Automated Accessibility testing suite.  
* Launch the "Laboratory" Showcase with zero-JS live code editing simulation.