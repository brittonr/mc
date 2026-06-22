use bevy_ecs::prelude::*;
use valence_server::text::IntoText;

type LayerId = valence_server::entity::EntityLayerId;
type DisplayPosition = valence_server::protocol::packets::play::ScoreboardPosition;
type RenderType = valence_server::protocol::packets::play::ObjectiveRenderType;
type DisplayText = valence_server::Text;

/// A string that identifies an objective. There is one scoreboard per
/// objective.It's generally not safe to modify this after it's been created.
/// Limited to 16 characters.
///
/// Directly analogous to an Objective's Name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, derive_more::Deref)]
pub struct Objective(pub(crate) String);

impl Objective {
    pub fn new<N: Into<String>>(name: N) -> Self {
        let name = name.into();
        debug_assert!(
            name.len() <= 16,
            "Objective name {} is too long ({} > 16)",
            name,
            name.len()
        );
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

/// Optional display name for an objective. If not present, the objective's name
/// is used.
#[derive(Debug, Clone, PartialEq, Component, derive_more::Deref, derive_more::DerefMut)]
pub struct ObjectiveDisplay(pub DisplayText);

/// A mapping of keys to their scores.
#[derive(Debug, Clone, Component, Default)]
pub struct ObjectiveScores(pub(crate) std::collections::HashMap<String, i32>);

impl ObjectiveScores {
    pub fn new() -> Self {
        Self(std::collections::HashMap::new())
    }

    pub fn with_map<M: Into<std::collections::HashMap<String, i32>>>(map: M) -> Self {
        Self(map.into())
    }

    pub fn get(&self, key: &str) -> Option<&i32> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut i32> {
        self.0.get_mut(key)
    }

    pub fn insert<K: Into<String>>(&mut self, key: K, value: i32) -> Option<i32> {
        self.0.insert(key.into(), value)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Component)]
pub struct OldObjectiveScores(pub(crate) std::collections::HashMap<String, i32>);

impl OldObjectiveScores {
    pub fn diff<'a>(&'a self, scores: &'a ObjectiveScores) -> Vec<&'a str> {
        let max_diff_count = self.0.len().saturating_add(scores.0.len());
        let mut diff = Vec::with_capacity(max_diff_count);

        for (key, value) in &self.0 {
            if scores.0.get(key) != Some(value) {
                diff.push(key.as_str());
            }
        }

        let new_keys = scores
            .0
            .keys()
            .filter(|key| !self.0.contains_key(key.as_str()))
            .map(|key| key.as_str());

        let removed_keys = self
            .0
            .keys()
            .filter(|key| !scores.0.contains_key(key.as_str()))
            .map(|key| key.as_str());

        diff.extend(new_keys);
        diff.extend(removed_keys);
        diff
    }
}

#[derive(Bundle)]
pub struct ObjectiveBundle {
    pub name: Objective,
    pub display: ObjectiveDisplay,
    pub render_type: RenderType,
    pub scores: ObjectiveScores,
    pub old_scores: OldObjectiveScores,
    pub position: DisplayPosition,
    pub layer: LayerId,
}

impl Default for ObjectiveBundle {
    fn default() -> Self {
        Self {
            name: Objective::new(""),
            display: ObjectiveDisplay("".into_text()),
            render_type: RenderType::Integer,
            scores: ObjectiveScores::new(),
            old_scores: OldObjectiveScores(std::collections::HashMap::new()),
            position: DisplayPosition::Sidebar,
            layer: valence_server::entity::EntityLayerId(bevy_ecs::prelude::Entity::PLACEHOLDER),
        }
    }
}
