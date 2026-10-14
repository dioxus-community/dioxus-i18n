# dioxus-i18n 🌍

Dioxus internationalization powered by [Project Fluent](https://github.com/projectfluent/fluent-rs).

## Support

- **Dioxus v0.8** 🧬
- Dioxus [platform support](https://dioxuslabs.com/learn/0.8/guides/platforms/)

## Example:

```ftl
# en-US.ftl

hello = Hello, {$name}!
```

```ftl
# es-ES.ftl

hello = ¡Hola, {$name}!
```

```rs
fn app() -> Element {
    let i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./en-US.ftl")))
            .with_locale((langid!("es-ES"), include_str!("./es-ES.ftl")))
    });

    rsx! {
        label { { t!("hello", name: "World") } }
    }
}
```

### Config examples

Load resources from the filesystem at launch on native targets:

```rs
use_init_i18n(|| {
    I18nConfig::new(langid!("en-US"))
        .with_locale((langid!("en-US"), include_str!("./en-US.ftl")))
        .with_locale((langid!("es-ES"), PathBuf::from("./es-ES.ftl")))
});
```

Share one embedded resource between locale identifiers:

```rs
use_init_i18n(|| {
    I18nConfig::new(langid!("en-US"))
        .with_locale((langid!("en-US"), include_str!("./en-US.ftl")))
        .with_locale((langid!("en"), include_str!("./en-US.ftl")))
});
```

Construct locales explicitly:

```rs
use_init_i18n(|| {
    I18nConfig::new(langid!("en-US"))
        .with_locale(Locale::new_static(
            langid!("en-US"),
            include_str!("./en-US.ftl"),
        ))
        .with_locale(Locale::new_dynamic(
            langid!("es-ES"),
            PathBuf::from("./es-ES.ftl"),
        ))
});
```

Provide an application-configured Fluent bundle:

```rs
use_init_i18n_from_bundle(|| {
    create_bundle(langid!("en-US"), include_str!("./en-US.ftl"))
});
```

## More examples

- Configuration:
  - [Auto locales](./examples/config-auto-locales.rs)
  - [Dynamic (`PathBuf`)](./examples/config-dynamic-pathbuf.rs)
  - [Static (`include_str!`)](./examples/config-static-includestr.rs)
  - [External Fluent bundle](./examples/config-external-bundle.rs)
- Applications:
  - [Dioxus desktop](./examples/dioxus-desktop.rs)
  - Fluent grammar: [application](./examples/fluent-grammar.rs) and
    [FTL file](./examples/data/fluent/en.ftl)

## Development

```bash
# Checks clean compile against `#[cfg(not(target_arch = "wasm32"))]`
cargo build --target wasm32-unknown-unknown

cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`dioxus-i18n` was originally part of the [Dioxus SDK](https://github.com/DioxusLabs/sdk).

[MIT License](./LICENSE.md)
