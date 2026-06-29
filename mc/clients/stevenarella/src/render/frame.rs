pub(super) const INITIAL_FRAME_ID: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FrameDimensions {
    pub(crate) logical_width: u32,
    pub(crate) logical_height: u32,
    pub(crate) physical_width: u32,
    pub(crate) physical_height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FramePlan {
    pub(crate) dimensions: FrameDimensions,
    pub(crate) next_frame_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FramePlanError {
    InvalidLogicalDimensions { width: u32, height: u32 },
    InvalidPhysicalDimensions { width: u32, height: u32 },
}

pub(crate) fn plan_frame(
    dimensions: FrameDimensions,
    current_frame_id: u32,
) -> Result<FramePlan, FramePlanError> {
    if dimensions.logical_width == 0 || dimensions.logical_height == 0 {
        return Err(FramePlanError::InvalidLogicalDimensions {
            width: dimensions.logical_width,
            height: dimensions.logical_height,
        });
    }
    if dimensions.physical_width == 0 || dimensions.physical_height == 0 {
        return Err(FramePlanError::InvalidPhysicalDimensions {
            width: dimensions.physical_width,
            height: dimensions.physical_height,
        });
    }

    Ok(FramePlan {
        dimensions,
        next_frame_id: current_frame_id.wrapping_add(1),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LOGICAL_WIDTH: u32 = 854;
    const TEST_LOGICAL_HEIGHT: u32 = 480;
    const TEST_PHYSICAL_WIDTH: u32 = 1708;
    const TEST_PHYSICAL_HEIGHT: u32 = 960;
    const TEST_FRAME_ID: u32 = 41;
    const EXPECTED_NEXT_FRAME_ID: u32 = 42;

    fn valid_dimensions() -> FrameDimensions {
        FrameDimensions {
            logical_width: TEST_LOGICAL_WIDTH,
            logical_height: TEST_LOGICAL_HEIGHT,
            physical_width: TEST_PHYSICAL_WIDTH,
            physical_height: TEST_PHYSICAL_HEIGHT,
        }
    }

    #[test]
    fn frame_plan_preserves_dimensions_and_advances_frame_id() {
        let plan = plan_frame(valid_dimensions(), TEST_FRAME_ID).unwrap();

        assert_eq!(plan.dimensions, valid_dimensions());
        assert_eq!(plan.next_frame_id, EXPECTED_NEXT_FRAME_ID);
    }

    #[test]
    fn frame_plan_rejects_invalid_dimensions() {
        let invalid_logical = FrameDimensions {
            logical_width: 0,
            ..valid_dimensions()
        };
        let invalid_physical = FrameDimensions {
            physical_height: 0,
            ..valid_dimensions()
        };

        assert_eq!(
            plan_frame(invalid_logical, TEST_FRAME_ID),
            Err(FramePlanError::InvalidLogicalDimensions {
                width: 0,
                height: TEST_LOGICAL_HEIGHT,
            })
        );
        assert_eq!(
            plan_frame(invalid_physical, TEST_FRAME_ID),
            Err(FramePlanError::InvalidPhysicalDimensions {
                width: TEST_PHYSICAL_WIDTH,
                height: 0,
            })
        );
    }
}
