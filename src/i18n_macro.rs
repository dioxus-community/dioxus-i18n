#[macro_export]
macro_rules! t {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        {
            let mut params_map = dioxus_i18n::fluent::FluentArgs::new();
            $(
                params_map.set(stringify!($name), $value);
            )*
            dioxus_i18n::prelude::i18n().translate_with_args($id, Some(&params_map))
        }
    };

    ($id:expr ) => {
        {
            dioxus_i18n::prelude::i18n().translate($id)
        }
    };
}
