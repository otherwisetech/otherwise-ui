//! [`ServerSwitcher`]: where the app's data lives, Local or a server of the tailnet (OpenBot
//! style). A tile with the server's initials and a status dot, the server's name and a line
//! under it (the app and the open document); clicked, a menu: **Servers** with a check on the
//! active one, each server with a submenu (Server settings…, Remove…), then **Add a server**.
//!
//! View only: the app passes the servers, their states and its words, and gets callbacks. No
//! networking here (that is `otherwise-remote`).
use crate::theme::{Theme, theme};
use crate::tokens::{radius, space, text};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_component::{IconName, Sizable as _};
use std::rc::Rc;

/// A connection's state, as the dot on its tile shows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerState {
    /// This computer.
    Local,
    Connected,
    /// Connecting or syncing (the last copy is shown meanwhile).
    Connecting,
    /// No answer: read-only, from the last copy.
    Offline,
    /// Connected, but read-only (role, licence, a newer server).
    ReadOnly,
    /// The app or the server needs an update.
    Incompatible,
    /// Refused (401 / 403): ask the server's owner.
    NoAccess,
}

impl ServerState {
    /// The dot's colour (none for Local).
    pub fn dot(self, t: &Theme) -> Option<Hsla> {
        match self {
            ServerState::Local => None,
            ServerState::Connected => Some(t.ok),
            ServerState::Connecting => Some(t.ink_muted),
            ServerState::Offline => Some(t.ink_faint),
            ServerState::ReadOnly => Some(t.brand),
            ServerState::Incompatible => Some(t.warn),
            ServerState::NoAccess => Some(t.danger),
        }
    }
}

/// One row of the switcher.
#[derive(Debug, Clone, PartialEq)]
pub struct ServerItem {
    /// `local` for this computer, else the app's server id.
    pub id: SharedString,
    pub name: SharedString,
    pub state: ServerState,
    pub active: bool,
}

/// The switcher's words (the app's i18n).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerLabels {
    /// The menu's title: "Servidores".
    pub servers: SharedString,
    /// "Añadir un servidor".
    pub add: SharedString,
    /// In a server's submenu, to switch to it: "Usar este servidor".
    pub use_server: SharedString,
    /// "Ajustes del servidor…".
    pub settings: SharedString,
    /// "Quitar servidor…".
    pub remove: SharedString,
    /// The tooltip: "Dónde están los datos".
    pub tooltip: SharedString,
}

/// The first letters of up to two words ("Mac Studio" → "MS", "Local" → "L").
pub fn initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().filter(|w| w.chars().next().is_some_and(|c| c.is_alphanumeric())).collect();
    let mut s: String = words.iter().take(2).filter_map(|w| w.chars().next()).flat_map(char::to_uppercase).collect();
    if s.is_empty() {
        s = "?".into();
    }
    s
}

/// A stable tile colour per server (Local: the app's brand).
pub fn tile_color(t: &Theme, id: &str) -> Hsla {
    if id == "local" {
        return t.brand;
    }
    let h = id.bytes().fold(5381u32, |h, b| h.wrapping_mul(33) ^ b as u32);
    hsla((h % 360) as f32 / 360., 0.45, if t.dark { 0.45 } else { 0.42 }, 1.)
}

type Pick = Rc<dyn Fn(&str, &mut Window, &mut App)>;
type Act = Rc<dyn Fn(&mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct ServerSwitcher {
    id: ElementId,
    items: Vec<ServerItem>,
    subtitle: Option<SharedString>,
    labels: ServerLabels,
    on_select: Option<Pick>,
    on_add: Option<Act>,
    on_settings: Option<Pick>,
    on_remove: Option<Pick>,
}

impl ServerSwitcher {
    pub fn new(id: impl Into<ElementId>, items: Vec<ServerItem>, labels: ServerLabels) -> Self {
        ServerSwitcher { id: id.into(), items, subtitle: None, labels, on_select: None, on_add: None, on_settings: None, on_remove: None }
    }

    /// The line under the active server's name ("expense · Acme SAS").
    pub fn subtitle(mut self, s: impl Into<SharedString>) -> Self {
        self.subtitle = Some(s.into());
        self
    }

    pub fn on_select(mut self, f: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }

    pub fn on_add(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_add = Some(Rc::new(f));
        self
    }

    /// "Server settings…" in each server's submenu (not Local's).
    pub fn on_settings(mut self, f: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_settings = Some(Rc::new(f));
        self
    }

    /// "Remove server…" in each server's submenu (not Local's).
    pub fn on_remove(mut self, f: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_remove = Some(Rc::new(f));
        self
    }
}

/// The tile: initials on the server's colour, the status dot at its corner.
pub fn server_tile(t: &Theme, id: &str, name: &str, state: ServerState, size: f32) -> Div {
    div()
        .relative()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded(px(radius::CONTROL))
        .bg(tile_color(t, id))
        .text_size(px(if size >= 30. { text::BODY } else { text::CAPTION }))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(gpui::white())
        .child(initials(name))
        .when_some(state.dot(t), |d, c| d.child(div().absolute().right(px(-2.)).bottom(px(-2.)).size(px(10.)).rounded_full().border_2().border_color(t.surface).bg(c)))
}

impl RenderOnce for ServerSwitcher {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let active = self.items.iter().find(|i| i.active).cloned().or_else(|| self.items.first().cloned()).unwrap_or(ServerItem { id: "local".into(), name: "Local".into(), state: ServerState::Local, active: true }); // i18n-ok: fallback, the app always passes Local
        let tile = server_tile(&t, &active.id, &active.name, active.state, 32.);
        let body = div()
            .flex()
            .items_center()
            .gap(px(space::S3))
            .w_full()
            .min_w_0()
            .child(tile)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .child(div().text_size(px(text::BODY)).font_weight(FontWeight::SEMIBOLD).text_color(t.ink).truncate().child(active.name.clone()))
                    .when_some(self.subtitle.clone(), |d, s| d.child(div().text_size(px(text::CAPTION)).text_color(t.ink_muted).truncate().child(s))),
            )
            .child(crate::icon(IconName::ChevronsUpDown, 14., t.ink_muted));
        let items = self.items.clone();
        let labels = self.labels.clone();
        let (on_select, on_add, on_settings, on_remove) = (self.on_select.clone(), self.on_add.clone(), self.on_settings.clone(), self.on_remove.clone());
        Button::new(self.id)
            .ghost()
            .small()
            .w_full()
            .h(px(48.))
            .px(px(space::S2))
            .tooltip(labels.tooltip.clone())
            .child(body)
            .dropdown_menu(move |mut menu, window, cx| {
                menu = menu.label(labels.servers.clone());
                for it in &items {
                    let id = it.id.to_string();
                    let sel = on_select.clone();
                    if it.id == "local" || (on_settings.is_none() && on_remove.is_none()) {
                        menu = menu.item(PopupMenuItem::new(it.name.clone()).checked(it.active).on_click(move |_, window, cx| {
                            if let Some(f) = &sel {
                                f(&id, window, cx);
                            }
                        }));
                        continue;
                    }
                    // a server: its submenu has "use it", its settings and removing it
                    let (st, rm, l) = (on_settings.clone(), on_remove.clone(), labels.clone());
                    let check = it.active.then(|| gpui_component::Icon::new(IconName::Check));
                    let active = it.active;
                    menu = menu.submenu_with_icon(check, it.name.clone(), window, cx, move |mut sub, _, _| {
                        let (i, f) = (id.clone(), sel.clone());
                        sub = sub.item(PopupMenuItem::new(l.use_server.clone()).checked(active).on_click(move |_, window, cx| {
                            if let Some(f) = &f {
                                f(&i, window, cx);
                            }
                        }));
                        sub = sub.separator();
                        if let Some(f) = st.clone() {
                            let i = id.clone();
                            sub = sub.item(PopupMenuItem::new(l.settings.clone()).icon(IconName::Settings).on_click(move |_, window, cx| f(&i, window, cx)));
                        }
                        if let Some(f) = rm.clone() {
                            let i = id.clone();
                            sub = sub.item(PopupMenuItem::new(l.remove.clone()).icon(IconName::Delete).on_click(move |_, window, cx| f(&i, window, cx)));
                        }
                        sub
                    });
                }
                let add = on_add.clone();
                menu.separator().item(PopupMenuItem::new(labels.add.clone()).icon(IconName::Plus).on_click(move |_, window, cx| {
                    if let Some(f) = &add {
                        f(window, cx);
                    }
                }))
            })
    }
}

impl ServerLabels {
    /// The words in `es`, `en`, `pt` or `fr` (anything else: Spanish), for apps without their own.
    pub fn for_locale(locale: &str) -> ServerLabels {
        let (servers, add, use_server, settings, remove, tooltip) = match locale.get(..2).unwrap_or("") {
            "en" => ("Servers", "Add a server", "Use this server", "Server settings…", "Remove server…", "Where the data lives"),
            "pt" => ("Servidores", "Adicionar um servidor", "Usar este servidor", "Ajustes do servidor…", "Remover servidor…", "Onde ficam os dados"),
            "fr" => ("Serveurs", "Ajouter un serveur", "Utiliser ce serveur", "Réglages du serveur…", "Retirer le serveur…", "Où sont les données"),
            _ => ("Servidores", "Añadir un servidor", "Usar este servidor", "Ajustes del servidor…", "Quitar servidor…", "Dónde están los datos"),
        };
        ServerLabels { servers: servers.into(), add: add.into(), use_server: use_server.into(), settings: settings.into(), remove: remove.into(), tooltip: tooltip.into() }
    }
}
