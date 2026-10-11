mod common;
use common::*;

use dioxus_i18n::{
    prelude::{use_init_i18n, I18n, I18nConfig},
    t,
};
use unic_langid::{langid, LanguageIdentifier};

#[test]
fn issue_15_recent_change_to_t_macro_unnecessarily_breaks_v0_3_code_test_attr() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let name = "World";
            let message_id = "hello".to_owned();
            t!(&message_id, name: name)
        });
        proxy.assert(panic.is_ok(), true, "translate_from_static_source");
        proxy.assert(
            panic.ok().unwrap(),
            "Hello, \u{2068}World\u{2069}!".to_string(),
            "translate_from_static_source",
        );
    });
}

#[test]
fn issue_15_recent_change_to_t_macro_unnecessarily_breaks_v0_3_code_test_no_attr() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let message_id = "simple".to_owned();
            t!(&message_id)
        });
        proxy.assert(panic.is_ok(), true, "translate_from_static_source");
        proxy.assert(
            panic.ok().unwrap(),
            "Hello, Zaphod!".to_string(),
            "translate_from_static_source",
        );
    });
}

const EN: LanguageIdentifier = langid!("en");

fn i18n_from_static() -> I18n {
    let config = I18nConfig::new(EN).with_locale((EN, include_str!("./data/i18n/en.ftl")));
    use_init_i18n(|| config)
}
