use super::*;

#[test]
fn check_namespace_and_path() {
    let id = ident!("namespace:path");
    assert_eq!(id.namespace(), "namespace");
    assert_eq!(id.path(), "path");
}

#[test]
fn parse_valid() {
    ident!("minecraft:whatever");
    ident!("_what-ever55_:.whatever/whatever123456789_");
    ident!("valence:frobnicator");
}

#[test]
#[should_panic]
fn parse_invalid_0() {
    Ident::new("").unwrap();
}

#[test]
#[should_panic]
fn parse_invalid_1() {
    Ident::new(":").unwrap();
}

#[test]
#[should_panic]
fn parse_invalid_2() {
    Ident::new("foo:bar:baz").unwrap();
}

#[test]
fn equality() {
    assert_eq!(ident!("minecraft:my.identifier"), ident!("my.identifier"));
}
