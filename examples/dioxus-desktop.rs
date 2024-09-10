use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};
use unic_langid::langid;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
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
