//! This example demonstrates how to use pathbuf derived I18nConfig.
//! This is useful for WASM targets; the paths to the translation files must be known at compile time.

use dioxus::prelude::*;
use dioxus_i18n::prelude::{i18n, use_init_i18n, I18nConfig, Locale};
use dioxus_i18n::t;
use unic_langid::langid;

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
        // The locales can be added with an implicitly derived locale (see config-dynamic-pathbuf.rs for a comparison)
        // or using an explicit Locale::new_static call.
        //
        // The two examples are functionally equivalent.
        //
        // It IS supported in WASM targets.
        I18nConfig::new(langid!("en-US"))
            // Implicit...
            .with_locale((langid!("es-ES"), include_str!("./data/i18n/es-ES.ftl")))
            // Explicit...
            .with_locale(Locale::new_static(
                langid!("en-US"),
                include_str!("./data/i18n/en-US.ftl"),
            ))
    });

    rsx! { Body {} }
}
