//! Content blocks: card, alert card, banner, stat tile, progress bar, empty state, privacy note,
//! the Vcity card.
use super::controls::{icon, section_title};
use crate::Tone;
use crate::theme::Theme;
use crate::tokens::{icon as icon_size, leading, radius, size, space, text};
use gpui::*;
use gpui_component::IconName;

/// A card: surface, 1 px border, radius 14, 16 px padding, no shadow.
pub fn card(t: &Theme) -> Div {
    div().flex().flex_col().gap(px(space::S5)).p(px(space::CARD)).rounded(px(radius::CARD)).border_1().border_color(t.border).bg(t.surface)
}

/// A card with a title row; add the content as children.
pub fn titled_card(t: &Theme, title: impl Into<SharedString>) -> Div {
    card(t).child(div().text_size(px(text::BODY)).font_weight(FontWeight::SEMIBOLD).text_color(t.ink).child(title.into()))
}

/// The icon that goes with a tone.
pub fn tone_icon(tone: Tone) -> IconName {
    match tone {
        Tone::Neutral | Tone::Info | Tone::Accent => IconName::Info,
        Tone::Ok => IconName::CircleCheck,
        Tone::Warn => IconName::TriangleAlert,
        Tone::Danger => IconName::CircleX,
    }
}

/// A one-line note or warning band inside content (Hub, Mi Huerta and Surco's `banner`).
pub fn banner(t: &Theme, tone: Tone, message: impl Into<SharedString>) -> Div {
    let (fg, bg) = t.tone(tone);
    let ink = if tone == Tone::Neutral { t.ink_muted } else { t.ink };
    div()
        .flex()
        .items_start()
        .gap(px(space::S4))
        .px(px(space::S6))
        .py(px(space::S4))
        .rounded(px(radius::BANNER))
        .bg(bg)
        .text_size(px(text::SMALL))
        .line_height(px(leading::SMALL))
        .text_color(ink)
        .child(div().pt(px(1.)).child(icon(tone_icon(tone), icon_size::SM, fg)))
        .child(div().flex_1().min_w_0().child(message.into()))
}

/// An alert card: a tinted card with an icon, a title, a body and room for an action
/// (`.child(primary_button(…))`). For things that need the user (an update, a failed sync).
pub fn alert_card(t: &Theme, tone: Tone, title: impl Into<SharedString>, body: Option<SharedString>) -> Div {
    let (fg, bg) = t.tone(tone);
    div().flex().items_center().gap(px(space::S6)).px(px(space::S7)).py(px(space::S5)).rounded(px(radius::PANEL)).bg(bg).child(icon(tone_icon(tone), icon_size::ALERT, fg)).child(
        div()
            .flex()
            .flex_col()
            .gap(px(space::S1))
            .flex_1()
            .min_w_0()
            .child(div().text_size(px(text::BODY)).font_weight(FontWeight::MEDIUM).text_color(t.ink).child(title.into()))
            .children(body.map(|b| div().text_size(px(text::SMALL)).line_height(px(leading::SMALL)).text_color(t.ink_muted).child(b))),
    )
}

/// A stat tile: a label, a headline number and an optional note under it, in a card.
pub fn stat_tile(t: &Theme, label: impl Into<SharedString>, value: impl Into<SharedString>, sub: Option<SharedString>, tone: Tone) -> Div {
    let color = if tone == Tone::Neutral { t.ink } else { t.tone(tone).0 };
    card(t)
        .gap(px(3.))
        .min_w(px(140.))
        .child(div().text_size(px(text::CAPTION)).text_color(t.ink_muted).child(label.into()))
        .child(div().text_size(px(text::STAT)).font_weight(FontWeight::SEMIBOLD).text_color(color).child(value.into()))
        .children(sub.map(|s| div().text_size(px(text::CAPTION)).text_color(t.ink_muted).child(s)))
}

/// Clamps a progress fraction to 0…1 (NaN is 0).
pub fn clamp_fraction(f: f32) -> f32 {
    if f.is_nan() { 0. } else { f.clamp(0., 1.) }
}

/// A thin progress bar, `fraction` 0…1, in a tone (accent for work in progress, warn or danger
/// for a budget running out).
pub fn progress_bar(t: &Theme, fraction: f32, tone: Tone) -> Div {
    let fill = if tone == Tone::Neutral { t.ink_muted } else { t.tone(tone).0 };
    let f = clamp_fraction(fraction);
    div().w_full().h(px(size::PROGRESS)).rounded_full().bg(t.border).overflow_hidden().child(div().h_full().w(relative(f)).rounded_full().bg(fill))
}

/// A progress bar with a label on the left and a value on the right above it.
pub fn labelled_progress(t: &Theme, label: impl Into<SharedString>, value: impl Into<SharedString>, fraction: f32, tone: Tone) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(space::S3))
        .child(
            div()
                .flex()
                .items_center()
                .text_size(px(text::SMALL))
                .child(div().flex_1().min_w_0().truncate().text_color(t.ink).child(label.into()))
                .child(div().flex_none().text_color(t.ink_muted).child(value.into())),
        )
        .child(progress_bar(t, fraction, tone))
}

/// An empty state: a muted icon, a title and a line of help, centred in the space it gets. Add
/// an action with `.child(primary_button(…))`.
pub fn empty_state(t: &Theme, icon_name: IconName, title: impl Into<SharedString>, body: Option<SharedString>) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(space::S4))
        .size_full()
        .p(px(space::PAGE))
        .child(icon(icon_name, icon_size::EMPTY, t.ink_muted))
        .child(div().text_size(px(text::TITLE)).font_weight(FontWeight::SEMIBOLD).text_color(t.ink).child(title.into()))
        .children(body.map(|b| div().max_w(px(420.)).text_center().text_size(px(text::BODY)).line_height(px(leading::BODY)).text_color(t.ink_muted).child(b)))
}

/// "Tus datos están en este computador": the local-first note, under the sidebar's items, in
/// onboarding and in Settings → data. `detail` is a second, quieter line (the folder, the cloud).
pub fn privacy_note(t: &Theme, message: impl Into<SharedString>, detail: Option<SharedString>) -> Div {
    div()
        .flex()
        .items_start()
        .gap(px(space::S3))
        .px(px(space::S2))
        .text_size(px(text::CAPTION))
        .line_height(px(leading::CAPTION))
        .text_color(t.ink_muted)
        .child(div().pt(px(1.)).child(icon(IconName::HardDrive, icon_size::XS, t.ink_muted)))
        .child(div().flex().flex_col().flex_1().min_w_0().child(div().child(message.into())).children(detail.map(|d| div().text_color(t.ink_faint).truncate().child(d))))
}

/// The quiet Vcity invitation under an app's main content (the Hub's, under the apps grid).
pub fn vcity_card(t: &Theme, id: impl Into<ElementId>, message: impl Into<SharedString>, link_label: impl Into<SharedString>, url: impl Into<SharedString>) -> Stateful<Div> {
    let accent = t.accent;
    let url: SharedString = url.into();
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(space::S6))
        .px(px(space::S7))
        .py(px(space::S5))
        .rounded(px(radius::PANEL))
        .border_1()
        .border_color(t.border)
        .text_size(px(text::SMALL))
        .text_color(t.ink_muted)
        .cursor_pointer()
        .hover(move |s| s.border_color(accent))
        .child(icon(IconName::Globe, icon_size::TOOLBAR, t.ink_muted))
        .child(div().flex_1().min_w_0().child(message.into()))
        .child(div().flex_none().text_color(accent).font_weight(FontWeight::MEDIUM).child(link_label.into()))
        .on_click(move |_, _, cx| cx.open_url(&url))
}

/// A toast: dark pill near the bottom of the window (place it in an absolutely positioned row).
pub fn toast(t: &Theme, message: impl Into<SharedString>) -> Div {
    div().px(px(space::S8)).py(px(space::S5)).rounded(px(radius::PANEL)).bg(t.ink).text_color(t.on_fill).text_size(px(text::BODY)).shadow_lg().child(message.into())
}

/// A settings row: a label (and an optional hint) on the left, the control on the right.
pub fn setting_row(t: &Theme, label: impl Into<SharedString>, hint: Option<SharedString>, control: impl IntoElement) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(space::S8))
        .py(px(space::S5))
        .border_b_1()
        .border_color(t.border)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(space::S1))
                .flex_1()
                .min_w_0()
                .child(div().text_size(px(text::BODY)).child(label.into()))
                .children(hint.map(|h| div().text_size(px(text::SMALL)).text_color(t.ink_muted).child(h))),
        )
        .child(control)
}

/// A group title over settings rows or sidebar items, uppercased by the caller's locale rules.
pub fn group_title(t: &Theme, title: impl Into<SharedString>) -> Div {
    section_title(t, title).pt(px(space::S5)).pb(px(3.)).px(px(space::S5))
}
