use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
pub use valence_nbt::compound;

type Map<K, V> = std::collections::BTreeMap<K, V>;
type Compound = valence_nbt::Compound;
type Ident<T> = valence_ident::Ident<T>;
type List = valence_nbt::List;
type Value = valence_nbt::Value;

pub(super) fn build(app: &mut App) {
    app.init_resource::<RegistryCodec>()
        .add_systems(PostUpdate, cache_registry_codec.in_set(crate::RegistrySet));
}

/// Contains the registry codec sent to all players while joining. This contains
/// information for biomes and dimensions among other things.
///
/// Generally, end users should not manipulate the registry codec directly. Use
/// one of the other registry resources instead.
#[derive(Resource, Debug)]
pub struct RegistryCodec {
    pub registries: Map<Ident<String>, Vec<RegistryValue>>,
    // TODO: store this in binary form?
    cached_codec: Compound,
}

#[derive(Clone, Debug)]
pub struct RegistryValue {
    pub name: Ident<String>,
    pub element: Compound,
}

impl RegistryCodec {
    pub fn cached_codec(&self) -> &Compound {
        &self.cached_codec
    }

    // Compatibility API: direct registry indexing has historically panicked when
    // absent.
    #[allow(unknown_lints, no_panic)]
    pub fn registry(&self, registry_key: Ident<&str>) -> &Vec<RegistryValue> {
        self.registries
            .get(registry_key.as_str())
            .unwrap_or_else(|| panic!("missing registry for {registry_key}"))
    }

    // Compatibility API: direct registry indexing has historically panicked when
    // absent.
    #[allow(unknown_lints, no_panic)]
    pub fn registry_mut(&mut self, registry_key: Ident<&str>) -> &mut Vec<RegistryValue> {
        self.registries
            .get_mut(registry_key.as_str())
            .unwrap_or_else(|| panic!("missing registry for {registry_key}"))
    }

    fn empty() -> Self {
        Self {
            registries: Map::new(),
            cached_codec: Compound::new(),
        }
    }
}

impl Default for RegistryCodec {
    fn default() -> Self {
        let codec = include_bytes!("../extracted/registry_codec.dat");
        let Ok((compound, _)) = valence_nbt::from_binary(&mut codec.as_slice()) else {
            tracing::error!("failed to decode vanilla registry codec");
            return Self::empty();
        };

        let registry_count_bound = compound.len();
        let mut registries = Map::new();

        for (k, v) in compound {
            let Ok(reg_name) = Ident::new(k) else {
                tracing::error!("invalid registry name in vanilla registry codec");
                continue;
            };
            let reg_name: Ident<String> = reg_name.into();
            let mut reg_values = Vec::new();

            let Value::Compound(mut outer) = v else {
                tracing::error!("registry {reg_name} is not a compound");
                continue;
            };

            let values = match outer.remove("value") {
                Some(Value::List(List::Compound(values))) => values,
                Some(Value::List(List::End)) => continue,
                _ => {
                    tracing::error!("missing \"value\" compound in {reg_name}");
                    continue;
                }
            };

            reg_values.reserve_exact(values.len());
            for mut value in values {
                let Some(Value::String(name)) = value.remove("name") else {
                    tracing::error!("missing \"name\" string in value for {reg_name}");
                    continue;
                };

                let name = match Ident::new(name) {
                    Ok(n) => n.into(),
                    Err(e) => {
                        tracing::error!("invalid registry value name \"{}\"", e.0);
                        continue;
                    }
                };

                let Some(Value::Compound(element)) = value.remove("element") else {
                    tracing::error!("missing \"element\" compound in value for {reg_name}");
                    continue;
                };

                reg_values.push(RegistryValue { name, element });
            }

            if registries.len() >= registry_count_bound {
                break;
            }
            registries.insert(reg_name, reg_values);
        }

        Self {
            registries,
            // Cache will be created later.
            cached_codec: Compound::new(),
        }
    }
}

fn cache_registry_codec(codec: ResMut<RegistryCodec>) {
    if codec.is_changed() {
        let codec = codec.into_inner();

        codec.cached_codec.clear();

        for (reg_name, reg) in &codec.registries {
            let mut value = Vec::with_capacity(reg.len());

            for (id, v) in reg.iter().enumerate() {
                let Ok(id) = i32::try_from(id) else {
                    continue;
                };
                value.push(compound! {
                    "id" => id,
                    "name" => v.name.as_str(),
                    "element" => v.element.clone(),
                });
            }

            let registry = compound! {
                "type" => reg_name.as_str(),
                "value" => List::Compound(value),
            };

            codec.cached_codec.insert(reg_name.as_str(), registry);
        }
    }
}
