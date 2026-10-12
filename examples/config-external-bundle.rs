//! Supply complete, application-configured Fluent bundles to Dioxus.
//!
//! This form is useful when the application owns resource composition, fallback order, or other
//! Fluent configuration. [`use_init_i18n_from_bundle`] provides an `I18nBundle` through Dioxus
//! context, which can translate and replace the complete bundle with one reactive update.
//!
//! Parsing resources, composing fallbacks, and configuring the bundle remain the application's
//! responsibility; the Dioxus adapter stores and reactively replaces the completed bundle.
//! When that construction can fail, [`try_use_init_i18n_from_bundle`] returns the application-owned
//! error unchanged and provides the Dioxus contexts only after construction succeeds.

use dioxus::prelude::*;
use dioxus_i18n::fluent::{FluentBundle, FluentResource};
use dioxus_i18n::prelude::{i18n_bundle, use_init_i18n_from_bundle};
use dioxus_i18n::t;
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

#[component]
fn Body() -> Element {
    let mut i18n = i18n_bundle();
    let en = move |_| {
        i18n.set_bundle(create_bundle(
            langid!("en-US"),
            include_str!("./data/i18n/en-US.ftl"),
        ));
    };
    let es = move |_| {
        i18n.set_bundle(create_bundle(
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

fn app() -> Element {
    use_init_i18n_from_bundle(|| {
        create_bundle(langid!("en-US"), include_str!("./data/i18n/en-US.ftl"))
    });

    rsx! { Body {} }
}

// This module demonstrates how application-owned Fluent configuration can be tested
// without mounting the Dioxus stack.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_bundle_formats_without_a_dioxus_root() {
        let bundle = create_bundle(langid!("en-US"), include_str!("./data/i18n/en-US.ftl"));
        let message = bundle
            .get_message("hello_world")
            .expect("example bundle contains hello_world");
        let pattern = message
            .value()
            .expect("hello_world contains a message value");
        let mut errors = Vec::new();

        let translation = bundle.format_pattern(pattern, None, &mut errors);

        assert!(errors.is_empty(), "message formats without Fluent errors");
        assert_eq!(translation, "Hello, World!");
    }
}
