use super::model_state::{
    animation_frame, visible_model_parts, AnimationInput, PlayerModel, PlayerModelPart,
};
use crate::ecs;
use crate::entity::{GameInfo, Light, Position, Rotation};
use crate::format;
use crate::render;
use crate::render::model::{self, FormatState};
use crate::world;
use cgmath::{Decomposed, Matrix4, Quaternion, Rad, Rotation3, Vector3};

const FIRST_PERSON_YAW_OFFSET: f64 = std::f64::consts::PI / 2.0;
const FIRST_PERSON_POSITION_OFFSET: f64 = 0.25;
const MODEL_SCALE: f32 = 1.0;
const NAME_TAG_Y_OFFSET: f32 = (-24.0 / 16.0) - 0.6;
const HEAD_Y_OFFSET: f32 = -12.0 / 16.0 - 12.0 / 16.0;
const BODY_Y_OFFSET: f32 = -12.0 / 16.0 - 6.0 / 16.0;
const LEG_X_OFFSET: f32 = 2.0 / 16.0;
const LEG_Y_OFFSET: f32 = -12.0 / 16.0;
const ARM_X_OFFSET: f32 = 6.0 / 16.0;
const ARM_Y_OFFSET: f32 = -12.0 / 16.0 - 12.0 / 16.0;
const ARM_WALK_SWING_SCALE: f64 = 0.75;
const ARM_IDLE_SWING_SCALE: f64 = 0.06;
const ARM_SWING_MIDPOINT: f64 = 7.5;
const NAME_TEXT_Y_SCALE: f32 = 0.16;
const NAME_TEXT_X_SCALE: f32 = 0.01;
const NAME_TEXT_CENTER_SCALE: f32 = 0.5;
const NAME_TEXT_SHADOW_OFFSET: f32 = 0.01;
const NAME_TEXT_DEPTH_OFFSET: f32 = 0.05;
const NAME_TEXT_DARK_CHANNEL: u8 = 64;
const NAME_TEXT_LIGHT_CHANNEL: u8 = 255;
const SKIN_TEXTURE_SIZE: f32 = 64.0;
const MODEL_PIXEL: f32 = 16.0;
const PLAYER_MODEL_LIMB_PART_COUNT: usize = 4;

pub(super) struct PlayerRenderer {
    filter: ecs::Filter,
    player_model: ecs::Key<PlayerModel>,
    position: ecs::Key<Position>,
    rotation: ecs::Key<Rotation>,
    game_info: ecs::Key<GameInfo>,
    light: ecs::Key<Light>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RendererShellPlan {
    RebuildAndRender,
    RenderExisting,
    SkipMissingModel,
}

impl PlayerRenderer {
    pub(super) fn new(m: &mut ecs::Manager) -> PlayerRenderer {
        let player_model = m.get_key();
        let position = m.get_key();
        let rotation = m.get_key();
        let light = m.get_key();
        PlayerRenderer {
            filter: ecs::Filter::new()
                .with(player_model)
                .with(position)
                .with(rotation)
                .with(light),
            player_model,
            position,
            rotation,
            game_info: m.get_key(),
            light,
        }
    }
}

pub(super) fn plan_renderer_shell(dirty: bool, model_present: bool) -> RendererShellPlan {
    if dirty {
        RendererShellPlan::RebuildAndRender
    } else if model_present {
        RendererShellPlan::RenderExisting
    } else {
        RendererShellPlan::SkipMissingModel
    }
}

// TODO: Setup culling
impl ecs::System for PlayerRenderer {
    fn filter(&self) -> &ecs::Filter {
        &self.filter
    }

    fn update(
        &mut self,
        m: &mut ecs::Manager,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        use std::f32::consts::PI;
        let world_entity = m.get_world();
        let delta = m.get_component(world_entity, self.game_info).unwrap().delta;
        for e in m.find(&self.filter) {
            let light = *m.get_component(e, self.light).unwrap();
            let render_plan = {
                let model = m.get_component(e, self.player_model).unwrap();
                plan_renderer_shell(model.dirty, model.model.is_some())
            };
            if render_plan == RendererShellPlan::RebuildAndRender {
                self.entity_removed(m, e, world, renderer);
                self.entity_added(m, e, world, renderer);
            }
            let (player_model, position, rotation) = m
                .get_three_components_mut(e, self.player_model, self.position, self.rotation)
                .unwrap();

            let Some(pmodel) = player_model.model else {
                continue;
            };
            let Some(mdl) = renderer.model.get_model(pmodel) else {
                continue;
            };

            mdl.block_light = light.block_light;
            mdl.sky_light = light.sky_light;

            let offset = if player_model.first_person {
                let ox =
                    (rotation.yaw - FIRST_PERSON_YAW_OFFSET).cos() * FIRST_PERSON_POSITION_OFFSET;
                let oz =
                    -(rotation.yaw - FIRST_PERSON_YAW_OFFSET).sin() * FIRST_PERSON_POSITION_OFFSET;
                Vector3::new(
                    position.position.x as f32 - ox as f32,
                    -position.position.y as f32,
                    position.position.z as f32 - oz as f32,
                )
            } else {
                Vector3::new(
                    position.position.x as f32,
                    -position.position.y as f32,
                    position.position.z as f32,
                )
            };
            let offset_matrix = Matrix4::from(Decomposed {
                scale: MODEL_SCALE,
                rot: Quaternion::from_angle_y(Rad(PI + rotation.yaw as f32)),
                disp: offset,
            });

            let visible_parts = visible_model_parts(player_model.visibility());
            if visible_parts[PlayerModelPart::NameTag.as_index()] {
                mdl.matrix[PlayerModelPart::NameTag.as_index()] = Matrix4::from(Decomposed {
                    scale: MODEL_SCALE,
                    rot: Quaternion::from_angle_y(Rad(renderer.camera.yaw as f32)),
                    disp: offset + Vector3::new(0.0, NAME_TAG_Y_OFFSET, 0.0),
                });
            }

            mdl.matrix[PlayerModelPart::Head.as_index()] = offset_matrix
                * Matrix4::from(Decomposed {
                    scale: MODEL_SCALE,
                    rot: Quaternion::from_angle_x(Rad(-rotation.pitch as f32)),
                    disp: Vector3::new(0.0, HEAD_Y_OFFSET, 0.0),
                });
            mdl.matrix[PlayerModelPart::Body.as_index()] = offset_matrix
                * Matrix4::from(Decomposed {
                    scale: MODEL_SCALE,
                    rot: Quaternion::from_angle_x(Rad(0.0)),
                    disp: Vector3::new(0.0, BODY_Y_OFFSET, 0.0),
                });

            let animation = animation_frame(
                player_model.animation_state(),
                AnimationInput {
                    delta,
                    position_moved: position.moved,
                },
            );
            let ang = animation.render.walk_angle;
            let idle_time = animation.render.idle_time;
            let arm_time = animation.render.arm_time;

            mdl.matrix[PlayerModelPart::LegRight.as_index()] = offset_matrix
                * Matrix4::from(Decomposed {
                    scale: MODEL_SCALE,
                    rot: Quaternion::from_angle_x(Rad(ang as f32)),
                    disp: Vector3::new(LEG_X_OFFSET, LEG_Y_OFFSET, 0.0),
                });
            mdl.matrix[PlayerModelPart::LegLeft.as_index()] = offset_matrix
                * Matrix4::from(Decomposed {
                    scale: MODEL_SCALE,
                    rot: Quaternion::from_angle_x(Rad(-ang as f32)),
                    disp: Vector3::new(-LEG_X_OFFSET, LEG_Y_OFFSET, 0.0),
                });

            mdl.matrix[PlayerModelPart::ArmRight.as_index()] = offset_matrix
                * Matrix4::from_translation(Vector3::new(ARM_X_OFFSET, ARM_Y_OFFSET, 0.0))
                * Matrix4::from(Quaternion::from_angle_x(Rad(
                    -(ang * ARM_WALK_SWING_SCALE) as f32
                )))
                * Matrix4::from(Quaternion::from_angle_z(Rad((idle_time.cos()
                    * ARM_IDLE_SWING_SCALE
                    - ARM_IDLE_SWING_SCALE)
                    as f32)))
                * Matrix4::from(Quaternion::from_angle_x(Rad((idle_time.sin()
                    * ARM_IDLE_SWING_SCALE
                    - ((ARM_SWING_MIDPOINT - (arm_time - ARM_SWING_MIDPOINT).abs())
                        / ARM_SWING_MIDPOINT))
                    as f32)));

            mdl.matrix[PlayerModelPart::ArmLeft.as_index()] = offset_matrix
                * Matrix4::from_translation(Vector3::new(-ARM_X_OFFSET, ARM_Y_OFFSET, 0.0))
                * Matrix4::from(Quaternion::from_angle_x(Rad(
                    (ang * ARM_WALK_SWING_SCALE) as f32
                )))
                * Matrix4::from(Quaternion::from_angle_z(Rad(-(idle_time.cos()
                    * ARM_IDLE_SWING_SCALE
                    - ARM_IDLE_SWING_SCALE)
                    as f32)))
                * Matrix4::from(Quaternion::from_angle_x(Rad(-(idle_time.sin()
                    * ARM_IDLE_SWING_SCALE)
                    as f32)));

            player_model.set_animation_state(animation.next_state);
        }
    }

    fn entity_added(
        &mut self,
        m: &mut ecs::Manager,
        e: ecs::Entity,
        _: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let player_model = m.get_component_mut(e, self.player_model).unwrap();

        player_model.dirty = false;

        let skin = if let Some(url) = player_model.skin_url.as_ref() {
            renderer.get_skin(renderer.get_textures_ref(), url)
        } else {
            render::Renderer::get_texture(renderer.get_textures_ref(), "entity/steve")
        };

        macro_rules! srel {
            ($x:expr, $y:expr, $w:expr, $h:expr) => {
                Some(skin.relative(
                    ($x) / SKIN_TEXTURE_SIZE,
                    ($y) / SKIN_TEXTURE_SIZE,
                    ($w) / SKIN_TEXTURE_SIZE,
                    ($h) / SKIN_TEXTURE_SIZE,
                ))
            };
        }

        let visible_parts = visible_model_parts(player_model.visibility());
        let mut head_verts = vec![];
        if visible_parts[PlayerModelPart::Head.as_index()] {
            model::append_box(
                &mut head_verts,
                -4.0 / MODEL_PIXEL,
                0.0,
                -4.0 / MODEL_PIXEL,
                8.0 / MODEL_PIXEL,
                8.0 / MODEL_PIXEL,
                8.0 / MODEL_PIXEL,
                [
                    srel!(16.0, 0.0, 8.0, 8.0), // Down
                    srel!(8.0, 0.0, 8.0, 8.0),  // Up
                    srel!(8.0, 8.0, 8.0, 8.0),  // North
                    srel!(24.0, 8.0, 8.0, 8.0), // South
                    srel!(16.0, 8.0, 8.0, 8.0), // West
                    srel!(0.0, 8.0, 8.0, 8.0),  // East
                ],
            );
            model::append_box(
                &mut head_verts,
                -4.2 / MODEL_PIXEL,
                -0.2 / MODEL_PIXEL,
                -4.2 / MODEL_PIXEL,
                8.4 / MODEL_PIXEL,
                8.4 / MODEL_PIXEL,
                8.4 / MODEL_PIXEL,
                [
                    srel!((16.0 + 32.0), 0.0, 8.0, 8.0), // Down
                    srel!((8.0 + 32.0), 0.0, 8.0, 8.0),  // Up
                    srel!((8.0 + 32.0), 8.0, 8.0, 8.0),  // North
                    srel!((24.0 + 32.0), 8.0, 8.0, 8.0), // South
                    srel!((16.0 + 32.0), 8.0, 8.0, 8.0), // West
                    srel!((0.0 + 32.0), 8.0, 8.0, 8.0),  // East
                ],
            );
        }

        // TODO: Cape
        let mut body_verts = vec![];
        model::append_box(
            &mut body_verts,
            -4.0 / MODEL_PIXEL,
            -6.0 / MODEL_PIXEL,
            -2.0 / MODEL_PIXEL,
            8.0 / MODEL_PIXEL,
            12.0 / MODEL_PIXEL,
            4.0 / MODEL_PIXEL,
            [
                srel!(28.0, 16.0, 8.0, 4.0),  // Down
                srel!(20.0, 16.0, 8.0, 4.0),  // Up
                srel!(20.0, 20.0, 8.0, 12.0), // North
                srel!(32.0, 20.0, 8.0, 12.0), // South
                srel!(16.0, 20.0, 4.0, 12.0), // West
                srel!(28.0, 20.0, 4.0, 12.0), // East
            ],
        );
        model::append_box(
            &mut body_verts,
            -4.2 / MODEL_PIXEL,
            -6.2 / MODEL_PIXEL,
            -2.2 / MODEL_PIXEL,
            8.4 / MODEL_PIXEL,
            12.4 / MODEL_PIXEL,
            4.4 / MODEL_PIXEL,
            [
                srel!(28.0, 16.0 + 16.0, 8.0, 4.0),  // Down
                srel!(20.0, 16.0 + 16.0, 8.0, 4.0),  // Up
                srel!(20.0, 20.0 + 16.0, 8.0, 12.0), // North
                srel!(32.0, 20.0 + 16.0, 8.0, 12.0), // South
                srel!(16.0, 20.0 + 16.0, 4.0, 12.0), // West
                srel!(28.0, 20.0 + 16.0, 4.0, 12.0), // East
            ],
        );

        let mut part_verts = vec![vec![]; PLAYER_MODEL_LIMB_PART_COUNT];

        for (i, offsets) in [
            [16.0, 48.0, 0.0, 48.0],  // Left Leg
            [0.0, 16.0, 0.0, 32.0],   // Right Leg
            [32.0, 48.0, 48.0, 48.0], // Left arm
            [40.0, 16.0, 40.0, 32.0], // Right arm
        ]
        .iter()
        .enumerate()
        {
            let (ox, oy) = (offsets[0], offsets[1]);
            model::append_box(
                &mut part_verts[i],
                -2.0 / MODEL_PIXEL,
                -12.0 / MODEL_PIXEL,
                -2.0 / MODEL_PIXEL,
                4.0 / MODEL_PIXEL,
                12.0 / MODEL_PIXEL,
                4.0 / MODEL_PIXEL,
                [
                    srel!(ox + 8.0, oy + 0.0, 4.0, 4.0),   // Down
                    srel!(ox + 4.0, oy + 0.0, 4.0, 4.0),   // Up
                    srel!(ox + 4.0, oy + 4.0, 4.0, 12.0),  // North
                    srel!(ox + 12.0, oy + 4.0, 4.0, 12.0), // South
                    srel!(ox + 8.0, oy + 4.0, 4.0, 12.0),  // West
                    srel!(ox + 0.0, oy + 4.0, 4.0, 12.0),  // East
                ],
            );
            let (ox, oy) = (offsets[2], offsets[3]);
            model::append_box(
                &mut part_verts[i],
                -2.2 / MODEL_PIXEL,
                -12.2 / MODEL_PIXEL,
                -2.2 / MODEL_PIXEL,
                4.4 / MODEL_PIXEL,
                12.4 / MODEL_PIXEL,
                4.4 / MODEL_PIXEL,
                [
                    srel!(ox + 8.0, oy + 0.0, 4.0, 4.0),   // Down
                    srel!(ox + 4.0, oy + 0.0, 4.0, 4.0),   // Up
                    srel!(ox + 4.0, oy + 4.0, 4.0, 12.0),  // North
                    srel!(ox + 12.0, oy + 4.0, 4.0, 12.0), // South
                    srel!(ox + 8.0, oy + 4.0, 4.0, 12.0),  // West
                    srel!(ox + 0.0, oy + 4.0, 4.0, 12.0),  // East
                ],
            );
        }

        let mut name_verts = vec![];
        if visible_parts[PlayerModelPart::NameTag.as_index()] {
            let mut state = FormatState {
                width: 0.0,
                offset: 0.0,
                text: Vec::new(),
                renderer,
                y_scale: NAME_TEXT_Y_SCALE,
                x_scale: NAME_TEXT_X_SCALE,
            };
            let mut name = format::Component::Text(format::TextComponent::new(&player_model.name));
            format::convert_legacy(&mut name);
            state.build(&name, format::Color::Black);
            let width = state.width;
            // Center align text
            for vert in &mut state.text {
                vert.x += width * NAME_TEXT_CENTER_SCALE;
                vert.r = NAME_TEXT_DARK_CHANNEL;
                vert.g = NAME_TEXT_DARK_CHANNEL;
                vert.b = NAME_TEXT_DARK_CHANNEL;
            }
            name_verts.extend_from_slice(&state.text);
            for vert in &mut state.text {
                vert.x -= NAME_TEXT_SHADOW_OFFSET;
                vert.y -= NAME_TEXT_SHADOW_OFFSET;
                vert.z -= NAME_TEXT_DEPTH_OFFSET;
                vert.r = NAME_TEXT_LIGHT_CHANNEL;
                vert.g = NAME_TEXT_LIGHT_CHANNEL;
                vert.b = NAME_TEXT_LIGHT_CHANNEL;
            }
            name_verts.extend_from_slice(&state.text);
        }

        player_model.model = Some(renderer.model.create_model(
            model::DEFAULT,
            vec![
                head_verts,
                body_verts,
                part_verts[0].clone(),
                part_verts[1].clone(),
                part_verts[2].clone(),
                part_verts[3].clone(),
                name_verts,
            ],
        ));
    }

    fn entity_removed(
        &mut self,
        m: &mut ecs::Manager,
        e: ecs::Entity,
        _: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let player_model = m.get_component_mut(e, self.player_model).unwrap();
        if let Some(model) = player_model.model.take() {
            renderer.model.remove_model(model);
            if let Some(url) = player_model.skin_url.as_ref() {
                renderer
                    .get_textures_ref()
                    .read()
                    .unwrap()
                    .release_skin(url);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_renderer_plan_rebuilds_dirty_model() {
        assert_eq!(
            plan_renderer_shell(true, false),
            RendererShellPlan::RebuildAndRender
        );
    }

    #[test]
    fn player_renderer_plan_renders_existing_model() {
        assert_eq!(
            plan_renderer_shell(false, true),
            RendererShellPlan::RenderExisting
        );
    }

    #[test]
    fn player_renderer_plan_skips_missing_model_resource() {
        assert_eq!(
            plan_renderer_shell(false, false),
            RendererShellPlan::SkipMissingModel
        );
    }
}
