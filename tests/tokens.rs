//! Token values and theme switching (no window needed).
use otherwise_ui::theme::{hex, hex_a, palette};
use otherwise_ui::tokens::{alpha, radius, size, space, text};
use otherwise_ui::{Accent, Appearance, Labels, Theme, Tone, accents};

fn lum(hex: u32) -> f64 {
    let ch = |v: u32| {
        let c = v as f64 / 255.0;
        if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * ch((hex >> 16) & 0xFF) + 0.7152 * ch((hex >> 8) & 0xFF) + 0.0722 * ch(hex & 0xFF)
}

fn contrast(a: u32, b: u32) -> f64 {
    let (x, y) = (lum(a), lum(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

#[test]
fn the_family_scale_is_the_hubs() {
    assert_eq!(otherwise_ui::tokens::SANS, "Inter");
    assert_eq!((text::CAPTION, text::SMALL, text::BODY, text::TITLE, text::HEADING), (11., 12., 13., 15., 16.));
    assert_eq!((text::DISPLAY, text::STAT), (22., 24.));
    assert_eq!((size::CONTROL, size::SEGMENT, size::CHIP), (32., 26., 20.));
    assert_eq!((size::HEADER, size::FOOTER, size::SIDEBAR, size::NAV_ITEM), (56., 28., 232., 34.));
    assert_eq!((size::ABOUT_WIDTH, size::ABOUT_LOGO), (420., 88.));
    assert_eq!((radius::CHIP, radius::SEGMENT, radius::CONTROL, radius::BANNER, radius::PANEL, radius::CARD, radius::MODAL), (6., 7., 9., 10., 12., 14., 18.));
    assert_eq!((alpha::BACKDROP_LIGHT, alpha::BACKDROP_DARK), (0.32, 0.55));
}

#[test]
fn spacing_scale_is_increasing_and_even() {
    assert!(space::SCALE.windows(2).all(|w| w[0] < w[1]));
    assert!(space::SCALE.iter().all(|s| *s as u32 % 2 == 0));
    assert!(space::SCALE.contains(&space::PAGE) && space::SCALE.contains(&space::CARD));
}

#[test]
fn neutrals_are_the_hubs() {
    let l = Theme::light(accents::HUB);
    assert_eq!(l.canvas, hex(0xF2F1EE));
    assert_eq!(l.surface, hex(0xFFFFFF));
    assert_eq!(l.border, hex(0xE2DFD8));
    assert_eq!(l.ink, hex(0x1C1B19));
    assert_eq!(l.ink_muted, hex(0x6B675F));
    assert_eq!(l.accent, hex(0x1F6A72));
    assert_eq!(l.brand, hex(0x1F5F67));
    let d = Theme::dark(accents::HUB);
    assert_eq!(d.canvas, hex(0x141413));
    assert_eq!(d.surface, hex(0x1C1B1A));
    assert_eq!(d.ink, hex(0xF2F0EA));
    assert_eq!(d.accent, hex(0x6CC3C9));
    assert_eq!(d.on_fill, hex(0x141413));
    assert_eq!(l.on_fill, hex(0xFFFFFF));
}

#[test]
fn only_the_accent_changes_between_apps() {
    for dark in [false, true] {
        let hub = Theme::new(dark, accents::HUB);
        for (_, a) in accents::ALL {
            let other = Theme::new(dark, a);
            assert_eq!(other.canvas, hub.canvas);
            assert_eq!(other.surface, hub.surface);
            assert_eq!(other.border, hub.border);
            assert_eq!(other.ink, hub.ink);
            assert_eq!(other.ink_muted, hub.ink_muted);
            assert_eq!(other.danger, hub.danger);
            assert_eq!(other.warn, hub.warn);
            assert_eq!(other.ok, hub.ok);
            assert_eq!(other.accent, hex(if dark { a.dark } else { a.light }));
            assert_eq!(other.accent_soft, hex_a(if dark { a.dark } else { a.light }, if dark { alpha::ACCENT_SOFT_DARK } else { alpha::ACCENT_SOFT_LIGHT }));
            assert_eq!(other.brand, hex(a.brand));
        }
    }
}

#[test]
fn switching_modes_keeps_the_accent() {
    let l = Theme::light(accents::MI_HUERTA);
    let d = l.toggled();
    assert!(d.dark && !l.dark);
    assert_eq!(d.source, accents::MI_HUERTA);
    assert_eq!(d, Theme::dark(accents::MI_HUERTA));
    assert_eq!(d.toggled(), l);
    assert_eq!(l.with_accent(accents::SURCO), Theme::light(accents::SURCO));
    assert_eq!(Theme::new(true, accents::HELIO), Theme::dark(accents::HELIO));
}

#[test]
fn appearance_setting_round_trips() {
    for a in [Appearance::Light, Appearance::Dark, Appearance::System] {
        assert_eq!(Appearance::parse(a.as_setting()), a);
    }
    assert_eq!(Appearance::parse(Some("system")), Appearance::System);
    assert_eq!(Appearance::parse(Some("sepia")), Appearance::System);
    assert_eq!(Appearance::parse(None), Appearance::System);
    assert_eq!(Appearance::default(), Appearance::System);
}

#[test]
fn text_reads_on_every_surface() {
    use palette::{dark as d, light as l};
    assert!(contrast(l::INK, l::SURFACE) >= 7.0);
    assert!(contrast(l::INK_MUTED, l::SURFACE) >= 4.5);
    assert!(contrast(l::INK_MUTED, l::CANVAS) >= 4.5);
    assert!(contrast(d::INK, d::SURFACE) >= 7.0);
    assert!(contrast(d::INK_MUTED, d::SURFACE) >= 4.5);
    assert!(contrast(d::INK_MUTED, d::CANVAS) >= 4.5);
    for (fg, bg) in
        [(l::DANGER, l::SURFACE), (l::WARN, l::SURFACE), (l::OK, l::SURFACE), (l::INFO, l::SURFACE), (d::DANGER, d::SURFACE), (d::WARN, d::SURFACE), (d::OK, d::SURFACE), (d::INFO, d::SURFACE)]
    {
        assert!(contrast(fg, bg) >= 4.5, "{fg:06X} on {bg:06X}: {:.2}", contrast(fg, bg));
    }
}

#[test]
fn every_accent_reads_and_carries_white_or_dark_text() {
    for (app, a) in accents::ALL {
        assert!(contrast(a.light, palette::light::SURFACE) >= 3.0, "{app} light accent on white");
        assert!(contrast(a.dark, palette::dark::SURFACE) >= 3.0, "{app} dark accent on dark surface");
        assert!(contrast(palette::light::ON_FILL, a.light) >= 3.0, "{app}: white on its light accent");
        assert!(contrast(palette::dark::ON_FILL, a.dark) >= 3.0, "{app}: dark text on its dark accent");
        assert!(contrast(0xFFFFFF, a.brand) >= 3.0, "{app}: white mark on its badge");
        assert!(lum(a.dark) > lum(a.light), "{app}: the dark-mode accent is the lighter one");
    }
}

#[test]
fn accents_are_distinct_except_diskmap() {
    let mut seen: Vec<(&str, Accent)> = Vec::new();
    for (app, a) in accents::ALL {
        for (other, b) in &seen {
            if a.light == b.light {
                assert_eq!((*other, app), ("genealogy", "diskmap"), "{app} repeats {other}'s accent");
            }
        }
        seen.push((app, a));
    }
}

#[test]
fn tones_map_to_status_colours() {
    let t = Theme::light(accents::HUB);
    assert_eq!(t.tone(Tone::Danger), (t.danger, t.danger_soft));
    assert_eq!(t.tone(Tone::Warn), (t.warn, t.warn_soft));
    assert_eq!(t.tone(Tone::Ok), (t.ok, t.ok_soft));
    assert_eq!(t.tone(Tone::Info), (t.info, t.info_soft));
    assert_eq!(t.tone(Tone::Accent), (t.accent, t.accent_soft));
    assert_eq!(t.tone(Tone::Neutral), (t.ink_muted, t.raised));
}

#[test]
fn labels_cover_four_languages() {
    assert_eq!(Labels::for_locale("es").about, "Acerca de");
    assert_eq!(Labels::for_locale("es-CO").locale, "es");
    assert_eq!(Labels::for_locale("pt_BR").locale, "pt");
    assert_eq!(Labels::for_locale("FR").locale, "fr");
    assert_eq!(Labels::for_locale("de").locale, "en");
    assert_eq!(Labels::fill(Labels::for_locale("es").made_by, "Mi Huerta"), "Hecho por Otherwise Technologies LLC");
    assert_eq!(Labels::fill(Labels::for_locale("es").about_app, "Mi Huerta"), "Acerca de Mi Huerta");
    assert_eq!(Labels::for_locale("es").privacy_local, "Tus datos están en este computador");
    for l in otherwise_ui::labels::ALL {
        for s in [
            l.about,
            l.about_app,
            l.made_by,
            l.place,
            l.visit,
            l.source,
            l.license_open,
            l.license_proprietary,
            l.privacy_local,
            l.privacy_local_detail,
            l.vcity_invite,
            l.vcity_link,
            l.assistant,
            l.close,
        ] {
            assert!(!s.trim().is_empty(), "{}: empty label", l.locale);
        }
        assert!(l.made_by.contains("{company}") && l.about_app.contains("{app}"), "{}", l.locale);
    }
}

#[test]
fn footer_credit_reads_app_company_about() {
    assert_eq!(otherwise_ui::footer_credit("Mi Huerta", Some("0.3.1"), "Acerca de"), "Mi Huerta 0.3.1 · Otherwise Technologies LLC · Acerca de");
    assert_eq!(otherwise_ui::footer_credit("Surco", None, "About"), "Surco · Otherwise Technologies LLC · About");
    assert_eq!(otherwise_ui::footer_credit("Surco", Some(""), "About"), "Surco · Otherwise Technologies LLC · About");
}

#[test]
fn progress_fraction_is_clamped() {
    assert_eq!(otherwise_ui::clamp_fraction(-1.), 0.);
    assert_eq!(otherwise_ui::clamp_fraction(0.4), 0.4);
    assert_eq!(otherwise_ui::clamp_fraction(7.), 1.);
    assert_eq!(otherwise_ui::clamp_fraction(f32::NAN), 0.);
}

#[test]
fn vcity_url_is_tagged_per_app() {
    assert_eq!(otherwise_ui::vcity_url("mi-huerta"), "https://vcity.io/?utm_source=mi-huerta&utm_medium=app&utm_campaign=mi-huerta");
}

#[cfg(feature = "fonts")]
#[test]
fn inter_is_embedded() {
    for f in otherwise_ui::fonts::INTER {
        assert!(f.len() > 100_000);
    }
    assert!(otherwise_ui::fonts::INTER_LICENSE.contains("SIL OPEN FONT LICENSE"));
}
