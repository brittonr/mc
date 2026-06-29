use super::atlas;

#[derive(Debug, Clone)]
pub(super) struct PendingTextureUpload {
    pub(super) atlas: i32,
    pub(super) rect: atlas::Rect,
    pub(super) data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextureArrayResize {
    pub(crate) current_layers: usize,
    pub(crate) required_layers: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextureUploadPlan {
    pub(crate) resize: Option<TextureArrayResize>,
    pub(crate) pending_count: usize,
}

impl TextureUploadPlan {
    pub(crate) fn has_pending_uploads(self) -> bool {
        self.pending_count > 0
    }
}

pub(super) fn pending_texture_upload(
    atlas: i32,
    rect: atlas::Rect,
    data: Vec<u8>,
) -> PendingTextureUpload {
    PendingTextureUpload { atlas, rect, data }
}

pub(crate) fn plan_texture_uploads(
    current_layers: usize,
    required_layers: usize,
    pending_count: usize,
) -> TextureUploadPlan {
    let resize = if current_layers == required_layers {
        None
    } else {
        Some(TextureArrayResize {
            current_layers,
            required_layers,
        })
    };

    TextureUploadPlan {
        resize,
        pending_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CURRENT_TEXTURE_LAYERS: usize = 1;
    const RESIZED_TEXTURE_LAYERS: usize = 2;
    const PENDING_UPLOAD_COUNT: usize = 3;

    #[test]
    fn upload_plan_records_resize_and_pending_uploads() {
        assert_eq!(
            plan_texture_uploads(
                CURRENT_TEXTURE_LAYERS,
                RESIZED_TEXTURE_LAYERS,
                PENDING_UPLOAD_COUNT,
            ),
            TextureUploadPlan {
                resize: Some(TextureArrayResize {
                    current_layers: CURRENT_TEXTURE_LAYERS,
                    required_layers: RESIZED_TEXTURE_LAYERS,
                }),
                pending_count: PENDING_UPLOAD_COUNT,
            }
        );
    }

    #[test]
    fn upload_plan_without_work_has_no_side_effect_steps() {
        let plan = plan_texture_uploads(CURRENT_TEXTURE_LAYERS, CURRENT_TEXTURE_LAYERS, 0);

        assert_eq!(
            plan,
            TextureUploadPlan {
                resize: None,
                pending_count: 0,
            }
        );
        assert!(!plan.has_pending_uploads());
    }
}
