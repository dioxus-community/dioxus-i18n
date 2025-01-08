# Changelog

## [0.4.0]

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
