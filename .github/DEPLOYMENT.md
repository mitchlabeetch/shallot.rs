# Shallot Showcase — Deployment & CI/CD Guide

## Overview

The Shallot showcase uses a **static site deployment model** with **GitHub Actions CI/CD** for automated testing and Vercel for hosting.

### Architecture

```
Local Development
        ↓
Git Push (showcase/refactor/roadmap or main)
        ↓
GitHub Actions CI Pipeline
  ├─ Test Suite (15 tests)
  ├─ Lint & Quality Checks
  ├─ Security Audit (Zero-JS verification)
  ├─ Accessibility Audit
  └─ Build Output
        ↓
Merge to main (if all checks pass)
        ↓
GitHub Actions Release Workflow
        ↓
Static output/ → Vercel (CDN)
        ↓
Live Site (shallot.vercel.app)
```

## GitHub Actions Workflows

### CI Pipeline (`ci.yml`)

Runs on every push and pull request to verify code quality.

**Jobs:**

1. **test** — Runs all Rust tests
   - `cargo test --all`
   - `cargo test --test render_snapshots` (15 tests)
   - Ensures all functionality works correctly

2. **build** — Builds the showcase binary
   - `cargo build --release --bin shallot_website`
   - `cargo run --release --bin shallot_website` (generates output/)
   - Verifies static output files are generated

3. **lint** — Code quality checks
   - `cargo fmt` — Rust formatting
   - `cargo clippy` — Linting
   - Verifies `target/` not accidentally committed
   - Checks for debug strings in output

4. **security** — Verifies Zero-JS principle
   - No `<script>` tags in HTML
   - No inline event handlers
   - No `unsafe` blocks in source (warnings only)

5. **accessibility** — Verifies accessibility features
   - Skip links present
   - Semantic HTML landmarks
   - ARIA attributes present

**Status Badges:**

Add this to README.md to show CI status:

```markdown
[![CI Status](https://github.com/yourusername/shallot/workflows/CI/badge.svg)](https://github.com/yourusername/shallot/actions)
```

### Release Workflow (`release.yml`)

Runs automatically when code is pushed to `main` branch.

**Steps:**

1. Build showcase binary (release mode)
2. Generate `output/` directory
3. Verify output integrity
4. Verify Zero-JavaScript
5. Deploy to Vercel (if secrets configured)

**Required Secrets:**

Set these in GitHub Settings → Secrets → Repository secrets:

```
VERCEL_TOKEN        — Vercel authentication token
VERCEL_PROJECT_ID   — Vercel project ID
VERCEL_ORG_ID       — Vercel organization ID
```

To obtain these:
1. Go to https://vercel.com/dashboard
2. Select your project
3. Settings → General → Copy Project ID and Org ID
4. Account Settings → Tokens → Create a new token

## Local Development Workflow

### Build the Showcase Locally

```bash
# Build the binary
cargo build --bin shallot_website

# Run and generate output/
cargo run --bin shallot_website

# Output files created:
# - output/index.html
# - output/feed.xml
# - output/styles/main.css
# - output/styles/retro.css
# - output/styles/showcase.css
```

### Preview Locally

```bash
# Option 1: Python HTTP server
cd output
python3 -m http.server 8000
# Visit: http://localhost:8000

# Option 2: Use any other HTTP server
# (serve, http-server, etc.)
```

### Run Tests Locally

```bash
# All tests
cargo test

# Only showcase tests
cargo test --test render_snapshots

# With output
cargo test -- --nocapture
```

## Vercel Configuration

### Current Setup (Static Site)

The `vercel.json` is configured to serve the pre-built `output/` directory:

```json
{
  "version": 2,
  "outputDirectory": "output",
  "installCommand": "echo 'Static site - no dependencies needed'",
  "buildCommand": "echo 'Static output pre-built via GitHub Actions CI'"
}
```

**Why static?**
- Faster deployments (no build on Vercel)
- More reliable (built locally with full Rust toolchain)
- Cheaper (no server-side compute)
- Demonstrates Zero-JS philosophy
- Better caching strategy

### Domain Configuration

To use a custom domain:

1. Go to Vercel project settings
2. Domains → Add Domain
3. Configure DNS records as shown
4. Update this documentation

## Deployment Process

### Step 1: Code Review & Testing

```bash
# Create feature branch
git checkout -b showcase/feature/my-feature

# Make changes
git add .
git commit -m "feat: Description of changes"

# Push to origin
git push origin showcase/feature/my-feature
```

**Wait for GitHub Actions to complete:**
- ✅ All tests pass
- ✅ Lint checks pass
- ✅ Security audit pass
- ✅ Accessibility audit pass

### Step 2: Create Pull Request

Open PR against `main` branch. GitHub Actions will run CI pipeline automatically.

### Step 3: Review & Merge

Once CI passes:
1. Code review by team
2. Merge to `main` branch
3. Delete feature branch

### Step 4: Automatic Deployment

Release workflow runs automatically:
1. Builds showcase binary
2. Generates output/
3. Deploys to Vercel
4. Site is live in ~30 seconds

## CI/CD Checks Explained

### Zero-JavaScript Verification

Ensures no client-side JavaScript:

```bash
grep -E "<script|onclick|onchange" output/index.html
# Should return: (nothing)
```

**Why?**
- Shallot principle: prove beautiful UI without JavaScript
- Better security (no XSS vulnerabilities from client JS)
- Better performance (smaller bundle)
- Works everywhere (even Tor, NoScript)

### No Debug Artifacts

Checks for merge conflicts and debug strings:

```bash
grep -iE "<<<<<<< HEAD|=======|>>>>>>>|commit" output/index.html
# Should return: (nothing)
```

### No Accidentally Committed Build Artifacts

```bash
git ls-files --cached | grep -E "^target/"
# Should return: (nothing)
```

**Why?**
- `target/` can be gigabytes (binary artifacts)
- `.gitignore` prevents this, but we double-check
- Keeps repository lean

## Performance Monitoring

### Build Time

CI pipeline typically completes in **2-3 minutes:**
- Rust compilation: ~60-90 seconds
- Test execution: ~30-60 seconds
- Linting: ~10-20 seconds
- Deployment: ~30 seconds

### Page Load Performance

The static site is highly optimized:

- **HTML**: Minified, no JavaScript
- **CSS**: Loaded from CDN with long cache headers
- **RSS**: Static XML feed
- **Time to First Byte**: < 100ms (Vercel CDN)
- **Lighthouse Score**: 95+

Check performance:
```bash
# Build locally and measure
cargo run --release --bin shallot_website
du -h output/index.html
du -h output/styles/*.css
```

## Troubleshooting

### CI Pipeline Fails

1. **Test failure**
   - Run locally: `cargo test --test render_snapshots`
   - Check error message in GitHub Actions
   - Fix locally, push again

2. **Lint failure**
   - Format code: `cargo fmt`
   - Fix clippy warnings: `cargo clippy --fix`
   - Push again

3. **Output not generated**
   - Build locally: `cargo build --bin shallot_website`
   - Run: `cargo run --bin shallot_website`
   - Verify `output/` directory exists
   - Check file sizes are reasonable

### Vercel Deployment Fails

1. **Missing secrets**
   - Verify `VERCEL_TOKEN`, `VERCEL_PROJECT_ID` in GitHub secrets
   - Or manually deploy via Vercel CLI

2. **Static files not serving**
   - Check `vercel.json` configuration
   - Verify `output/` directory exists in repo
   - Re-run build locally

### Zero-JS Check Fails

If the security audit fails:

```bash
# Check for scripts
grep -n "<script" output/index.html

# Check for event handlers
grep -nE "on[a-z]+=" output/index.html

# Remove the offending lines and rebuild
cargo run --bin shallot_website
```

## Manual Deployment

If automated deployment fails, deploy manually:

### Via Vercel CLI

```bash
# Install Vercel CLI
npm install -g vercel

# Login (first time)
vercel login

# Deploy
vercel --prod
```

### Via Git Push

Simply push to `main` and GitHub Actions will deploy:

```bash
git push origin main
```

## Monitoring & Alerting

### GitHub Actions

Monitor deployment status:
- GitHub repo → Actions tab
- See all workflow runs
- Click failed run for details

### Vercel

Monitor production site:
- https://vercel.com/dashboard
- View deployment status
- Monitor performance
- Check analytics

## Rollback Procedure

If deployment causes issues:

### Option 1: Revert via Git

```bash
# Revert the last commit
git revert HEAD
git push origin main

# CI will automatically redeploy with previous version
```

### Option 2: Manual Rollback

```bash
# Check deployment history
git log --oneline | head -10

# Revert to specific commit
git revert <commit-hash>
git push origin main
```

## Future Enhancements

### Dynamic Build on Vercel

Currently, `output/` is pre-built on GitHub Actions. In the future, you could:

1. Install Rust on Vercel (increases build time & cost)
2. Run `cargo run --bin shallot_website` on Vercel
3. Allow dynamic rebuilds

**Trade-offs:**
- ✅ No need to commit `output/`
- ❌ Longer deployment time (3-5 minutes vs 30 seconds)
- ❌ Higher Vercel costs (server compute)
- ❌ Less reliable (Rust toolchain version issues)

**Current approach is better** for this use case.

### Preview Deployments

Enable preview deployments for pull requests:

1. Go to Vercel project settings
2. Git Integration → Deploy on Pull Request
3. Each PR gets a unique preview URL

## Best Practices

1. **Commit output/** — Pre-built site serves faster
2. **Keep vercel.json simple** — Fewer things can fail
3. **Monitor CI logs** — Catch issues early
4. **Test locally first** — `cargo test` before pushing
5. **Use conventional commits** — Makes history clear
6. **Write descriptive PR messages** — Helps reviewers

## References

- **GitHub Actions Docs**: https://docs.github.com/actions
- **Vercel Docs**: https://vercel.com/docs
- **Rust Toolchain**: https://rustup.rs
- **Shallot Principles**: See `/AGENTS.md`

## Support

For CI/CD issues:
1. Check GitHub Actions logs
2. Run tests locally (`cargo test`)
3. Review this guide for common issues
4. Check Vercel deployment logs
5. Ask in project discussions

---

**Built with principle and care. Zero JavaScript. 🦀✨**