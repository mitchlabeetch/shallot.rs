# **Rust Best Practices for Shallot.rs**

## **1\. State Machine Components (Phantom Types)**

In the "Iron" layer (The Bulb), we reject the "plastic" nature of boolean flags. Booleans like is\_disabled or is\_loading create a state space of ![][image1], most of which are invalid (e.g., a button cannot be both disabled and loading simultaneously). We use **Phantom Types** to enforce mutually exclusive states at compile time.

pub struct Button\<S: ButtonState\> {  
    label: String,  
    variant: Variant,  
    \_state: PhantomData\<S\>,  
}

// State Definitions  
pub struct Idle;  
pub struct Loading;  
pub struct Disabled;  
pub trait ButtonState {}  
impl ButtonState for Idle {}  
impl ButtonState for Loading {}  
impl ButtonState for Disabled {}

impl Button\<Idle\> {  
    pub fn to\_loading(self) \-\> Button\<Loading\> {  
        Button { label: self.label, variant: self.variant, \_state: PhantomData }  
    }  
}

### **Implications**

* **Zero Runtime Overhead**: These types vanish during compilation.  
* **Compile-Time Safety**: An attempt to trigger a "click" handler on a Button\<Loading\> will result in a compiler error, not a runtime exception. This transforms UI logic into a mathematical proof.  
* **Testing**: We no longer need to write unit tests for "what happens if a button is loading and someone clicks it." The compiler has already verified that this path is unreachable.

## **2\. The "Maud" Standard for Structural Skins**

HTML generation must be treated as an extension of the Rust compiler. We use Maud exclusively because it provides **compile-time template validation** and high-performance escaping.

* **Avoid String Concatenation**: Manual string building is an injection vector. Maud ensures all dynamic content is properly escaped, eliminating 100% of XSS vulnerabilities at the "Skin" layer.  
* **Semantic Integrity**: Every component must use the most restrictive semantic tag possible.  
  * Use \<nav\> for navigation, not \<div class="nav"\>.  
  * Use \<button\> for actions, never \<a\> tags with href="\#".  
* **Macro Efficiency**: Maud templates are compiled into optimized Rust code that writes directly to a buffer. This makes Shallot's SSR (Server Side Rendering) significantly faster than JS-based virtual DOM reconcilers.

html\! {  
    section.hero {  
        h1 { (self.title) }  
        @if let Some(sub) \= \&self.subtitle {  
            p.lead { (sub) }  
        }  
    }  
}

## **3\. Builder Pattern & The Allium Model**

Every component is a logic unit (The Bulb) configured via a **Fluent Builder API**. This ensures that even complex components remain ergonomic and discoverable via IDE IntelliSense.

* **Logic Separation**: The constructor (::new()) should only require the "Iron" essentials (e.g., an ID or a Label). All "Glass" aesthetic choices (colors, blurs, animations) are optional modifiers.  
* **Defaults**: Every builder must provide a "Sensible Default" that adheres to the design system's governance.

let modal \= Modal::new("delete-confirm")  
    .title("Are you sure?")  
    .glass\_intensity(Intensity::High) // Refractive Aura  
    .on\_close("\#dashboard")           // Zero-JS navigation  
    .render();

## **4\. Design Token Governance (The Aura)**

We enforce a **Typed Design System** to eliminate "magic strings" and "CSS-drift." If a style is not in the type system, it does not exist.

* **Colors as Hsl**: We do not use Hex or RGB strings. We use a Hsl struct that allows for programmatic derivation of secondary "Glass" colors (e.g., primary.to\_glass()).  
* **Spacing & Elevation**: Use enums like Spacing::Md or Elevation::Refractive. This ensures that a developer cannot accidentally set a padding of 13px when the system requires multiples of 4px.  
* **Theming**: The system generates a single :root block of CSS variables. Components reference these variables, ensuring that a "Theme Swap" (e.g., Light to Dark) is a single-node update in the DOM.

## **5\. Granular Feature Gating**

To support the "Zero-JS" mandate while offering a massive 150+ component library, we utilize hierarchical feature flags in Cargo.toml. This ensures the final binary contains only the code necessary for the specific petals used by the developer.

\[features\]  
default \= \["core"\]  
core \= \["layout", "typography"\]  
forms \= \["core", "validation\_logic"\]  
overlays \= \["core", "animation\_engine"\]  
data\_viz \= \["core", "svg\_generator"\]

### **Implications for Binary Size**

A landing page using only the typography petal should not pay the binary size cost of the data\_viz engine. By gating logic at the crate level, we keep the WASM/Server binary lean, ensuring the "100ms load time" goal remains reachable even as the library grows to enterprise scale.

## **6\. Composition Over Inheritance**

Rust does not support traditional UI inheritance. Shallot.rs embraces this by using **Trait-based Composition**.

* **The Render Trait**: All components must implement maud::Render.  
* **The Transition Trait**: Components that move (Modals, Drawers) must implement a Transition trait that defines their CSS-only entry and exit keyframes.  
* **Component Wrapping**: Higher-order components (like a Card) should accept any impl Render as children, allowing for infinite, type-safe nesting without breaking the "Glass" aesthetic.

## **7\. Logic Validation (Unit Testing the Bulb)**

Because Shallot logic is pure Rust, we can achieve 100% test coverage without a headless browser or Selenium.

* **Snapshot Testing**: We use insta to ensure that structural changes in the "Skin" layer don't result in unintended HTML regressions.  
* **State Verification**: We write unit tests to ensure that a Form state machine correctly transitions from Pristine to Invalid when the Rust-side validation logic fails, long before the HTML is even rendered.

[image1]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABMAAAAXCAYAAADpwXTaAAAA4ElEQVR4Xu2RIQ7CQBRECwGBxrXdViDQSCwah2swcAmOABaLqOEeBIdEcBMIBEFSZptd+jNQYFNHeMlP/s5MJ9vW8/5UoRHH8QaTRVF0YDMMwxb0M9YmMqnOYfqcyzFmghmZPfN9v2t9FN0wa8xFPmP3B1pUSk1Yo3DN5KYyI/xCZKNMs3sQBG3cciX9HIQSGHPS3pbp1zbarkiU8KKsjnMq/CO+qULpQmSeQbCnixAcs+eMKZqx7gxKTii7su4MSrb49R3WncGNlijbkzaU569AycD+PR7OfoQLKpX9+RHuyslQHe1JH7gAAAAASUVORK5CYII=>