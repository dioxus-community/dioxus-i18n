mod common;
use common::*;

use dioxus_i18n::prelude::{use_init_i18n, I18n, I18nConfig};
use unic_langid::{langid, LanguageIdentifier};

#[test]
fn exact_locale_match_will_use_translation() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            value
                .try_translate("variants")
                .expect("test message id must exist"),
            "variants only".to_string(),
            "exact_locale_match_will_use_translation",
        );
    });
}

#[test]
fn non_exact_locale_match_will_use_region() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            value
                .try_translate("region")
                .expect("test message id must exist"),
            "region only".to_string(),
            "non_exact_locale_match_will_use_region",
        );
    });
}

#[test]
fn non_exact_locale_match_will_use_script() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            value
                .try_translate("script")
                .expect("test message id must exist"),
            "script only".to_string(),
            "non_exact_locale_match_will_use_script",
        );
    });
}

#[test]
fn non_exact_locale_match_will_use_language() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            value
                .try_translate("language")
                .expect("test message id must exist"),
            "language only".to_string(),
            "non_exact_locale_match_will_use_language",
        );
    });
}

#[test]
fn no_locale_match_will_use_fallback() {
    test_hook(i18n, |value, proxy| {
        proxy.assert(
            value
                .try_translate("fallback")
                .expect("test message id must exist"),
            "fallback only".to_string(),
            "no_locale_match_will_use_fallback",
        );
    });
}

fn i18n() -> I18n {
    const FALLBACK_LANG: LanguageIdentifier = langid!("fb-FB");
    const LANGUAGE_LANG: LanguageIdentifier = langid!("la");
    const SCRIPT_LANG: LanguageIdentifier = langid!("la-Scpt");
    const REGION_LANG: LanguageIdentifier = langid!("la-Scpt-LA");
    let variants_lang: LanguageIdentifier = langid!("la-Scpt-LA-variants");

    let config = I18nConfig::new(variants_lang.clone())
        .with_locale((LANGUAGE_LANG, include_str!("../tests/data/fallback/la.ftl")))
        .with_locale((
            SCRIPT_LANG,
            include_str!("../tests/data/fallback/la-Scpt.ftl"),
        ))
        .with_locale((
            REGION_LANG,
            include_str!("../tests/data/fallback/la-Scpt-LA.ftl"),
        ))
        .with_locale((
            variants_lang.clone(),
            include_str!("../tests/data/fallback/la-Scpt-LA-variants.ftl"),
        ))
        .with_locale((
            FALLBACK_LANG,
            include_str!("../tests/data/fallback/fb-FB.ftl"),
        ))
        .with_fallback(FALLBACK_LANG);
    use_init_i18n(|| config)
}
