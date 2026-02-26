# **CSS Best Practices for Zero-JS Interactivity**

## **1\. The Selector Toolkit (Zero-JS State Management)**

In Shallot.rs, we treat CSS as a declarative state engine rather than a mere styling layer. To achieve interactivity without a JS runtime, we master the following tools:

* **:target (The Global Entry Point)**: Used for high-level state changes like Modals, Drawers, and Tab switching.  
  * *Implication*: This allows for deep-linking. A user can share a URL that automatically opens a specific modal (/dashboard\#settings).  
  * *Constraint*: To ensure accessibility, we use hidden "close" anchors that reset the hash, and we leverage the "autofocus" attribute on elements within the target to manage keyboard focus shifts.  
* **:checked (The Finite State Machine)**: The primary mechanism for persistent UI states like Toggles, Accordions, and Sidebar expansions.  
  * *Implementation*: We utilize hidden \<input type="checkbox"\> or \<input type="radio"\> elements as the "Bulb" (Logic) and use the adjacent sibling selector (+) or the general sibling selector (\~) to style the "Aura" (Skin).  
  * *Example*: A sidebar toggle uses a checkbox; when :checked, it triggers a transform: translateX(0) on the navigation drawer.  
* **:focus-within (Contextual Interaction)**: Critical for animated input labels and complex dropdowns. It detects if any child element has focus, allowing the parent container to react (e.g., highlighting a whole form group when a field is active).  
* **:hover & Peer Selectors (\~ and \+)**: Used for "Magnetic" effects and spotlight reveals.  
  * *The "Spotlight" Technique*: We use a multi-layered container where a background "glow" follows the mouse position roughly via a large :hover grid or CSS-only proximity detection (using hundreds of tiny transparent divs, though we prefer lighter radial-gradient transitions).

## **2\. Performance & Hardware Acceleration**

Maintaining a 60fps experience is a hard requirement for the "Glass" aesthetic. We optimize the rendering pipeline by adhering to these rules:

* **The "Composite-Only" Rule**: We strictly animate transform and opacity. These properties avoid the "Layout" and "Paint" stages of the browser rendering engine, moving the work directly to the GPU.  
  * *Avoid*: Animating height, width, margin, or top/left causes expensive reflows that result in "jank" on mobile devices.  
* **will-change Strategy**: We apply will-change: transform, opacity only to elements that are frequently interacted with (like buttons or modals). Overusing this can exhaust GPU memory, so it is gated behind our Transition logic in Rust.  
* **Intelligent Rendering with content-visibility**: For data-heavy components like DataGrid or Timeline, we use content-visibility: auto. This tells the browser to skip the rendering of elements currently off-screen, drastically improving the "Time to Interactive" for large documents.  
* **contain Property**: We use contain: layout style; on complex components to isolate them from the rest of the DOM. This ensures that a visual change in a nested Card does not trigger a re-calculation of the entire page layout.

## **3\. The "Refractive" Look (Aura)**

Our aesthetic is "Iron & Glass"—logic wrapped in light. We achieve this through specific optical physics simulated in CSS:

* **Glassmorphism (Backdrop Blurs)**: We use backdrop-filter: blur(12px) saturate(180%) combined with semi-transparent background colors (e.g., rgba(255, 255, 255, 0.05)).  
  * *Refractive Index*: To simulate depth, higher "z-index" elements receive a higher blur value and a lighter border, creating a physical sense of "elevation."  
* **Layered "Ambient" Shadows**: We reject the single-line box-shadow. Every Shallot component uses at least 3-5 shadow layers:  
  1. A sharp, dark "Cast" shadow for proximity.  
  2. A soft, wide "Ambient" shadow for depth.  
  3. An inner "Glow" (inset) to simulate light catching on the edge of the glass.  
* **Specular Border Glows**: box-shadow: inset 0 0 0 1px rgba(255,255,255,0.1). This creates a "micro-border" that makes glass components "pop" against dark backgrounds, essential for the "Iron & Glass" look.

## **4\. Responsive Fluidity & Adaptive Design**

Shallot.rs is built for every screen, from the TOR browser on a desktop to a modern mobile device.

* **The clamp() Typography Standard**: We avoid static media queries for font sizes. Instead, we use font-size: clamp(1rem, 5vw, 2.5rem);. This ensures the "Skin" is intrinsically fluid, scaling smoothly as the viewport changes without sudden "breakpoints."  
* **Aspect Ratio Preservation**: We use the aspect-ratio property on all media containers (Image, Video, Skeleton). This prevents Cumulative Layout Shift (CLS) by reserving the exact space required before the asset loads.  
* **Container Queries (@container)**: Unlike traditional media queries that look at the viewport, we use container queries for our components. This allows a Card component to look different when placed in a narrow Sidebar versus a wide Main section, enabling true modular design.  
* **Logical Properties**: We use padding-inline, margin-block, and border-start instead of left/right/top/bottom. This ensures Shallot.rs is ready for RTL (Right-to-Left) languages out of the box, supporting our goal of universal accessibility.

## **5\. Animation Physics (Bionic Naturalism)**

Motion in Shallot.rs must feel "heavy" and "natural," never mechanical.

* **Cubic-Bezier Easing**: We strictly use cubic-bezier(0.16, 1, 0.3, 1). This specific curve provides a high-velocity start (responding to the user's action instantly) and a long, smooth deceleration (mimicking the friction of a physical object).  
* **Reduced Motion Support**: We wrap all animations in the @media (prefers-reduced-motion: reduce) query. For users with vestibular disorders, we switch the "Aura" from motion-based transitions to simple opacity fades, ensuring the "Iron" logic remains accessible to everyone.