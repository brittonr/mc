// Copyright 2016 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub(crate) const DEFAULT_IMAGE_COLOUR: (u8, u8, u8, u8) = (255, 255, 255, 255);
pub(crate) const TRANSPARENT_IMAGE_COLOUR: (u8, u8, u8, u8) = (0, 0, 0, 0);
pub(crate) const DEFAULT_TEXTURE_COORDS: (f64, f64, f64, f64) = (0.0, 0.0, 1.0, 1.0);

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ImageRenderState<'a> {
    pub(crate) texture: &'a str,
    pub(crate) colour: (u8, u8, u8, u8),
    pub(crate) texture_coords: (f64, f64, f64, f64),
}

pub(crate) fn image_visual_state_changed(
    previous: &ImageRenderState<'_>,
    current: &ImageRenderState<'_>,
) -> bool {
    previous != current
}

pub(crate) fn batch_visual_state_changed() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTURE_A: &str = "gui/a";
    const TEXTURE_B: &str = "gui/b";

    #[test]
    fn image_state_detects_texture_and_colour_changes() {
        let previous = ImageRenderState {
            texture: TEXTURE_A,
            colour: DEFAULT_IMAGE_COLOUR,
            texture_coords: DEFAULT_TEXTURE_COORDS,
        };
        let changed = ImageRenderState {
            texture: TEXTURE_B,
            colour: TRANSPARENT_IMAGE_COLOUR,
            texture_coords: DEFAULT_TEXTURE_COORDS,
        };

        assert!(image_visual_state_changed(&previous, &changed));
    }

    #[test]
    fn batch_state_has_no_own_visual_dirty_flag() {
        assert!(!batch_visual_state_changed());
    }
}
