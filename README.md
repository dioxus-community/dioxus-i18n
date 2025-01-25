# dioxus-i18n 🌍

i18n integration for Dioxus apps based on the [Project Fluent](https://github.com/projectfluent/fluent-rs).

> This crate used to be in the [Dioxus SDK](https://github.com/DioxusLabs/sdk).

## Support

- **Dioxus v0.6** 🧬
- Renderers:
  - [web](https://dioxuslabs.com/learn/0.6/guides/web/),
  - [desktop](https://dioxuslabs.com/learn/0.6/guides/desktop/),
  - [freya](https://github.com/marc2332/freya) (doesn't support Dioxus 0.6 yet)
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
            // implicit [`Locale`]
            .with_locale(( // Embed
                langid!("en-US"),
                include_str!("./en-US.ftl")
            ))
            .with_locale(( // Load at launch
                langid!("es-ES"),
                PathBuf::from("./es-ES.ftl"),
            ))
            .with_locale((     // Locales will share duplicated locale_resources
                langid!("en"), // which is useful to assign a specific region for
                include_str!("./en-US.ftl") // the primary language
            ))
            // explicit [`Locale`]
            .with_locale(Locale::new_static( // Embed
                langid!("en-US"),
                include_str!("./en-US.ftl"),
            ))
            .with_locale(Locale::new_dynamic( // Load at launch
                langid!("es-ES"),
                PathBuf::from("./es-ES.ftl"),
            ))
    });

    rsx!(
        label { { t!("hello", name: "World") } }
    )
}
```

## Development

```bash
# Checks clean compile against `#[cfg(not(target_arch = "wasm32"))]`
cargo build --target wasm32-unknown-unknown

# Runs all tests
cargo test
```

[MIT License](./LICENSE.md)
