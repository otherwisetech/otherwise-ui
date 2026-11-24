//! Design tokens: the family's type scale, spacing, radius, sizes and motion, as plain numbers.
//!
//! Every value here was taken from what the apps already do (the Hub, `otherwise-apps`, is the
//! reference). Use the constants instead of literals so a change lands in every app at once.
//! Colours live in [`crate::theme`].

/// The family's typeface. Every app renders its UI in Inter; [`crate::init`] registers it.
pub const SANS: &str = "Inter";

/// Monospace for paths, codes and numbers that must line up (the system's own).
#[cfg(target_os = "macos")]
pub const MONO: &str = "Menlo";
#[cfg(target_os = "windows")]
pub const MONO: &str = "Consolas";
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub const MONO: &str = "DejaVu Sans Mono";

/// Font sizes in px. Line heights for running text are in [`leading`].
pub mod text {
    /// Footer, chips, section titles (uppercase), captions, hints.
    pub const CAPTION: f32 = 11.;
    /// Secondary text: labels above controls, banners, links in About, segments.
    pub const SMALL: f32 = 12.;
    /// Body: buttons, inputs, sidebar items, table cells, card text.
    pub const BODY: f32 = 13.;
    /// The app name in the header and sidebar, empty-state titles, card titles that lead a page.
    pub const TITLE: f32 = 15.;
    /// Modal and dialog titles.
    pub const HEADING: f32 = 16.;
    /// A screen's own title at the top of its content (optional; most screens rely on the header).
    pub const SCREEN: f32 = 20.;
    /// The app name in About.
    pub const DISPLAY: f32 = 22.;
    /// The headline number of a stat tile.
    pub const STAT: f32 = 24.;
}

/// Line heights in px for multi-line text.
pub mod leading {
    /// For [`super::text::CAPTION`] (11 px).
    pub const CAPTION: f32 = 15.;
    /// For [`super::text::SMALL`] (12 px).
    pub const SMALL: f32 = 17.;
    /// For [`super::text::BODY`] (13 px).
    pub const BODY: f32 = 19.;
}

/// The spacing scale in px (padding, gaps, margins). Stick to these steps.
pub mod space {
    pub const S1: f32 = 2.;
    pub const S2: f32 = 4.;
    pub const S3: f32 = 6.;
    pub const S4: f32 = 8.;
    pub const S5: f32 = 10.;
    pub const S6: f32 = 12.;
    pub const S7: f32 = 14.;
    pub const S8: f32 = 16.;
    pub const S10: f32 = 24.;
    pub const S12: f32 = 32.;
    /// Padding around a page's content (the Hub's grid, Today, Calendar).
    pub const PAGE: f32 = 24.;
    /// Padding inside a card.
    pub const CARD: f32 = 16.;
    /// All steps, smallest first.
    pub const SCALE: [f32; 10] = [S1, S2, S3, S4, S5, S6, S7, S8, S10, S12];
}

/// Corner radii in px.
pub mod radius {
    /// Chips and badges.
    pub const CHIP: f32 = 6.;
    /// A segment inside a segmented control.
    pub const SEGMENT: f32 = 7.;
    /// Buttons, inputs, icon buttons, sidebar items, the segmented control's track.
    pub const CONTROL: f32 = 9.;
    /// Banners and alert cards, the progress track's container.
    pub const BANNER: f32 = 10.;
    /// Toasts, the Vcity card, small panels.
    pub const PANEL: f32 = 12.;
    /// Cards.
    pub const CARD: f32 = 14.;
    /// Modals (About, confirmations, settings).
    pub const MODAL: f32 = 18.;
}

/// Fixed sizes of the shared chrome, in px.
pub mod size {
    /// Height of buttons, inputs and icon buttons.
    pub const CONTROL: f32 = 32.;
    /// Height of a segment.
    pub const SEGMENT: f32 = 26.;
    /// Height of a chip.
    pub const CHIP: f32 = 20.;
    /// The header (top bar).
    pub const HEADER: f32 = 56.;
    /// The footer (status line).
    pub const FOOTER: f32 = 28.;
    /// The sidebar.
    pub const SIDEBAR: f32 = 232.;
    /// A sidebar item.
    pub const NAV_ITEM: f32 = 34.;
    /// The app's logo badge in the header and sidebar.
    pub const LOGO_BADGE: f32 = 32.;
    /// The mark inside the logo badge.
    pub const LOGO_MARK: f32 = 18.;
    /// The logo in About.
    pub const ABOUT_LOGO: f32 = 88.;
    /// About's width.
    pub const ABOUT_WIDTH: f32 = 420.;
    /// A confirmation dialog's width.
    pub const DIALOG_WIDTH: f32 = 440.;
    /// The progress bar's thickness.
    pub const PROGRESS: f32 = 6.;
}

/// Icon sizes in px.
pub mod icon {
    /// Inside chips, the footer, inline with caption text.
    pub const XS: f32 = 12.;
    /// Inline with body text, inside buttons, search fields, banners.
    pub const SM: f32 = 14.;
    /// Sidebar items.
    pub const NAV: f32 = 15.;
    /// Icon buttons in the header.
    pub const TOOLBAR: f32 = 16.;
    /// Alert cards.
    pub const ALERT: f32 = 18.;
    /// Empty states.
    pub const EMPTY: f32 = 28.;
}

/// Motion. The family barely animates: state changes (hover, selection) are instant, and
/// nothing moves for decoration.
pub mod motion {
    use std::time::Duration;
    /// A short delay before acting on a hover or a quick refresh (the Hub's debounce).
    pub const FAST: Duration = Duration::from_millis(120);
    /// Upper bound for any transition (expand / collapse) if an app adds one.
    pub const NORMAL: Duration = Duration::from_millis(200);
    /// How long a toast stays.
    pub const TOAST: Duration = Duration::from_secs(5);
}

/// Opacities used on top of the palette.
pub mod alpha {
    /// The modal backdrop (black) in light mode.
    pub const BACKDROP_LIGHT: f32 = 0.32;
    /// The modal backdrop (black) in dark mode.
    pub const BACKDROP_DARK: f32 = 0.55;
    /// A filled button's hover.
    pub const FILL_HOVER: f32 = 0.88;
    /// The accent's soft tint in light mode (selected sidebar item, chips, the self-update banner).
    pub const ACCENT_SOFT_LIGHT: f32 = 0.11;
    /// The accent's soft tint in dark mode.
    pub const ACCENT_SOFT_DARK: f32 = 0.16;
}
