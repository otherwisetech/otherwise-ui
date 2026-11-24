//! The shared chrome and building blocks.
//!
//! Functions take a [`crate::Theme`] (get it once per render with [`crate::theme`]) and return
//! GPUI elements you can keep styling, like the apps' own `ui.rs` helpers. The chrome
//! ([`Sidebar`], [`AppHeader`], [`AppFooter`], [`AboutModal`]) are builders that read the theme
//! themselves.
mod blocks;
mod chrome;
mod controls;

pub use blocks::*;
pub use chrome::*;
pub use controls::*;
