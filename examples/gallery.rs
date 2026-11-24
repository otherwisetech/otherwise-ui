//! Every component of otherwise-ui in one window.
//!
//! `cargo run --example gallery` · `-- --dark` starts in dark mode · `-- --accent "Mi Huerta"`
//! picks an app's accent. The header has the light / dark switch and the accent menu; the
//! blocks are drawn twice, on a light and a dark pane, whatever the window's mode.
use gpui::*;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_component::{IconName, Root, Sizable as _};
use otherwise_ui::tokens::{size, space, text};
use otherwise_ui::*;

const LOGO: &str = "icons/otherwise-mark.svg";

struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if path == LOGO {
            return Ok(Some(std::borrow::Cow::Borrowed(MARK.as_bytes())));
        }
        gpui_component_assets::Assets.load(path)
    }
    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        gpui_component_assets::Assets.list(path)
    }
}

/// The demo's app icon for About: the mark on a petrol rounded square.
const ICON: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 88 88"><rect width="88" height="88" rx="20" fill="#1F5F67"/><circle cx="44" cy="44" r="22" fill="none" stroke="#fff" stroke-width="8"/></svg>"##;

/// A plain "O" mark for the demo.
const MARK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4"><circle cx="12" cy="12" r="7.5"/></svg>"#;

#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Overview,
    Cards,
    Inputs,
    Empty,
}

struct Gallery {
    screen: Screen,
    about: bool,
    locale: &'static str,
    logo: std::sync::Arc<Image>,
}

impl Gallery {
    fn labels(&self) -> &'static Labels {
        Labels::for_locale(self.locale)
    }

    fn pane(&self, t: &Theme) -> Div {
        let l = self.labels();
        div()
            .flex()
            .flex_col()
            .gap(px(space::S6))
            .flex_1()
            .min_w_0()
            .p(px(space::S8))
            .rounded(px(tokens::radius::CARD))
            .bg(t.canvas)
            .text_color(t.ink)
            .child(section_title(t, if t.dark { "DARK" } else { "LIGHT" }))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(space::S4))
                    .child(primary_button(t, "b-primary").child("Primary"))
                    .child(outline_button(t, "b-outline").child("Secondary"))
                    .child(ghost_button(t, "b-ghost").child("Ghost"))
                    .child(danger_button(t, "b-danger").child("Delete"))
                    .child(icon_button(t, "b-icon", true).child(icon(IconName::Settings, tokens::icon::TOOLBAR, t.ink))),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(space::S3))
                    .child(tone_chip(t, "Neutral", Tone::Neutral))
                    .child(tone_chip(t, "Accent", Tone::Accent))
                    .child(tone_chip(t, "Info", Tone::Info))
                    .child(tone_chip(t, "Ok", Tone::Ok))
                    .child(tone_chip(t, "Warn", Tone::Warn))
                    .child(tone_chip(t, "Danger", Tone::Danger)),
            )
            .child(segmented(t).child(segment(t, "s1", true).child("Mes")).child(segment(t, "s2", false).child("Semana")).child(segment(t, "s3", false).child("Agenda")))
            .child(div().flex().gap(px(space::S4)).child(stat_tile(t, "Gastos del mes", "$ 1.240.000", Some("12 % menos que octubre".into()), Tone::Neutral)).child(stat_tile(
                t,
                "Ahorro",
                "18 %",
                None,
                Tone::Ok,
            )))
            .child(
                titled_card(t, "Card")
                    .child(div().text_size(px(text::BODY)).text_color(t.ink_muted).child("Surface, 1 px border, radius 14, padding 16."))
                    .child(labelled_progress(t, "Presupuesto · Mercado", "72 %", 0.72, Tone::Accent))
                    .child(progress_bar(t, 0.93, Tone::Warn)),
            )
            .child(alert_card(t, Tone::Warn, "Hay una actualización", Some("Mi Huerta 0.4.0 está lista para instalar.".into())).child(primary_button(t, "alert-go").child("Actualizar")))
            .child(alert_card(t, Tone::Danger, "No se pudo sincronizar", None))
            .child(banner(t, Tone::Info, "Los recibos se leen en este computador."))
            .child(banner(t, Tone::Neutral, "Un banner neutral."))
            .child(privacy_note(t, l.privacy_local, Some("~/.otherwise/mi-huerta".into())))
            .child(vcity_card(t, if t.dark { "vc-d" } else { "vc-l" }, l.vcity_invite, l.vcity_link, vcity_url("otherwise-ui-gallery")))
            .child(div().flex().justify_center().child(toast(t, "Guardado")))
    }

    fn set_dark(&mut self, dark: bool, window: &mut Window, cx: &mut Context<Self>) {
        apply(if dark { Appearance::Dark } else { Appearance::Light }, Some(window), cx);
        cx.notify();
    }
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = theme(cx);
        let l = self.labels();
        let screen = self.screen;
        let nav = |id: &'static str, label: &'static str, s: Screen, ic: IconName| NavItem::new(id, label).icon(ic).active(screen == s);
        let servers = vec![
            ServerItem { id: "local".into(), name: "Local".into(), state: ServerState::Local, active: false },
            ServerItem { id: "studio".into(), name: "Mac Studio".into(), state: ServerState::Connected, active: true },
            ServerItem { id: "imac".into(), name: "iMac oficina".into(), state: ServerState::Offline, active: false },
        ];
        let sidebar = Sidebar::new("sidebar")
            .header(div().pb(px(space::S4)).child(ServerSwitcher::new("servers", servers, ServerLabels::for_locale(l.locale)).subtitle("Gallery · Otherwise SAS").on_select(|_, _, _| {}).on_add(|_, _| {}).on_settings(|_, _, _| {}).on_remove(|_, _, _| {})))
            .switcher(Button::new("book-switch").label("Otherwise SAS").ghost().small().dropdown_caret(true).dropdown_menu(|menu, _, _| menu.label("Libros").item(PopupMenuItem::new("Otherwise SAS").checked(true)).item(PopupMenuItem::new("Otherwise LLC"))))
            .group(
                NavGroup::new(None::<&str>)
                    .item(nav("n-overview", "Resumen", Screen::Overview, IconName::LayoutDashboard).shortcut("⌘1").on_click(cx.listener(|this, _, _, cx| {
                        this.screen = Screen::Overview;
                        cx.notify();
                    })))
                    .item(nav("n-cards", "Tarjetas", Screen::Cards, IconName::Inbox).badge(Some("3")).shortcut("⌘2").on_click(cx.listener(|this, _, _, cx| {
                        this.screen = Screen::Cards;
                        cx.notify();
                    }))),
            )
            .group(
                NavGroup::new(Some("Más"))
                    .item(nav("n-inputs", "Controles", Screen::Inputs, IconName::Settings).on_click(cx.listener(|this, _, _, cx| {
                        this.screen = Screen::Inputs;
                        cx.notify();
                    })))
                    .item(nav("n-empty", "Vacío", Screen::Empty, IconName::Search).on_click(cx.listener(|this, _, _, cx| {
                        this.screen = Screen::Empty;
                        cx.notify();
                    }))),
            )
            .footer(assistant_button(&t, "assistant", l.assistant, Some("⌘J".into())))
            .footer(privacy_note(&t, l.privacy_local, None));

        let weak = cx.entity().downgrade();
        let current = accent(cx);
        let accent_menu = Button::new("accent").ghost().label("Accent").icon(IconName::Palette).dropdown_menu(move |mut menu, _, _| {
            for (name, a) in accents::ALL {
                let w = weak.clone();
                menu = menu.item(PopupMenuItem::new(name).checked(a == current).on_click(move |_, _, cx| {
                    set_accent(a, cx);
                    w.update(cx, |_, cx| cx.notify()).ok();
                }));
            }
            menu
        });
        let weak = cx.entity().downgrade();
        let here = self.locale;
        let lang_menu = Button::new("lang").ghost().icon(IconName::Globe).label(here.to_uppercase()).dropdown_menu(move |mut menu, _, _| {
            for code in ["es", "en", "pt", "fr"] {
                let w = weak.clone();
                menu = menu.item(PopupMenuItem::new(code.to_uppercase()).checked(code == here).on_click(move |_, _, cx| {
                    w.update(cx, |this, cx| {
                        this.locale = code;
                        cx.notify();
                    })
                    .ok();
                }));
            }
            menu
        });
        let dark = t.dark;
        let mode = segmented(&t)
            .child(segment(&t, "light", !dark).child(icon(IconName::Sun, 13., if dark { t.ink_muted } else { t.ink })).on_click(cx.listener(|this, _, w, cx| this.set_dark(false, w, cx))))
            .child(segment(&t, "dark", dark).child(icon(IconName::Moon, 13., if dark { t.ink } else { t.ink_muted })).on_click(cx.listener(|this, _, w, cx| this.set_dark(true, w, cx))));
        let header = AppHeader::new(logo_mark(LOGO), "otherwise-ui")
            .subtitle(format!("{} · {}", if dark { "dark" } else { "light" }, accents::ALL.iter().find(|(_, a)| *a == current).map(|(n, _)| *n).unwrap_or("custom")))
            .center(mode)
            .action(lang_menu)
            .action(accent_menu)
            .action(icon_button(&t, "h-bot", true).child(icon(IconName::Bot, tokens::icon::TOOLBAR, t.ink)))
            .action(icon_button(&t, "h-settings", true).child(icon(IconName::Settings, tokens::icon::TOOLBAR, t.ink)));
        let footer = AppFooter::new("Gallery", l.about).version(env!("CARGO_PKG_VERSION")).status(div().truncate().child("~/.otherwise/gallery · OneDrive")).on_about(cx.listener(|this, _, _, cx| {
            this.about = true;
            cx.notify();
        }));

        let body: AnyElement = match self.screen {
            Screen::Empty => {
                empty_state(&t, IconName::Search, "Nada por aquí", Some("Prueba con otra búsqueda o crea el primero.".into())).child(primary_button(&t, "empty-new").child("Crear")).into_any_element()
            }
            Screen::Inputs => div()
                .p(px(space::PAGE))
                .flex()
                .flex_col()
                .gap(px(space::S6))
                .max_w(px(560.))
                .child(search_frame(&t).w(px(260.)).child(div().text_color(t.ink_faint).child("Buscar…")))
                .child(labelled(&t, "Nombre", input_frame(&t).child("Tomate cherry")))
                .child(setting_row(
                    &t,
                    "Tema",
                    Some("Claro, oscuro o el del sistema".into()),
                    segmented(&t).child(segment(&t, "t1", false).child("Claro")).child(segment(&t, "t2", true).child("Sistema")),
                ))
                .child(setting_row(&t, "Idioma", None, outline_button(&t, "lang-btn").child("Español")))
                .child(div().flex().gap(px(space::S4)).child(link(&t, "l1").child("Un enlace")).child(shortcut_hint(&t, "⌘K")))
                .child(divider(&t))
                .into_any_element(),
            _ => div()
                .id("panes")
                .size_full()
                .overflow_y_scroll()
                .p(px(space::PAGE))
                .child(div().flex().gap(px(space::S8)).child(self.pane(&Theme::light(current))).child(self.pane(&Theme::dark(current))))
                .into_any_element(),
        };
        div()
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .bg(t.canvas)
            .text_color(t.ink)
            .font_family(tokens::SANS)
            .child(header)
            .child(div().flex().flex_1().min_h_0().child(sidebar).child(div().flex_1().min_w_0().h_full().child(body)))
            .child(footer)
            .when(self.about, |d| {
                d.child(
                    AboutModal::new("Gallery", env!("CARGO_PKG_VERSION"), Labels::fill(l.made_by, "Gallery"))
                        .logo(self.logo.clone())
                        .description("Todos los componentes de otherwise-ui.")
                        .place(l.place)
                        .website(l.visit, COMPANY_URL)
                        .vcity(l.vcity_invite, l.vcity_link, vcity_url("otherwise-ui-gallery"))
                        .source(l.source, "https://github.com/otherwisetech/otherwise-ui")
                        .license(l.license_open)
                        .on_dismiss(cx.listener(|this, _: &Dismiss, _, cx| {
                            this.about = false;
                            cx.notify();
                        })),
                )
            })
            .child(div().absolute().bottom(px(size::FOOTER + 12.)).left_0().right_0().flex().justify_center().child(div().text_size(px(text::CAPTION)).text_color(t.ink_faint).child(format!(
                "{} px sidebar · {} px header · {} px footer",
                size::SIDEBAR,
                size::HEADER,
                size::FOOTER
            ))))
    }
}

use gpui::prelude::FluentBuilder as _;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dark = args.iter().any(|a| a == "--dark");
    let pick = args
        .iter()
        .position(|a| a == "--accent")
        .and_then(|i| args.get(i + 1))
        .and_then(|n| accents::ALL.iter().find(|(name, _)| name.eq_ignore_ascii_case(n)).map(|(_, a)| *a))
        .unwrap_or(accents::HUB);
    gpui_platform::application().with_assets(Assets).run(move |cx| {
        gpui_component::init(cx);
        otherwise_ui::init(pick, cx);
        apply(if dark { Appearance::Dark } else { Appearance::Light }, None, cx);
        let options = WindowOptions { window_bounds: Some(WindowBounds::centered(gpui::size(px(1240.), px(860.)), cx)), ..Default::default() };
        cx.spawn(async move |cx| {
            cx.open_window(options, |window, cx| {
                let logo = std::sync::Arc::new(Image::from_bytes(ImageFormat::Svg, ICON.as_bytes().to_vec()));
                let view = cx.new(|_| Gallery { screen: Screen::Overview, about: false, locale: "es", logo });
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("open window");
        })
        .detach();
    });
}
