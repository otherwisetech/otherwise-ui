//! Small styled controls: buttons, icon buttons, links, chips, section titles, the segmented
//! control and the input frame. Same metrics as the Hub's `ui.rs`.
use crate::Tone;
use crate::theme::Theme;
use crate::tokens::{alpha, icon as icon_size, radius, size, space, text};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{Icon, IconName};

/// A gpui-component icon at a size and colour.
pub fn icon(name: IconName, px_size: f32, color: Hsla) -> Icon {
    Icon::new(name).size(px(px_size)).text_color(color)
}

/// One of the app's own one-colour SVG icons (registered by the app's asset source).
pub fn glyph(path: impl Into<SharedString>, px_size: f32, color: Hsla) -> Svg {
    svg().path(path).flex_none().size(px(px_size)).text_color(color)
}

fn control(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .gap(px(space::S3))
        .h(px(size::CONTROL))
        .rounded(px(radius::CONTROL))
        .text_size(px(text::BODY))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
}

/// Secondary button: outlined, 32 px.
pub fn outline_button(t: &Theme, id: impl Into<ElementId>) -> Stateful<Div> {
    let hover = t.border_strong;
    control(id).px(px(space::S6)).border_1().border_color(t.border).bg(t.surface).text_color(t.ink).hover(move |s| s.border_color(hover))
}

/// A filled button in any colour (accent, danger), 32 px.
pub fn filled_button(t: &Theme, id: impl Into<ElementId>, bg: Hsla) -> Stateful<Div> {
    let hover = bg.opacity(alpha::FILL_HOVER);
    control(id).px(px(space::S7)).bg(bg).text_color(t.on_fill).hover(move |s| s.bg(hover))
}

/// Primary button: filled with the app's accent. One per view at most.
pub fn primary_button(t: &Theme, id: impl Into<ElementId>) -> Stateful<Div> {
    filled_button(t, id, t.accent)
}

/// Destructive button: filled with danger. Only inside a confirmation.
pub fn danger_button(t: &Theme, id: impl Into<ElementId>) -> Stateful<Div> {
    filled_button(t, id, t.danger)
}

/// Ghost button: text only, a tint on hover. For toolbars and low-emphasis actions.
pub fn ghost_button(t: &Theme, id: impl Into<ElementId>) -> Stateful<Div> {
    let hover = t.canvas;
    control(id).px(px(space::S5)).text_color(t.ink).hover(move |s| s.bg(hover))
}

/// Square, icon-only 32 px button (header actions). Give it a tooltip.
pub fn icon_button(t: &Theme, id: impl Into<ElementId>, enabled: bool) -> Stateful<Div> {
    let hover = t.canvas;
    div().id(id).flex().flex_none().items_center().justify_center().size(px(size::CONTROL)).rounded(px(radius::CONTROL)).when(enabled, |d| d.cursor_pointer().hover(move |s| s.bg(hover)))
}

/// A quiet text link in the accent.
pub fn link(t: &Theme, id: impl Into<ElementId>) -> Stateful<Div> {
    div().id(id).text_color(t.accent).cursor_pointer().hover(|s| s.underline())
}

/// The small muted title over a group (sidebar groups, settings sections). Pass it uppercased
/// when it heads a sidebar group.
pub fn section_title(t: &Theme, label: impl Into<SharedString>) -> Div {
    div().text_size(px(text::CAPTION)).font_weight(FontWeight::SEMIBOLD).text_color(t.ink_muted).child(label.into())
}

/// A rounded label with a tinted background.
pub fn chip(label: impl Into<SharedString>, fg: Hsla, bg: Hsla) -> Div {
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(px(space::S2))
        .h(px(size::CHIP))
        .px(px(7.))
        .rounded(px(radius::CHIP))
        .bg(bg)
        .text_color(fg)
        .text_size(px(text::CAPTION))
        .font_weight(FontWeight::MEDIUM)
        .child(label.into())
}

/// A chip in a semantic tone.
pub fn tone_chip(t: &Theme, label: impl Into<SharedString>, tone: Tone) -> Div {
    let (fg, bg) = t.tone(tone);
    chip(label, fg, bg)
}

/// The track of a segmented control (view toggles, settings choices).
pub fn segmented(t: &Theme) -> Div {
    div().flex().flex_none().items_center().gap(px(space::S1)).p(px(space::S1)).rounded(px(radius::CONTROL)).bg(t.border.opacity(0.6))
}

/// One segment of a segmented control.
pub fn segment(t: &Theme, id: impl Into<ElementId>, on: bool) -> Stateful<Div> {
    let muted = t.ink_muted;
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .gap(px(5.))
        .h(px(size::SEGMENT))
        .px(px(9.))
        .rounded(px(radius::SEGMENT))
        .text_size(px(text::SMALL))
        .cursor_pointer()
        .when(on, |d| d.bg(t.surface).text_color(t.ink).font_weight(FontWeight::SEMIBOLD).shadow_sm())
        .when(!on, |d| d.text_color(muted))
}

/// The frame around an `Input` drawn with `.appearance(false)`: 32 px, the canvas colour,
/// a border. Put a leading icon and the input inside.
pub fn input_frame(t: &Theme) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(space::S3))
        .h(px(size::CONTROL))
        .pl(px(space::S5))
        .pr(px(space::S2))
        .rounded(px(radius::CONTROL))
        .border_1()
        .border_color(t.border)
        .bg(t.canvas)
        .text_size(px(text::BODY))
}

/// A search field's frame with its magnifier; add the `Input` as a child.
pub fn search_frame(t: &Theme) -> Div {
    input_frame(t).child(icon(IconName::Search, icon_size::SM, t.ink_muted))
}

/// A label above a control.
pub fn labelled(t: &Theme, label: impl Into<SharedString>, control: impl IntoElement) -> Div {
    div().flex().flex_col().gap(px(space::S2)).child(div().text_size(px(text::SMALL)).text_color(t.ink_muted).child(label.into())).child(control)
}

/// A 1 px horizontal rule.
pub fn divider(t: &Theme) -> Div {
    div().w_full().h(px(1.)).bg(t.border)
}

/// A keyboard shortcut hint ("⌘J").
pub fn shortcut_hint(t: &Theme, keys: impl Into<SharedString>) -> Div {
    div().flex_none().text_size(px(text::CAPTION)).text_color(t.ink_faint).child(keys.into())
}
