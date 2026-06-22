impl<'a> super::IntoText<'a> for crate::Text {
    fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
        crate::Cow::Owned(self)
    }
}

impl<'a> super::IntoText<'a> for &'a crate::Text {
    fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
        crate::Cow::Borrowed(self)
    }
}

impl<'a> From<&'a crate::Text> for crate::Text {
    fn from(value: &'a crate::Text) -> Self {
        value.clone()
    }
}

impl<'a> super::IntoText<'a> for crate::Cow<'a, crate::Text> {
    fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
        self
    }
}

impl<'a> From<crate::Cow<'a, crate::Text>> for crate::Text {
    fn from(value: crate::Cow<'a, crate::Text>) -> Self {
        value.into_owned()
    }
}

impl<'a> super::IntoText<'a> for &'a crate::Cow<'_, crate::Text> {
    fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
        self.clone()
    }
}

impl<'a, 'b> From<&'a crate::Cow<'b, crate::Text>> for crate::Text {
    fn from(value: &'a crate::Cow<'b, crate::Text>) -> Self {
        value.clone().into_owned()
    }
}

impl<'a> super::IntoText<'a> for String {
    fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
        crate::Cow::Owned(crate::Text::text(self))
    }
}

impl From<String> for crate::Text {
    fn from(value: String) -> Self {
        super::IntoText::into_text(value)
    }
}

impl<'b> super::IntoText<'b> for &String {
    fn into_cow_text(self) -> crate::Cow<'b, crate::Text> {
        crate::Cow::Owned(crate::Text::text(self.clone()))
    }
}

impl<'a> From<&'a String> for crate::Text {
    fn from(value: &'a String) -> Self {
        super::IntoText::into_text(value)
    }
}

impl<'a> super::IntoText<'a> for crate::Cow<'static, str> {
    fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
        crate::Cow::Owned(crate::Text::text(self))
    }
}

impl From<crate::Cow<'static, str>> for crate::Text {
    fn from(value: crate::Cow<'static, str>) -> Self {
        super::IntoText::into_text(value)
    }
}

impl super::IntoText<'static> for &crate::Cow<'static, str> {
    fn into_cow_text(self) -> crate::Cow<'static, crate::Text> {
        crate::Cow::Owned(crate::Text::text(self.clone()))
    }
}

impl<'a> From<&'a crate::Cow<'static, str>> for crate::Text {
    fn from(value: &'a crate::Cow<'static, str>) -> Self {
        super::IntoText::into_text(value)
    }
}

impl<'a> super::IntoText<'a> for &'static str {
    fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
        crate::Cow::Owned(crate::Text::text(self))
    }
}

impl From<&'static str> for crate::Text {
    fn from(value: &'static str) -> Self {
        super::IntoText::into_text(value)
    }
}

impl<'a, 'b, T: super::IntoText<'a>, const N: usize> super::IntoText<'b> for [T; N] {
    fn into_cow_text(self) -> crate::Cow<'b, crate::Text> {
        let mut txt = crate::Text::text("");

        for child in self {
            txt = super::IntoText::add_child(txt, child.into_cow_text().into_owned());
        }

        crate::Cow::Owned(txt)
    }
}

impl<'a, 'c, T: super::IntoText<'a> + Clone, const N: usize> super::IntoText<'c> for &[T; N] {
    fn into_cow_text(self) -> crate::Cow<'c, crate::Text> {
        let mut txt = crate::Text::text("");

        for child in self {
            txt = super::IntoText::add_child(txt, child.clone().into_cow_text().into_owned());
        }

        crate::Cow::Owned(txt)
    }
}

macro_rules! impl_primitives {
    ($($primitive:ty),+) => {
        $(
            impl<'a> super::IntoText<'a> for $primitive {
                fn into_cow_text(self) -> crate::Cow<'a, crate::Text> {
                    crate::Cow::Owned(crate::Text::text(self.to_string()))
                }
            }
        )+
    };
}

impl_primitives! {char, bool, f32, f64, isize, usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128}
