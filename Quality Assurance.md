# **Full Success Criteria: The "Golden Shallot" Standard**

This document establishes the uncompromising quality bar for **Shallot.rs**. Every component—from the simplest Box to the most complex MagneticButton—must pass these rigorous audits before being merged into the core "Petal" catalog.

## **1\. The "Tor Test" (Network & Engine Robustness)**

The "Tor Test" is our ultimate benchmark for functional sovereignty. It ensures the UI remains resilient under high-latency onion routing and restricted browser environments.

* **The Procedure**: Open the showcase in a TOR browser configured to the "Safer" or "Safest" security settings.  
* **The Functional Criteria**:  
  * Every interactive element (Modals, Drawers, Tabs) must function perfectly using only CSS pseudo-classes.  
  * Navigation must remain fluid via hash-based :target routing without requiring page reloads or JS state-history management.  
  * Animations must not fail or "stutter" due to the lack of a JS-driven requestAnimationFrame loop.  
* **The Rationale**: If a user cannot access 100% of a site's features in a zero-script environment, we have failed to deliver a truly "Iron" logic foundation.

## **2\. The "NoScript Audit" (Security & Purity)**

The NoScript Audit verifies that our "Zero-JS" mandate is a physical reality, not a marketing claim.

* **The Procedure**: Run the application with the NoScript extension or a hard browser block on all scripts.  
* **The Logic Criteria**:  
  * **Zero Layout Shift**: The page must not "jump" or flicker as scripts fail to load (because there are no scripts to fail).  
  * **Zero Broken Functionality**: Dropdowns and menus must trigger via :hover, :focus-within, or :checked mechanisms.  
  * **The "Invisible Bridge" Check**: A network audit must confirm that zero bytes of .js, .mjs, or .ts files are transferred.  
* **The Implications**: By passing this audit, we guarantee immunity to 100% of client-side XSS (Cross-Site Scripting) and prototype pollution vulnerabilities.

## **3\. Accessibility Score (The "Two-Lens" View)**

We believe that beauty is the vehicle for inclusion. Our components are audited through a "Two-Lens" perspective: one for the sighted user and one for the assistive device.

* **Metric**: A sustained **100/100 score** on Google Lighthouse or Axe DevTools.  
* **Structural Requirements**:  
  * **The Iron Core**: Every icon must have an explicit aria-label. Every interactive "target" must maintain a minimum hit area of 44x44px.  
  * **Contrast Ratios**: All text and meaningful Glassmorphism effects must exceed a **4.5:1 contrast ratio** against their backgrounds.  
  * **Focus Integrity**: Keyboard navigation must follow a logical tab order, and Modals must utilize the CSS-only focus-trap pattern to ensure users don't get lost in the background DOM.

## **4\. Performance Metrics (Bionic Speed)**

Shallot.rs targets a performance profile that JS frameworks simulate but can never truly achieve due to runtime overhead.

* **LCP (Largest Contentful Paint)**: **\< 1.0s**. The "Skin" must render the primary content immediately upon HTML parsing.  
* **CLS (Cumulative Layout Shift)**: **0.00**. We strictly enforce pre-calculated aspect-ratio and "Iron" spacing logic to prevent any visual movement during asset loading.  
* **TBT (Total Blocking Time)**: **0ms**. Because there is no JavaScript engine to initialize or virtual DOM to reconcile, the main thread is never blocked.  
* **The "60fps Aura"**: All transitions (Glass blurs, transforms) must utilize hardware-accelerated composite properties only, ensuring smooth motion on low-power devices.

## **5\. Developer Experience (DX & Integrity)**

We treat the developer as a first-class citizen of the Allium model. If a project compiles, it must be correct.

* **The "Zero Magic String" Policy**: 100% of theme variables, spacing units, and component properties must be typed in Rust.  
  * *Violation*: padding("15px") or color("\#ff0000").  
  * *Compliance*: padding(Spacing::Md) or color(Theme::Primary).  
* **Compile-Time Verification**: We utilize Phantom Types to ensure components cannot enter illegal states. A Button\<Loading\> cannot mathematically be assigned a click\_action at compile time.  
* **Incremental Feature Gating**: The library must support granular Cargo.toml features. Developers should be able to import shallot-core without the overhead of the shallot-data-viz SVG engine.

## **6\. The "Glass" Aesthetic Standard (Refractive Quality)**

Finally, every component must pass the aesthetic audit of the "Aura" layer.

* **Bionic Naturalism**: Every animation must utilize our standard cubic-bezier(0.16, 1, 0.3, 1\) easing to mimic physical inertia.  
* **Refractive Depth**: Overlays must utilize backdrop-filter: blur() and layered ambient shadows (at least 3 layers) to indicate elevation and priority. A "Glass" component that looks "Flat" is a failure.