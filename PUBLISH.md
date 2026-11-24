# Publishing otherwise-ui

The crate was built and committed locally (no remote yet). To publish it:

```bash
cd ~/otherwise-ui
gh repo create otherwisetech/otherwise-ui --public --source ~/otherwise-ui --push \
  --description "The Otherwise design system for GPUI apps: tokens and shared chrome"
git tag -a v0.1.0 -m "otherwise-ui 0.1.0"
git push origin v0.1.0
gh release create v0.1.0 --repo otherwisetech/otherwise-ui --title "otherwise-ui 0.1.0" \
  --notes "First release: tokens (Inter, type scale, spacing, radius, light/dark palette, per-app accents) and the shared chrome (Sidebar, AppHeader, AppFooter, AboutModal, PrivacyNote, cards, alert cards, stat tiles, progress bars, empty states). See ADOPTING.md."
```

## Why public, MIT OR Apache-2.0

It is styling only (colours, sizes, layout helpers), with no business logic or secrets, and
the open-source apps (the Hub, Mi Huerta, genealogy, Vigía, Nodo, Rama, diskmap, Mosaico) must be
able to depend on it from public CI and from anyone's clone. The private apps can use a
permissive dependency without restriction. Same dual licence as the rest of the family's
open-source crates (`otherwise-link`, `otherwise-agent-panel`). Inter is bundled under the SIL
Open Font License 1.1 (`assets/fonts/Inter-OFL.txt`).

After publishing, apps depend on it by tag (`ADOPTING.md`), and the suite doc
`docs/diseno-apps.md` already links https://github.com/otherwisetech/otherwise-ui.
