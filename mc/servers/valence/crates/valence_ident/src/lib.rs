#![doc = include_str!("../README.md")]

type Cow<'a, T> = std::borrow::Cow<'a, T>;

const DEFAULT_NAMESPACE: &str = "minecraft";
const NAMESPACE_SEPARATOR: char = ':';

/// Used internally by the `ident` macro. Not public API.
#[doc(hidden)]
pub use valence_ident_macros::parse_ident_str;

/// Creates a new [`Ident`] at compile time from a string literal. A compile
/// error is raised if the string is not a valid resource identifier.
///
/// The type of the expression returned by this macro is `Ident<&'static str>`.
/// The expression is usable in a `const` context.
///
/// # Examples
///
/// ```
/// # use valence_ident::{ident, Ident};
/// let my_ident: Ident<&'static str> = ident!("apple");
///
/// println!("{my_ident}");
/// ```
#[macro_export]
macro_rules! ident {
    ($string:literal) => {
        $crate::Ident::<&'static str>::new_unchecked($crate::parse_ident_str!($string))
    };
}

/// A wrapper around a string type `S` which guarantees the wrapped string is a
/// valid resource identifier.
///
/// A resource identifier is a string divided into a "namespace" part and a
/// "path" part. For instance `minecraft:apple` and `valence:frobnicator` are
/// both valid identifiers. A string must match the regex
/// `^([a-z0-9_.-]+:)?[a-z0-9_.-\/]+$` to be successfully parsed.
///
/// While parsing, if the namespace part is left off (the part before and
/// including the colon) then "minecraft:" is inserted at the beginning of the
/// string.
#[derive(Copy, Clone, Eq, Ord, Hash)]
pub struct Ident<S> {
    string: S,
}

/// The error type created when an [`Ident`] cannot be parsed from a
/// string. Contains the string that failed to parse.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, thiserror::Error)]
#[error("invalid resource identifier \"{0}\"")]
pub struct IdentError(pub String);

impl<'a> Ident<Cow<'a, str>> {
    pub fn new<S: Into<Cow<'a, str>>>(string: S) -> Result<Self, IdentError> {
        parse(string.into())
    }
}

impl<S> Ident<S> {
    /// Used internally by the `ident` macro. Not public API.
    #[doc(hidden)]
    pub const fn new_unchecked(string: S) -> Self {
        Self { string }
    }

    pub fn as_str(&self) -> &str
    where
        S: AsRef<str>,
    {
        self.string.as_ref()
    }

    pub fn as_str_ident(&self) -> Ident<&str>
    where
        S: AsRef<str>,
    {
        Ident {
            string: self.as_str(),
        }
    }

    pub fn to_string_ident(&self) -> Ident<String>
    where
        S: AsRef<str>,
    {
        Ident {
            string: self.as_str().to_owned(),
        }
    }

    pub fn into_inner(self) -> S {
        self.string
    }

    /// Returns the namespace part of this resource identifier (the part before
    /// the colon).
    pub fn namespace(&self) -> &str
    where
        S: AsRef<str>,
    {
        self.namespace_and_path().0
    }

    /// Returns the path part of this resource identifier (the part after the
    /// colon).
    pub fn path(&self) -> &str
    where
        S: AsRef<str>,
    {
        self.namespace_and_path().1
    }

    pub fn namespace_and_path(&self) -> (&str, &str)
    where
        S: AsRef<str>,
    {
        match self.as_str().split_once(NAMESPACE_SEPARATOR) {
            Some(namespace_and_path) => namespace_and_path,
            None => (DEFAULT_NAMESPACE, self.as_str()),
        }
    }
}

impl Ident<Cow<'_, str>> {
    pub fn borrowed(&self) -> Ident<Cow<'_, str>> {
        Ident::new_unchecked(Cow::Borrowed(self.as_str()))
    }
}

fn is_valid_namespace_char(value: char) -> bool {
    matches!(value, 'a'..='z' | '0'..='9' | '_' | '.' | '-')
}

fn is_valid_path_char(value: char) -> bool {
    matches!(value, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/')
}

fn is_valid_namespace(value: &str) -> bool {
    !value.is_empty() && value.chars().all(is_valid_namespace_char)
}

fn is_valid_path(value: &str) -> bool {
    !value.is_empty() && value.chars().all(is_valid_path_char)
}

fn with_default_namespace(string: Cow<str>) -> Ident<Cow<str>> {
    Ident {
        string: format!("{DEFAULT_NAMESPACE}{NAMESPACE_SEPARATOR}{string}").into(),
    }
}

fn parse(string: Cow<str>) -> Result<Ident<Cow<str>>, IdentError> {
    match string.split_once(NAMESPACE_SEPARATOR) {
        Some((namespace, path)) if is_valid_namespace(namespace) && is_valid_path(path) => {
            Ok(Ident { string })
        }
        Some(_) => Err(IdentError(string.into())),
        None if is_valid_path(&string) => Ok(with_default_namespace(string)),
        None => Err(IdentError(string.into())),
    }
}

mod codec;
mod convert;

#[cfg(test)]
mod tests;
