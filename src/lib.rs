#![doc = include_str!("../README.md")]
mod error;
pub mod i18n_macro;
pub mod use_i18n;

pub use error::Error as DioxusI18nError;
pub use fluent;
pub use unic_langid;
pub use use_i18n::*;

/// Implementation details required by exported macros.
#[doc(hidden)]
pub mod __private {
    use crate::{error::Error, fluent::FluentArgs, use_i18n::I18nContext};
    use dioxus::prelude::consume_context;

    #[inline]
    pub fn try_translate_from_context(msg: &str) -> Result<String, Error> {
        consume_context::<I18nContext>().try_translate(msg)
    }

    #[inline]
    pub fn try_translate_with_args_from_context(
        msg: &str,
        args: Option<&FluentArgs>,
    ) -> Result<String, Error> {
        consume_context::<I18nContext>().try_translate_with_args(msg, args)
    }
}

pub mod prelude {
    pub use crate::use_i18n::*;
    pub use crate::DioxusI18nError;
}
