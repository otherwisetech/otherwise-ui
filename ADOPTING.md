# Adopting otherwise-ui

Every Otherwise desktop app adopts this crate. The values were taken from what the apps already
share (the Hub, `otherwise-apps`, is the reference; where apps diverge the Hub's value or the
majority's was chosen), so for most apps adoption is a few pixels and a colour or two. Small
visual changes are expected and accepted.

The design rules are in the suite repo: `docs/diseno-apps.md` (Spanish).

## 1. Depend on it

Once `v0.1.0` is published (see `PUBLISH.md`):

```toml
# Cargo.toml (workspace)
[workspace.dependencies]
otherwise-ui = { git = "https://github.com/otherwisetech/otherwise-ui", tag = "v0.1.0" }
```

```toml
# crates/app/Cargo.toml
otherwise-ui.workspace = true
```

The crate depends on `gpui` and `gpui-component` by the same git URLs as the apps, without a
`rev`, so the app's `Cargo.lock` keeps pinning zed `eb5483528be6` and gpui-component `972a3ebfd01a`
and only one copy is built. Add it with `cargo update -p otherwise-ui` (never a bare
`cargo update`), then check that `git diff Cargo.lock` touches only `otherwise-ui`.

## 2. Wire it

```rust
application.run(move |cx| {
    gpui_component::init(cx);
    otherwise_ui::init(otherwise_ui::accents::MI_HUERTA, cx); // accent + Inter
    otherwise_ui::apply(otherwise_ui::Appearance::parse(settings.theme.as_deref()), None, cx);
    // …
});
```

In renders: `let t = otherwise_ui::theme(cx);`. The `Theme` fields keep the apps' `Tokens`
names (`canvas`, `surface`, `raised`, `border`, `border_strong`, `ink`, `ink_muted`, `accent`,
`accent_soft`, `danger`, `danger_soft`, `ok`, `brand`, `dark`), plus `ink_faint`, `warn`/`warn_soft`
(the Hub's `update`), `ok_soft`, `info`/`info_soft`, `backdrop`; `t.on_fill()` still works.

A shim keeps the diff small on day one:

```rust
// theme.rs, while the app migrates
pub use otherwise_ui::Theme as Tokens;
pub use otherwise_ui::tokens::SANS;
pub fn tokens(cx: &gpui::App) -> Tokens { otherwise_ui::theme(cx) }
```

**Servers** (0.1.2): an app that can work on an Otherwise server's documents puts
`ServerSwitcher` in `Sidebar::header` (instead of `brand`): Local plus the servers it knows,
each with a state (`ServerState`: the tile's dot), its own words (`ServerLabels`, or
`ServerLabels::for_locale`) and callbacks (`on_select`, `on_add`, `on_settings`, `on_remove`).
It does no networking; the app gets its servers from `otherwise-remote`.

Suites with one accent per tool (Voltia, Surco, Brújula) replace their `set_accent` with
`otherwise_ui::set_accent(accent, cx)` and their `Accent` with `otherwise_ui::Accent` (same
fields: `light`, `dark`, `brand`).

## 3. Delete from the app

- `theme.rs`: the `Tokens` struct, `light()`, `dark()`, `c()`, `a()`, `apply()`, `SANS`, `MONO`,
  `Accent`, `set_accent`. Keep only domain colours that live **inside content** (chart series,
  genealogy's male/female, CACFU's copper, Helio's sun/grid/battery) in a small `domain.rs`.
- `ui.rs`: `icon`, `glyph`, `outline_button`, `filled_button`, `icon_button`, `section_title`,
  `chip`, `segment`, `segmented`, `link`, `labelled`, `panel`/`card`, `banner`, `stat`,
  `logo_glyph`/`logo_mark`. Keep app-specific helpers.
- The hand-written top bar, sidebar, footer and About: replace with `AppHeader`, `Sidebar` /
  `NavGroup` / `NavItem`, `AppFooter`, `AboutModal`; the modal frame with `modal`.
- `assets/fonts/Inter-*.ttf` and the `add_fonts` call (the crate embeds Inter). Keep the app's own
  icons and logo.
- i18n: keep the app's own keys for About, footer, Vcity and privacy (the components take strings
  from the caller); align their text with `Labels` (es/en/pt/fr): **"Hecho por {company}"**,
  "Tus datos están en este computador".

## 4. Order and per-app changes

Adopt in this order, one app per PR, each **after that app's current agent work has merged**
(other agents are editing most repos). Verify with the app's tests and a look in light and dark.

### Wave 1: the reference

| # | App | Accent | What changes visually |
|---|---|---|---|
| 1 | **Otherwise Apps (Hub)** | `HUB` petrol | Almost nothing: `accent_soft` becomes a 11 % tint (was solid `#E0EEEE`); footer adds "· Acerca de"; settings title 17 → 16; adds the privacy note and an `ink_faint` for hints. Proves the API; fix the crate here before wave 2. |

### Wave 2: open-source apps

| # | App | Accent | What changes visually |
|---|---|---|---|
| 2 | **Mi Huerta** | `MI_HUERTA` | Neutrals lose their green tint (canvas `#F3F2EC` → `#F2F1EE`, borders, ink); buttons 30 → 32, radius 8 → 9; sidebar 212 → 232; panel radius 12 → 14; About 440 → 420, logo 84 → 88; credit "Hecha por" → "Hecho por"; footer credit moves right; keeps its rain colour as `info`. |
| 3 | **genealogy** | `GENEALOGY` | Canvas `#EFEDE8` → `#F2F1EE`, dark surface `#1F1E1C` → `#1C1B1A`; buttons 36 → 32, radius 10 → 9; top bar 52 → 56; footer 24 → 28; "Desarrollado por" → "Hecho por". Tree colours stay (content). |
| 4 | **Vigía** | `VIGIA` | Danger `#B3361F` → `#A3382A`, warn `#C2620A` → `#9A5B0C`; buttons 34 → 32, radius 10 → 9; top bar 52 → 56; sidebar 210 → 232, rows 32 → 34, sub-rows 12.5 → 12; credit wording; gains the Vcity invitation in About. |
| 5 | **Nodo** | `NODO` | Buttons 34 → 32, radius 10 → 9; top bar 52 → 56; footer 26 → 28; 11.5 px text → 11 or 12; credit wording; gains the Vcity invitation. |
| 6 | **diskmap** | `DISKMAP` (today = genealogy's green; consider its own) | Canvas and dark surface as genealogy; buttons 34 → 32; top bars 52/64 → 56; credit wording. |
| 7 | **Rama** | `RAMA` | Buttons 34 → 32, radius 10 → 9; top bar 52 → 56; footer 26 → 28; its sidebar (260, a file tree) keeps its width as a content panel, but the app's navigation follows `Sidebar`; credit wording. |
| 8 | **Mosaico** | `MOSAICO` | Warn `#9A6408` → `#9A5B0C`; buttons 34 → 32; top bar 52 → 56; footer 26 → 28; timeline sidebar 236 stays as content; credit wording. |

### Wave 3: private apps

| # | App | Accent | What changes visually |
|---|---|---|---|
| 9 | **expense me** | `EXPENSE_ME` green (was terracotta `#B9482F`) | Neutrals `#F4F1EC` → `#F2F1EE`; accent terracotta → green (the editions redesign); sidebar 228 → 232, rows text 13.5 → 13, group titles 10.5 → 11; buttons 34 → 32; footer "Hecho por {company}" → the standard credit. The redesign agent already targets these tokens. |
| 10 | **expense Business** (expense-rs) | `EXPENSE_BUSINESS` navy (was teal `#0E7468`) | Accent teal → navy; warn `#9A6408` → `#9A5B0C`; buttons 34 → 32; top bar 52 → 56; 12.5 px → 12; licence line stays proprietary. |
| 11 | **Helio** | `HELIO` amber | Neutrals `#F4F1EA` / dark `#12110F` → the family's; danger `#B3412F` → `#A3382A`; top bar 64 → 56; footer 26 → 28; 11.5 / 12.5 / 13.5 px → the scale; nav radius 10 → 9. Energy colours stay (content). |
| 12 | **Brújula** | `BRUJULA` | Neutrals `#F3F0EA` family → the Hub's (slightly cooler); danger and warn to the family's; nav rows 32 → 34; footer 26 → 28; About 470 → 420. Its `Accent`/`set_accent` map 1:1. |
| 13 | **OS CACFU** | `CACFU` copper | Same neutrals change as Brújula; sidebar 264 → 232 (two-line rows 40 → 34 single line, detail in a tooltip or the content); footer credit. |
| 14 | **Surco** (launcher + tools) | `SURCO` + its tool accents | Same neutrals change; sidebar 268 → 232 (embedded 212 stays via `Sidebar::width`); "Un producto de" → "Hecho por"; `ui-common` shrinks to suite-specific pieces (document shell, tables, memo). |
| 15 | **Voltia** (launcher + tools) | `VOLTIA` slate + tool accents | Like Surco: neutrals, sidebar 268 → 232, rows 42 → 34, credit wording; `ui-common` keeps the shell and the engineering widgets. |

Boardera is not part of this family: never touch Boardera's plate.

## 5. Checklist per app

- [ ] `otherwise-ui` by tag; `Cargo.lock` diff only adds it.
- [ ] `init(accent)` + `apply(appearance)`; no own `Tokens`, no own Inter files.
- [ ] Header / sidebar / footer / About / modal from the crate.
- [ ] Credit "Hecho por Otherwise Technologies LLC"; footer "<App> <versión> · Otherwise Technologies LLC · Acerca de".
- [ ] Privacy note under the sidebar (or in the footer without one).
- [ ] Vcity invitation in About and the Help menu.
- [ ] AI panel on ⌘J, docked right, `PanelStyle` from the theme.
- [ ] No 11.5 / 12.5 / 13.5 px text, no card shadows.
- [ ] Light and dark checked; tests and clippy clean.
