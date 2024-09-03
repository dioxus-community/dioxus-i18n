#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_i18n::{prelude::*, translate};
use freya::prelude::*;
use unic_langid::langid;

fn main() {
    launch_with_props(app, "freya + i18n", (300.0, 200.0));
}

#[allow(non_snake_case)]
fn Body() -> Element {
    let mut i18n = use_i18n();

    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));

    rsx!(
        rect {
            rect {
                direction: "horizontal",
                Button {
                    onclick: change_to_english,
                    label {
                        "English"
                    }
                }
                Button {
                    onclick: change_to_spanish,
                    label {
                        "Spanish"
                    }
                }
            }

            label { {translate!(i18n, "hello", name: "Dioxus")} }
        }
    )
}

fn app() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale(Locale::new_static(
                langid!("en-US"),
                include_str!("./en-US.ftl"),
            ))
            .with_locale(Locale::new_dynamic(
                langid!("es-ES"),
                "./examples/es-ES.ftl",
            ))
    });

    rsx!(Body {})
}
