use super::{PlayerModel, PlayerMovement};
use crate::ecs;
use crate::entity::{
    Bounds, Digging, Gravity, Light, MouseButtons, Position, Rotation, TargetPosition,
    TargetRotation, Velocity,
};
use crate::types::Gamemode;
use cgmath::Point3;
use collision::Aabb3;

const LOCAL_LERP_DIVISOR: f64 = 3.0;
const PLAYER_HALF_WIDTH: f64 = 0.3;
const PLAYER_HEIGHT: f64 = 1.8;
const PLAYER_FOOT_Y: f64 = 0.0;
const PLAYER_ORIGIN: f64 = 0.0;

#[derive(Debug, PartialEq)]
pub(super) struct PlayerConstructionFacts {
    pub(super) name: String,
    pub(super) has_head: bool,
    pub(super) has_name_tag: bool,
    pub(super) first_person: bool,
    pub(super) has_target_rotation: bool,
    pub(super) has_gamemode: bool,
    pub(super) has_gravity: bool,
    pub(super) has_player_movement: bool,
    pub(super) has_digging: bool,
    pub(super) has_mouse_buttons: bool,
    pub(super) target_lerp_amount: f64,
}

pub(super) fn local_player_facts() -> PlayerConstructionFacts {
    PlayerConstructionFacts {
        name: String::new(),
        has_head: false,
        has_name_tag: false,
        first_person: true,
        has_target_rotation: false,
        has_gamemode: true,
        has_gravity: true,
        has_player_movement: true,
        has_digging: true,
        has_mouse_buttons: true,
        target_lerp_amount: 1.0 / LOCAL_LERP_DIVISOR,
    }
}

pub(super) fn remote_player_facts(name: &str) -> PlayerConstructionFacts {
    PlayerConstructionFacts {
        name: name.to_owned(),
        has_head: true,
        has_name_tag: true,
        first_person: false,
        has_target_rotation: true,
        has_gamemode: false,
        has_gravity: false,
        has_player_movement: false,
        has_digging: false,
        has_mouse_buttons: false,
        target_lerp_amount: TargetPosition::zero().lerp_amount,
    }
}

pub fn create_local(m: &mut ecs::Manager) -> ecs::Entity {
    let facts = local_player_facts();
    let entity = m.create_entity();
    m.add_component_direct(
        entity,
        Position::new(PLAYER_ORIGIN, PLAYER_ORIGIN, PLAYER_ORIGIN),
    );
    let mut tpos = TargetPosition::new(PLAYER_ORIGIN, PLAYER_ORIGIN, PLAYER_ORIGIN);
    tpos.lerp_amount = facts.target_lerp_amount;
    m.add_component_direct(entity, tpos);
    m.add_component_direct(entity, Rotation::new(PLAYER_ORIGIN, PLAYER_ORIGIN));
    m.add_component_direct(
        entity,
        Velocity::new(PLAYER_ORIGIN, PLAYER_ORIGIN, PLAYER_ORIGIN),
    );
    m.add_component_direct(entity, Gamemode::Survival);
    m.add_component_direct(entity, Gravity::new());
    m.add_component_direct(entity, PlayerMovement::new());
    m.add_component_direct(entity, player_bounds());
    m.add_component_direct(
        entity,
        PlayerModel::new(
            &facts.name,
            facts.has_head,
            facts.has_name_tag,
            facts.first_person,
        ),
    );
    m.add_component_direct(entity, Light::new());
    m.add_component_direct(entity, Digging::new());
    m.add_component_direct(entity, MouseButtons::new());
    entity
}

pub fn create_remote(m: &mut ecs::Manager, name: &str) -> ecs::Entity {
    let facts = remote_player_facts(name);
    let entity = m.create_entity();
    m.add_component_direct(
        entity,
        Position::new(PLAYER_ORIGIN, PLAYER_ORIGIN, PLAYER_ORIGIN),
    );
    m.add_component_direct(
        entity,
        TargetPosition::new(PLAYER_ORIGIN, PLAYER_ORIGIN, PLAYER_ORIGIN),
    );
    m.add_component_direct(entity, Rotation::new(PLAYER_ORIGIN, PLAYER_ORIGIN));
    m.add_component_direct(entity, TargetRotation::new(PLAYER_ORIGIN, PLAYER_ORIGIN));
    m.add_component_direct(
        entity,
        Velocity::new(PLAYER_ORIGIN, PLAYER_ORIGIN, PLAYER_ORIGIN),
    );
    m.add_component_direct(entity, player_bounds());
    m.add_component_direct(
        entity,
        PlayerModel::new(
            &facts.name,
            facts.has_head,
            facts.has_name_tag,
            facts.first_person,
        ),
    );
    m.add_component_direct(entity, Light::new());
    entity
}

fn player_bounds() -> Bounds {
    Bounds::new(Aabb3::new(
        Point3::new(-PLAYER_HALF_WIDTH, PLAYER_FOOT_Y, -PLAYER_HALF_WIDTH),
        Point3::new(PLAYER_HALF_WIDTH, PLAYER_HEIGHT, PLAYER_HALF_WIDTH),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const REMOTE_NAME: &str = "remote-player";

    #[test]
    fn player_construction_core_describes_local_player_facts() {
        assert_eq!(
            local_player_facts(),
            PlayerConstructionFacts {
                name: String::new(),
                has_head: false,
                has_name_tag: false,
                first_person: true,
                has_target_rotation: false,
                has_gamemode: true,
                has_gravity: true,
                has_player_movement: true,
                has_digging: true,
                has_mouse_buttons: true,
                target_lerp_amount: 1.0 / LOCAL_LERP_DIVISOR,
            }
        );
    }

    #[test]
    fn player_construction_core_describes_remote_player_facts() {
        assert_eq!(
            remote_player_facts(REMOTE_NAME),
            PlayerConstructionFacts {
                name: REMOTE_NAME.to_owned(),
                has_head: true,
                has_name_tag: true,
                first_person: false,
                has_target_rotation: true,
                has_gamemode: false,
                has_gravity: false,
                has_player_movement: false,
                has_digging: false,
                has_mouse_buttons: false,
                target_lerp_amount: TargetPosition::zero().lerp_amount,
            }
        );
    }

    #[test]
    fn player_local_shell_attaches_existing_local_components() {
        let mut manager = ecs::Manager::new();
        let player = create_local(&mut manager);

        assert!(manager.get_component_direct::<Position>(player).is_some());
        assert_eq!(
            manager
                .get_component_direct::<TargetPosition>(player)
                .unwrap()
                .lerp_amount,
            1.0 / LOCAL_LERP_DIVISOR
        );
        assert!(manager.get_component_direct::<Rotation>(player).is_some());
        assert!(manager.get_component_direct::<Velocity>(player).is_some());
        assert!(manager.get_component_direct::<Gamemode>(player).is_some());
        assert!(manager.get_component_direct::<Gravity>(player).is_some());
        assert!(manager
            .get_component_direct::<PlayerMovement>(player)
            .is_some());
        assert!(manager.get_component_direct::<Bounds>(player).is_some());
        assert!(manager.get_component_direct::<Light>(player).is_some());
        assert!(manager.get_component_direct::<Digging>(player).is_some());
        assert!(manager
            .get_component_direct::<MouseButtons>(player)
            .is_some());
        let model = manager.get_component_direct::<PlayerModel>(player).unwrap();
        assert_eq!(model.name(), "");
        assert_eq!(model.visibility(), local_player_facts().visibility());
    }

    #[test]
    fn player_remote_shell_attaches_remote_components_without_local_controls() {
        let mut manager = ecs::Manager::new();
        let player = create_remote(&mut manager, REMOTE_NAME);

        assert!(manager.get_component_direct::<Position>(player).is_some());
        assert!(manager
            .get_component_direct::<TargetPosition>(player)
            .is_some());
        assert!(manager.get_component_direct::<Rotation>(player).is_some());
        assert!(manager
            .get_component_direct::<TargetRotation>(player)
            .is_some());
        assert!(manager.get_component_direct::<Velocity>(player).is_some());
        assert!(manager.get_component_direct::<Bounds>(player).is_some());
        assert!(manager.get_component_direct::<Light>(player).is_some());
        assert!(manager.get_component_direct::<Gamemode>(player).is_none());
        assert!(manager.get_component_direct::<Gravity>(player).is_none());
        assert!(manager
            .get_component_direct::<PlayerMovement>(player)
            .is_none());
        assert!(manager.get_component_direct::<Digging>(player).is_none());
        assert!(manager
            .get_component_direct::<MouseButtons>(player)
            .is_none());
        let model = manager.get_component_direct::<PlayerModel>(player).unwrap();
        assert_eq!(model.name(), REMOTE_NAME);
        assert_eq!(
            model.visibility(),
            remote_player_facts(REMOTE_NAME).visibility()
        );
    }

    #[test]
    fn player_empty_entity_has_no_player_components() {
        let mut manager = ecs::Manager::new();
        let entity = manager.create_entity();

        assert!(manager
            .get_component_direct::<PlayerModel>(entity)
            .is_none());
        assert!(manager
            .get_component_direct::<PlayerMovement>(entity)
            .is_none());
    }
}

#[cfg(test)]
impl PlayerConstructionFacts {
    fn visibility(&self) -> super::model_state::PlayerVisibility {
        super::model_state::PlayerVisibility {
            has_head: self.has_head,
            has_name_tag: self.has_name_tag,
            first_person: self.first_person,
        }
    }
}
