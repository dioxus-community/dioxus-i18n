#[macro_export]
macro_rules! translate {
    ( $i18n:expr, $id:expr, $( $name:ident : $value:expr ),* ) => {
        {
            let mut params_map = dioxus_i18n::fluent::FluentArgs::new();
            $(
                params_map.set(stringify!($name), $value);
            )*
            $i18n.translate_with_args($id, Some(&params_map))
        }
    };

    ( $i18n:expr, $id:expr ) => {
        {
            $i18n.translate($id)
        }
    };
}
