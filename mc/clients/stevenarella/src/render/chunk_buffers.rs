use crate::gl;

const CHUNK_VERTICAL_OFFSET_SCALE: i32 = 4096;

#[derive(Default)]
pub struct ChunkBuffer {
    pub(super) solid: Option<ChunkRenderInfo>,
    pub(super) trans: Option<ChunkRenderInfo>,
}

impl ChunkBuffer {
    pub fn new() -> ChunkBuffer {
        Default::default()
    }

    pub(crate) fn layer_counts(&self) -> ChunkLayerCounts {
        ChunkLayerCounts {
            solid_count: self.solid.as_ref().map_or(0, |info| info.count),
            translucent_count: self.trans.as_ref().map_or(0, |info| info.count),
        }
    }
}

pub(super) struct ChunkRenderInfo {
    pub(super) array: gl::VertexArray,
    pub(super) buffer: gl::Buffer,
    pub(super) buffer_size: usize,
    pub(super) count: usize,
}

impl ChunkRenderInfo {
    pub(super) fn empty() -> Self {
        Self {
            array: gl::VertexArray::new(),
            buffer: gl::Buffer::new(),
            buffer_size: 0,
            count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ChunkLayerCounts {
    pub(crate) solid_count: usize,
    pub(crate) translucent_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ChunkRenderSummary {
    pub(crate) position: (i32, i32, i32),
    pub(crate) counts: ChunkLayerCounts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChunkRenderLayer {
    Solid,
    Translucent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ChunkDrawPlan {
    pub(crate) position: (i32, i32, i32),
    pub(crate) offset_y: i32,
}

pub(crate) fn plan_chunk_render_order(
    summaries: &[ChunkRenderSummary],
    layer: ChunkRenderLayer,
) -> Vec<ChunkDrawPlan> {
    let mut ordered = Vec::new();
    match layer {
        ChunkRenderLayer::Solid => {
            for summary in summaries {
                push_if_visible(&mut ordered, *summary, summary.counts.solid_count);
            }
        }
        ChunkRenderLayer::Translucent => {
            for summary in summaries.iter().rev() {
                push_if_visible(&mut ordered, *summary, summary.counts.translucent_count);
            }
        }
    }
    ordered
}

pub(crate) fn chunk_offset_y(section_y: i32) -> i32 {
    section_y * CHUNK_VERTICAL_OFFSET_SCALE
}

fn push_if_visible(ordered: &mut Vec<ChunkDrawPlan>, summary: ChunkRenderSummary, count: usize) {
    if count == 0 {
        return;
    }
    ordered.push(ChunkDrawPlan {
        position: summary.position,
        offset_y: chunk_offset_y(summary.position.1),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOWER_SECTION_Y: i32 = 2;
    const UPPER_SECTION_Y: i32 = 3;
    const LOWER_OFFSET_Y: i32 = LOWER_SECTION_Y * CHUNK_VERTICAL_OFFSET_SCALE;
    const UPPER_OFFSET_Y: i32 = UPPER_SECTION_Y * CHUNK_VERTICAL_OFFSET_SCALE;
    const FIRST_POSITION: (i32, i32, i32) = (10, LOWER_SECTION_Y, 12);
    const SECOND_POSITION: (i32, i32, i32) = (11, UPPER_SECTION_Y, 13);
    const SOLID_COUNT: usize = 6;
    const TRANSLUCENT_COUNT: usize = 12;

    fn summaries() -> [ChunkRenderSummary; 2] {
        [
            ChunkRenderSummary {
                position: FIRST_POSITION,
                counts: ChunkLayerCounts {
                    solid_count: SOLID_COUNT,
                    translucent_count: 0,
                },
            },
            ChunkRenderSummary {
                position: SECOND_POSITION,
                counts: ChunkLayerCounts {
                    solid_count: 0,
                    translucent_count: TRANSLUCENT_COUNT,
                },
            },
        ]
    }

    #[test]
    fn chunk_render_plan_filters_empty_layers_and_preserves_layer_order() {
        assert_eq!(
            plan_chunk_render_order(&summaries(), ChunkRenderLayer::Solid),
            vec![ChunkDrawPlan {
                position: FIRST_POSITION,
                offset_y: LOWER_OFFSET_Y,
            }]
        );
        assert_eq!(
            plan_chunk_render_order(&summaries(), ChunkRenderLayer::Translucent),
            vec![ChunkDrawPlan {
                position: SECOND_POSITION,
                offset_y: UPPER_OFFSET_Y,
            }]
        );
    }

    #[test]
    fn empty_chunk_buffers_produce_no_draw_plan() {
        let empty_summary = [ChunkRenderSummary {
            position: FIRST_POSITION,
            counts: ChunkLayerCounts {
                solid_count: 0,
                translucent_count: 0,
            },
        }];

        assert!(plan_chunk_render_order(&empty_summary, ChunkRenderLayer::Solid).is_empty());
        assert!(plan_chunk_render_order(&empty_summary, ChunkRenderLayer::Translucent).is_empty());
    }
}
