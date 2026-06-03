use super::*;

#[test]
// Clippy: this test deliberately borrows values to verify borrowed/owned behavior.
#[allow(clippy::needless_borrows_for_generic_args)]
fn intotext_trait() {
    fn is_borrowed<'a>(value: impl IntoText<'a>) -> bool {
        matches!(value.into_cow_text(), crate::Cow::Borrowed(..))
    }

    assert!(is_borrowed(&"this should be borrowed".into_text()));
    assert!(is_borrowed(&"this should be borrowed too".bold()));
    assert!(!is_borrowed("this should be owned?".bold()));
    assert!(!is_borrowed("this should be owned"));
    assert!(!is_borrowed(465));
    assert!(!is_borrowed(false));
}
