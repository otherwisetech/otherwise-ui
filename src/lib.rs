//! # otherwise-ui
//!
//! The Otherwise design system for GPUI apps: one family, the same chrome everywhere, a
//! per-app accent at most.
//!
//! - [`tokens`]: type scale, spacing, radius, sizes, icon sizes, motion.
//! - [`theme`]: the shared light / dark palette, [`Accent`], [`Theme`], the appearance switch.
//! - [`components`]: the shared chrome ([`Sidebar`], [`AppHeader`], [`AppFooter`],
//!   [`AboutModal`], [`PrivacyNote`]) and building blocks (buttons, card, alert card, stat tile,
//!   progress bar, empty state, chips, segmented control).
//! - [`labels`]: the crate's own words in es / en / pt / fr, for apps that want them. Every
//!   component takes its text from the caller, so the app's i18n stays in charge.
//!
//! ```ignore
//! application.run(|cx| {
//!     gpui_component::init(cx);
//!     otherwise_ui::init(otherwise_ui::accents::MI_HUERTA, cx);
//!     otherwise_ui::apply(otherwise_ui::Appearance::parse(settings.theme.as_deref()), None, cx);
//!     // …
//! });
//! // in a render:
//! let t = otherwise_ui::theme(cx);
//! ```
//!
//! The rules behind these values are in the family's design document (`docs/diseno-apps.md` in
//! the suite repo); this crate is their source of truth in code.

pub mod components;
pub mod fonts;
pub mod labels;
pub mod theme;
pub mod tokens;

pub use components::*;
pub use labels::Labels;
pub use theme::{Accent, Appearance, Theme, accent, accents, apply, set_accent, theme};

/// The company every app credits.
pub const COMPANY: &str = "Otherwise Technologies LLC"; // i18n-ok: company name
/// The company's website.
pub const COMPANY_URL: &str = "https://otherwisetech.com";
/// Vcity, invited from About, the Help menu and under content.
pub const VCITY_URL: &str = "https://vcity.io";

/// Vcity's link with the app's campaign tags (`app_id` like `mi-huerta`).
pub fn vcity_url(app_id: &str) -> String {
    format!("{VCITY_URL}/?utm_source={app_id}&utm_medium=app&utm_campaign={app_id}")
}

/// A semantic colour for chips, alerts, stat values and progress bars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    Neutral,
    Accent,
    Info,
    Ok,
    Warn,
    Danger,
}

/// Sets the app's accent and registers the family font (with the `fonts` feature). Call once,
/// after `gpui_component::init`.
pub fn init(accent: Accent, cx: &mut gpui::App) {
    theme::set_accent(accent, cx);
    #[cfg(feature = "fonts")]
    if let Err(e) = fonts::register(cx) {
        eprintln!("otherwise-ui fonts: {e}"); // i18n-ok: developer diagnostic
    }
}
