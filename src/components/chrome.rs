//! The shared chrome: [`Sidebar`], [`AppHeader`], [`AppFooter`], [`AboutModal`] and the modal
//! frame. These read the colours from [`crate::theme`] (the app's accent and the current mode),
//! so they look the same in every app; the app passes only its words, icons and handlers.
use super::controls::{chip, icon, icon_button, primary_button, shortcut_hint};
use crate::theme::{Theme, theme};
use crate::tokens::{icon as icon_size, leading, radius, size, space, text};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::IconName;
use std::rc::Rc;

/// A click handler, compatible with `cx.listener(|this, _: &ClickEvent, window, cx| …)`.
pub type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Sent when a modal is dismissed (close button, backdrop click). Use it with
/// `cx.listener(|this, _: &Dismiss, window, cx| …)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dismiss;

/// A dismiss handler.
pub type DismissHandler = Rc<dyn Fn(&Dismiss, &mut Window, &mut App)>;

/// The app's logo badge: its white mark on the brand colour, 32 px, radius 9.
pub fn logo_badge(t: &Theme, mark: impl IntoElement) -> Div {
    div().flex().flex_none().items_center().justify_center().size(px(size::LOGO_BADGE)).rounded(px(radius::CONTROL)).bg(t.brand).child(mark)
}

/// The app's one-colour SVG mark (registered by the app's asset source), white, for
/// [`logo_badge`].
pub fn logo_mark(path: impl Into<SharedString>) -> Svg {
    svg().path(path).flex_none().size(px(size::LOGO_MARK)).text_color(gpui::white())
}

// ── sidebar ──────────────────────────────────────────────────────────────────────────────────

/// One row of the [`Sidebar`]: icon, label, optional badge and shortcut hint.
#[derive(IntoElement)]
pub struct NavItem {
    id: ElementId,
    label: SharedString,
    icon: Option<IconName>,
    badge: Option<SharedString>,
    shortcut: Option<SharedString>,
    active: bool,
    on_click: Option<ClickHandler>,
}

impl NavItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        NavItem { id: id.into(), label: label.into(), icon: None, badge: None, shortcut: None, active: false, on_click: None }
    }
    pub fn icon(mut self, name: IconName) -> Self {
        self.icon = Some(name);
        self
    }
    /// A count or short word on the right ("3", "Nuevo"). Hidden when `None`.
    pub fn badge(mut self, badge: Option<impl Into<SharedString>>) -> Self {
        self.badge = badge.map(Into::into);
        self
    }
    /// The keyboard shortcut that opens this screen ("⌘1"), shown faint on the right.
    pub fn shortcut(mut self, keys: impl Into<SharedString>) -> Self {
        self.shortcut = Some(keys.into());
        self
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for NavItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let on = self.active;
        let hover = t.canvas;
        div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .gap(px(9.))
            .h(px(size::NAV_ITEM))
            .px(px(space::S5))
            .rounded(px(radius::CONTROL))
            .text_size(px(text::BODY))
            .cursor_pointer()
            .when(on, |d| d.bg(t.accent_soft).text_color(t.accent).font_weight(FontWeight::SEMIBOLD))
            .when(!on, |d| d.text_color(t.ink).hover(move |s| s.bg(hover)))
            .children(self.icon.map(|n| icon(n, icon_size::NAV, if on { t.accent } else { t.ink_muted })))
            .child(div().flex_1().min_w_0().truncate().child(self.label))
            .children(self.badge.map(|b| chip(b, t.on_fill, t.accent)))
            .children(self.shortcut.map(|k| shortcut_hint(&t, k)))
            .when_some(self.on_click, |d, f| d.on_click(move |e, w, cx| f(e, w, cx)))
    }
}

/// A titled group of [`NavItem`]s. The title is shown uppercased; `None` for the first group.
pub struct NavGroup {
    title: Option<SharedString>,
    items: Vec<NavItem>,
}

impl NavGroup {
    pub fn new(title: Option<impl Into<SharedString>>) -> Self {
        NavGroup { title: title.map(Into::into), items: Vec::new() }
    }
    pub fn item(mut self, item: NavItem) -> Self {
        self.items.push(item);
        self
    }
    pub fn items(mut self, items: impl IntoIterator<Item = NavItem>) -> Self {
        self.items.extend(items);
        self
    }
}

/// The left sidebar: 232 px, surface colour, a right border. On top the app's badge and name
/// ([`Sidebar::brand`]) or any header; then the groups (scrolling when they don't fit); at the
/// bottom the footer slot (the AI panel button, the privacy note).
#[derive(IntoElement)]
pub struct Sidebar {
    id: ElementId,
    width: f32,
    header: Option<AnyElement>,
    switcher: Option<AnyElement>,
    groups: Vec<NavGroup>,
    footer: Vec<AnyElement>,
}

impl Sidebar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Sidebar { id: id.into(), width: size::SIDEBAR, header: None, switcher: None, groups: Vec::new(), footer: Vec::new() }
    }
    /// The app's badge and name at the top (when the app has no [`AppHeader`]).
    pub fn brand(mut self, mark: impl IntoElement, name: impl Into<SharedString>) -> Self {
        self.header = Some(SidebarBrand { mark: mark.into_any_element(), name: name.into() }.into_any_element());
        self
    }
    /// Any element at the top instead of the brand (a workspace switcher).
    pub fn header(mut self, el: impl IntoElement) -> Self {
        self.header = Some(el.into_any_element());
        self
    }
    /// A switcher right under the brand, full width (the open book, company or workspace: a
    /// ghost dropdown button). It stays put while the groups scroll.
    pub fn switcher(mut self, el: impl IntoElement) -> Self {
        self.switcher = Some(el.into_any_element());
        self
    }
    pub fn group(mut self, g: NavGroup) -> Self {
        self.groups.push(g);
        self
    }
    /// An element pinned to the bottom (AI panel button, privacy note, climate line).
    pub fn footer(mut self, el: impl IntoElement) -> Self {
        self.footer.push(el.into_any_element());
        self
    }
    /// Only for an embedded sidebar inside a suite's document (Surco's 212 px).
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
}

#[derive(IntoElement)]
struct SidebarBrand {
    mark: AnyElement,
    name: SharedString,
}

impl RenderOnce for SidebarBrand {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        div()
            .flex()
            .items_center()
            .gap(px(9.))
            .px(px(space::S3))
            .pt(px(space::S2))
            .pb(px(space::S7))
            .child(logo_badge(&t, self.mark))
            .child(div().text_size(px(text::TITLE)).font_weight(FontWeight::SEMIBOLD).text_color(t.ink).truncate().child(self.name))
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let mut list = div().id("otherwise-sidebar-items").flex().flex_col().flex_1().min_h_0().gap(px(space::S1)).overflow_y_scroll();
        for (i, g) in self.groups.into_iter().enumerate() {
            if let Some(title) = g.title {
                list = list.child(
                    div()
                        .pt(px(if i == 0 { space::S1 } else { space::S5 }))
                        .pb(px(3.))
                        .px(px(space::S5))
                        .text_size(px(text::CAPTION))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(t.ink_muted)
                        .child(title.to_uppercase()),
                );
            }
            list = list.children(g.items);
        }
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .flex_none()
            .w(px(self.width))
            .h_full()
            .gap(px(space::S1))
            .p(px(space::S5))
            .border_r_1()
            .border_color(t.border)
            .bg(t.surface)
            .children(self.header)
            .children(self.switcher.map(|el| div().flex().flex_col().flex_none().pb(px(space::S5)).child(el)))
            .child(list)
            .when(!self.footer.is_empty(), |d| d.child(div().flex().flex_col().flex_none().gap(px(space::S4)).pt(px(space::S4)).children(self.footer)))
    }
}

// ── header ───────────────────────────────────────────────────────────────────────────────────

/// The top bar: 56 px, surface colour, a bottom border. Badge, app name and a one-line status
/// on the left; an optional centre (page switcher, search); actions on the right (icon buttons
/// with tooltips, at most one primary button; the AI panel and Settings last).
#[derive(IntoElement)]
pub struct AppHeader {
    mark: AnyElement,
    title: SharedString,
    subtitle: Option<SharedString>,
    subtitle_color: Option<Hsla>,
    center: Option<AnyElement>,
    actions: Vec<AnyElement>,
}

impl AppHeader {
    /// `mark`: the app's white mark ([`logo_mark`]); `title`: the app (or document) name.
    pub fn new(mark: impl IntoElement, title: impl Into<SharedString>) -> Self {
        AppHeader { mark: mark.into_any_element(), title: title.into(), subtitle: None, subtitle_color: None, center: None, actions: Vec::new() }
    }
    /// The muted line under the name ("3 instaladas", the open file).
    pub fn subtitle(mut self, s: impl Into<SharedString>) -> Self {
        self.subtitle = Some(s.into());
        self
    }
    /// Colours the subtitle (warn when updates are waiting).
    pub fn subtitle_color(mut self, c: Hsla) -> Self {
        self.subtitle_color = Some(c);
        self
    }
    pub fn center(mut self, el: impl IntoElement) -> Self {
        self.center = Some(el.into_any_element());
        self
    }
    pub fn action(mut self, el: impl IntoElement) -> Self {
        self.actions.push(el.into_any_element());
        self
    }
}

impl RenderOnce for AppHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let sub_color = self.subtitle_color.unwrap_or(t.ink_muted);
        div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(space::S4))
            .h(px(size::HEADER))
            .px(px(space::S8))
            .border_b_1()
            .border_color(t.border)
            .bg(t.surface)
            .child(logo_badge(&t, self.mark))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    .flex_1()
                    .child(div().text_size(px(text::TITLE)).font_weight(FontWeight::SEMIBOLD).text_color(t.ink).truncate().child(self.title))
                    .children(self.subtitle.map(|s| div().text_size(px(text::CAPTION)).text_color(sub_color).truncate().child(s))),
            )
            .children(self.center)
            .children(self.actions)
    }
}

// ── footer ───────────────────────────────────────────────────────────────────────────────────

/// The status line: 28 px, surface colour, a top border, caption text. Status on the left; on
/// the right "<app> <version> · Otherwise Technologies LLC · Acerca de", which opens About.
#[derive(IntoElement)]
pub struct AppFooter {
    app: SharedString,
    version: Option<SharedString>,
    about_label: SharedString,
    status: Option<AnyElement>,
    items: Vec<AnyElement>,
    on_about: Option<ClickHandler>,
}

impl AppFooter {
    /// `about_label`: the caller's "Acerca de".
    pub fn new(app: impl Into<SharedString>, about_label: impl Into<SharedString>) -> Self {
        AppFooter { app: app.into(), version: None, about_label: about_label.into(), status: None, items: Vec::new(), on_about: None }
    }
    pub fn version(mut self, v: impl Into<SharedString>) -> Self {
        self.version = Some(v.into());
        self
    }
    /// The left side: a status line ("Comprobado hace 2 min", the data folder).
    pub fn status(mut self, el: impl IntoElement) -> Self {
        self.status = Some(el.into_any_element());
        self
    }
    /// Extra items before the credit (a GitHub star link, a sync dot).
    pub fn item(mut self, el: impl IntoElement) -> Self {
        self.items.push(el.into_any_element());
        self
    }
    pub fn on_about(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_about = Some(Rc::new(f));
        self
    }
}

/// The footer's credit text: "<app> <version> · Otherwise Technologies LLC · <about>".
pub fn footer_credit(app: &str, version: Option<&str>, about_label: &str) -> String {
    let name = match version {
        Some(v) if !v.is_empty() => format!("{app} {v}"),
        _ => app.to_string(),
    };
    format!("{name} · {} · {about_label}", crate::COMPANY) // i18n-ok: separators
}

impl RenderOnce for AppFooter {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let ink = t.ink;
        let credit = footer_credit(&self.app, self.version.as_deref(), &self.about_label);
        div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(space::S6))
            .h(px(size::FOOTER))
            .px(px(space::S8))
            .border_t_1()
            .border_color(t.border)
            .bg(t.surface)
            .text_size(px(text::CAPTION))
            .text_color(t.ink_muted)
            .child(div().flex().items_center().gap(px(space::S3)).flex_1().min_w_0().overflow_hidden().whitespace_nowrap().children(self.status))
            .children(self.items)
            .child(
                div()
                    .id("otherwise-footer-about")
                    .flex_none()
                    .cursor_pointer()
                    .hover(move |s| s.text_color(ink).underline())
                    .child(credit)
                    .when_some(self.on_about, |d, f| d.on_click(move |e, w, cx| f(e, w, cx))),
            )
    }
}

// ── modal ────────────────────────────────────────────────────────────────────────────────────

/// The modal frame: a dimmed backdrop over the whole window (dismisses on click) with a
/// centred card (radius 18, shadow, a close button top-right). Put it last in the root, which
/// must be `.relative()`.
pub fn modal(t: &Theme, id: impl Into<ElementId>, width: f32, on_dismiss: Option<DismissHandler>, card: impl FnOnce(Stateful<Div>) -> Stateful<Div>) -> Stateful<Div> {
    let close = on_dismiss.clone();
    let inner = div()
        .id(id)
        .relative()
        .flex()
        .flex_col()
        .gap(px(space::S6))
        .w(px(width))
        .max_h(relative(0.9))
        .overflow_y_scroll()
        .p(px(space::S10))
        .rounded(px(radius::MODAL))
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .text_color(t.ink)
        .shadow_lg()
        // clicks inside the card must not reach the backdrop
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation());
    let inner = card(inner).when_some(close, |d, f| {
        d.child(
            div()
                .absolute()
                .top(px(space::S5))
                .right(px(space::S5))
                .child(icon_button(t, "otherwise-modal-close", true).child(icon(IconName::Close, 15., t.ink_muted)).on_click(move |_, w, cx| f(&Dismiss, w, cx))),
        )
    });
    div()
        .id("otherwise-modal-backdrop")
        .absolute()
        .inset_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(t.backdrop)
        .when_some(on_dismiss, |d, f| d.on_mouse_down(MouseButton::Left, move |_, w, cx| f(&Dismiss, w, cx)))
        .child(inner)
}

// ── About ────────────────────────────────────────────────────────────────────────────────────

/// About: the app's logo, name and version, a one-line description, the credit
/// ("Hecho por Otherwise Technologies LLC", "Wyoming, EE. UU."), the website button, the Vcity
/// invitation, an optional source link, extras (a GitHub star), and the licence line.
/// Every string comes from the caller.
#[derive(IntoElement)]
pub struct AboutModal {
    logo: Option<ImageSource>,
    name: SharedString,
    version: SharedString,
    description: Option<SharedString>,
    credit: SharedString,
    place: Option<SharedString>,
    website: Option<(SharedString, SharedString)>,
    vcity: Option<(SharedString, SharedString, SharedString)>,
    source: Option<(SharedString, SharedString)>,
    license: Option<SharedString>,
    extra: Vec<AnyElement>,
    on_dismiss: Option<DismissHandler>,
}

impl AboutModal {
    /// `credit`: the caller's "Hecho por Otherwise Technologies LLC".
    pub fn new(name: impl Into<SharedString>, version: impl Into<SharedString>, credit: impl Into<SharedString>) -> Self {
        AboutModal {
            logo: None,
            name: name.into(),
            version: version.into(),
            description: None,
            credit: credit.into(),
            place: None,
            website: None,
            vcity: None,
            source: None,
            license: None,
            extra: Vec::new(),
            on_dismiss: None,
        }
    }
    /// The app icon (88 px).
    pub fn logo(mut self, src: impl Into<ImageSource>) -> Self {
        self.logo = Some(src.into());
        self
    }
    pub fn description(mut self, s: impl Into<SharedString>) -> Self {
        self.description = Some(s.into());
        self
    }
    /// The line under the credit ("Wyoming, EE. UU.").
    pub fn place(mut self, s: impl Into<SharedString>) -> Self {
        self.place = Some(s.into());
        self
    }
    /// The website button: its label ("Visitar otherwisetech.com") and URL.
    pub fn website(mut self, label: impl Into<SharedString>, url: impl Into<SharedString>) -> Self {
        self.website = Some((label.into(), url.into()));
        self
    }
    /// The Vcity invitation: its sentence, the link's label and the URL ([`crate::vcity_url`]).
    pub fn vcity(mut self, invite: impl Into<SharedString>, link_label: impl Into<SharedString>, url: impl Into<SharedString>) -> Self {
        self.vcity = Some((invite.into(), link_label.into(), url.into()));
        self
    }
    /// Open-source apps: the source link ("Código fuente en GitHub") and its URL.
    pub fn source(mut self, label: impl Into<SharedString>, url: impl Into<SharedString>) -> Self {
        self.source = Some((label.into(), url.into()));
        self
    }
    /// The licence line ("Libre y de código abierto · MIT o Apache-2.0", or the © line).
    pub fn license(mut self, s: impl Into<SharedString>) -> Self {
        self.license = Some(s.into());
        self
    }
    /// Anything else, above the licence (a GitHub star button).
    pub fn extra(mut self, el: impl IntoElement) -> Self {
        self.extra.push(el.into_any_element());
        self
    }
    pub fn on_dismiss(mut self, f: impl Fn(&Dismiss, &mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(f));
        self
    }
}

fn open_link(t: &Theme, id: &'static str, label: SharedString, url: SharedString) -> Stateful<Div> {
    div().id(id).text_size(px(text::SMALL)).text_color(t.accent).cursor_pointer().hover(|s| s.underline()).child(label).on_click(move |_, _, cx| cx.open_url(&url))
}

impl RenderOnce for AboutModal {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let AboutModal { logo, name, version, description, credit, place, website, vcity, source, license, extra, on_dismiss } = self;
        let tt = t;
        modal(&t, "otherwise-about", size::ABOUT_WIDTH, on_dismiss, move |card| {
            let t = tt;
            card.items_center()
                .px(px(space::S12))
                .pt(px(space::S12))
                .children(logo.map(|l| img(l).size(px(size::ABOUT_LOGO))))
                .child(
                    div()
                        .flex()
                        .items_baseline()
                        .gap(px(space::S4))
                        .child(div().text_size(px(text::DISPLAY)).font_weight(FontWeight::SEMIBOLD).child(name))
                        .child(div().text_size(px(text::BODY)).text_color(t.ink_muted).child(version)),
                )
                .children(description.map(|d| div().text_center().text_size(px(text::BODY)).line_height(px(leading::BODY)).text_color(t.ink_muted).child(d)))
                .child(div().w_full().h(px(1.)).my(px(space::S2)).bg(t.border))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(space::S1))
                        .child(div().text_size(px(text::BODY)).child(credit))
                        .children(place.map(|p| div().text_size(px(text::SMALL)).text_color(t.ink_muted).child(p))),
                )
                .children(website.map(|(label, url)| primary_button(&t, "otherwise-about-site").mt(px(space::S2)).child(label).on_click(move |_, _, cx| cx.open_url(&url))))
                .children(vcity.map(|(invite, label, url)| {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(space::S2))
                        .child(div().text_center().text_size(px(text::SMALL)).line_height(px(leading::SMALL)).text_color(t.ink_muted).child(invite))
                        .child(open_link(&t, "otherwise-about-vcity", label, url))
                }))
                .children(source.map(|(label, url)| open_link(&t, "otherwise-about-source", label, url)))
                .children(extra)
                .children(license.map(|l| div().text_center().text_size(px(text::CAPTION)).text_color(t.ink_muted).child(l)))
        })
    }
}

// ── AI panel ─────────────────────────────────────────────────────────────────────────────────

/// The AI panel's button at the bottom of the sidebar (full width, ⌘J). In apps without a
/// sidebar, use an [`icon_button`] with `IconName::Bot` before Settings in the header.
pub fn assistant_button(t: &Theme, id: impl Into<ElementId>, label: impl Into<SharedString>, keys: Option<SharedString>) -> Stateful<Div> {
    super::controls::outline_button(t, id).w_full().child(icon(IconName::Bot, icon_size::SM, t.ink)).child(label.into()).children(keys.map(|k| shortcut_hint(t, k)))
}
