impl std::str::FromStr for super::Text {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(super::Text::default())
        } else {
            serde_json::from_str(s)
        }
    }
}

impl<'de> serde::Deserialize<'de> for super::Text {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ComponentVisitor)
    }
}

struct ComponentVisitor;

impl<'de> serde::de::Visitor<'de> for ComponentVisitor {
    type Value = super::Text;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a text component data type")
    }

    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(super::Text::text(v.to_string()))
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(super::Text::text(v.to_string()))
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(super::Text::text(v.to_string()))
    }

    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(super::Text::text(v.to_string()))
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(super::Text::text(v.to_owned()))
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(super::Text::text(v))
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let Some(mut res) = seq.next_element()? else {
            return Ok(super::Text::default());
        };

        while let Some(child) = seq.next_element::<super::Text>()? {
            res += child;
        }

        Ok(res)
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        Ok(super::Text(Box::new(
            <super::TextInner as serde::Deserialize>::deserialize(
                serde::de::value::MapAccessDeserializer::new(map),
            )?,
        )))
    }
}
