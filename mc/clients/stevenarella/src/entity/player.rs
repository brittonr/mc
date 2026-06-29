mod collision;
mod construction;
mod model_state;
mod movement;
mod rendering;
mod systems;

pub use construction::{create_local, create_remote};
pub use model_state::PlayerModel;
pub use movement::PlayerMovement;

use crate::ecs;

pub fn add_systems(m: &mut ecs::Manager) {
    systems::add_systems(m);
}
