use dioxus::prelude::*;
use dioxus_i18n::prelude::{i18n, use_init_i18n, I18nConfig};
use dioxus_i18n::t;
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
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./data/i18n/en-US.ftl")))
            .with_locale((
                langid!("es-ES"),
                PathBuf::from("./examples/data/i18n/es-ES.ftl"),
            ))
    });

    rsx! { Body {} }
}
