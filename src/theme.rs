//! Colours: the family's warm neutrals and status colours (shared by every app, light and dark),
//! plus one [`Accent`] per app, and the light / dark / follow-the-system switch.
//!
//! The neutrals are the Hub's (`otherwise-apps`), which genealogy-rs, vigia, nodo, rama, mosaico
//! and expense already use. An app changes only its accent.
use crate::tokens::alpha;
use gpui::{App, Global, Hsla, Window, rgb};
use gpui_component::{ActiveTheme, Theme as ComponentTheme, ThemeMode};

/// An app's accent: its light-mode and dark-mode colour and its logo badge colour, as `0xRRGGBB`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Accent {
    /// The accent on light surfaces (selected items, links, primary buttons).
    pub light: u32,
    /// The accent on dark surfaces (lighter, so it keeps its contrast).
    pub dark: u32,
    /// The logo badge background behind the app's white mark (same in light and dark).
    pub brand: u32,
}

impl Accent {
    pub const fn new(light: u32, dark: u32, brand: u32) -> Self {
        Accent { light, dark, brand }
    }
}

/// The accent of every app in the family, taken from what each app already uses.
pub mod accents {
    use super::Accent;
    /// Otherwise Apps (the Hub): petrol.
    pub const HUB: Accent = Accent::new(0x1F6A72, 0x6CC3C9, 0x1F5F67);
    /// Mi Huerta: leaf green.
    pub const MI_HUERTA: Accent = Accent::new(0x2F7D3B, 0x7FCB83, 0x2E7A3A);
    /// genealogy-rs (Árbol): forest green.
    pub const GENEALOGY: Accent = Accent::new(0x2F6B4F, 0x4F9A75, 0x2F6B4F);
    /// Vigía: deep blue.
    pub const VIGIA: Accent = Accent::new(0x2B4C8C, 0x8FB0F0, 0x1E2F5C);
    /// Nodo: ocean blue.
    pub const NODO: Accent = Accent::new(0x1D6F91, 0x5FB7D6, 0x1D6F91);
    /// Rama: violet.
    pub const RAMA: Accent = Accent::new(0x5B4B9A, 0x9D8CE0, 0x5B4B9A);
    /// diskmap: today it shares genealogy's green accent; its badge is blue.
    pub const DISKMAP: Accent = Accent::new(0x2F6B4F, 0x4F9A75, 0x3F628F);
    /// Mosaico: terracotta.
    pub const MOSAICO: Accent = Accent::new(0xB24E33, 0xEB8C6B, 0xC0583A);
    /// expense Business (expense-rs): navy. Replaces the teal `0x0E7468` with the editions redesign.
    pub const EXPENSE_BUSINESS: Accent = Accent::new(0x1F3F6E, 0x8AA8E0, 0x1A3460);
    /// expense me: green. Replaces the terracotta `0xB9482F` with the editions redesign.
    pub const EXPENSE_ME: Accent = Accent::new(0x1F7A52, 0x6CCB98, 0x1A6644);
    /// Helio: amber.
    pub const HELIO: Accent = Accent::new(0xB8690F, 0xF5B54E, 0x9A570C);
    /// Brújula: compass blue.
    pub const BRUJULA: Accent = Accent::new(0x1F5F8B, 0x6FB0DC, 0x18496B);
    /// OS CACFU: copper.
    pub const CACFU: Accent = Accent::new(0xB0612B, 0xE39257, 0x8E4A22);
    /// Voltia (the suite's launcher): slate. Each Voltia tool keeps its own accent.
    pub const VOLTIA: Accent = Accent::new(0x3E5566, 0x8FA9BB, 0x2B3A46);
    /// Surco (the suite's launcher): field green. Each Surco tool keeps its own accent.
    pub const SURCO: Accent = Accent::new(0x4A7A33, 0x93C46F, 0x355A24);

    /// Every named accent with its app, for docs, the gallery and tests.
    pub const ALL: [(&str, Accent); 15] = [
        ("Otherwise Apps", HUB),
        ("Mi Huerta", MI_HUERTA),
        ("genealogy", GENEALOGY),
        ("Vigía", VIGIA),
        ("Nodo", NODO),
        ("Rama", RAMA),
        ("diskmap", DISKMAP),
        ("Mosaico", MOSAICO),
        ("expense Business", EXPENSE_BUSINESS),
        ("expense me", EXPENSE_ME),
        ("Helio", HELIO),
        ("Brújula", BRUJULA),
        ("OS CACFU", CACFU),
        ("Voltia", VOLTIA),
        ("Surco", SURCO),
    ]; // i18n-ok: product names
}

/// The shared palette as `0xRRGGBB` (alpha, where a colour has one, is applied in [`Theme`]).
pub mod palette {
    /// Light mode.
    pub mod light {
        pub const CANVAS: u32 = 0xF2F1EE;
        pub const SURFACE: u32 = 0xFFFFFF;
        pub const RAISED: u32 = 0xF8F7F4;
        pub const BORDER: u32 = 0xE2DFD8;
        pub const BORDER_STRONG: u32 = 0xC9C5BB;
        pub const INK: u32 = 0x1C1B19;
        pub const INK_MUTED: u32 = 0x6B675F;
        pub const INK_FAINT: u32 = 0xA39C8E;
        pub const DANGER: u32 = 0xA3382A;
        pub const DANGER_SOFT: u32 = 0xD0503F;
        pub const WARN: u32 = 0x9A5B0C;
        pub const WARN_SOFT: u32 = 0xF2A93B;
        pub const OK: u32 = 0x1F7A44;
        pub const OK_SOFT: u32 = 0x2E9A5A;
        pub const INFO: u32 = 0x2F6FA8;
        pub const INFO_SOFT: u32 = 0x4A8FD0;
        pub const ON_FILL: u32 = 0xFFFFFF;
    }
    /// Dark mode.
    pub mod dark {
        pub const CANVAS: u32 = 0x141413;
        pub const SURFACE: u32 = 0x1C1B1A;
        pub const RAISED: u32 = 0x232220;
        pub const BORDER: u32 = 0x302E2B;
        pub const BORDER_STRONG: u32 = 0x4A4741;
        pub const INK: u32 = 0xF2F0EA;
        pub const INK_MUTED: u32 = 0xA19C93;
        pub const INK_FAINT: u32 = 0x6F695E;
        pub const DANGER: u32 = 0xE07767;
        pub const DANGER_SOFT: u32 = 0xE5675A;
        pub const WARN: u32 = 0xF2B95A;
        pub const WARN_SOFT: u32 = 0xF2A93B;
        pub const OK: u32 = 0x6FD39A;
        pub const OK_SOFT: u32 = 0x4FB87D;
        pub const INFO: u32 = 0x7DB6EA;
        pub const INFO_SOFT: u32 = 0x5FA3E0;
        pub const ON_FILL: u32 = 0x141413;
    }
    /// Alpha of the `*_SOFT` tints, (light, dark).
    pub const DANGER_SOFT_ALPHA: (f32, f32) = (0.10, 0.14);
    pub const WARN_SOFT_ALPHA: (f32, f32) = (0.18, 0.16);
    pub const OK_SOFT_ALPHA: (f32, f32) = (0.12, 0.14);
    pub const INFO_SOFT_ALPHA: (f32, f32) = (0.13, 0.16);
}

/// The resolved colours for one mode and one accent. Field names match the apps' `Tokens`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    /// The window background, behind cards.
    pub canvas: Hsla,
    /// Cards, the header, footer and sidebar, modals.
    pub surface: Hsla,
    /// Slightly raised areas inside a surface (info banners, table headers, hovered rows).
    pub raised: Hsla,
    pub border: Hsla,
    /// Hovered borders, dividers that must read.
    pub border_strong: Hsla,
    /// Primary text.
    pub ink: Hsla,
    /// Secondary text: subtitles, labels, the footer.
    pub ink_muted: Hsla,
    /// Tertiary text: placeholders, disabled.
    pub ink_faint: Hsla,
    pub accent: Hsla,
    /// The accent's tint: selected sidebar item, accent chips.
    pub accent_soft: Hsla,
    /// The logo badge behind the app's mark.
    pub brand: Hsla,
    pub danger: Hsla,
    pub danger_soft: Hsla,
    pub warn: Hsla,
    pub warn_soft: Hsla,
    pub ok: Hsla,
    pub ok_soft: Hsla,
    pub info: Hsla,
    pub info_soft: Hsla,
    /// Text on a filled (accent, danger, ink) background.
    pub on_fill: Hsla,
    /// The modal backdrop.
    pub backdrop: Hsla,
    pub dark: bool,
    /// The accent these colours were made with.
    pub source: Accent,
}

/// `0xRRGGBB` as a colour.
pub fn hex(rgb_hex: u32) -> Hsla {
    rgb(rgb_hex).into()
}

/// `0xRRGGBB` with an alpha.
pub fn hex_a(rgb_hex: u32, a: f32) -> Hsla {
    let mut h = hex(rgb_hex);
    h.a = a;
    h
}

impl Theme {
    pub fn light(accent: Accent) -> Self {
        use palette::light as p;
        let backdrop = hex_a(0x000000, alpha::BACKDROP_LIGHT);
        Theme {
            canvas: hex(p::CANVAS),
            surface: hex(p::SURFACE),
            raised: hex(p::RAISED),
            border: hex(p::BORDER),
            border_strong: hex(p::BORDER_STRONG),
            ink: hex(p::INK),
            ink_muted: hex(p::INK_MUTED),
            ink_faint: hex(p::INK_FAINT),
            accent: hex(accent.light),
            accent_soft: hex_a(accent.light, alpha::ACCENT_SOFT_LIGHT),
            brand: hex(accent.brand),
            danger: hex(p::DANGER),
            danger_soft: hex_a(p::DANGER_SOFT, palette::DANGER_SOFT_ALPHA.0),
            warn: hex(p::WARN),
            warn_soft: hex_a(p::WARN_SOFT, palette::WARN_SOFT_ALPHA.0),
            ok: hex(p::OK),
            ok_soft: hex_a(p::OK_SOFT, palette::OK_SOFT_ALPHA.0),
            info: hex(p::INFO),
            info_soft: hex_a(p::INFO_SOFT, palette::INFO_SOFT_ALPHA.0),
            on_fill: hex(p::ON_FILL),
            backdrop,
            dark: false,
            source: accent,
        }
    }

    pub fn dark(accent: Accent) -> Self {
        use palette::dark as p;
        Theme {
            canvas: hex(p::CANVAS),
            surface: hex(p::SURFACE),
            raised: hex(p::RAISED),
            border: hex(p::BORDER),
            border_strong: hex(p::BORDER_STRONG),
            ink: hex(p::INK),
            ink_muted: hex(p::INK_MUTED),
            ink_faint: hex(p::INK_FAINT),
            accent: hex(accent.dark),
            accent_soft: hex_a(accent.dark, alpha::ACCENT_SOFT_DARK),
            brand: hex(accent.brand),
            danger: hex(p::DANGER),
            danger_soft: hex_a(p::DANGER_SOFT, palette::DANGER_SOFT_ALPHA.1),
            warn: hex(p::WARN),
            warn_soft: hex_a(p::WARN_SOFT, palette::WARN_SOFT_ALPHA.1),
            ok: hex(p::OK),
            ok_soft: hex_a(p::OK_SOFT, palette::OK_SOFT_ALPHA.1),
            info: hex(p::INFO),
            info_soft: hex_a(p::INFO_SOFT, palette::INFO_SOFT_ALPHA.1),
            on_fill: hex(p::ON_FILL),
            backdrop: hex_a(0x000000, alpha::BACKDROP_DARK),
            dark: true,
            source: accent,
        }
    }

    /// The colours for `dark` / light with `accent`.
    pub fn new(dark: bool, accent: Accent) -> Self {
        if dark { Theme::dark(accent) } else { Theme::light(accent) }
    }

    /// Text on a filled background (same as the field; the apps' `Tokens` had this method).
    pub fn on_fill(&self) -> Hsla {
        self.on_fill
    }

    /// The same mode with another accent (a suite's tool inside its launcher).
    pub fn with_accent(&self, accent: Accent) -> Self {
        Theme::new(self.dark, accent)
    }

    /// The other mode, same accent.
    pub fn toggled(&self) -> Self {
        Theme::new(!self.dark, self.source)
    }

    /// Foreground and tint for an [`crate::Tone`].
    pub fn tone(&self, tone: crate::Tone) -> (Hsla, Hsla) {
        match tone {
            crate::Tone::Neutral => (self.ink_muted, self.raised),
            crate::Tone::Accent => (self.accent, self.accent_soft),
            crate::Tone::Info => (self.info, self.info_soft),
            crate::Tone::Ok => (self.ok, self.ok_soft),
            crate::Tone::Warn => (self.warn, self.warn_soft),
            crate::Tone::Danger => (self.danger, self.danger_soft),
        }
    }
}

/// The user's appearance setting. Apps store it as a string: `"light"`, `"dark"`, or anything
/// else (usually nothing) to follow the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Appearance {
    Light,
    Dark,
    #[default]
    System,
}

impl Appearance {
    /// Reads the stored setting; unknown values follow the system.
    pub fn parse(pref: Option<&str>) -> Self {
        match pref {
            Some("light") => Appearance::Light,
            Some("dark") => Appearance::Dark,
            _ => Appearance::System,
        }
    }

    /// The value to store (`None` for the system).
    pub fn as_setting(self) -> Option<&'static str> {
        match self {
            Appearance::Light => Some("light"),
            Appearance::Dark => Some("dark"),
            Appearance::System => None,
        }
    }
}

/// The app's accent, kept as a GPUI global by [`crate::init`] / [`set_accent`].
#[derive(Debug, Clone, Copy)]
pub(crate) struct ActiveAccent(pub Accent);

impl Global for ActiveAccent {}

/// Changes the app's accent (a suite's launcher switching to a tool's document).
pub fn set_accent(accent: Accent, cx: &mut App) {
    cx.set_global(ActiveAccent(accent));
}

/// The app's accent ([`accents::HUB`] before [`crate::init`]).
pub fn accent(cx: &App) -> Accent {
    cx.try_global::<ActiveAccent>().map(|a| a.0).unwrap_or(accents::HUB)
}

/// The current colours: the mode gpui-component is in, with the app's accent.
pub fn theme(cx: &App) -> Theme {
    Theme::new(cx.theme().mode.is_dark(), accent(cx))
}

/// Applies the appearance setting (light, dark or the system's) and the family font to
/// gpui-component, so its inputs, menus and switches match.
pub fn apply(appearance: Appearance, window: Option<&mut Window>, cx: &mut App) {
    match appearance {
        Appearance::Light => ComponentTheme::change(ThemeMode::Light, window, cx),
        Appearance::Dark => ComponentTheme::change(ThemeMode::Dark, window, cx),
        Appearance::System => ComponentTheme::sync_system_appearance(window, cx),
    }
    ComponentTheme::global_mut(cx).font_family = crate::tokens::SANS.into();
}
