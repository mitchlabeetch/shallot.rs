# **Showcase Layout & Requirements**

The Shallot.rs showcase is not merely a documentation site or a component gallery; it is the **Manifesto in Motion**. It serves as the physical proof that a web experience can be breathtakingly beautiful and highly interactive without a single line of JavaScript. Every animation, state change, and layout shift in this showcase is a victory for the "Iron & Glass" philosophy.

## **1\. The "Laboratory" Grid (Bulb vs. Skin)**

Each component is presented within a "Laboratory" card, designed to highlight the duality of the **Allium Model**.

* **The Live Render (The Aura)**: The top section of the card displays the component in its full refractive glory. It must demonstrate all states (Hover, Focus, Active) purely through CSS. For complex components like the MagneticButton, the render area includes proximity-sensing "ghost" layers to trigger CSS-only motion.  
* **The Zero-JS Source Viewer (The Iron)**: The bottom section displays the Rust source code.  
  * **Highlighter Logic**: Since we reject JS highlighters like Prism.js, the showcase uses a Rust-native, server-side syntax highlighter that wraps tokens in semantic spans (e.g., .sh-keyword, .sh-string).  
  * **Copy-Paste Safety**: A standard \<textarea\> or hidden input combined with the "Checkbox Hack" allows users to "Copy Code" using native browser behavior, ensuring the logic is as accessible as the visuals.  
* **State Matrix**: Beneath the render, a grid of "Logic Toggles" (utilizing radio buttons) allows the user to switch the live render between different states (e.g., Loading, Success, Error) to see how the Rust state machine maps to the CSS Aura.

## **2\. Navigation & Wayfinding (The Wayfarer Petal)**

The navigation architecture of the showcase is a demonstration of secure, scriptless routing.

* **The Sidebar (Structural Anchor)**: A vertical, sticky navigation system grouped by "Petals" (Layout, Forms, Navigation, Overlays, Data, Signature).  
  * **The Disclosure Pattern**: Sub-categories utilize the :checked selector on hidden checkboxes to expand and collapse, providing a fluid, animated interaction that rivals JS-based accordions.  
  * **Mobile Adaptivity**: On smaller viewports, the sidebar collapses into a "Hamburger" drawer triggered by a radio button, ensuring the laboratory is fully functional on mobile TOR browsers.  
* **The CSS-Only Search**: A high-end interaction proof-of-concept.  
  * **Mechanism**: A text input paired with the :placeholder-shown and general sibling selectors (\~) filters the Laboratory cards. While true fuzzy-search is server-side, the client-side "Quick Filter" uses CSS attribute selectors to hide components that don't match the current category or tag, providing instant visual narrowing.

## **3\. The "Glass" Room (Signature Aesthetics)**

The "Glass Room" is a dedicated, high-intensity immersive environment designed to showcase Shallot’s most advanced refractive effects. It is the "stress test" for our Zero-JS animations.

* **The Magnetic Field**: A section where buttons appear to follow the user's cursor. This is achieved by a hidden grid of transparent "Proximity Divs" that, when hovered, use the \+ selector to shift the transform coordinates of the button in the center. It proves that "Physical" motion is a matter of layout, not math.  
* **The Frosted Pane (Depth Stack)**: A live demo of backdrop-blur stackability. Multiple modals and drawers are opened simultaneously using nested :target hashes to demonstrate how Shallot handles "Refractive Elevation" without slowing down the browser's composite thread.  
* **The Mesh Background Generator**: A procedural background engine. Users can adjust "Bulb" parameters (Rust const values) which are re-rendered server-side to update the radial-gradient stops of the page background, simulating a real-time "Design Studio" experience.

## **4\. Real-World Mockups (Proof of Scale)**

To move beyond isolated components, the showcase features three full-page mockups built entirely with the library.

* **The Bionic Dashboard**: A data-dense environment featuring the DataGrid, Sparklines, and Statistic components. It utilizes content-visibility: auto to prove that Shallot can handle 1,000+ rows of interactive data at 60fps with zero JS lag.  
* **The Refractive Landing Page**: A marketing-focused page demonstrating "Bionic Naturalism." It features ScrollReveal animations triggered by the CSS view() timeline and SpotlightCards that glow as they enter the viewport.  
* **The Documentation Portal**: A content-heavy layout proving the library's readability. It includes the TableOfContents (using anchor-link :target highlighting) and Kbd command displays, ensuring that developer documentation is as beautiful as the code it describes.

## **5\. The "Tor Compliance" Bench**

A sidebar widget that displays real-time performance metrics for the showcase itself.

* **JS Byte Count**: Always displays 0.00 KB.  
* **Security Headers**: Displays the current CSP (Content Security Policy) which is set to script-src 'none', serving as a permanent badge of honor for the project’s security stance.  
* **Lighthouse Simulator**: Displays pre-cached Lighthouse scores for the page to remind visitors that beauty and performance are not mutually exclusive.