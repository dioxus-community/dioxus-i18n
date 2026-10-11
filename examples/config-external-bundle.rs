//! Supply complete, application-configured Fluent bundles to Dioxus.
//!
//! This form is useful when the application owns resource composition, fallback order, or other
//! Fluent configuration. [`use_init_i18n_from_bundle`] provides the ordinary [`I18n`] reader
//! through Dioxus context and returns an [`ExternalBundleHandle`], which can replace the complete
//! bundle with one reactive update.
//!
//! Bundles must contain at least one locale. [`try_use_init_i18n_from_bundle`] and
//! [`ExternalBundleHandle::try_set_bundle`] report an invalid bundle without replacing the active
//! one.

use dioxus::prelude::*;
use dioxus_i18n::{
    fluent::{FluentBundle, FluentResource},
    prelude::*,
    t,
};
use unic_langid::{langid, LanguageIdentifier};

fn main() {
    launch(app);
}

fn create_bundle(language: LanguageIdentifier, source: &str) -> FluentBundle<FluentResource> {
    let resource = FluentResource::try_new(source.to_string()).expect("valid Fluent resource");
    let mut bundle = FluentBundle::new(vec![language]);

    // Add fallback resources and configure other Fluent options here before injection.
    bundle
        .add_resource(resource)
        .expect("non-conflicting Fluent resource");
    bundle
}

fn app() -> Element {
    let mut i18 = use_init_i18n_from_bundle(|| {
        create_bundle(langid!("en-US"), include_str!("./data/i18n/en-US.ftl"))
    });
    let en = move |_| {
        i18.set_bundle(create_bundle(
            langid!("en-US"),
            include_str!("./data/i18n/en-US.ftl"),
        ));
    };
    let es = move |_| {
        i18.set_bundle(create_bundle(
            langid!("es-ES"),
            include_str!("./data/i18n/es-ES.ftl"),
        ));
    };

    rsx! {
        button {
            onclick: en,
            "English"
        }
        button {
            onclick: es,
            "Español"
        }
        p { { t!("hello_world") } }
        p { { t!("hello", name: "Dioxus") } }
    }
}
