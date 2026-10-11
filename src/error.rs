use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("invalid message id: '{0}'")]
    InvalidMessageId(String),

    #[error("message id not found for key: '{0}'")]
    MessageIdNotFound(String),

    #[error("attribute id not found for key: '{0}'")]
    AttributeIdNotFound(String),

    #[error("message pattern not found for key: '{0}'")]
    MessagePatternNotFound(String),

    #[error("fluent errors during lookup:\n{0}")]
    FluentErrorsDetected(String),

    #[error("failed to read locale resource from path: {0}")]
    LocaleResourcePathReadFailed(String),

    #[error("fallback for \"{0}\" must have locale")]
    FallbackMustHaveLocale(String),

    #[error("language id cannot be determined - reason: {0}")]
    InvalidLanguageId(String),

    #[error("invalid path: {0}")]
    InvalidPath(String),
}

#[derive(Clone, Debug, Error)]
pub enum ExternalBundleError {
    #[error("externally configured Fluent bundle must contain at least one locale")]
    MissingLocale,

    #[error("Fluent bundle can only be replaced for an externally configured i18n provider")]
    NotExternalBundleProvider,
}
