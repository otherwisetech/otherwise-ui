//! The family font: Inter (Regular, Medium, SemiBold), SIL Open Font License 1.1
//! (`assets/fonts/Inter-OFL.txt`).
//!
//! Weights used: regular for text, [`gpui::FontWeight::MEDIUM`] for buttons and chips,
//! [`gpui::FontWeight::SEMIBOLD`] for titles, the selected sidebar item and stat values. Nothing
//! is bold or light.

/// Inter's files, embedded (with the `fonts` feature).
#[cfg(feature = "fonts")]
pub static INTER: [&[u8]; 3] = [include_bytes!("../assets/fonts/Inter-Regular.ttf"), include_bytes!("../assets/fonts/Inter-Medium.ttf"), include_bytes!("../assets/fonts/Inter-SemiBold.ttf")];

/// Inter's licence text, for an app's "third-party licences" screen.
pub const INTER_LICENSE: &str = include_str!("../assets/fonts/Inter-OFL.txt");

/// Registers Inter with GPUI's text system. [`crate::init`] calls it.
#[cfg(feature = "fonts")]
pub fn register(cx: &gpui::App) -> Result<(), String> {
    use std::borrow::Cow;
    cx.text_system().add_fonts(INTER.iter().map(|f| Cow::Borrowed(*f)).collect()).map_err(|e| e.to_string())
}
