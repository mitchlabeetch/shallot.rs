# **The Complete Petal Catalog (150+ Components)**

This catalog defines the full scope of **Shallot.rs**. Every component listed is a "Petal" branching from the core "Allium" architecture. They are designed to prove that the "Iron" of Rust logic and the "Glass" of CSS aesthetics can replace the modern web's dependency on JavaScript.

## **1\. Primitives & Layout (25 Components)**

The "Bulb" layer. These components enforce structural integrity and spacing governance. They utilize Rust's type system to prevent layout shifts and ensure responsive fluidity via clamp() and CSS Grid.

* **Foundation**: Box (The Universal Primitive), Flex (One-dimensional Layout), Grid (Two-dimensional Logic), Stack (Vertical/Horizontal Spacing), ZStack (Layered/Overlay Layout).  
* **Containers**: Container (Max-width Logic), Section (Semantic Wrapper), Center (Absolute/Flex Alignment), Spacer (Flexible Gap), Divider (Decorative/Semantic Rule).  
* **Constraints**: AspectRatio (Prevention of Layout Shift), Inset (Padding Governance), Bleed (Negative Margin Logic), FullWidth (Viewport Escape), ViewportFixed (Screen-pinned Layout).  
* **Advanced Patterns**: MasonryLayout (CSS-only Column Flow), GoldenRatioGrid (Proportional Layout), AutoGrid (Responsive Repeat Logic), HolyGrailLayout (Sticky Header/Footer/Sidebars), StickyHeader (Hardware Accelerated), StickyFooter, SidebarLayout, ResponsiveHide (Type-safe Breakpoint Visibility), Island (Isolated Component Wrapper), Frame (Border & Inset Logic).

**Implications**: By defining layout primitives in Rust, we eliminate "CSS-drift." Developers cannot "guess" at margins; they must use the design system's spacing scale, ensuring mathematical consistency across the entire application.

## **2\. Typography (15 Components)**

The "Aura" of the content. These components handle fluid scaling and refractive text effects, ensuring readability is maintained across all devices without JS-driven resizing.

* **Headers**: Heading1 through Heading6 (Semantic Hierarchy), Lead (Introductory Text).  
* **Body**: Text (Standard Paragraph), Span (Inline Logic), Code (Monospace Block), Kbd (Keyboard Command Style), Blockquote (Semantic Citation).  
* **Lists**: List (Ordered/Unordered Logic), ListItem (Bullet Logic), DescriptionList (Meta-data Pairs).  
* **Refractive Effects**: Highlight (Glassy Background Marker), GradientText (Multi-stop Aura), AnimatedTextReveal (CSS-only Scroll Trigger), TypewriterText (Keyframe Animation), CapDrop (Initial Letter Logic).

**Implications**: Fluid typography via clamp() ensures that text is always legible. We reject media-query-based jumping in favor of a smooth, "Iron" scaling algorithm.

## **3\. Forms & Inputs (35 Components)**

The most challenging Petal for Zero-JS. We utilize the :invalid, :checked, and :focus-within selectors to provide immediate visual feedback and state management.

* **Standard Inputs**: Input (Text), PasswordInput (Native Toggle), EmailInput, NumberInput, SearchInput, UrlInput, TelInput.  
* **Multi-line**: TextArea (Auto-expanding via CSS Field-sizing), RichText (Pure CSS Formatting).  
* **Selection**: Checkbox, CheckboxGroup, Radio, RadioGroup, Switch (The Checkbox Hack), NativeSelect, GlassSelect (Custom Styled via Peer Selectors), MultiSelect (CSS-hack Tagging).  
* **Complex Interaction**: Slider (Range Logic), RangeSelect (Dual-handle Logic), FileUpload (Drag-over Aura), ImageUpload (Server-rendered Preview Logic), ColorPicker (Native Interface).  
* **Specialized Logic**: PinInput (OTP/Security), CreditCardInput (Pattern Verification), DatePicker (Native Implementation), TimePicker, DateTimePicker.  
* **Structure**: InputGroup (Appended/Prepended Icons), FormLabel (Floating Aura), ErrorMessage (Conditional Visibility via :invalid), HelperText, Fieldset, Legend, ValidationSummary (Server-side Aggregation).

**Implications**: Validation happens on the "Iron" side (Server/Rust) but is reflected instantly on the "Glass" side (CSS). This prevents "Empty Submission" errors and provides a secure, No-JS UX.

## **4\. Navigation & Wayfinding (25 Components)**

Interactive navigation that remains deep-linkable and functional in "Safer" browser environments through clever :target usage.

* **Primary**: Navbar (Top-level Navigation), Sidebar (Collapsible Drawers), BottomNav (Mobile-first Navigation).  
* **Hierarchy**: Breadcrumbs (Path Logic), Pagination (Numeric/Symbolic), Steps (Process Indicator), Wizard (Multi-page Logic).  
* **Menus**: DropdownMenu (Hover/Focus Logic), ContextualMenu (Target-based Reveal), MegaMenu (Grid-based Overlay), CommandPalette (CSS-only Search Simulation).  
* **Indicators**: AnchorScroll (Internal Jump Links), SubNav (Sectional Navigation), Indicator (Active State Marker), SkipLink (Accessibility First), UserMenu (Profile Overlay), PaginationSimple (Next/Prev Only), PaginationComplex (Jump-to-page).  
* **Links**: NavLink (Active State Recognition), LinkCard (Actionable Containers), BackToTop (Scroll-position Awareness), SiteMap (Automatic Generation), TableOfContents (Heading Observer Hack).

**Implications**: Zero-JS navigation ensures that a user can browse the entire architecture of a complex app with JS disabled, maintaining the "Universal Access" pillar of our Manifesto.

## **5\. Overlays & Feedback (25 Components)**

The "Glass" showcase. These components utilize the highest refractive indices and backdrop filters to indicate depth and priority.

* **Overlays**: Modal (The :target Entry), Drawer (Left/Right/Top/Bottom Variants), Popover (Proximity Disclosure), Tooltip (Hover-only Logic).  
* **Messaging**: Alert (Contextual Feedback), Toast (Non-blocking Notifications), Notification (Permanent Messaging), Banner (System-wide Status), StatusBadge.  
* **Loading States**: Spinner (SVG Animation), ProgressBar (Determinate/Indeterminate), SkeletonLoader (Ghost Content), CircularProgress (Gauge Aura), LoadingOverlay (Blocker).  
* **States**: EmptyState (Illustration \+ CTA), ResultSuccess (Celebratory Aura), ResultError (Warning Aura), ResultInfo, ResultWarning.  
* **Interactive Aura**: Badge (Count/Label), Tag (Categorization), Confetti (CSS Particles), Backdrop (Overlay Shadow Logic), ToastProvider (Slot-based logic).

**Implications**: We prove that "Glassmorphism" isn't a performance drain. By using composite-only animations, Shallot overlays feel lighter and more responsive than JS-heavy equivalents.

## **6\. Data Display (25 Components)**

High-performance rendering of information using content-visibility to maintain 60fps scrolling even with massive datasets.

* **Tables**: Table (Semantic Markup), DataGrid (Complex Interactivity), BionicTable (High-density Logic), SortableHeader (Link-based Sorting).  
* **Content**: Card (Universal Container), Accordion (Disclosure Logic), Timeline (Chronological Aura), Statistic (Metric Highlight), AccordionGroup.  
* **Media**: Avatar (User Representation), AvatarGroup (Overlapping Logic), Carousel (Scroll-snap Logic), ImageWithFallback (Iron-logic Loading), VideoPlayer (Native Zero-JS UI).  
* **Hierarchical**: TreeSelect (Nested Disclosure), TreeView (Folder Logic), ListGroup (Clean Aggregation), DescriptionList (Detail Definition).  
* **Cards**: UserCard, PricingTable, ProductCard, ArticleCard, FeatureItem, MetricGroup, CalendarView (Server-rendered Grid).

**Implications**: By leveraging semantic HTML tables and modern CSS Grid, we handle complex data visualization without the "Virtual DOM" overhead that slows down React-based dashboards.

## **7\. Signature Animated Experience (25 Components)**

The "Manifesto in Motion." These components use advanced CSS physics, proximities, and light simulations to provide breathtaking visual quality.

* **Physical Motion**: MagneticButton (Proximity Hover), SpotlightCard (Radial Reveal), TiltCard (Pure CSS Perspective), LiquidButton (SVG Filter Warp).  
* **Environment**: MeshGradientBackground (Procedural Aura), FloatingOrbs (Physics Simulation), ParallaxLayer (Depth Simulation), ScrollReveal (Observation Hack), NeonBorder (Glowing Logic).  
* **Visual Data**: Sparkline (SVG Micro-charts), RefractiveGauge, ProgressBarAnimated, SkeletonPulse.  
* **Texture**: GlassCard (High Refractive Index), FrostedPane (Maximum Blur), SpecularButton (Light-catching Edges), ShadowElevator (Dynamic Depth).  
* **Experience**: Confetti (CSS Particles), TextScramble (Keyframe Shuffle), GlowText (Outer Radiance), MaskedImage (Creative Shapes), StickyReveal (Scroll-linked Aura), HeroSection (The "Iron & Glass" Entry), GlassIcon (Refractive SVG).

**Implications**: This petal is our definitive proof. We show that "Beauty" is a compile-time requirement. If a button can react to a user without JavaScript, the "JS is for beauty" argument is officially dead.