use std::borrow::Cow;

pub use bitcoinsuite_error_derive::ErrorMeta;
pub use eyre::{bail, Report, Result, WrapErr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    InvalidUserInput,
    InvalidClientInput,
    Warning,
    Bug,
    Critical,
}

pub trait ErrorMeta {
    fn severity(&self) -> ErrorSeverity;
    fn error_code(&self) -> Cow<'static, str>;
    fn tags(&self) -> Cow<'static, [(Cow<'static, str>, Cow<'static, str>)]>;
}

pub trait ErrorFmt {
    fn fmt_err(&self) -> String;
}

impl ErrorFmt for eyre::Report {
    fn fmt_err(&self) -> String {
        format!("{:#}", self)
    }
}
