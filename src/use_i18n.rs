use std::path::PathBuf;

use dioxus_lib::prelude::*;
use fluent::{FluentArgs, FluentBundle, FluentResource};

use unic_langid::LanguageIdentifier;

pub struct Locale {
    id: LanguageIdentifier,
    resource: LocaleResource,
}

impl Locale {
    pub fn new_static(id: LanguageIdentifier, str: &'static str) -> Self {
        Self {
            id,
            resource: LocaleResource::Static(str),
        }
    }

    pub fn new_dynamic(id: LanguageIdentifier, path: impl Into<PathBuf>) -> Self {
        Self {
            id,
            resource: LocaleResource::Path(path.into()),
        }
    }
}

pub enum LocaleResource {
    Static(&'static str),
    Path(PathBuf),
}

impl LocaleResource {
    pub fn to_string(&self) -> String {
        match self {
            Self::Static(str) => str.to_string(),
            Self::Path(path) => {
                std::fs::read_to_string(path).expect("Failed to read locale resource")
            }
        }
    }
}

pub struct I18nConfig {
    id: LanguageIdentifier,
    fallback: Option<LanguageIdentifier>,
    locales: Vec<Locale>,
}

impl I18nConfig {
    /// Create an i18n config with the selected [LanguageIdentifier].
    pub fn new(id: LanguageIdentifier) -> Self {
        Self {
            id,
            fallback: None,
            locales: Vec::new(),
        }
    }

    /// Set a fallback [LanguageIdentifier].
    pub fn with_fallback(mut self, fallback: LanguageIdentifier) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// Add [Locale].
    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locales.push(locale);
        self
    }
}

/// Initialize an i18n provider.
pub fn use_init_i18n(init: impl FnOnce() -> I18nConfig) -> I18n {
    use_context_provider(move || {
        let I18nConfig {
            id,
            fallback,
            locales,
        } = init();

        let bundles = locales
            .into_iter()
            .map(|Locale { id, resource }| {
                let mut bundle = FluentBundle::new(vec![id]);
                let resource = FluentResource::try_new(resource.to_string())
                    .expect("Failed to ceate Resource.");
                bundle
                    .add_resource(resource)
                    .expect("Failed to add resource.");
                bundle
            })
            .collect::<Vec<FluentBundle<FluentResource>>>();

        I18n {
            selected_language: Signal::new(id),
            fallback_language: Signal::new(fallback),
            bundles: Signal::new(bundles),
        }
    })
}

#[derive(Clone, Copy)]
pub struct I18n {
    selected_language: Signal<LanguageIdentifier>,
    fallback_language: Signal<Option<LanguageIdentifier>>,
    bundles: Signal<Vec<FluentBundle<FluentResource>>>,
}

impl I18n {
    pub fn translate_with_args(&self, msg: &str, args: Option<&FluentArgs>) -> String {
        let bundles = self.bundles.read();

        let Some(bundle) = bundles
            .iter()
            .find(|bundle| bundle.locales.contains(&*self.selected_language.read()))
            .or_else(|| {
                if let Some(fcb) = &*self.fallback_language.read() {
                    bundles.iter().find(|bundle| bundle.locales.contains(fcb))
                } else {
                    None
                }
            })
        else {
            return msg.to_owned();
        };

        let message = bundle.get_message(msg).expect("Failed to get message.");
        let pattern = message.value().expect("Failed to get the message pattern.");
        let mut errors = vec![];

        bundle
            .format_pattern(pattern, args, &mut errors)
            .to_string()
    }

    pub fn translate(&self, msg: &str) -> String {
        self.translate_with_args(msg, None)
    }

    /// Get the selected language.
    pub fn language(&mut self) -> LanguageIdentifier {
        self.selected_language.read().clone()
    }

    /// Get the fallback language.
    pub fn fallback_language(&mut self) -> Option<LanguageIdentifier> {
        self.fallback_language.read().clone()
    }

    /// Update the selected language.
    pub fn set_language(&mut self, id: LanguageIdentifier) {
        *self.selected_language.write() = id;
    }

    /// Update the fallback language.
    pub fn set_fallback_language(&mut self, id: LanguageIdentifier) {
        *self.fallback_language.write() = Some(id);
    }
}

pub fn use_i18n() -> I18n {
    consume_context()
}
