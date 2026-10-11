#![doc = include_str!("../README.md")]
mod error;
pub mod i18n_macro;
pub mod use_i18n;

pub use fluent;
pub use unic_langid;

pub mod prelude {
    pub use crate::error::Error as DioxusI18nError;
    pub use crate::use_i18n::*;
}
