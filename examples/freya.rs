#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//! Run with `cargo run --example freya --features freya`.

use dioxus_i18n::prelude::{i18n, use_init_i18n, I18nConfig};
use dioxus_i18n::t;
use freya::prelude::*;
use unic_langid::langid;

use std::path::PathBuf;

fn main() {
    launch_with_props(app, "freya + i18n", (300.0, 200.0));
}

#[component]
fn Body() -> Element {
    let mut i18n = i18n();
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
            label { { t!("hello_world") } }
            label { { t!("hello", name: "Dioxus") } }
        }
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
