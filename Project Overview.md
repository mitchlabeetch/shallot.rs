# **Shallot.rs 🧅**

### **Iron & Glass: The World-Class Zero-JS Rust UI Library**

**Shallot.rs** is a manifesto written in code. We believe the modern web has become "plastic"—ubiquitous and malleable, yet fragile and bloated. It relies on heavy JavaScript bridges and interpreted runtimes to simulate a stability that should be enforced at the compiler level.

We propose a return to the fundamentals, elevated by modern engineering. We prove that you can have **breathtaking aesthetics**—glassmorphism, advanced physics-based animations, and complex interactivity—using only **Rust** and **Modern CSS**. This is the "Post-JS" movement: logic that is unbreakable, and beauty that is refractive.

## **🚀 Key Features**

* **Absolutely Zero JS**: Shallot.rs produces 0 bytes of client-side JavaScript. It is fully functional on TOR (High Security), NoScript environments, and high-privacy browsers where scripts are a liability.  
* **Iron Logic (The Bulb)**: We utilize Rust's type system and Phantom Types to build Finite State Machines (FSM) for every component. A button doesn't just "have" a loading state; it *is* a loading state that the compiler verifies can only transition to a success or error result.  
* **Refractive Beauty (The Aura)**: Inspired by the depth of Hero UI and the motion of Magic UI, we implement "Bionic Naturalism." We use CSS anchor positioning, backdrop filters, and hardware-accelerated transitions to create interfaces that feel heavy, real, and responsive.  
* **State of the Art Standards**: 100% Type-Safe, WCAG 2.1 AA compliant by default, and optimized for sub-100ms first paints. We trade runtime complexity for compile-time certainty.

## **🏗 Architecture (The Allium Model)**

Like the layers of a shallot, our architecture is fractal and self-contained, ensuring that every component is progressively enhanced and logically sound.

1. **The Bulb (Logic)**: The core nucleus built in pure Rust. This layer defines the component's behavior and valid state transitions. It manages properties like Intensity, Direction, and RefractionIndex.  
2. **The Skin (Structure)**: The physical manifestation using **Maud** templates. We generate ultra-lean, semantic HTML. Our skins are designed with a "Two-Lens View": they look like a clean, logical outline to screen readers and a refractive masterpiece to sighted users.  
3. **The Aura (Style)**: A **Typed Design System**. We reject magic strings like class="p-4". In Shallot, you use Spacing::Md. The Aura engine compiles these tokens into a single, minified CSS variable block that handles theming, light/dark modes, and physics-based easing.

## **🛠 Usage & Implementation**

### **Basic Component Rendering**

Shallot components use a fluent builder pattern to ensure that if your UI compiles, it adheres to the design system.

use shallot\_components::{Button, ButtonVariant, ButtonSize};  
use shallot\_foundation::{Spacing, Hsl};

fn render\_cta() \-\> Markup {  
    html\! {  
        (Button::new("Initiate Sequence")  
            .variant(ButtonVariant::Glass)  
            .size(ButtonSize::Lg)  
            .padding(Spacing::Xl)  
            .render())  
    }  
}

### **Zero-JS Interactivity (The Modal Pattern)**

We achieve complex behavior by leveraging CSS pseudo-selectors as state triggers. Below is how the "Iron" logic maps to "Glass" interaction:

use shallot\_overlays::{Modal, ModalIntensity};

fn settings\_view() \-\> Markup {  
    let content \= html\! { p { "Secure configuration managed by Rust." } };  
      
    html\! {  
        // This modal is triggered by the URL hash \#settings  
        (Modal::new("settings", content)  
            .title("System Settings")  
            .intensity(ModalIntensity::Frosted)  
            .render())  
              
        // The trigger is a standard anchor link  
        a href="\#settings" class="sh-btn-primary" { "Open Settings" }  
    }  
}

## **⚡ Performance Ceiling**

By eliminating the "Parse-Compile-Execute" cycle of JavaScript engines, Shallot.rs achieves a performance profile unreachable by traditional frameworks:

* **TBT (Total Blocking Time)**: 0ms.  
* **CLS (Layout Shift)**: 0.00 (enforced by pre-calculated aspect ratios).  
* **Security**: Immunity to XSS, Prototype Pollution, and client-side logic tampering.

*Part of the "Post-JS" movement. Built with the power of 🦀 Rust and the clarity of 💎 Glass.*