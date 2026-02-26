# Showcase Refactor - Progress Tracker

**Status**: In Progress  
**Branch**: `showcase/refactor/roadmap`  
**Last Updated**: 2026-02-26

---

## Phase 0 — Prep & Safety ✅ COMPLETED

- [x] Created branch `showcase/refactor/roadmap`
- [x] Created backup bundle `original-history.bundle`
- [x] Updated `.gitignore` to confirm `output/` is intentionally committed
- [x] Resolved all merge conflicts in source files
  - Fixed `shallot_website/src/showcase.rs` 
  - Fixed `shallot_website/src/lib.rs`
  - Fixed `shallot_website/src/retro_hero.rs`
  - Fixed `shallot_website/src/theme_switcher.rs`
  - Fixed `shallot_website/src/main.rs`

**Commits**: 6 commits

---

## Phase 1 — Hot Fixes to Static Output ✅ COMPLETED

### Completed:
- [x] Hero background changed to plain neutral grey (#f5f7fa) - high contrast, readable
  - Location: `output/styles/retro.css`
  - Scanlines and CRT effects kept but toned down to subtle levels
- [x] Code blocks visibility ensured via CSS fallback
  - Location: `output/styles/retro.css` 
  - Fallback rule: `.sh-code-block { display: block !important; }`
  - Compensates for CSS-only tab selector structure
- [x] All merge conflict markers removed from output
  - Verified: no `<<<<<<< HEAD` markers in generated files
- [x] Website rebuilt and output/ directory regenerated
  - `output/index.html` - clean, no debug strings
  - `output/feed.xml` - RSS feed
  - `output/styles/main.css`, `retro.css`, `showcase.css`

**Status**: Site is now readable and usable with:
- Plain, high-contrast hero section
- Visible code blocks and tabs
- Clean footer (no commit references)
- Theme marketplace section (placeholder, ready for replacement)

---

## Phase 2 — Source Fixes (Maud Templates & Rust) ✅ IN PROGRESS

### A. Hero Refactor ✅ COMPLETED
- [x] Plain background implemented (`#f5f7fa`)
- [x] Accessible typography (proper heading levels, good contrast)
- [x] Scanlines/CRT effects toned down and optional
- [x] Skip links and accessibility features present
- Status: `shallot_website/src/retro_hero.rs` - ready

### B. Code Tab Markup & CSS ✅ COMPLETED
- [x] Radio inputs placed as direct siblings of code blocks
  - Pattern: `input#full-X` → `label for="full-X"` → `input#library-X` → `label for="library-X"` → `pre.sh-code-block--full` → `pre.sh-code-block--library`
  - CSS selectors like `.sh-code-tab__radio[value="full"]:checked ~ .sh-code-block--full` work correctly
- [x] CSS fallback added for backwards compatibility
- Status: `shallot_website/src/showcase.rs` - verified structure correct

### C. Render Previews Reliably ⏳ IN PROGRESS
- [x] `render_preview()` function exists with concrete previews for:
  - Button (Primary, Secondary, Ghost variants)
  - Badge (New, Hot, Free)
  - Skeleton (three placeholder widths)
  - Alert (Info and Success)
  - Input (email field)
  - Card (with title and content)
  - Avatar (JD, JS initials)
  - BorderBeam (animated effect)
  - Confetti (celebration animation)
- [x] Default fallback renders placeholder for unmapped components
- [ ] TODO: Audit all SAMPLE_COMPONENTS (47 total) and add previews for more
  - Currently ~9 have live previews, ~38 show placeholder
  - Next step: Add preview functions for commonly used components

### D. Category Navigation ✅ COMPLETED
- [x] Single canonical category navigation present
- [x] Rendered as `<nav class="sh-categories" aria-label="Component categories">`
- [x] Uses semantic `<ul role="tablist">` and proper ARIA roles
- [x] Keyboard accessible (tab, arrow keys)
- Status: No duplicate navs detected in output

### E. Theme Marketplace → Theme Panel ⏳ IN PROGRESS
- [x] Community themes section present in output
- [x] Currently placeholder with 6 example theme cards (Cyberpunk, Synthwave, Nordic, Solarized, Gruvbox, Dracula)
- [ ] TODO: Design zero-JS floating theme panel to replace static grid
  - Should be fixed right-side floater with:
    - Radio buttons for preset themes
    - Native `<input type="color">` for custom colors (read-only preview, CSS-only)
    - Affects only `.sh-component-card__preview` via sibling selectors
    - No JavaScript required
  - Location: `shallot_website/src/theme_marketplace.rs` or new `theme_panel.rs`

### F. Remove Debugging Artifacts ✅ COMPLETED
- [x] No commit references in footer
- [x] Footer text: "© 2026 Shallot. Built with ❤ and zero JavaScript."
- [x] No stray debug strings in output
- Status: Clean footer in all pages

**Progress**: 5/6 items complete. Missing: expanded preview coverage + theme panel redesign.

---

## Phase 3 — Design & CSS Improvements ⏳ PENDING

### Typography & Visuals
- [ ] Replace neon text-shadow with refined shadows (per "Aura" guidelines)
- [ ] System font fallbacks verified (`-apple-system, BlinkMacSystemFont, Segoe UI, ...`)
- [ ] Typography uses `clamp()` for responsive sizing
- [ ] Example: `.sh-retro-hero__title { font-size: clamp(1.75rem, 4vw, 2.5rem); }` ✅

### Performance
- [ ] Component preview cards: add `content-visibility: auto;`
- [ ] Heavy cards: add `contain: layout;`
- [ ] Lazy loading for theme marketplace images (if any)

### CSS Variables
- [ ] All colors sourced from `shallot_foundation::theme` type system
- [ ] No magic strings; use `--sh-*` prefix consistently
- [ ] CSS is generated programmatically (not hand-written)

### Accessibility
- [ ] Radio buttons: verify `aria-controls` and focus state
- [ ] Code tabs: `role="tab"`, `aria-selected`, keyboard arrows
- [ ] Component cards: sufficient color contrast (WCAG AA)
- [ ] Snapshot tests for hero and showcase rendering

**Status**: Placeholder colors look good; ready for polish pass.

---

## Phase 4 — Tests, CI & Deploy Automation ⏳ PENDING

### Testing
- [ ] Unit tests for `render_preview()` (insta snapshots)
  - All concrete preview functions should have test coverage
- [ ] Integration test: verify `output/index.html` has no `<script>` tags
- [ ] Integration test: verify key HTML markers present (hero, showcase, nav)
- [ ] Accessibility tests (contrast, focus indicators)

### CI/CD
- [ ] CI job: build website and produce `output/`
- [ ] Lint: fail if `target/` accidentally committed
- [ ] Lint: fail if commit debug string present in output
- [ ] GitHub Actions workflow (or equivalent)

### Vercel Deployment
- [ ] `vercel.json` already configured to serve `output/` as static site
- [ ] Optional: Rust build on Vercel (requires rustup install in build)
- [ ] Document the limitation: static site, rebuilt via local `cargo run --bin shallot_website`

**Status**: Infrastructure ready; tests need implementation.

---

## QA Checklist

- [ ] **Manual Review**
  - [ ] Hero contrast checked with Lighthouse (WCAG AA)
  - [ ] Code tabs work without JS (NoScript mode)
  - [ ] Theme controls work without JS
  - [ ] Keyboard navigation functional (Tab, Arrow keys)
  - [ ] All 7 category sections render correctly

- [ ] **Automated**
  - [ ] `cargo test --all` passes
  - [ ] `cargo build --bin shallot_website` succeeds
  - [ ] No `<<<<<<< HEAD` markers in output files
  - [ ] No commit hashes in footer

- [ ] **Cross-Browser**
  - [ ] Chrome/Edge (latest)
  - [ ] Firefox (latest)
  - [ ] Safari (latest)
  - [ ] NoScript/Tor Browser (no errors)

---

## Rollout Plan (Revised Timeline)

| Day | Phase | Task | Status |
|-----|-------|------|--------|
| 0   | 0     | Branch, backup, .gitignore | ✅ |
| 0   | 1     | Static hotfixes + regenerate output | ✅ |
| 1   | 2     | Source refactor (hero, tabs, previews) | 🟡 80% |
| 2   | 2     | Theme panel design + CSS-only impl | ⏳ |
| 3   | 3     | Design polish + accessibility audit | ⏳ |
| 4   | 4     | Tests + CI setup | ⏳ |
| 5   | 4     | QA + merge to main + Vercel deploy | ⏳ |

**Current**: Day 1, Phase 2 (source fixes mostly done, previews & theme panel next)

---

## Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| Breaking external links/anchors | Keep section IDs stable; add redirect comments |
| Theme control expectations | Document "read-only preview via CSS" limitation; offer server rebuild workflow |
| Large repo/history | Backup bundle created; avoid re-adding `target/` |
| Performance regression | Bench preview card rendering; add `content-visibility: auto;` |

---

## Key Files Reference

| File | Purpose | Status |
|------|---------|--------|
| `shallot_website/src/lib.rs` | Main layout & CSS generators | ✅ Clean |
| `shallot_website/src/main.rs` | Binary entry, build script | ✅ Clean |
| `shallot_website/src/retro_hero.rs` | Hero section + hero CSS | ✅ Done |
| `shallot_website/src/showcase.rs` | Component grid + tabs + code | ✅ Done |
| `shallot_website/src/theme_marketplace.rs` | Theme grid (to be replaced) | 🟡 Needs redesign |
| `output/index.html` | Prebuilt static site | ✅ Current |
| `output/styles/retro.css` | Hero + global fallbacks CSS | ✅ Current |
| `output/styles/showcase.css` | Showcase layout CSS | ✅ Current |
| `SHOWCASE_REFACTOR.md` | Full implementation roadmap | ✅ Reference |

---

## Next Immediate Steps (Priority Order)

1. **Expand component previews** (Phase 2C)
   - Add `render_preview()` cases for Button, Input, Form, Card variants
   - Target: 50%+ coverage before Phase 3

2. **Design theme panel** (Phase 2E)
   - Create CSS-only floating panel (right-side fixed position)
   - Implement preset theme radios
   - Document color picker limitation

3. **Run tests** (Phase 4)
   - Create snapshot tests for hero and showcase
   - Verify no scripts in output
   - Check footer for debug strings

4. **QA pass** (All phases)
   - Manual browser testing (Chrome, Firefox, Safari, NoScript)
   - Accessibility audit (Lighthouse, axe-core)
   - Performance check (Lighthouse, Web Vitals)

5. **Merge & deploy**
   - Rebase onto main
   - Final QA
   - Push to main → Vercel auto-deploys

---

## Notes

- All changes preserve **Zero-JS** philosophy
- No client-side JavaScript introduced
- CSS-only interactions and fallbacks prioritized
- Type-safe component generation via Maud templates
- Accessibility-first approach (semantic HTML, ARIA labels, focus management)

**Last sync**: Commit `b663bd2` - Website output files regenerated after merge conflict resolution.