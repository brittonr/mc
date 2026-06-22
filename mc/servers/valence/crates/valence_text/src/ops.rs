// API: preserve field-style access to TextInner for existing Valence callers.
#[allow(deref_polymorphism)]
impl std::ops::Deref for super::Text {
    type Target = super::TextInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for super::Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: super::IntoText<'static>> std::ops::Add<T> for super::Text {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        super::IntoText::add_child(self, rhs)
    }
}

impl<T: super::IntoText<'static>> std::ops::AddAssign<T> for super::Text {
    fn add_assign(&mut self, rhs: T) {
        self.extra.push(rhs.into_text());
    }
}

impl From<super::Text> for super::Cow<'_, super::Text> {
    fn from(value: super::Text) -> Self {
        Self::Owned(value)
    }
}

impl<'a> From<&'a super::Text> for super::Cow<'a, super::Text> {
    fn from(value: &'a super::Text) -> Self {
        Self::Borrowed(value)
    }
}

impl From<super::Text> for String {
    fn from(value: super::Text) -> Self {
        format!("{value}")
    }
}

impl From<super::Text> for valence_nbt::Value {
    fn from(value: super::Text) -> Self {
        valence_nbt::Value::String(value.into())
    }
}

impl std::fmt::Debug for super::Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for super::Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = if f.alternate() {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
        .map_err(|_| std::fmt::Error)?;

        f.write_str(&string)
    }
}

impl Default for super::TextContent {
    fn default() -> Self {
        Self::Text { text: "".into() }
    }
}
