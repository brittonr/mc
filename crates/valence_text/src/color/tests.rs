use super::*;

#[test]
fn colors() {
    assert_eq!(
        Color::try_from("#aBcDeF"),
        Ok(RgbColor::new(0xab, 0xcd, 0xef).into())
    );
    assert_eq!(
        Color::try_from("#fFfFfF"),
        Ok(RgbColor::new(255, 255, 255).into())
    );
    assert_eq!(Color::try_from("#000000"), Ok(NamedColor::Black.into()));
    assert_eq!(Color::try_from("red"), Ok(NamedColor::Red.into()));
    assert_eq!(Color::try_from("blue"), Ok(NamedColor::Blue.into()));
    assert!(Color::try_from("#ffTf00").is_err());
    assert!(Color::try_from("#ffš00").is_err());
    assert!(Color::try_from("#00000000").is_err());
    assert!(Color::try_from("#").is_err());
}
