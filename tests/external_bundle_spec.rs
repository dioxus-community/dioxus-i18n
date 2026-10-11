mod common;
use common::test_hook::test_hook_after_first_render;
use common::*;

use dioxus_i18n::{
    prelude::{
        i18n, try_use_init_i18n_from_bundle, use_init_i18n_from_bundle, DioxusI18nError,
        DioxusI18nExternalBundleError, ExternalBundleHandle,
    },
    t,
};
use fluent::{FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

const EN: LanguageIdentifier = langid!("en");
const DE: LanguageIdentifier = langid!("de");

#[test]
fn fallible_bundle_hook_provides_configured_bundle_through_context() {
    test_hook(
        || try_use_init_i18n_from_bundle(english_bundle),
        |result, proxy| {
            proxy.assert(result.is_ok(), true, "external_bundle_try_hook");
            let name = "Ada";
            proxy.assert(
                t!("hello", name: name),
                "Hello, Ada!".to_string(),
                "external_bundle_preserves_fluent_options",
            );
        },
    );
}

#[test]
fn externally_configured_bundle_requires_a_locale() {
    test_hook(
        || try_use_init_i18n_from_bundle(|| FluentBundle::<FluentResource>::new(Vec::new())),
        |result, proxy| {
            proxy.assert(
                matches!(result, Err(DioxusI18nExternalBundleError::MissingLocale)),
                true,
                "external_bundle_requires_locale",
            );
        },
    );
}

#[test]
fn external_bundle_handle_rejects_a_replacement_without_a_locale() {
    test_hook(use_english_bundle, |mut handle, proxy| {
        proxy.assert(
            matches!(
                handle.try_set_bundle(FluentBundle::<FluentResource>::new(Vec::new())),
                Err(DioxusI18nExternalBundleError::MissingLocale)
            ),
            true,
            "external_bundle_replacement_requires_locale",
        );
        proxy.assert(
            t!("simple"),
            "Hello, Zaphod!".to_string(),
            "rejected_external_bundle_replacement_preserves_state",
        );
    });
}

#[test]
fn resource_driven_language_setters_reject_an_external_bundle() {
    test_hook(
        || {
            use_english_bundle();
            i18n()
        },
        |mut value, proxy| {
            proxy.assert(
                matches!(
                    value.try_set_language(DE),
                    Err(DioxusI18nError::InvalidLanguageId(_))
                ),
                true,
                "external_bundle_rejects_language_setter",
            );
            proxy.assert(
                matches!(
                    value.try_set_fallback_language(DE),
                    Err(DioxusI18nError::InvalidLanguageId(_))
                ),
                true,
                "external_bundle_rejects_fallback_setter",
            );
            proxy.assert(
                value
                    .try_translate("simple")
                    .expect("rejected setters must leave the bundle active"),
                "Hello, Zaphod!".to_string(),
                "external_bundle_rejected_setters_preserve_state",
            );
        },
    );
}

#[test]
fn replacing_an_external_bundle_rerenders_consumers_atomically() {
    let bundle_handle = Rc::new(Cell::new(None));
    let renders = Rc::new(RefCell::new(Vec::new()));
    let hook_bundle_handle = bundle_handle.clone();
    let observed_renders = renders.clone();

    test_hook_after_first_render(
        move || {
            let bundle_handle = use_english_bundle();
            let i18n = i18n();
            hook_bundle_handle.set(Some(bundle_handle));
            (i18n.language().to_string(), t!("simple"))
        },
        move |render, _| observed_renders.borrow_mut().push(render),
        move || {
            let mut bundle_handle = bundle_handle
                .get()
                .expect("the bundle hook publishes its handle");
            bundle_handle.set_bundle(bundle(DE, "simple = Einfacher Wert"));
        },
    );

    assert_eq!(
        *renders.borrow(),
        [
            ("en".to_string(), "Hello, Zaphod!".to_string()),
            ("de".to_string(), "Einfacher Wert".to_string()),
        ]
    );
}

fn use_english_bundle() -> ExternalBundleHandle {
    use_init_i18n_from_bundle(english_bundle)
}

fn english_bundle() -> FluentBundle<FluentResource> {
    bundle(EN, include_str!("./data/i18n/en.ftl"))
}

fn bundle(language: LanguageIdentifier, source: &str) -> FluentBundle<FluentResource> {
    let resource = FluentResource::try_new(source.to_string())
        .unwrap_or_else(|(_, errors)| panic!("test Fluent resource must parse: {errors:?}"));
    let mut bundle = FluentBundle::new(vec![language]);
    bundle.set_use_isolating(false);
    bundle
        .add_resource(resource)
        .unwrap_or_else(|errors| panic!("test Fluent resource must be accepted: {errors:?}"));
    bundle
}
