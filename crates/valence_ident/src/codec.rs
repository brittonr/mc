impl<T: serde::Serialize> serde::Serialize for super::Ident<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.string.serialize(serializer)
    }
}

impl<'de, S> serde::Deserialize<'de> for super::Ident<S>
where
    S: serde::Deserialize<'de>,
    super::Ident<S>: TryFrom<S, Error = super::IdentError>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        super::Ident::try_from(<S as serde::Deserialize>::deserialize(deserializer)?)
            .map_err(<D::Error as serde::de::Error>::custom)
    }
}
