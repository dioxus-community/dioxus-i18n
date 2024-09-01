pub mod tanslate;
pub mod use_i18n;

pub use fluent;
pub use unic_langid;

pub mod prelude {
    pub use crate::use_i18n::*;
}
