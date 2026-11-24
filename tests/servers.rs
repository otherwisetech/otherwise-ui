//! The server switcher's pure parts (no window needed).
use otherwise_ui::{ServerLabels, ServerState, Theme, accents, initials, tile_color};

#[test]
fn initials_colours_and_dots() {
    assert_eq!(initials("Mac Studio"), "MS");
    assert_eq!(initials("Local"), "L");
    assert_eq!(initials("  oficina  de la casa "), "OD");
    assert_eq!(initials("—"), "?");
    let t = Theme::new(false, accents::EXPENSE_BUSINESS);
    assert_eq!(tile_color(&t, "local"), t.brand);
    assert_eq!(tile_color(&t, "studio"), tile_color(&t, "studio"), "stable per server");
    assert_ne!(tile_color(&t, "studio"), tile_color(&t, "imac"));
    assert_eq!(ServerState::Local.dot(&t), None);
    assert_eq!(ServerState::Connected.dot(&t), Some(t.ok));
    assert_eq!(ServerState::NoAccess.dot(&t), Some(t.danger));
    assert_eq!(ServerLabels::for_locale("en-US").add, "Add a server");
    assert_eq!(ServerLabels::for_locale("es").servers, "Servidores");
    assert_eq!(ServerLabels::for_locale("xx").remove, "Quitar servidor…");
}
