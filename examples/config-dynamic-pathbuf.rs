//! This example demonstrates how to use pathbuf derived I18nConfig.
//! This is useful when the path to the translation files is not known at compile time.

use dioxus::prelude::*;
use dioxus_i18n::{i18n, t, use_init_i18n, I18nConfig, Locale};
use unic_langid::langid;

use std::path::PathBuf;

fn main() {
    launch(app);
}

#[component]
fn Body() -> Element {
    let mut i18n = i18n();
    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));

    rsx!(
        button {
            onclick: change_to_english,
            label {
                "English"
            }
        }
        button {
            onclick: change_to_spanish,
            label {
                "Spanish"
            }
        }
        p { { t!("hello_world") } }
        p { { t!("hello", name: "Dioxus") }  }
    )
}

fn app() -> Element {
    use_init_i18n(|| {
        // This initialisation allows individual translation files to be selected.
        // The locales can be added with an implicitly derived locale (see config-static-includestr.rs for a comparison)
        // or using an explicit Locale::new_dynamic call.
        //
        // The two examples are functionally equivalent.
        //
        // It IS NOT supported in WASM targets.
        I18nConfig::new(langid!("en-US"))
            // Implicit...
            .with_locale((
                langid!("es-ES"),
                PathBuf::from("./examples/data/i18n/es-ES.ftl"),
            ))
            // Explicit...
            .with_locale(Locale::new_dynamic(
                langid!("en-US"),
                PathBuf::from("./examples/data/i18n/en-US.ftl"),
            ))
    });

    rsx! { Body {} }
}
