# Changelog

## [0.7.0-alpha.1] 2026-10-15

### Added

- Re-exported the complete i18n API from the crate root. Existing
  `dioxus_i18n::prelude` imports remain supported.

### Changed

- Updated the Dioxus dependency floor to `0.8.0-alpha.1`, allowing later compatible Dioxus 0.8
  releases.
- Updated examples and documentation to use the simpler crate-root imports.

### Breaking

- Dropped support for Dioxus 0.7. Consumers must upgrade to Dioxus 0.8.

## [0.6.0] 2026-10-12

### Added

- Added `use_init_i18n_from_bundle` and `try_use_init_i18n_from_bundle` for applications that
  construct and configure their own Fluent bundles.
- Added the `I18nBundle` controller and `i18n_bundle` context accessor for translating with and
  reactively replacing an externally configured bundle.
- Added an external-bundle example that also demonstrates testing Fluent configuration without a
  Dioxus runtime.

### Changed

- Managed and external providers now share the translation context used by `t!`, `te!`, and
  `tid!`.
- Managed language and fallback changes build the replacement bundle before publishing the new
  selection, preserving the previous valid state if construction fails.
- Compact `message.attribute` references are parsed without allocating and malformed references
  are rejected explicitly.
- Updated documentation and examples for the current Dioxus configuration and platform APIs.

### Breaking

- Removed public `I18n::new` and `I18n::try_new` construction. Initialize a managed provider with
  `use_init_i18n` or `try_use_init_i18n` so the controller and translation context are installed
  together.
- Removed public `I18n::decompose_identifier`; compact message-reference parsing is now an
  implementation detail.

Before:

```rust
let i18n = I18n::try_new(language, fallback, resources, locales)?;
use_context_provider(|| i18n);
```

After:

```rust
let i18n = try_use_init_i18n(|| {
    I18nConfig::new(language)
        .with_locale((langid!("en-US"), include_str!("./en-US.ftl")))
})?;
```

### Removed

- Removed the unmaintained Freya example and development dependency. The library remains
  renderer-independent across supported Dioxus platforms.

## [0.4.3]

- [Issue #19](https://github.com/dioxus-community/dioxus-i18n/issues/19) Enable use of "message-id.attribute-id"
  syntax in the `t!`, `te!`, `tid!` macros in order to extract attribute definition, e.g. `t!("mycomponent.placeholder")`
  in:
  ```
  mycomponent = Component Name
     .placeholder = Some placeholder
     .aria-text = Some aria text
  ```

- Added examples for all fluent grammar constructs and configuration variants.

## [0.4.2] 2025-02-08

### Fixed

- [Issue #15](https://github.com/dioxus-community/dioxus-i18n/issues/15) Recent change to t! macro unnecessarily breaks v0.3 code.

### Amended

- t! macro amended to use unwrap_or_else rather than panic!.

- Error messages made consistant across all macros.

## [0.4.1] 2025-02-02

### Added

- New methods (`I18nConfig::with_auto_locales`) to determine supported locales from deep search for translation files.

- New methods returning `Result<_, Error>` rather than `panic!`, such that:
  | __`panic!` version__                    | __`Result<_, Error>` vesion__            |
  |-----------------------------------------|------------------------------------------|
  | `LocaleResource::to_resource_string`    | `LocaleResource::try_to_resource_string` |
  | `I18n::translate`                       | `I18n::try_translate`                    |
  | `I18n::translate_with_args`             | `I18n::try_translate_with_args`          |
  | `I18n::set_fallback_language`           | `I18n::try_set_fallback_language`        |
  | `I18n::set_language`                    | `I18n::try_set_language`                 |
  | `use_init_i18n`                         | `try_use_init_i18n`                      |
  | `I18nConfig::with_auto_locales`         | `I18nConfig::try_with_auto_locales`      |

- New `te!` macro which acts like `t!` but returns `Error`.

- New `tid!` macro which acts like `t!` but returns the message-id.

### Change

- t! macro amended to use `try_translate` and `try_translate_with_args`, but will perform `.expect("..")`
  and therefore panic! on error. This retains backwards compatibility for this macro.

- Use of `set_fallback_language` / `try_set_fallback_language` without a corresponding locale
  translation is treated as an error.

## [0.4.0] 2025-01-25

### Added

- Code:
  - Doc comments
  - Module tests for `cargo test`

- Amended `I18nConfig::with_locale` so that the `Locale` dynamic or static
  constructors no longer have to be _explicitly_ given.
  They can be determined implicitly from `(LanguageIdentifier, &str)`  or
  `(LanguageIdentifer, PathBuf)`.

- Enabled shared 'LocaleResource's, where two dialect can use the same translation file.
  For example ["en", "en-GB"] share "en-GB.ftl".

### Changed

- The translations used are determined when `I18n::set_language` or
  `I18n::set_fallback_language` is called, and not each time a message is translated.

- __Fallback handling has changed__. It no longer just uses _fallback_language_ when the message
  id is missing from the current _locale_. It performs a graceful fallback from
  _<language>-<region>_ to _<language>_ before using the actual _fallback_ (in fact it
  falls back along the _<language>-<optionalScript>-<optionalRegion>-<optionalVariants>_
  hiearchy).

  __Note:__ this is a breaking change which may impact the selected translation.

- `LocaleResource::to_string` renamed to `LocaleResource::to_resource_string`

## [0.3.0] 2024-12-10

- [Dioxus 0.6](https://dioxuslabs.com/) support

## [0.2.4] 2024-09-11

- Hide new_dynamic in WASM
- New t!() macro

## [0.2.3] 2024-09-04

- Support dynamic loading of locales

## [0.2.2] 2024-09-02

- Enable macros instead of serde in unic-langid

## [0.2.1] 2024-09-02

- Export unic_langid and fluent
- Use absolute path to import fluent in the translate macro
- Updated freya example

## [0.2.0] 2024-09-01

- Now based in the [Fluent Project](https://github.com/projectfluent/fluent-rs)

## [0.1.0] 2024-08-31

- Initial release
