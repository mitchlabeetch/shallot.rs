# SHOWCASE_REFACTOR.md

Status: Draft  
Target path: `shallot/shallot_website/`  
Author: Shallot Engineering (refactor lead)

---

## Executive summary

This document is a complete, prioritized, and actionable implementation roadmap to refactor the Shallot showcase website so it:

- Restores readable, accessible hero visuals (no aggressive gradients or unreadable retro effects by default).
- Renders live component previews reliably (not "Live preview" placeholders) for the components implemented in `shallot_components`.
- Restores the code-view usability: full & library tabs, downloads and visible code blocks.
- Replaces the irrelevant "fake community themes" with a zero-JS, accessible, floating theme panel that lets editors preview library components under preselected themes and custom color picks (CSS-only).
- Unifies duplicate category navigation controls into a single consistent component.
- Removes debugging or accidental UI artifacts (e.g., commit reference printed at bottom).
- Keeps the Zero-JS, Maud-first, typed-design-token governance, and accessibility principles intact.

This roadmap covers discovery, design, implementation, testing, deployment and rollback steps.

---

## High-level goals & acceptance criteria

1. Hero background is plain medium gray (neutral mid-tone), high-contrast readable text, and optional retro layer disabled by default (accessible toggle).
   - Acceptance: hero complies with color contrast WCAG AA for title and supporting text; background is a single flat color by default.
2. Component previews show real rendered components where available, not placeholder text.
   - Acceptance: At least 80% of the `SAMPLE_COMPONENTS` in `showcase.rs` render concrete previews; others explicitly declare "preview unavailable" with reasoning metadata.
3. Code tabs reliably show code (full/library) and the radio-tab pattern works without JS in all browsers supported.
   - Acceptance: Tab toggling works using CSS-only selectors in prebuilt `output/index.html` (manual test in latest Chromium & Firefox and NoScript).
4. Theme panel is a no-JS right-floating control that updates CSS variables for previews (radio theme selection + native color pickers).
   - Acceptance: Selecting a theme immediately affects previews in the same HTML using CSS sibling selectors; color pickers adjust CSS variables locally for preview area only.
5. Duplicate category UI is removed and replaced by a single canonical category nav that supports keyboard focus and scroll-to sections.
   - Acceptance: Only one category navigation present in DOM; semantic `<nav>` used; tab stops and skip-links verified.
6. Remove stray debugging lines (commit hashes) from rendered pages.
   - Acceptance: No commit references printed in site footers.

---

## Root causes discovered (short)

- Hero: CSS uses aggressive multi-stop gradients, bright neon text-shadow effects and heavy scanline overlays which reduce legibility.
- Previews: `showcase.rs::render_preview()` falls back to placeholder for component names not handled. Some components are available in the crate but not imported or referenced by `render_preview`.
- Code tabs: CSS sibling selectors assumed radio inputs and code `<pre>` blocks were siblings. The Maud-produced structure nested radios inside `.sh-code-tabs` and `<pre>`s were outside, breaking selectors.
- Theme section: A static "community themes" grid was included as placeholder content and not aligned to the project's No-JS theme control philosophy. It must be replaced with a CSS-only theme panel with radios + color inputs.
- Duplicate category navs: Two independent nav renderings are present (one upstream and one downstream in the template) causing duplicated UI.
- Debug footer line: Build or template included a raw commit reference string due to leftover debug weave from earlier import commits.

---

## Implementation plan (phased)

Priority is to fix visible problems on the public `output/` site quickly (hotfixes), then backport durable, correct architecture changes to the Maud templates and source.

### Phase 0 — Prep & safety (1–2 hours)
- Create a branch: `showcase/refactor/roadmap`.
- Create an emergency backup bundle of current repository (done previously; ensure `original-history.bundle` is present). If not, create `git bundle create original-history.bundle --all`.
- Add/confirm `.gitignore` includes `target/`, `**/target`, `.DS_Store`, and any local build dumps; ensure `output/` is intentionally included (we want the static site).
- Add a short README `SHOWCASE_REFACTOR.md` (this file) at `shallot_website/`.

Deliverables:
- Branch created
- Backup bundle present

---

### Phase 1 — Hot fixes applied to prebuilt `output/` (quick deploy, safe) (2–4 hours)

Objective: Make the live site readable and usable immediately by editing files under `output/` (this is safe because `output/` is used by Vercel to serve).

1. Replace hero styles inside `output/styles/retro.css`:
   - Set hero background to a flat neutral mid-gray `#efefef` or `#f5f7fa` (choose according to theme), remove heavy neon text-shadow, reduce scanlines intensity or set them to `display: none` by default.
   - Ensure `.sh-retro-hero__title` and `.sh-retro-hero__subtitle` use accessible colors; remove Comic Sans emphasis; use system UI fonts; reduce animation intensity.
   - Provide a small "toggle" in the hero to re-enable retro overlay (a hidden checkbox :checked -> show overlays) for authors who want the retro look.

2. Ensure code blocks are visible by default:
   - Set `.sh-code-block { display: block !important; }` as fallback in `output/styles/showcase.css` (temporary but safe).
   - Add a small comment describing this is a temporary fallback until templates are fixed.

3. Remove the accidental commit reference from footer:
   - Edit `output/index.html` to remove the debug string and update footer text.

4. Remove the fake community grid and replace it with a placeholder "Theme panel (enabled via site update)" message pointing to a next-step link in the repo.

Quick tests:
- Open local `output/index.html` (or push to remote and open the site) and visually verify hero readability and code panels visible.

Rationale:
- These are minimal edits to the static artifacts so Vercel will show improved site immediately while source changes are prepared.

---

### Phase 2 — Source fixes (Maud templates & Rust) (3–6 days, depending on polishing)

All edits must be done in `shallot_website/src/` and recompiled to update `output/`.

A. Hero refactor (shallot_website/src/retro_hero.rs)
- Replace heavy gradient & neon text with plain background and accessible typography.
- Provide a CSS-only opt-in retro overlay (checkbox at top of hero: `#retro-toggle:checked ~ .sh-retro-hero .sh-retro-hero__scanlines { opacity: 1; }`).
- Implement accessible headings and skip-links.
- Add tests (insta snapshot) for hero HTML output.

B. Fix code tab markup & CSS (shallot_website/src/showcase.rs + showcase CSS)
- Change markup generation for each component card:
  - Place radio inputs that control code blocks as direct siblings of the code `<pre>` blocks. Example structure to generate:

    - hidden radio `input#full-<id>.sh-code-tab__radio` (checked default)
    - hidden radio `input#library-<id>.sh-code-tab__radio`
    - label[for=full-<id>] (tab label)
    - label[for=library-<id>]
    - pre.sh-code-block--full (sibling)
    - pre.sh-code-block--library (sibling)

  - This ensures selectors like `.sh-code-tab__radio[value="full"]:checked ~ .sh-code-block--full` work as intended.

- Update `showcase_css()` in `shallot_website/src/showcase.rs` (or the central CSS generator) so `.sh-code-block` is hidden by default but shown by sibling radio selectors (remove the fallback `!important` once markup is corrected).

- Add a unit test (insta snapshot) that renders one component card and asserts the radio inputs are siblings of the code blocks.

C. Render previews reliably (shallot_website/src/showcase.rs)
- Audit `render_preview()`:
  - For every `SAMPLE_COMPONENTS` entry, ensure one of the following:
    - A concrete preview is returned (call into the real component's `.render()`).
    - There is explicit metadata indicating "preview intentionally disabled" (explain why) and a helpful placeholder explaining how to enable (e.g., `feature = "forms"`).
  - Import the concrete components used in previews (`use shallot_components::{button::Button, card::Card, ...};`) at top of `showcase.rs`.
  - For components that are missing preview functions, open tickets or implement minimal renderers that show canonical usage (e.g., Button::new("Primary").render()).

- Add test coverage: for each previewable component add a small test that the generated markup contains `aria-label` and expected structure.

D. Merge and canonicalize category navigation
- Find both nav implementations. Choose a single canonical location for the category nav (prefer top-level nav rendered above the component sections).
- Implement a `render_category_nav()` function in `showcase.rs` that produces a single `<nav>` with `<ul role="tablist">` and anchors to the component sections.
- Remove the second nav render to avoid duplication.
- Add skip-links to jump to category navigation.

E. Replace the fake community theme grid with the zero-JS floating theme panel
- Design constraints:
  - Must be No-JS.
  - Must allow selecting pre-defined themes (radio inputs) and custom picks (native `<input type="color">`) that affect preview cards only.
  - Must be keyboard accessible and usable in NoScript environments.

- Implementation sketch:
  - Place a small theme control block near the page root (before the main content) — hidden radios that apply CSS variables via general sibling selector:
    - Example concept (Maud): `input#theme-cyberpunk:checked ~ .sh-site .sh-preview { --sh-accent: ... }`
  - For custom colors: place `<input type="color" id="custom-accent" name="custom-accent">` inside the form and use progressive enhancement: because we can't update CSS variables client-side without JS, use CSS-only fallback:
    - Provide a set of few custom color presets (radios). For advanced users, if server-side build process supports building custom preview pages, allow query-param based preview pages (optional).
  - Implement the panel as a floating `<aside class="sh-theme-panel">` fixed to the right; inside it, provide:
    - Theme radios (prebuilt theme names)
    - Contrast mode radio (auto/light/dark)
    - A "Preview scope" toggler (affects only `.sh-component-card__preview` via sibling selectors)
  - Document the limitation: native color pickers cannot update CSS variables across the site without client-side scripting; to remain strictly Zero-JS we provide a limited set of colors, and optionally a form that submits to the server to regenerate preview output (if you want server rebuilds).

F. Remove debugging artifacts (footer)
- Inspect `shallot_website/src/*` and find the template emitting the commit line (likely from `main.rs` or a leftover debug include).
- Remove or guard it behind `cfg!(debug_assertions)` so it is not present in production builds.
- Add a unit test that the footer does not contain hex-ish commit strings.

---

### Phase 3 — Design & CSS improvements (1–3 days)

- Replace any neon-based, unreadable CSS text-shadow with refined shadows per the project "Aura" guidelines.
- Ensure font fallbacks use system UI fonts and prefer `clamp()` typography rules.
- Implement container queries for card previews so they adapt within narrow contexts.
- Add `content-visibility: auto;` to long lists and `contain: layout` on heavy cards.
- Ensure all CSS variables used are defined in `shallot_foundation::theme` typed system and exported to the generated CSS (no magic strings).

Accessibility:
- For each interactive control (radios, checkboxes, code toggles), ensure:
  - `aria-controls` where appropriate
  - `aria-selected`, `role="tab"`, `role="tablist"` semantics
  - Keyboard navigable (tab, arrow keys for radio groups if used)

Snapshots:
- Snapshot tests for main pages:
  - `showcase::render()` snapshot
  - `retro_hero::render()` snapshot
  - `theme panel` snapshot

---

### Phase 4 — Tests, CI and deploy automation (1 day)

- Add tests:
  - Unit tests for `render_preview()` entries (insta).
  - Integration test that `output/index.html` contains no `<script>` and that key HTML markers exist.
- CI:
  - Add a job to build the website (cargo run --bin shallot_website or static build) and produce `output/`.
  - Add a check that fails the build if:
    - Any file in `target/` is accidentally committed (lint).
    - `output/index.html` contains the commit debug string.
- Vercel:
  - Current `vercel.json` uses `output` as `outputDirectory`. Keep that to serve static content.
  - For future dynamic build on Vercel: add an optional `vercel-build` script that installs Rust with `rustup` and runs `cargo run --bin shallot_website` to regenerate `output/`. Document tradeoffs.

---

## Developer checklist (concrete edits & files)

1. Files to edit (source):
   - `shallot_website/src/retro_hero.rs` — hero HTML + CSS generator
   - `shallot_website/src/showcase.rs` — code tabs markup, render_preview, category nav
   - `shallot_website/src/lib.rs` & `main.rs` — ensure no debug commit string emitted
   - `shallot_foundation/src/theme.rs` — add or ensure theme CSS variable exports used by theme panel
   - `shallot_website/README.md` — document theme control limitations (No-JS)
2. Files to inspect & update (static output)
   - `output/index.html`
   - `output/styles/retro.css`
   - `output/styles/showcase.css`
3. Tests:
   - `shallot_website/tests/render_snapshots.rs` — add insta snapshot tests
   - `shallot_testing` (if used) — add integration tests for showcase

---

## Example markup patterns (Maud) — canonical tab pattern

(Use these exact structural rules when coding `showcase.rs` — radios must be siblings of code blocks.)

```/dev/null/example_showcase_markup.md#L1-14
<!-- Maud-generated structure pattern (illustrative) -->
<input type="radio" id="full-box" name="tabs-box" class="sh-code-tab__radio" value="full" checked>
<label for="full-box" class="sh-code-tab">Full Code</label>

<input type="radio" id="library-box" name="tabs-box" class="sh-code-tab__radio" value="library">
<label for="library-box" class="sh-code-tab">In Library</label>

<pre class="sh-code-block sh-code-block--full"> ... code ... </pre>
<pre class="sh-code-block sh-code-block--library"> ... code ... </pre>
```

Note: The pickers or radios must be inserted in this order to make selectors such as:
`.sh-code-tab__radio[value="full"]:checked ~ .sh-code-block--full { display: block; }`
work correctly.

---

## QA plan

- Manual review:
  - check hero contrast with Lighthouse and axe-core browser devtools
  - open the site in NoScript/No JS mode and verify previews, code tabs, and theme panel behavior
  - test keyboard-only navigation across category nav and code tabs
- Automated:
  - Run `cargo test` for snapshot tests
  - CI lint job to ensure `target/` not committed and `output/` present
  - CI job to fail if commit debug string present
- Cross-browser:
  - Chrome, Firefox, Safari (desktop)
  - Confirm NoScript or Tor Browser (where possible) fallback

---

## Rollout & timeline (suggested)

- Day 0: Branch + backup (Phase 0)
- Day 0 half-day: Hotfix static `output/` (Phase 1) + deploy (fast)
- Days 1–3: Source refactor for tabs, hero, category nav, previews (Phase 2)
- Days 3–4: Theme panel implementation + design polish (Phase 3)
- Day 4: Tests, CI updates & deploy (Phase 4)
- Day 5: QA & small fixes; merge to `main` & trigger Vercel deployment

Total: ~3–5 working days depending on availability and number of preview implementations required.

---

## Risks & mitigations

- Risk: Changing HTML structure breaks external links, anchors, or tests.
  - Mitigation: Keep anchors and IDs stable (use the same `id` for sections), add backwards compatibility comments.
- Risk: Theme control expectations (users expect arbitrary color pickers to set variables globally).
  - Mitigation: Document limitations and provide an "export theme" workflow (server-side build) for custom colors.
- Risk: Large history & repo size causing pushes to fail again.
  - Mitigation: Do not re-add `target/` artifacts; ensure `.gitignore` is correct. Use `original-history.bundle` for archive instead of reintroducing history.

---

## Owner & review

- Primary implementer: (assign team member)
- Reviewer: (UI / accessibility lead)
- QA: (testing lead)

---

## Helpful commands & local dev tips

- Build and preview website locally:
  - Build the site (SSR/static generator) and write `output/`:
    - `cargo run --bin shallot_website` (or project-specific build command)
  - Serve local `output/`:
    - `python3 -m http.server --directory output 8000`
    - Open: `http://localhost:8000/`
- Add a snapshot test:
  - `cargo test --package shallot_website --test render_snapshots -- --nocapture`
- Lint for accidental build artifacts:
  - `git status --porcelain | grep -E '(^|/)target/'` should return nothing
- If you need to create a classic plain hero background change:
  - Edit `shallot_website/src/retro_hero.rs::retro_css()` and set `.sh-retro-hero { background: var(--sh-surface, #f5f7fa); }`

---

## Final notes

- The implementation must align with the project's manifesto: Zero-JS, Maud-first, typed design tokens, accessible and semantic HTML, and compile-time guarantees where possible.
- Prefer small, reversible commits. Each logical change should be its own commit with an explanatory message.
- Keep the `output/` static hotfix step minimal and ephemeral; ensure the source templates are the long-term single source of truth.

If you want, I can now:
- produce the concrete patch for `showcase.rs` to make radio inputs siblings of code blocks (small, focused diff), or
- produce a Maud template snippet for the theme panel (floating, CSS-only),
- or generate unit/snapshot test examples.

Which would you like me to produce next?