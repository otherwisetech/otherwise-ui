//! The crate's own words (About, footer, privacy note, Vcity) in Spanish, English, Portuguese
//! and French, taken from the Hub's catalogs.
//!
//! The components never call this: they take every string from the caller. An app with its own
//! i18n passes its catalog's strings; one that doesn't can pass these. `{app}` and `{company}`
//! are placeholders for [`Labels::fill`].

/// The crate's words in one language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Labels {
    pub locale: &'static str,
    /// The footer link and the menu item: "Acerca de".
    pub about: &'static str,
    /// The menu item with the app's name: "Acerca de {app}".
    pub about_app: &'static str,
    /// The credit: "Hecho por {company}".
    pub made_by: &'static str,
    /// Under the credit: "Wyoming, EE. UU.".
    pub place: &'static str,
    /// About's website button.
    pub visit: &'static str,
    /// About's source link (open-source apps).
    pub source: &'static str,
    /// Licence line of an open-source app.
    pub license_open: &'static str,
    /// Licence line of a proprietary app.
    pub license_proprietary: &'static str,
    /// The privacy note: "Tus datos están en este computador".
    pub privacy_local: &'static str,
    /// Its second line.
    pub privacy_local_detail: &'static str,
    /// The Vcity invitation in About.
    pub vcity_invite: &'static str,
    /// Its link.
    pub vcity_link: &'static str,
    /// The AI panel's button.
    pub assistant: &'static str,
    /// Close (modals).
    pub close: &'static str,
}

/// Español (the family's first language).
pub const ES: Labels = Labels {
    locale: "es",
    about: "Acerca de",
    about_app: "Acerca de {app}",
    made_by: "Hecho por {company}",
    place: "Wyoming, EE. UU.",
    visit: "Visitar otherwisetech.com",
    source: "Código fuente en GitHub",
    license_open: "Libre y de código abierto · MIT o Apache-2.0",
    license_proprietary: "© 2026 {company}. Todos los derechos reservados.",
    privacy_local: "Tus datos están en este computador",
    privacy_local_detail: "Nada sale de aquí sin que tú lo pidas.",
    vcity_invite: "¿Vendes productos o servicios? Crea la tienda virtual de tu empresa en Vcity: catálogo, pedidos, pagos, reservas, chat con clientes y mucho más.",
    vcity_link: "Crea tu tienda en Vcity",
    assistant: "Asistente",
    close: "Cerrar",
};

/// English.
pub const EN: Labels = Labels {
    locale: "en",
    about: "About",
    about_app: "About {app}",
    made_by: "Made by {company}",
    place: "Wyoming, USA",
    visit: "Visit otherwisetech.com",
    source: "Source code on GitHub",
    license_open: "Free and open source · MIT or Apache-2.0",
    license_proprietary: "© 2026 {company}. All rights reserved.",
    privacy_local: "Your data stays on this computer",
    privacy_local_detail: "Nothing leaves it unless you ask.",
    vcity_invite: "Sell products or services? Create your business's online store on Vcity: catalog, orders, payments, bookings, customer chat and more.",
    vcity_link: "Create your store on Vcity",
    assistant: "Assistant",
    close: "Close",
};

/// Português.
pub const PT: Labels = Labels {
    locale: "pt",
    about: "Sobre",
    about_app: "Sobre o {app}",
    made_by: "Feito por {company}",
    place: "Wyoming, EUA",
    visit: "Visitar otherwisetech.com",
    source: "Código-fonte no GitHub",
    license_open: "Livre e de código aberto · MIT ou Apache-2.0",
    license_proprietary: "© 2026 {company}. Todos os direitos reservados.",
    privacy_local: "Seus dados ficam neste computador",
    privacy_local_detail: "Nada sai daqui sem que você peça.",
    vcity_invite: "Vende produtos ou serviços? Crie a loja virtual da sua empresa na Vcity: catálogo, pedidos, pagamentos, reservas, chat com clientes e muito mais.",
    vcity_link: "Crie sua loja na Vcity",
    assistant: "Assistente",
    close: "Fechar",
};

/// Français.
pub const FR: Labels = Labels {
    locale: "fr",
    about: "À propos",
    about_app: "À propos de {app}",
    made_by: "Réalisé par {company}",
    place: "Wyoming, États-Unis",
    visit: "Visiter otherwisetech.com",
    source: "Code source sur GitHub",
    license_open: "Libre et open source · MIT ou Apache-2.0",
    license_proprietary: "© 2026 {company}. Tous droits réservés.",
    privacy_local: "Vos données restent sur cet ordinateur",
    privacy_local_detail: "Rien n'en sort sans votre demande.",
    vcity_invite: "Vous vendez des produits ou des services ? Créez la boutique en ligne de votre entreprise sur Vcity : catalogue, commandes, paiements, réservations, chat avec les clients et bien plus.",
    vcity_link: "Créez votre boutique sur Vcity",
    assistant: "Assistant",
    close: "Fermer",
};

/// Every table, Spanish first.
pub const ALL: [Labels; 4] = [ES, EN, PT, FR];

impl Labels {
    /// The table for a locale code (`es`, `es-CO`, `pt_BR`…); English when there is none.
    pub fn for_locale(code: &str) -> &'static Labels {
        let lang = code.split(['-', '_']).next().unwrap_or("").to_ascii_lowercase();
        ALL.iter().find(|l| l.locale == lang).unwrap_or(&ALL[1])
    }

    /// Replaces `{app}` and `{company}` in one of the strings.
    pub fn fill(text: &str, app: &str) -> String {
        text.replace("{app}", app).replace("{company}", crate::COMPANY)
    }
}
