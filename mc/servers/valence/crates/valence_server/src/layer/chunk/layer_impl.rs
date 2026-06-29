use bevy_ecs::prelude::Entity;
use valence_protocol::{BlockPos, ChunkPos};

use crate::layer::Layer;

use super::{
    ChunkLayer, ExceptWriter, RadiusExceptWriter, RadiusWriter, ViewExceptWriter, ViewWriter,
};

impl Layer for ChunkLayer {
    type ExceptWriter<'a> = ExceptWriter<'a>;

    type ViewWriter<'a> = ViewWriter<'a>;

    type ViewExceptWriter<'a> = ViewExceptWriter<'a>;

    type RadiusWriter<'a> = RadiusWriter<'a>;

    type RadiusExceptWriter<'a> = RadiusExceptWriter<'a>;

    fn except_writer(&mut self, except: Entity) -> Self::ExceptWriter<'_> {
        ExceptWriter {
            layer: self,
            except,
        }
    }

    fn view_writer(&mut self, pos: impl Into<ChunkPos>) -> Self::ViewWriter<'_> {
        ViewWriter {
            layer: self,
            pos: pos.into(),
        }
    }

    fn view_except_writer(
        &mut self,
        pos: impl Into<ChunkPos>,
        except: Entity,
    ) -> Self::ViewExceptWriter<'_> {
        ViewExceptWriter {
            layer: self,
            pos: pos.into(),
            except,
        }
    }

    fn radius_writer(
        &mut self,
        center: impl Into<BlockPos>,
        radius: u32,
    ) -> Self::RadiusWriter<'_> {
        RadiusWriter {
            layer: self,
            center: center.into(),
            radius_squared: radius,
        }
    }

    fn radius_except_writer(
        &mut self,
        center: impl Into<BlockPos>,
        radius: u32,
        except: Entity,
    ) -> Self::RadiusExceptWriter<'_> {
        RadiusExceptWriter {
            layer: self,
            center: center.into(),
            radius_squared: radius,
            except,
        }
    }
}
