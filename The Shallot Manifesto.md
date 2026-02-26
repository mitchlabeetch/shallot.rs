# **Principles & Philosophy: The "Iron & Glass" Manifesto**

## **1\. The Core Duality**

**Shallot.rs** is built on the convergence of two seemingly opposing forces, creating a synthesis that defines the next era of web architecture:

* **Iron (The Bulb \- Logic)**: The unbreakable structural integrity of Rust. We reject the "plastic" nature of modern JavaScript frameworks where state is often a suggestion rather than a rule. By utilizing the borrow checker, phantom types, and zero-cost abstractions, we ensure that UI state is mathematically correct at compile time. In Shallot, a "Press" state isn't just a boolean—it is a type-safe transition that the compiler guarantees can only happen from an "Idle" or "Hover" origin. This is engineering for the long term; logic that does not rot.  
* **Glass (The Aura \- Aesthetics)**: The fluid, refractive, and transparent beauty of the modern web. We prove that light, depth, and motion do not require the overhead of a heavy JavaScript runtime or the "bridge" latency of interpreted scripts. Glass represents our commitment to visual excellence—utilizing backdrop blurs, layered specularity, and physical light simulation. We believe that a user's interface should feel like a premium material—a piece of frosted glass or polished metal—rather than a flat arrangement of digital pixels.

## **2\. The Allium Model**

We organize our components like the layers of a shallot, ensuring that every piece of the UI is fractal, self-contained, and progressively enhanced:

1. **The Bulb (Pure Logic)**: The core nucleus. This layer consists of Rust structs and enums representing the finite state machines (FSM) of a component. For example, a Button isn't just a tag; it is a logic unit that manages Idle, Hover, Loading, and Disabled states with mathematical certainty. By keeping logic in the Bulb, we allow the component to be tested and validated without ever opening a browser.  
2. **The Skin (Structure)**: The physical manifestation. Using optimized HTML generated via Maud, we produce markup that is semantic, accessible, and ultra-lightweight. The Skin layer maps the Bulb's states to the DOM. It ensures that a Loading state in Rust results in an aria-busy="true" attribute in HTML, making structural integrity a byproduct of the logic.  
3. **The Aura (Refractive Style)**: The sensory experience. This is a typed design system that generates valid, minified CSS. Visual beauty is not a "post-processing" step in Shallot—it is a hard requirement. The Aura handles the "refraction" of the component, defining how it reacts to light, shadows, and user proximity through CSS Custom Properties and hardware-accelerated transitions.

## **3\. The Zero-JS Mandate**

We reject the "JS-Bridge" entirely, viewing client-side JavaScript as a legacy addiction that compromises security and performance.

* **Interactivity**: We achieve rich, "app-like" behavior through the creative exploitation of CSS pseudo-selectors. We use :target for deep-linked modals and drawers, :checked for state-persistent toggles and accordions, and :focus-within for interactive form groups. This ensures that the interface is alive the millisecond the HTML is parsed.  
* **Performance & Environmental Impact**: If a site doesn't load in under 100ms on a TOR browser with NoScript enabled, we have failed. By eliminating the "parse-compile-execute" cycle of JavaScript, we reduce the CPU cycles required to view the web. This makes Shallot.rs the most resource-efficient choice for low-power devices and high-security environments.  
* **Security & Sovereignty**: By eliminating JS, we eliminate 90% of the modern web's attack surface. There are no cross-site scripting (XSS) vectors in a static CSS environment. There is no prototype pollution. There is no tracking script hidden in a vendor bundle. Shallot.rs restores sovereignty to the user, providing a "Read-Only" structural guarantee with "Read-Write" visual feedback.

## **4\. Bionic Naturalism**

Our design language avoids the "robotic" and "sterile" feel of many utility-first CSS frameworks. We believe the digital world should mimic the physical laws of nature:

* **Organic Curves**: The world is not made of 90-degree angles. We use 12px-24px radii as our standard, creating a "hand-held" feel that reduces visual fatigue and makes complex dashboards feel approachable.  
* **Physical Depth**: We use multi-layered shadows and backdrop filters (Glassmorphism) to simulate real-world objects. Instead of a flat "z-index," we think in terms of "Refractive Elevation"—how much light passes through a pane of digital glass to the layers beneath it.  
* **Intentional Motion**: Animation is information. Every transition in Shallot follows a cubic-bezier(0.16, 1, 0.3, 1\) easing pattern. This mimics the physics of a physical object being moved by a human hand—a fast, energetic start followed by a smooth, decelerating finish. We avoid "linear" motion because nature is never linear.

## **5\. Accessibility as a Pillar**

In the Shallot philosophy, beauty is not an excuse for exclusion; it is the vehicle for it. Every component must pass WCAG 2.1 AA standards as a baseline.

* **The Two-Lens View**: We design with a dual perspective. Screen readers and assistive technologies see the "Iron" structure—a clean, semantic, and perfectly labeled HTML outline. Sighted users see the "Glass" aura—the shimmering, animated, and refractive interface.  
* **Universal Access**: By making JS optional, we ensure that users on high-security networks, older hardware, or restricted browsers have the exact same level of functional access as those on the latest high-end devices. Accessibility is the ultimate proof of a robust architecture.