# **Pitfalls: Learning from the "Plastic" Web**

This document serves as the "Shadow Manifesto" for Shallot.rs. To build with **Iron & Glass**, we must first identify the structural rot inherent in the modern, JavaScript-addicted web. These pitfalls represent the industry's shift toward "plastic" solutions—fragile, opaque, and performance-heavy—and provide the engineering rationale for Shallot’s uncompromising architecture.

## **1\. The JS-Dependency Trap**

* **Pitfall**: Modern "Rust Web" libraries often rely on wasm-bindgen to call heavy JavaScript libraries for complex UI tasks (e.g., using Popper.js for tooltips or GreenSock for animations). This creates an "Invisible Bridge" where the developer thinks they are writing Rust, but the client is still paying the high cost of JS execution and security vulnerabilities.  
* **Shallot Solution**: We utilize the **CSS Toolset** for 100% of positioning logic. Tooltips use CSS anchor positioning or :hover wrappers with absolute containment. Motion is handled via hardware-accelerated CSS Transitions. By cutting the bridge, we eliminate the "JS-Tax" and close the XSS attack vectors associated with external script loading.

## **2\. The Hydration Headache**

* **Pitfall**: Frameworks like Leptos, Yew, or React require a WASM or JS runtime to "hydrate" static HTML. This results in the "Flash of Inactivity"—a site that looks rendered but is unresponsive for 500ms–2s while the engine reconciles the virtual DOM. In a high-security or low-bandwidth environment like TOR, this "Hydration Gap" makes a site feel broken.  
* **Shallot Solution**: We serve **Terminal HTML**. There is no reconciliation phase. Because our interactivity is baked into the CSS selectors (:target, :checked, :focus-within) at the moment of server-side generation, the site is interactive the microsecond the CSS is parsed. The "Skin" is inseparable from the "Logic."

## **3\. The CSS-in-JS Overhead**

* **Pitfall**: Runtime CSS-in-JS libraries dynamically generate style tags based on component state. This causes "Style Recalculation Thrashing," where the browser must re-parse the CSS tree on every interaction, leading to high CPU usage and "jank" in animations. It also makes themes opaque to the browser's pre-loader.  
* **Shallot Solution**: Styles are **Statistically Compiled**. The "Aura" layer generates a single, minified, static CSS file at server startup or build time. We use CSS Custom Properties (Variables) to handle dynamic state changes. This moves the weight of "Theming" to the browser’s highly optimized native engine rather than a heavy runtime script.

## **4\. The "Semantic div" Syndrome**

* **Pitfall**: Using \<div\> for everything (buttons, links, nav drawers) and "simulating" behavior with JS click handlers. This destroys accessibility, as screen readers cannot identify the purpose of the element, and keyboard users cannot tab through the interface without manual "focus-trapping" scripts.  
* **Shallot Solution**: Strict adherence to **Semantic Integrity**. If it acts like a button, it is a \<button\>. If it navigates, it is an \<a\>. We believe the HTML "Skin" should be a perfect outline of the "Bulb's" logic. This ensures Shallot components are WCAG 2.1 AA compliant out of the box, even in browsers where all CSS and JS are disabled.

## **5\. The "Boolean Soup" Anti-Pattern (State Fragility)**

* **Pitfall**: Managing UI state through dozens of loosely coupled booleans (e.g., is\_loading, is\_error, has\_data). This creates "Impossible States" where a component can accidentally be both loading and error simultaneously, leading to unpredictable UI bugs that are impossible to catch at compile time.  
* **Shallot Solution**: **Finite State Machines (FSM) via Enums**. We utilize Rust’s type system to ensure that a component can only exist in one valid state at a time. A Button state is an enum, not a list of flags. The compiler prevents you from rendering a "Success" icon if the button is currently in the "Loading" state.

## **6\. Layout Shift Fragility (The CLS Trap)**

* **Pitfall**: Relying on "Auto" sizing for images and containers. As assets load, the page "jumps," pushing content down and frustrating the user. Most developers fix this with JS "onLoad" handlers, which only adds to the execution debt.  
* **Shallot Solution**: **Aspect Ratio Pre-Calculation**. Our "Bulb" logic requires dimensions (or ratios) for all media-heavy primitives. We enforce the CSS aspect-ratio property at the component level. This locks the "Skin" into a stable grid before the first byte of an image even arrives, guaranteeing a Cumulative Layout Shift (CLS) of 0.00.

## **7\. The "Magic String" Drift**

* **Pitfall**: Hard-coding values like class="p-4 bg-\[\#f3f3f3\]" throughout a project. This leads to "Aesthetic Decay," where over time, the design system drifts as developers guess at spacing and colors, making global theme updates a nightmare of search-and-replace.  
* **Shallot Solution**: **Typed Design Tokens**. You cannot type a raw pixel value into a Shallot primitive. You must use Spacing::Md or Color::Primary. By making the "Aura" a compile-time requirement, we ensure that the entire 150+ component library remains mathematically synchronized with the central theme.

## **8\. Bling without Substance (Motion Fatigue)**

* **Pitfall**: Excessive, linear animations that distract the user or cause motion sickness. "Plastic" web libraries often use animation for the sake of flair, ignoring the psychological impact of unpredictable movement.  
* **Shallot Solution**: **Bionic Naturalism**. Every animation in Shallot must follow a physical easing curve (cubic-bezier(0.16, 1, 0.3, 1\)) and must inform the user of a state change. Furthermore, we mandate prefers-reduced-motion support at the "Aura" level, ensuring that our "Glass" aesthetics never compromise the user’s physical comfort.