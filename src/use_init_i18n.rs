use dioxus_lib::prelude::*;
use unic_langid::LanguageIdentifier;

use crate::use_i18n::UseI18;

use super::use_i18n::Language;

/// Initialize the i18n context with the given configuration.
pub fn use_init_i18n(
    selected_language: LanguageIdentifier,
    fallback_language: LanguageIdentifier,
    languages: impl FnOnce() -> Vec<Language>,
) -> UseI18 {
    use_context_provider(move || UseI18 {
        selected_language: Signal::new(selected_language),
        fallback_language: Signal::new(fallback_language),
        languages: Signal::new(languages()),
    })
}

/// Just like `use_init_i18n` but if `selected_language` or `fallback_language` change,
///  it will update the internal signal.
pub fn use_init_i18n_keep_sync(
    selected_language: LanguageIdentifier,
    fallback_language: LanguageIdentifier,
    languages: impl FnOnce() -> Vec<Language>,
) -> UseI18 {
    let mut selected_language_signal = use_signal(|| selected_language.clone());
    let mut fallback_language_signal = use_signal(|| fallback_language.clone());

    // Keep signals on sync in case the values change
    use_effect(use_reactive(&selected_language, move |selected_language| {
        selected_language_signal.set(selected_language);
    }));
    use_effect(use_reactive(&fallback_language, move |fallback_language| {
        fallback_language_signal.set(fallback_language);
    }));

    use_context_provider(move || UseI18 {
        selected_language: selected_language_signal,
        fallback_language: fallback_language_signal,
        languages: Signal::new(languages()),
    })
}
