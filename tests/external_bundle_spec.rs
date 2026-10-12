mod common;
use common::test_hook::test_hook_after_first_render;
use common::*;

use dioxus_i18n::{
    prelude::{i18n_bundle, use_init_i18n_from_bundle, I18nBundle},
    t,
};
use fluent::{FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

const EN: LanguageIdentifier = langid!("en");

#[test]
fn bundle_hook_provides_configured_bundle_through_context() {
    test_hook(use_english_bundle, |_, proxy| {
        let name = "Ada";
        proxy.assert(
            t!("hello", name: name),
            "Hello, Ada!".to_string(),
            "external_bundle_preserves_fluent_options",
        );
    });
}

#[test]
fn externally_configured_bundle_accepts_an_empty_locale_list() {
    test_hook(
        || {
            use_init_i18n_from_bundle(|| bundle_without_locale("simple = Locale-free value"));
            i18n_bundle()
        },
        |i18n, proxy| {
            proxy.assert(
                i18n.primary_language(),
                None,
                "external_bundle_without_locale_has_no_primary_language",
            );
            proxy.assert(
                i18n.translate("simple"),
                "Locale-free value".to_string(),
                "external_bundle_handle_translates",
            );
            proxy.assert(
                t!("simple"),
                "Locale-free value".to_string(),
                "external_bundle_without_locale_translates",
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
            hook_bundle_handle.set(Some(bundle_handle));
            (
                i18n_bundle()
                    .primary_language()
                    .map(|language| language.to_string()),
                t!("simple"),
            )
        },
        move |render, _| observed_renders.borrow_mut().push(render),
        move || {
            let mut bundle_handle = bundle_handle
                .get()
                .expect("the bundle hook publishes its handle");
            bundle_handle.set_bundle(bundle_without_locale("simple = Locale-free value"));
        },
    );

    assert_eq!(
        *renders.borrow(),
        [
            (Some("en".to_string()), "Hello, Zaphod!".to_string()),
            (None, "Locale-free value".to_string()),
        ]
    );
}

fn use_english_bundle() -> I18nBundle {
    use_init_i18n_from_bundle(english_bundle)
}

fn english_bundle() -> FluentBundle<FluentResource> {
    bundle(EN, include_str!("./data/i18n/en.ftl"))
}

fn bundle(language: LanguageIdentifier, source: &str) -> FluentBundle<FluentResource> {
    configured_bundle(vec![language], source)
}

fn bundle_without_locale(source: &str) -> FluentBundle<FluentResource> {
    configured_bundle(Vec::new(), source)
}

fn configured_bundle(
    languages: Vec<LanguageIdentifier>,
    source: &str,
) -> FluentBundle<FluentResource> {
    let resource = FluentResource::try_new(source.to_string())
        .unwrap_or_else(|(_, errors)| panic!("test Fluent resource must parse: {errors:?}"));
    let mut bundle = FluentBundle::new(languages);
    bundle.set_use_isolating(false);
    bundle
        .add_resource(resource)
        .unwrap_or_else(|errors| panic!("test Fluent resource must be accepted: {errors:?}"));
    bundle
}
