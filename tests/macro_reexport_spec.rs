// Test that macros work correctly when re-exported from another module
// This verifies that $crate is used correctly instead of hard-coded dioxus_i18n

mod reexport_module {
    // Re-export the macros as if they were from a different crate
    pub use dioxus_i18n::{t, te, tid};
}

mod common;
use common::*;

use dioxus_i18n::prelude::{use_init_i18n, I18n, I18nConfig};
use unic_langid::{langid, LanguageIdentifier};

#[test]
fn reexported_t_macro_works() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let name = "World";
            reexport_module::t!("hello", name: name)
        });
        proxy.assert(panic.is_ok(), true, "reexported_t_macro_works");
        proxy.assert(
            panic.ok().unwrap(),
            "Hello, \u{2068}World\u{2069}!".to_string(),
            "reexported_t_macro_works",
        );
    });
}

#[test]
fn reexported_te_macro_works() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let name = "World";
            reexport_module::te!("hello", name: name)
        });
        proxy.assert(panic.is_ok(), true, "reexported_te_macro_works");
        proxy.assert(
            panic.ok().unwrap().ok().unwrap(),
            "Hello, \u{2068}World\u{2069}!".to_string(),
            "reexported_te_macro_works",
        );
    });
}

#[test]
fn reexported_tid_macro_works() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let name = "World";
            reexport_module::tid!("hello", name: name)
        });
        proxy.assert(panic.is_ok(), true, "reexported_tid_macro_works");
        proxy.assert(
            panic.ok().unwrap(),
            "Hello, \u{2068}World\u{2069}!".to_string(),
            "reexported_tid_macro_works",
        );
    });
}

#[test]
fn reexported_macro_with_invalid_key_as_error() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| reexport_module::te!("invalid"));
        proxy.assert(
            panic.is_ok(),
            true,
            "reexported_macro_with_invalid_key_as_error",
        );
        proxy.assert(
            panic.ok().unwrap().err().unwrap().to_string(),
            "message id not found for key: 'invalid'".to_string(),
            "reexported_macro_with_invalid_key_as_error",
        );
    });
}

const EN: LanguageIdentifier = langid!("en");

fn i18n_from_static() -> I18n {
    let config = I18nConfig::new(EN).with_locale((EN, include_str!("./data/i18n/en.ftl")));
    use_init_i18n(|| config)
}
