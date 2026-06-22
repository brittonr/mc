use bevy_ecs::prelude::*;

type Children = bevy_hierarchy::Children;
type Ident<T> = valence_server::Ident<T>;
type ItemStack = valence_server::ItemStack;
type Text = valence_server::Text;

/// Advancement's id. May not be updated.
#[derive(Component, derive_more::Deref)]
pub struct Advancement(pub(crate) Ident<std::borrow::Cow<'static, str>>);

impl Advancement {
    pub fn new(ident: Ident<std::borrow::Cow<'static, str>>) -> Advancement {
        Self(ident)
    }

    pub fn get(&self) -> &Ident<std::borrow::Cow<'static, str>> {
        &self.0
    }
}

#[derive(Clone, Copy)]
pub enum AdvancementFrameType {
    Task,
    Challenge,
    Goal,
}

/// Advancement display. Optional component.
#[derive(Component)]
pub struct AdvancementDisplay {
    pub title: Text,
    pub description: Text,
    pub icon: ItemStack,
    pub frame_type: AdvancementFrameType,
    pub show_toast: bool,
    pub hidden: bool,
    pub background_texture: Option<Ident<std::borrow::Cow<'static, str>>>,
    pub x_coord: f32,
    pub y_coord: f32,
}

impl AdvancementDisplay {
    pub(crate) fn flags(&self) -> i32 {
        let mut flags = 0;
        flags |= i32::from(self.background_texture.is_some());
        flags |= i32::from(self.show_toast) << 1;
        flags |= i32::from(self.hidden) << 2;
        flags
    }
}

/// Criteria's identifier. May not be updated.
#[derive(Component, derive_more::Deref)]
pub struct AdvancementCriteria(pub(crate) Ident<std::borrow::Cow<'static, str>>);

impl AdvancementCriteria {
    pub fn new(ident: Ident<std::borrow::Cow<'static, str>>) -> Self {
        Self(ident)
    }

    pub fn get(&self) -> &Ident<std::borrow::Cow<'static, str>> {
        &self.0
    }
}

/// Requirements for advancement to be completed.
/// All columns should be completed, column is completed when any of criteria in
/// this column is completed.
#[derive(Component, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct AdvancementRequirements(pub Vec<Vec<Entity>>);

#[derive(Component, Default)]
pub struct AdvancementCachedBytes(pub(crate) Vec<u8>);

#[derive(Default, Debug, PartialEq)]
pub enum ForceTabUpdate {
    #[default]
    None,
    First,
    /// Should contain only root advancement otherwise the first will be chosen.
    Spec(Entity),
}

#[derive(Component, Debug)]
pub struct AdvancementClientUpdate {
    /// Which advancement's descriptions send to client.
    pub new_advancements: Vec<Entity>,
    /// Which advancements remove from client.
    pub remove_advancements: Vec<Entity>,
    /// Criteria progress update.
    /// If None then criteria is not done otherwise it is done.
    pub progress: Vec<(Entity, Option<i64>)>,
    /// Forces client to open a tab.
    pub force_tab_update: ForceTabUpdate,
    /// Defines if other advancements should be removed.
    /// Also with this flag, client will not show a toast for advancements,
    /// which are completed. When the packet is sent, turns to false.
    pub reset: bool,
}

impl Default for AdvancementClientUpdate {
    fn default() -> Self {
        Self {
            new_advancements: vec![],
            remove_advancements: vec![],
            progress: vec![],
            force_tab_update: ForceTabUpdate::None,
            reset: true,
        }
    }
}

impl AdvancementClientUpdate {
    pub(crate) fn walk_advancements(
        root: Entity,
        children_query: &Query<&Children>,
        advancement_check_query: &Query<(), With<Advancement>>,
        func: &mut impl FnMut(Entity),
    ) {
        func(root);
        if let Ok(children) = children_query.get(root) {
            for child in children {
                let child = *child;
                if advancement_check_query.get(child).is_ok() {
                    Self::walk_advancements(child, children_query, advancement_check_query, func);
                }
            }
        }
    }

    /// Sends all advancements from the root.
    pub fn send_advancements(
        &mut self,
        root: Entity,
        children_query: &Query<&Children>,
        advancement_check_query: &Query<(), With<Advancement>>,
    ) {
        Self::walk_advancements(
            root,
            children_query,
            advancement_check_query,
            &mut |entity| self.new_advancements.push(entity),
        );
    }

    /// Removes all advancements from the root.
    pub fn remove_advancements(
        &mut self,
        root: Entity,
        children_query: &Query<&Children>,
        advancement_check_query: &Query<(), With<Advancement>>,
    ) {
        Self::walk_advancements(
            root,
            children_query,
            advancement_check_query,
            &mut |entity| self.remove_advancements.push(entity),
        );
    }

    /// Marks criteria as done.
    pub fn criteria_done(&mut self, criteria: Entity) {
        self.progress.push((criteria, Some(current_time_millis())))
    }

    /// Marks criteria as undone.
    pub fn criteria_undone(&mut self, criteria: Entity) {
        self.progress.push((criteria, None))
    }
}

// API: this convenience method records criterion completion at wall-clock time.
#[allow(unknown_lints)]
#[allow(ambient_clock)]
fn current_time_millis() -> i64 {
    let Ok(duration) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) else {
        return 0;
    };
    match i64::try_from(duration.as_millis()) {
        Ok(millis) => millis,
        Err(_) => i64::MAX,
    }
}
