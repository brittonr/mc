impl serde::Serialize for super::Color {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&format!("{self}"), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for super::Color {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(JsonStringVisitor)
    }
}

struct JsonStringVisitor;

impl serde::de::Visitor<'_> for JsonStringVisitor {
    type Value = super::Color;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a hex color (#rrggbb), a normal color or 'reset'")
    }

    fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
        super::Color::try_from(s).map_err(|_| E::custom("invalid color"))
    }
}
