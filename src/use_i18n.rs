use dioxus_lib::prelude::*;
use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

/// `Locale` is a "place-holder" around what will eventually be a `fluent::FluentBundle`
#[cfg_attr(test, derive(Debug, PartialEq))]
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_dynamic(id: LanguageIdentifier, path: impl Into<PathBuf>) -> Self {
        Self {
            id,
            resource: LocaleResource::Path(path.into()),
        }
    }
}

impl<T> From<(LanguageIdentifier, T)> for Locale
where
    T: Into<LocaleResource>,
{
    fn from((id, resource): (LanguageIdentifier, T)) -> Self {
        let resource = resource.into();
        Self { id, resource }
    }
}

/// A `LocaleResource` can be static text, or a filesystem file (not supported in WASM).
#[derive(Debug, PartialEq)]
pub enum LocaleResource {
    Static(&'static str),
    #[cfg(not(target_arch = "wasm32"))]
    Path(PathBuf),
}

impl LocaleResource {
    pub fn to_resource_string(&self) -> String {
        match self {
            Self::Static(str) => str.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Path(path) => {
                std::fs::read_to_string(path).expect("Failed to read locale resource")
            }
        }
    }
}

impl From<&'static str> for LocaleResource {
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<PathBuf> for LocaleResource {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

/// The configuration for `I18n`.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct I18nConfig {
    /// The initial language, can be later changed with [`I18n::set_language`]
    id: LanguageIdentifier,

    /// The final fallback language if no other locales are found for `id`.
    /// A `Locale` must exist in `locales' if `fallback` is defined.
    fallback: Option<LanguageIdentifier>,

    /// The locale_resources added to the configuration.
    locale_resources: Vec<LocaleResource>,

    /// The locales added to the configuration.
    locales: HashMap<LanguageIdentifier, usize>,
}

impl I18nConfig {
    /// Create an i18n config with the selected [LanguageIdentifier].
    pub fn new(id: LanguageIdentifier) -> Self {
        Self {
            id,
            fallback: None,
            locale_resources: Vec::new(),
            locales: HashMap::new(),
        }
    }

    /// Set a fallback [LanguageIdentifier].
    pub fn with_fallback(mut self, fallback: LanguageIdentifier) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// Add [Locale].
    /// It is possible to share locales resources. If this locale's resource
    /// matches a previously added one, then this locale will use the existing one.
    /// This is primarily for the static locale_resources to avoid string duplication.
    pub fn with_locale<T>(mut self, locale: T) -> Self
    where
        T: Into<Locale>,
    {
        let locale = locale.into();
        let locale_resources_len = self.locale_resources.len();

        let index = self
            .locale_resources
            .iter()
            .position(|r| *r == locale.resource)
            .unwrap_or(locale_resources_len);

        if index == locale_resources_len {
            self.locale_resources.push(locale.resource)
        };

        self.locales.insert(locale.id, index);
        self
    }
}

/// Initialize an i18n provider.
pub fn use_init_i18n(init: impl FnOnce() -> I18nConfig) -> I18n {
    use_context_provider(move || {
        // Coverage false -ve: See https://github.com/xd009642/tarpaulin/issues/1675
        let I18nConfig {
            id,
            fallback,
            locale_resources,
            locales,
        } = init();

        I18n::new(id, fallback, locale_resources, locales)
    })
}

#[derive(Clone, Copy)]
pub struct I18n {
    selected_language: Signal<LanguageIdentifier>,
    fallback_language: Signal<Option<LanguageIdentifier>>,
    locale_resources: Signal<Vec<LocaleResource>>,
    locales: Signal<HashMap<LanguageIdentifier, usize>>,
    active_bundle: Signal<FluentBundle<FluentResource>>,
}

impl I18n {
    pub fn new(
        selected_language: LanguageIdentifier,
        fallback_language: Option<LanguageIdentifier>,
        locale_resources: Vec<LocaleResource>,
        locales: HashMap<LanguageIdentifier, usize>,
    ) -> Self {
        let bundle = create_bundle(
            &selected_language,
            &fallback_language,
            &locale_resources,
            &locales,
        );
        Self {
            selected_language: Signal::new(selected_language),
            fallback_language: Signal::new(fallback_language),
            locale_resources: Signal::new(locale_resources),
            locales: Signal::new(locales),
            active_bundle: Signal::new(bundle),
        }
    }

    pub fn translate_with_args(&self, msg: &str, args: Option<&FluentArgs>) -> String {
        let bundle = self.active_bundle.read();
        let message = bundle
            .get_message(msg)
            .unwrap_or_else(|| panic!("Failed to get message: {}.", msg));
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
    pub fn language(&self) -> LanguageIdentifier {
        self.selected_language.read().clone()
    }

    /// Get the fallback language.
    pub fn fallback_language(&self) -> Option<LanguageIdentifier> {
        self.fallback_language.read().clone()
    }

    /// Update the selected language.
    pub fn set_language(&mut self, id: LanguageIdentifier) {
        *self.selected_language.write() = id;
        self.update_active_bundle();
    }

    /// Update the fallback language.
    pub fn set_fallback_language(&mut self, id: LanguageIdentifier) {
        *self.fallback_language.write() = Some(id);
        self.update_active_bundle();
    }

    fn update_active_bundle(&mut self) {
        let bundle = create_bundle(
            &self.selected_language.peek(),
            &self.fallback_language.peek(),
            &self.locale_resources.peek(),
            &self.locales.peek(),
        );
        self.active_bundle.set(bundle);
    }
}

fn create_bundle(
    selected_language: &LanguageIdentifier,
    fallback_language: &Option<LanguageIdentifier>,
    locale_resources: &[LocaleResource],
    locales: &HashMap<LanguageIdentifier, usize>,
) -> FluentBundle<FluentResource> {
    let add_resource = move |bundle: &mut FluentBundle<FluentResource>,
                             langid: &LanguageIdentifier,
                             locale_resources: &[LocaleResource]| {
        if let Some(&i) = locales.get(langid) {
            let resource = &locale_resources[i];
            let resource = FluentResource::try_new(resource.to_resource_string())
                .expect("Failed to ceate Resource.");
            bundle.add_resource_overriding(resource);
        }
    };

    let mut bundle = FluentBundle::new(vec![selected_language.clone()]);
    if let Some(fallback_language) = fallback_language {
        add_resource(&mut bundle, fallback_language, locale_resources);
    }

    let (language, script, region, variants) = selected_language.clone().into_parts();
    let variants_lang = LanguageIdentifier::from_parts(language, script, region, &variants);
    let region_lang = LanguageIdentifier::from_parts(language, script, region, &[]);
    let script_lang = LanguageIdentifier::from_parts(language, script, None, &[]);
    let language_lang = LanguageIdentifier::from_parts(language, None, None, &[]);

    add_resource(&mut bundle, &language_lang, locale_resources);
    add_resource(&mut bundle, &script_lang, locale_resources);
    add_resource(&mut bundle, &region_lang, locale_resources);
    add_resource(&mut bundle, &variants_lang, locale_resources);

    bundle
}

pub fn i18n() -> I18n {
    consume_context()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use unic_langid::langid;

    #[test]
    fn can_add_locale_to_config_explicit_locale() {
        const LANG_A: LanguageIdentifier = langid!("la-LA");
        const LANG_B: LanguageIdentifier = langid!("la-LB");
        const LANG_C: LanguageIdentifier = langid!("la-LC");

        let config = I18nConfig::new(LANG_A)
            .with_locale(Locale::new_static(LANG_B, "lang = lang_b"))
            .with_locale(Locale::new_dynamic(LANG_C, PathBuf::new()));

        assert_eq!(
            config,
            I18nConfig {
                id: LANG_A,
                fallback: None,
                locale_resources: vec![
                    LocaleResource::Static("lang = lang_b"),
                    LocaleResource::Path(PathBuf::new()),
                ],
                locales: HashMap::from([(LANG_B, 0), (LANG_C, 1)]),
            }
        );
    }

    #[test]
    fn can_add_locale_to_config_implicit_locale() {
        const LANG_A: LanguageIdentifier = langid!("la-LA");
        const LANG_B: LanguageIdentifier = langid!("la-LB");
        const LANG_C: LanguageIdentifier = langid!("la-LC");

        let config = I18nConfig::new(LANG_A)
            .with_locale((LANG_B, "lang = lang_b"))
            .with_locale((LANG_C, PathBuf::new()));

        assert_eq!(
            config,
            I18nConfig {
                id: LANG_A,
                fallback: None,
                locale_resources: vec![
                    LocaleResource::Static("lang = lang_b"),
                    LocaleResource::Path(PathBuf::new())
                ],
                locales: HashMap::from([(LANG_B, 0), (LANG_C, 1)]),
            }
        );
    }

    #[test]
    fn can_add_locale_string_to_config() {
        const LANG_A: LanguageIdentifier = langid!("la-LA");
        const LANG_B: LanguageIdentifier = langid!("la-LB");

        let config = I18nConfig::new(LANG_A).with_locale((LANG_B, "lang = lang_b"));

        assert_eq!(
            config,
            I18nConfig {
                id: LANG_A,
                fallback: None,
                locale_resources: vec![LocaleResource::Static("lang = lang_b")],
                locales: HashMap::from([(LANG_B, 0)]),
            }
        );
    }

    #[test]
    fn can_add_shared_locale_string_to_config() {
        const LANG_A: LanguageIdentifier = langid!("la-LA");
        const LANG_B: LanguageIdentifier = langid!("la-LB");
        const LANG_C: LanguageIdentifier = langid!("la-LC");

        let shared_string = "lang = a language";
        let config = I18nConfig::new(LANG_A)
            .with_locale((LANG_B, shared_string))
            .with_locale((LANG_C, shared_string));

        assert_eq!(
            config,
            I18nConfig {
                id: LANG_A,
                fallback: None,
                locale_resources: vec![LocaleResource::Static(shared_string)],
                locales: HashMap::from([(LANG_B, 0), (LANG_C, 0)]),
            }
        );
    }

    #[test]
    fn can_add_locale_pathbuf_to_config() {
        const LANG_A: LanguageIdentifier = langid!("la-LA");
        const LANG_C: LanguageIdentifier = langid!("la-LC");

        let config = I18nConfig::new(LANG_A)
            .with_locale((LANG_C, PathBuf::from("./test/data/fallback/la.ftl")));

        assert_eq!(
            config,
            I18nConfig {
                id: LANG_A,
                fallback: None,
                locale_resources: vec![LocaleResource::Path(PathBuf::from(
                    "./test/data/fallback/la.ftl"
                ))],
                locales: HashMap::from([(LANG_C, 0)]),
            }
        );
    }

    #[test]
    fn can_add_shared_locale_pathbuf_to_config() {
        const LANG_A: LanguageIdentifier = langid!("la-LA");
        const LANG_B: LanguageIdentifier = langid!("la-LB");
        const LANG_C: LanguageIdentifier = langid!("la-LC");

        let shared_pathbuf = PathBuf::from("./test/data/fallback/la.ftl");

        let config = I18nConfig::new(LANG_A)
            .with_locale((LANG_B, shared_pathbuf.clone()))
            .with_locale((LANG_C, shared_pathbuf.clone()));

        assert_eq!(
            config,
            I18nConfig {
                id: LANG_A,
                fallback: None,
                locale_resources: vec![LocaleResource::Path(shared_pathbuf)],
                locales: HashMap::from([(LANG_B, 0), (LANG_C, 0)]),
            }
        );
    }
}
