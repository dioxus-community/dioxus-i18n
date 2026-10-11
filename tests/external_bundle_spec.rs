mod common;
use common::*;

use dioxus::{dioxus_core::NoOpMutations, prelude::*};
use dioxus_i18n::{
    prelude::{
        i18n, try_use_init_i18n_from_bundle, use_init_i18n, use_init_i18n_from_bundle,
        DioxusI18nError, DioxusI18nExternalBundleError, I18n, I18nConfig,
    },
    t,
};
use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

const EN: LanguageIdentifier = langid!("en");
const DE: LanguageIdentifier = langid!("de");

#[test]
fn externally_configured_bundle_formats_values_attributes_and_arguments() {
    test_hook(i18n_from_bundle, |value, proxy| {
        proxy.assert(
            value
                .try_translate("simple")
                .expect("simple message must format"),
            "Simple value".to_string(),
            "external_bundle_value",
        );
        proxy.assert(
            value
                .try_translate("card.description")
                .expect("attribute must format"),
            "Card description".to_string(),
            "external_bundle_attribute",
        );

        let mut args = FluentArgs::new();
        args.set("name", "Ada");
        proxy.assert(
            value
                .try_translate_with_args("hello", Some(&args))
                .expect("message arguments must format"),
            "Hello, Ada!".to_string(),
            "external_bundle_arguments",
        );
        proxy.assert(
            t!("simple"),
            "Simple value".to_string(),
            "external_bundle_macro",
        );
    });
}

#[test]
fn fallible_bundle_hook_provides_successful_context() {
    test_hook(
        || try_use_init_i18n_from_bundle(english_bundle),
        |result, proxy| {
            proxy.assert(result.is_ok(), true, "external_bundle_try_hook");
            proxy.assert(
                t!("simple"),
                "Simple value".to_string(),
                "external_bundle_try_hook_context",
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
fn resource_driven_language_setters_reject_an_external_bundle() {
    test_hook(i18n_from_bundle, |mut value, proxy| {
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
            "Simple value".to_string(),
            "external_bundle_rejected_setters_preserve_state",
        );
    });
}

#[test]
fn resource_configured_provider_rejects_external_bundle_replacement() {
    test_hook(
        || use_init_i18n(|| I18nConfig::new(EN).with_locale((EN, "simple = Configured value"))),
        |mut value, proxy| {
            proxy.assert(
                matches!(
                    value.try_set_bundle(english_bundle()),
                    Err(DioxusI18nExternalBundleError::NotExternalBundleProvider)
                ),
                true,
                "configured_provider_rejects_external_bundle",
            );
            proxy.assert(
                value
                    .try_translate("simple")
                    .expect("rejected replacement must preserve configured state"),
                "Configured value".to_string(),
                "configured_provider_preserves_state",
            );
        },
    );
}

#[derive(Clone, Props)]
struct ReactiveBundleProbeProps {
    i18n: Rc<Cell<Option<I18n>>>,
    renders: Rc<RefCell<Vec<(String, String)>>>,
}

impl PartialEq for ReactiveBundleProbeProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

fn reactive_bundle_probe(props: ReactiveBundleProbeProps) -> Element {
    use_init_i18n_from_bundle(english_bundle);
    let i18n = i18n();
    props.i18n.set(Some(i18n));
    props
        .renders
        .borrow_mut()
        .push((i18n.language().to_string(), t!("simple")));
    rsx! {}
}

#[test]
fn replacing_an_external_bundle_rerenders_consumers_atomically() {
    let i18n = Rc::new(Cell::new(None));
    let renders = Rc::new(RefCell::new(Vec::new()));
    let mut dom = VirtualDom::new_with_props(
        reactive_bundle_probe,
        ReactiveBundleProbeProps {
            i18n: i18n.clone(),
            renders: renders.clone(),
        },
    );
    dom.rebuild_in_place();

    let mut i18n = i18n.get().expect("the bundle probe publishes its handle");
    i18n.set_bundle(bundle(DE, "simple = Einfacher Wert"));
    dom.render_immediate(&mut NoOpMutations);

    assert_eq!(
        *renders.borrow(),
        [
            ("en".to_string(), "Simple value".to_string()),
            ("de".to_string(), "Einfacher Wert".to_string()),
        ]
    );
}

fn i18n_from_bundle() -> I18n {
    use_init_i18n_from_bundle(english_bundle)
}

fn english_bundle() -> FluentBundle<FluentResource> {
    bundle(
        EN,
        "simple = Simple value\nhello = Hello, {$name}!\ncard = Card\n    .description = Card description",
    )
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
