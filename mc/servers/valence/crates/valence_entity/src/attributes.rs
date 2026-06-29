use std::collections::HashMap;

use bevy_ecs::prelude::*;
use indexmap::IndexMap;
use uuid::Uuid;
pub use valence_generated::attributes::{EntityAttribute, EntityAttributeOperation};
use valence_protocol::packets::play::entity_attributes_s2c::*;
use valence_protocol::Ident;

fn compute_attribute_value_core(
    base_value: f64,
    min_value: f64,
    max_value: f64,
    add_modifiers: impl IntoIterator<Item = f64>,
    multiply_base_modifiers: impl IntoIterator<Item = f64>,
    multiply_total_modifiers: impl IntoIterator<Item = f64>,
) -> f64 {
    let mut value = base_value;

    for modifier in add_modifiers {
        value += modifier;
    }

    let base_plus_add = value;

    for modifier in multiply_base_modifiers {
        value += base_plus_add * modifier;
    }

    for modifier in multiply_total_modifiers {
        value += value * modifier;
    }

    value.clamp(min_value, max_value)
}

/// An instance of an Entity Attribute.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct EntityAttributeInstance {
    /// The attribute.
    attribute: EntityAttribute,
    /// The base value of the attribute.
    base_value: f64,
    /// The add modifiers of the attribute.
    add_modifiers: IndexMap<Uuid, f64>,
    /// The multiply base modifiers of the attribute.
    multiply_base_modifiers: IndexMap<Uuid, f64>,
    /// The multiply total modifiers of the attribute.
    multiply_total_modifiers: IndexMap<Uuid, f64>,
}

impl EntityAttributeInstance {
    /// Creates a new instance of an Entity Attribute.
    pub fn new(attribute: EntityAttribute) -> Self {
        Self {
            attribute,
            base_value: attribute.default_value(),
            add_modifiers: IndexMap::new(),
            multiply_base_modifiers: IndexMap::new(),
            multiply_total_modifiers: IndexMap::new(),
        }
    }

    /// Creates a new instance of an Entity Attribute with a value.
    pub fn new_with_value(attribute: EntityAttribute, base_value: f64) -> Self {
        Self {
            attribute,
            base_value,
            add_modifiers: IndexMap::new(),
            multiply_base_modifiers: IndexMap::new(),
            multiply_total_modifiers: IndexMap::new(),
        }
    }

    /// Gets the attribute.
    pub fn attribute(&self) -> EntityAttribute {
        self.attribute
    }

    /// Gets the base value of the attribute.
    pub fn base_value(&self) -> f64 {
        self.base_value
    }

    /// Gets the computed value of the attribute.
    pub fn compute_value(&self) -> f64 {
        compute_attribute_value_core(
            self.base_value,
            self.attribute.min_value(),
            self.attribute.max_value(),
            self.add_modifiers.values().copied(),
            self.multiply_base_modifiers.values().copied(),
            self.multiply_total_modifiers.values().copied(),
        )
    }

    /// Sets an add modifier.
    ///
    /// If the modifier already exists, it will be overwritten.
    ///
    /// Returns a mutable reference to self.
    pub fn with_add_modifier(&mut self, uuid: Uuid, modifier: f64) -> &mut Self {
        self.add_modifiers.insert(uuid, modifier);
        self
    }

    /// Sets a multiply base modifier.
    ///
    /// If the modifier already exists, it will be overwritten.
    ///
    /// Returns a mutable reference to self.
    pub fn with_multiply_base_modifier(&mut self, uuid: Uuid, modifier: f64) -> &mut Self {
        self.multiply_base_modifiers.insert(uuid, modifier);
        self
    }

    /// Sets a multiply total modifier.
    ///
    /// If the modifier already exists, it will be overwritten.
    ///
    /// Returns a mutable reference to self.
    pub fn with_multiply_total_modifier(&mut self, uuid: Uuid, modifier: f64) -> &mut Self {
        self.multiply_total_modifiers.insert(uuid, modifier);
        self
    }

    /// Sets a value modifier based on the operation.
    ///
    /// If the modifier already exists, it will be overwritten.
    ///
    /// Returns a mutable reference to self.
    pub fn with_modifier(
        &mut self,
        uuid: Uuid,
        modifier: f64,
        operation: EntityAttributeOperation,
    ) -> &mut Self {
        match operation {
            EntityAttributeOperation::Add => self.with_add_modifier(uuid, modifier),
            EntityAttributeOperation::MultiplyBase => {
                self.with_multiply_base_modifier(uuid, modifier)
            }
            EntityAttributeOperation::MultiplyTotal => {
                self.with_multiply_total_modifier(uuid, modifier)
            }
        }
    }

    /// Removes a modifier.
    pub fn remove_modifier(&mut self, uuid: Uuid) {
        self.add_modifiers.swap_remove(&uuid);
        self.multiply_base_modifiers.swap_remove(&uuid);
        self.multiply_total_modifiers.swap_remove(&uuid);
    }

    /// Clears all modifiers.
    pub fn clear_modifiers(&mut self) {
        self.add_modifiers.clear();
        self.multiply_base_modifiers.clear();
        self.multiply_total_modifiers.clear();
    }

    /// Checks if a modifier exists.
    pub fn has_modifier(&self, uuid: Uuid) -> bool {
        self.add_modifiers.contains_key(&uuid)
            || self.multiply_base_modifiers.contains_key(&uuid)
            || self.multiply_total_modifiers.contains_key(&uuid)
    }

    /// Converts to a `TrackedEntityProperty` for use in the
    /// `EntityAttributesS2c` packet.
    pub(crate) fn to_property(&self) -> TrackedEntityProperty {
        TrackedEntityProperty {
            key: self.attribute.name().into(),
            value: self.base_value(),
            modifiers: self
                .add_modifiers
                .iter()
                .map(|(&uuid, &amount)| TrackedAttributeModifier {
                    uuid,
                    amount,
                    operation: 0,
                })
                .chain(self.multiply_base_modifiers.iter().map(|(&uuid, &amount)| {
                    TrackedAttributeModifier {
                        uuid,
                        amount,
                        operation: 1,
                    }
                }))
                .chain(
                    self.multiply_total_modifiers
                        .iter()
                        .map(|(&uuid, &amount)| TrackedAttributeModifier {
                            uuid,
                            amount,
                            operation: 2,
                        }),
                )
                .collect(),
        }
    }
}

/// The attributes of a Living Entity.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct EntityAttributes {
    attributes: HashMap<EntityAttribute, EntityAttributeInstance>,
    recently_changed: Vec<EntityAttribute>,
}

impl EntityAttributes {
    /// Gets and clears the recently changed attributes.
    pub(crate) fn take_recently_changed(&mut self) -> Vec<EntityAttribute> {
        std::mem::take(&mut self.recently_changed)
    }

    /// Marks an attribute as recently changed.
    pub(crate) fn mark_recently_changed(&mut self, attribute: EntityAttribute) {
        if attribute.tracked() && !self.recently_changed.contains(&attribute) {
            self.recently_changed.push(attribute);
        }
    }
}

impl EntityAttributes {
    /// Creates a new instance of `EntityAttributes`.
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            recently_changed: Vec::new(),
        }
    }

    /// Gets the instance of an attribute.
    pub fn get(&self, attribute: EntityAttribute) -> Option<&EntityAttributeInstance> {
        self.attributes.get(&attribute)
    }

    /// Gets the base value of an attribute.
    ///
    /// Returns [`None`] if the attribute does not exist.
    pub fn get_base_value(&self, attribute: EntityAttribute) -> Option<f64> {
        self.get(attribute).map(|instance| instance.base_value())
    }

    /// Gets the computed value of an attribute.
    ///
    /// Returns [`None`] if the attribute does not exist.
    pub fn get_compute_value(&self, attribute: EntityAttribute) -> Option<f64> {
        self.get(attribute).map(|instance| instance.compute_value())
    }

    /// Checks if an attribute exists.
    pub fn has_attribute(&self, attribute: EntityAttribute) -> bool {
        self.attributes.contains_key(&attribute)
    }

    /// Creates an attribute if it does not exist.
    pub fn create_attribute(&mut self, attribute: EntityAttribute) {
        self.mark_recently_changed(attribute);
        self.attributes
            .entry(attribute)
            .or_insert_with(|| EntityAttributeInstance::new(attribute));
    }

    /// Creates an attribute if it does not exist and sets its base value.
    ///
    /// Returns self.
    ///
    /// ## Note
    ///
    /// Only to be used in builder-like patterns.
    pub(crate) fn with_attribute_and_value(
        mut self,
        attribute: EntityAttribute,
        base_value: f64,
    ) -> Self {
        self.attributes
            .entry(attribute)
            .or_insert_with(|| EntityAttributeInstance::new_with_value(attribute, base_value))
            .base_value = base_value;
        self
    }

    /// Sets the base value of an attribute.
    pub fn set_base_value(&mut self, attribute: EntityAttribute, value: f64) {
        self.mark_recently_changed(attribute);
        self.attributes
            .entry(attribute)
            .or_insert_with(|| EntityAttributeInstance::new(attribute))
            .base_value = value;
    }

    /// Sets an add modifier of an attribute.
    pub fn set_add_modifier(&mut self, attribute: EntityAttribute, uuid: Uuid, modifier: f64) {
        self.mark_recently_changed(attribute);
        self.attributes
            .entry(attribute)
            .or_insert_with(|| EntityAttributeInstance::new(attribute))
            .with_add_modifier(uuid, modifier);
    }

    /// Sets a multiply base modifier of an attribute.
    pub fn set_multiply_base_modifier(
        &mut self,
        attribute: EntityAttribute,
        uuid: Uuid,
        modifier: f64,
    ) {
        self.mark_recently_changed(attribute);
        self.attributes
            .entry(attribute)
            .or_insert_with(|| EntityAttributeInstance::new(attribute))
            .with_multiply_base_modifier(uuid, modifier);
    }

    /// Sets a multiply total modifier of an attribute.
    pub fn set_multiply_total_modifier(
        &mut self,
        attribute: EntityAttribute,
        uuid: Uuid,
        modifier: f64,
    ) {
        self.mark_recently_changed(attribute);
        self.attributes
            .entry(attribute)
            .or_insert_with(|| EntityAttributeInstance::new(attribute))
            .with_multiply_total_modifier(uuid, modifier);
    }

    /// Sets a value modifier of an attribute based on the operation.
    pub fn set_modifier(
        &mut self,
        attribute: EntityAttribute,
        uuid: Uuid,
        modifier: f64,
        operation: EntityAttributeOperation,
    ) {
        self.mark_recently_changed(attribute);
        self.attributes
            .entry(attribute)
            .or_insert_with(|| EntityAttributeInstance::new(attribute))
            .with_modifier(uuid, modifier, operation);
    }

    /// Removes a modifier of an attribute.
    pub fn remove_modifier(&mut self, attribute: EntityAttribute, uuid: Uuid) {
        self.mark_recently_changed(attribute);
        if let Some(instance) = self.attributes.get_mut(&attribute) {
            instance.remove_modifier(uuid);
        }
    }

    /// Clears all modifiers of an attribute.
    pub fn clear_modifiers(&mut self, attribute: EntityAttribute) {
        self.mark_recently_changed(attribute);
        if let Some(instance) = self.attributes.get_mut(&attribute) {
            instance.clear_modifiers();
        }
    }

    /// Checks if a modifier exists on an attribute.
    pub fn has_modifier(&self, attribute: EntityAttribute, uuid: Uuid) -> bool {
        self.attributes
            .get(&attribute)
            .is_some_and(|inst| inst.has_modifier(uuid))
    }

    /// **For internal use only.**
    ///
    /// Converts to a [`Vec`] of [`AttributeProperty`]s.
    pub fn to_properties(&self) -> Vec<AttributeProperty<'_>> {
        self.attributes
            .iter()
            .filter(|(_, instance)| instance.attribute().tracked())
            .map(|(_, instance)| instance.to_property().to_property())
            .collect()
    }
}

/// Tracks the attributes of a Living Entity.
#[derive(Component, Clone, Debug, Default)]
pub struct TrackedEntityAttributes {
    /// The attributes that have been modified.
    modified: IndexMap<EntityAttribute, TrackedEntityProperty>,
}

#[derive(Clone, Debug)]
pub(crate) struct TrackedEntityProperty {
    key: String,
    value: f64,
    modifiers: Vec<TrackedAttributeModifier>,
}

#[derive(Clone, Debug)]
pub(crate) struct TrackedAttributeModifier {
    uuid: Uuid,
    amount: f64,
    operation: u8,
}

impl TrackedEntityProperty {
    /// Converts to an [`AttributeProperty`]s.
    fn to_property(&self) -> AttributeProperty<'static> {
        AttributeProperty {
            key: Ident::new(self.key.clone()).unwrap(),
            value: self.value,
            modifiers: self
                .modifiers
                .iter()
                .map(|modifier| AttributeModifier {
                    uuid: modifier.uuid,
                    amount: modifier.amount,
                    operation: modifier.operation,
                })
                .collect(),
        }
    }
}

impl TrackedEntityAttributes {
    /// Creates a new instance of [`TrackedEntityAttributes`].
    pub fn new() -> Self {
        Self {
            modified: IndexMap::new(),
        }
    }

    /// Marks an attribute as modified.
    pub fn mark_modified(&mut self, attributes: &EntityAttributes, attribute: EntityAttribute) {
        if let Some(instance) = attributes.get(attribute) {
            self.modified.insert(attribute, instance.to_property());
        }
    }

    /// Returns the properties turned into a [`Vec`] of [`AttributeProperty`]s.
    pub fn get_properties(&self) -> Vec<AttributeProperty<'static>> {
        self.modified
            .iter()
            .map(|(_, property)| property.to_property())
            .collect()
    }

    /// Clears the modified attributes.
    pub fn clear(&mut self) {
        self.modified.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADD_MODIFIER_UUID: u128 = 0x0000_0000_0000_0000_0000_0000_0000_0001;
    const FIRST_MULTIPLY_BASE_MODIFIER_UUID: u128 = 0x0000_0000_0000_0000_0000_0000_0000_0002;
    const SECOND_MULTIPLY_BASE_MODIFIER_UUID: u128 = 0x0000_0000_0000_0000_0000_0000_0000_0003;
    const MULTIPLY_TOTAL_MODIFIER_UUID: u128 = 0x0000_0000_0000_0000_0000_0000_0000_0004;
    const BASE_HEALTH: f64 = 20.0;
    const ADD_MODIFIER_AMOUNT: f64 = 10.0;
    const FIRST_MULTIPLY_BASE_AMOUNT: f64 = 0.2;
    const SECOND_MULTIPLY_BASE_AMOUNT: f64 = 0.2;
    const MULTIPLY_TOTAL_AMOUNT: f64 = 0.5;
    const REPLACEMENT_ADD_MODIFIER_AMOUNT: f64 = 12.0;
    const COMPUTED_WITH_ADD_MODIFIER: f64 = 63.0;
    const COMPUTED_AFTER_ADD_REMOVAL: f64 = 42.0;
    const CLAMPED_MAX_HEALTH_VALUE: f64 = 1_024.0;

    #[test]
    fn attribute_core_computes_modifier_chain() {
        let computed = compute_attribute_value_core(
            BASE_HEALTH,
            EntityAttribute::GenericMaxHealth.min_value(),
            EntityAttribute::GenericMaxHealth.max_value(),
            [ADD_MODIFIER_AMOUNT],
            [FIRST_MULTIPLY_BASE_AMOUNT, SECOND_MULTIPLY_BASE_AMOUNT],
            [MULTIPLY_TOTAL_AMOUNT],
        );

        assert_eq!(computed, COMPUTED_WITH_ADD_MODIFIER);
    }

    #[test]
    fn attribute_core_clamps_out_of_range_value() {
        let computed = compute_attribute_value_core(
            CLAMPED_MAX_HEALTH_VALUE,
            EntityAttribute::GenericMaxHealth.min_value(),
            EntityAttribute::GenericMaxHealth.max_value(),
            [],
            [],
            [],
        );

        assert_eq!(computed, EntityAttribute::GenericMaxHealth.max_value());
    }

    #[test]
    fn duplicate_modifier_replaces_previous_value() {
        let add_uuid = Uuid::from_u128(ADD_MODIFIER_UUID);
        let mut attribute =
            EntityAttributeInstance::new_with_value(EntityAttribute::GenericMaxHealth, BASE_HEALTH);

        attribute.with_add_modifier(add_uuid, ADD_MODIFIER_AMOUNT);
        attribute.with_add_modifier(add_uuid, REPLACEMENT_ADD_MODIFIER_AMOUNT);

        assert_eq!(attribute.add_modifiers.len(), 1);
        assert_eq!(
            attribute.add_modifiers.get(&add_uuid),
            Some(&REPLACEMENT_ADD_MODIFIER_AMOUNT)
        );
    }

    #[test]
    fn test_compute_value() {
        let add_uuid = Uuid::from_u128(ADD_MODIFIER_UUID);
        let mut attributes = EntityAttributes::new();
        attributes.set_base_value(EntityAttribute::GenericMaxHealth, BASE_HEALTH);
        attributes.set_add_modifier(
            EntityAttribute::GenericMaxHealth,
            add_uuid,
            ADD_MODIFIER_AMOUNT,
        );
        attributes.set_multiply_base_modifier(
            EntityAttribute::GenericMaxHealth,
            Uuid::from_u128(FIRST_MULTIPLY_BASE_MODIFIER_UUID),
            FIRST_MULTIPLY_BASE_AMOUNT,
        );
        attributes.set_multiply_base_modifier(
            EntityAttribute::GenericMaxHealth,
            Uuid::from_u128(SECOND_MULTIPLY_BASE_MODIFIER_UUID),
            SECOND_MULTIPLY_BASE_AMOUNT,
        );
        attributes.set_multiply_total_modifier(
            EntityAttribute::GenericMaxHealth,
            Uuid::from_u128(MULTIPLY_TOTAL_MODIFIER_UUID),
            MULTIPLY_TOTAL_AMOUNT,
        );

        assert_eq!(
            attributes.get_compute_value(EntityAttribute::GenericMaxHealth),
            Some(COMPUTED_WITH_ADD_MODIFIER)
        );

        attributes.remove_modifier(EntityAttribute::GenericMaxHealth, add_uuid);

        assert_eq!(
            attributes.get_compute_value(EntityAttribute::GenericMaxHealth),
            Some(COMPUTED_AFTER_ADD_REMOVAL)
        );
    }
}
