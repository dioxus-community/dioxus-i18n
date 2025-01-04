mod common;
use common::*;

use dioxus_i18n::{
    prelude::{use_init_i18n, I18n, I18nConfig},
    t, te, tid,
};
use unic_langid::{langid, LanguageIdentifier};

use std::path::PathBuf;

#[test]
fn translate_from_static_source() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let name = "World";
            t!("hello", name: name)
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
fn failed_to_translate_with_invalid_key() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let _ = &t!("invalid");
        });
        proxy.assert(panic.is_err(), true, "failed_to_translate_with_invalid_key");
    });
}

#[test]
fn failed_to_translate_with_invalid_key_as_error() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| te!("invalid"));
        proxy.assert(
            panic.is_ok(),
            true,
            "failed_to_translate_with_invalid_key_as_error",
        );
        proxy.assert(
            panic.ok().unwrap().err().unwrap().to_string(),
            "message id not found for key: 'invalid'".to_string(),
            "failed_to_translate_with_invalid_key_as_error",
        );
    });
}

#[test]
fn failed_to_translate_with_invalid_key_with_args_as_error() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| te!("invalid", name: "<don't care>"));
        proxy.assert(
            panic.is_ok(),
            true,
            "failed_to_translate_with_invalid_key_with_args_as_error",
        );
        proxy.assert(
            panic.ok().unwrap().err().unwrap().to_string(),
            "message id not found for key: 'invalid'".to_string(),
            "failed_to_translate_with_invalid_key_with_args_as_error",
        );
    });
}

#[test]
fn failed_to_translate_with_invalid_key_as_id() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| tid!("invalid"));
        proxy.assert(
            panic.is_ok(),
            true,
            "failed_to_translate_with_invalid_key_as_id",
        );
        proxy.assert(
            panic.ok().unwrap(),
            "message-id: \"invalid\" should be translated".to_string(),
            "failed_to_translate_with_invalid_key_as_id",
        );
    });
}

#[test]
fn failed_to_translate_with_invalid_key_with_args_as_id() {
    test_hook(i18n_from_static, |_, proxy| {
        let panic = std::panic::catch_unwind(|| tid!("invalid", name: "<don't care>"));
        proxy.assert(
            panic.is_ok(),
            true,
            "failed_to_translate_with_invalid_key_with_args_as_id",
        );
        proxy.assert(
            panic.ok().unwrap(),
            "message-id: \"invalid\" should be translated".to_string(),
            "failed_to_translate_with_invalid_key_with_args_as_id",
        );
    });
}

#[test]
fn translate_from_dynamic_source() {
    test_hook(i18n_from_dynamic, |_, proxy| {
        let panic = std::panic::catch_unwind(|| {
            let name = "World";
            t!("hello", name: name)
        });
        proxy.assert(panic.is_ok(), true, "translate_from_dynamic_source");
        proxy.assert(
            panic.ok().unwrap(),
            "Hello, \u{2068}World\u{2069}!".to_string(),
            "translate_from_dynamic_source",
        );
    });
}

#[test]
#[should_panic]
#[ignore] // Panic hidden within test_hook.
fn fail_translate_from_dynamic_source_when_file_does_not_exist() {
    test_hook(i18n_from_dynamic_none_existing, |_, _| unreachable!());
}

#[test]
fn initial_language_is_set() {
    test_hook(i18n_from_static, |value, proxy| {
        proxy.assert(value.language(), EN, "initial_language_is_set");
    });
}

#[test]
fn language_can_be_set() {
    test_hook(i18n_from_static, |mut value, proxy| {
        value
            .try_set_language(JP)
            .expect("set_language must succeed");
        proxy.assert(value.language(), JP, "language_can_be_set");
    });
}

#[test]
fn no_default_fallback_language() {
    test_hook(i18n_from_static, |value, proxy| {
        proxy.assert(
            format!("{:?}", value.fallback_language()),
            "None".to_string(),
            "no_default_fallback_language",
        );
    });
}

#[test]
fn some_default_fallback_language() {
    test_hook(i18n_from_static_with_fallback, |value, proxy| {
        proxy.assert(
            format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"jp\")".to_string(),
            "some_default_fallback_language",
        );
    });
}

#[test]
fn fallback_language_can_be_set() {
    test_hook(i18n_from_static_with_fallback, |mut value, proxy| {
        value
            .try_set_fallback_language(EN)
            .expect("try_set_fallback_language must succeed");
        proxy.assert(
            format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"en\")".to_string(),
            "fallback_language_can_be_set",
        );
    });
}

#[test]
fn fallback_language_must_have_locale_translation() {
    test_hook(i18n_from_static_with_fallback, |mut value, proxy| {
        let result = value.try_set_fallback_language(IT);

        proxy.assert(
            result.is_err(),
            true,
            "fallback_language_must_have_locale_translation",
        );
        proxy.assert(
            result.err().unwrap().to_string(),
            "fallback for \"it\" must have locale".to_string(),
            "fallback_language_must_have_locale_translation",
        );
        proxy.assert(
            format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"jp\")".to_string(),
            "fallback_language_must_have_locale_translation",
        );
    });
}

const EN: LanguageIdentifier = langid!("en");
const IT: LanguageIdentifier = langid!("it");
const JP: LanguageIdentifier = langid!("jp");

fn i18n_from_static() -> I18n {
    let config = I18nConfig::new(EN).with_locale((EN, include_str!("./data/i18n/en.ftl")));
    use_init_i18n(|| config)
}

fn i18n_from_static_with_fallback() -> I18n {
    let config = I18nConfig::new(EN)
        .with_locale((EN, include_str!("./data/i18n/en.ftl")))
        .with_fallback(JP);
    use_init_i18n(|| config)
}

fn i18n_from_dynamic() -> I18n {
    let config = I18nConfig::new(EN).with_locale((
        EN,
        PathBuf::from(format!(
            "{}/tests/data/i18n/en.ftl",
            env!("CARGO_MANIFEST_DIR")
        )),
    ));
    use_init_i18n(|| config)
}

fn i18n_from_dynamic_none_existing() -> I18n {
    let config = I18nConfig::new(EN).with_locale((
        EN,
        PathBuf::from(format!(
            "{}/tests/data/i18n/non_existing.ftl",
            env!("CARGO_MANIFEST_DIR")
        )),
    ));
    use_init_i18n(|| config)
}
