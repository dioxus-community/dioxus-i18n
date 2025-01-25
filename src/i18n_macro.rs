//! Key translation macros.
//!
//! Using file:
//!
//! ```ftl
//! # en-US.ftl
//! #
//! hello = Hello, {$name}!
//! ```

/// Translate message from key, returning [`crate::prelude::DioxusI18nError`] if id not found...
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_i18n::{te, prelude::*};
/// # use unic_langid::langid;
/// # #[component]
/// # fn Example() -> Element {
/// #   let lang = langid!("en-US");
/// #   let config = I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}")).with_fallback(lang.clone());
/// #   let mut i18n = use_init_i18n(|| config);
/// let name = "Avery Gigglesworth";
/// let hi = te!("hello", name: {name}).expect("message id 'name' should be present");
/// assert_eq!(hi, "Hello, Avery Gigglesworth");
/// #   rsx! { "" }
/// # }
/// ```
///
#[macro_export]
macro_rules! te {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        {
            let mut params_map = dioxus_i18n::fluent::FluentArgs::new();
            $(
                params_map.set(stringify!($name), $value);
            )*
            dioxus_i18n::prelude::i18n().try_translate_with_args($id, Some(&params_map))
        }
    };

    ($id:expr ) => {
        {
            dioxus_i18n::prelude::i18n().try_translate($id)
        }
    };
}

/// Translate message from key, panic! if id not found...
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_i18n::{t, prelude::*};
/// # use unic_langid::langid;
/// # #[component]
/// # fn Example() -> Element {
/// #   let lang = langid!("en-US");
/// #   let config = I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}")).with_fallback(lang.clone());
/// #   let mut i18n = use_init_i18n(|| config);
/// let name = "Avery Gigglesworth";
/// let hi = t!("hello", name: {name});
/// assert_eq!(hi, "Hello, Avery Gigglesworth");
/// #   rsx! { "" }
/// # }
/// ```
///
#[macro_export]
macro_rules! t {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        dioxus_i18n::te!($id, $( $name : $value ),*).expect(concat!("message-id: ", $id, " should be translated"))
    };

    ($id:expr ) => {{
        dioxus_i18n::te!($id).expect(concat!("message-id: ", $id, " should be translated"))
    }};
}

/// Translate message from key, return id if no translation found...
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_i18n::{tid, prelude::*};
/// # use unic_langid::langid;
/// # #[component]
/// # fn Example() -> Element {
/// #   let lang = langid!("en-US");
/// #   let config = I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}")).with_fallback(lang.clone());
/// #   let mut i18n = use_init_i18n(|| config);
/// let message = tid!("no-key");
/// assert_eq!(message, "message-id: no-key should be translated");
/// #   rsx! { "" }
/// # }
/// ```
///
#[macro_export]
macro_rules! tid {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        dioxus_i18n::te!($id, $( $name : $value ),*).unwrap_or(format!("message-id: {:?} should be translated", $id))
    };

    ($id:expr ) => {{
        dioxus_i18n::te!($id).unwrap_or(format!("message-id: {:?} should be translated", $id))
    }};
}
