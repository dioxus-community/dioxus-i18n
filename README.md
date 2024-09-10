# dioxus-i18n 🌍

i18n integration for Dioxus apps based on the [Project Fluent](https://github.com/projectfluent/fluent-rs).

> This crate used to be in the [Dioxus SDK](https://github.com/DioxusLabs/sdk).

## Support

- **Dioxus v0.5** 🧬
- All renderers ([web](https://dioxuslabs.com/learn/0.5/getting_started/wasm), [desktop](https://dioxuslabs.com/learn/0.5/getting_started/desktop), [freya](https://github.com/marc2332/freya), etc)
- Both WASM and native targets

## Example:

```ftl
# en-US.ftl

hello = Hello, {$name}!
```

```rs
// main.rs

fn app() -> Element {
    let i18 = use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale(Locale::new_static( // Embed
                langid!("en-US"),
                include_str!("./en-US.ftl"),
            ))
            .with_locale(Locale::new_dynamic( // Load at launch
                langid!("es-ES"),
                include_str!("./es-ES.ftl"),
            ))
    });

    rsx!(
        label { { t!("hello", name: "World") } }
    )
}
```

[MIT License](./LICENSE.md)
