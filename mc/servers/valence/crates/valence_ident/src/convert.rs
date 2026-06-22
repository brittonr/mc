use std::borrow::Borrow;
use std::str::FromStr;

impl<S: AsRef<str>> AsRef<str> for super::Ident<S> {
    fn as_ref(&self) -> &str {
        self.string.as_ref()
    }
}

impl<S> AsRef<S> for super::Ident<S> {
    fn as_ref(&self) -> &S {
        &self.string
    }
}

impl<S: Borrow<str>> Borrow<str> for super::Ident<S> {
    fn borrow(&self) -> &str {
        self.string.borrow()
    }
}

impl From<super::Ident<&str>> for String {
    fn from(value: super::Ident<&str>) -> Self {
        value.as_str().to_owned()
    }
}

impl From<super::Ident<String>> for String {
    fn from(value: super::Ident<String>) -> Self {
        value.into_inner()
    }
}

impl<'a> From<super::Ident<super::Cow<'a, str>>> for super::Cow<'a, str> {
    fn from(value: super::Ident<super::Cow<'a, str>>) -> Self {
        value.into_inner()
    }
}

impl<'a> From<super::Ident<super::Cow<'a, str>>> for super::Ident<String> {
    fn from(value: super::Ident<super::Cow<'a, str>>) -> Self {
        Self {
            string: value.string.into(),
        }
    }
}

impl From<super::Ident<String>> for super::Ident<super::Cow<'_, str>> {
    fn from(value: super::Ident<String>) -> Self {
        Self {
            string: value.string.into(),
        }
    }
}

impl<'a> From<super::Ident<&'a str>> for super::Ident<super::Cow<'a, str>> {
    fn from(value: super::Ident<&'a str>) -> Self {
        super::Ident {
            string: value.string.into(),
        }
    }
}

impl<'a> From<super::Ident<&'a str>> for super::Ident<String> {
    fn from(value: super::Ident<&'a str>) -> Self {
        super::Ident {
            string: value.string.into(),
        }
    }
}

impl FromStr for super::Ident<String> {
    type Err = super::IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(super::Ident::new(s)?.into())
    }
}

impl FromStr for super::Ident<super::Cow<'static, str>> {
    type Err = super::IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Ident::<String>::try_from(s).map(From::from)
    }
}

impl<'a> TryFrom<&'a str> for super::Ident<String> {
    type Error = super::IdentError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(super::Ident::new(value)?.into())
    }
}

impl TryFrom<String> for super::Ident<String> {
    type Error = super::IdentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(super::Ident::new(value)?.into())
    }
}

impl<'a> TryFrom<super::Cow<'a, str>> for super::Ident<String> {
    type Error = super::IdentError;

    fn try_from(value: super::Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(super::Ident::new(value)?.into())
    }
}

impl<'a> TryFrom<&'a str> for super::Ident<super::Cow<'a, str>> {
    type Error = super::IdentError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for super::Ident<super::Cow<'_, str>> {
    type Error = super::IdentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a> TryFrom<super::Cow<'a, str>> for super::Ident<super::Cow<'a, str>> {
    type Error = super::IdentError;

    fn try_from(value: super::Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<S: std::fmt::Debug> std::fmt::Debug for super::Ident<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string.fmt(f)
    }
}

impl<S: std::fmt::Display> std::fmt::Display for super::Ident<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string.fmt(f)
    }
}

impl<S, T> PartialEq<super::Ident<T>> for super::Ident<S>
where
    S: PartialEq<T>,
{
    fn eq(&self, other: &super::Ident<T>) -> bool {
        self.string == other.string
    }
}

impl<S, T> PartialOrd<super::Ident<T>> for super::Ident<S>
where
    S: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &super::Ident<T>) -> Option<std::cmp::Ordering> {
        self.string.partial_cmp(&other.string)
    }
}
