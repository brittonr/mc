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

pub(crate) const TEXT_DEFAULT_HEIGHT: f64 = 18.0;
pub(crate) const TEXT_WIDTH_PADDING: f64 = 2.0;
pub(crate) const TEXT_DEFAULT_SCALE: f64 = 1.0;
pub(crate) const TEXT_DEFAULT_ROTATION: f64 = 0.0;
pub(crate) const TEXT_OPAQUE_ALPHA: u8 = 255;
pub(crate) const TEXT_DEFAULT_COLOUR: (u8, u8, u8, u8) = (
    TEXT_OPAQUE_ALPHA,
    TEXT_OPAQUE_ALPHA,
    TEXT_OPAQUE_ALPHA,
    TEXT_OPAQUE_ALPHA,
);
pub(crate) const TEXT_TRANSPARENT_COLOUR: (u8, u8, u8, u8) = (0, 0, 0, 0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TextMetrics {
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) scale_x: f64,
    pub(crate) scale_y: f64,
}

pub(crate) fn scaled_text_size(metrics: TextMetrics) -> (f64, f64) {
    (
        (metrics.width + TEXT_WIDTH_PADDING) * metrics.scale_x,
        metrics.height * metrics.scale_y,
    )
}

pub(crate) fn text_visual_state_changed<T: PartialEq>(previous: T, current: T) -> bool {
    previous != current
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT_WIDTH: f64 = 30.0;
    const TEXT_HEIGHT: f64 = TEXT_DEFAULT_HEIGHT;
    const SCALE_X: f64 = 2.0;
    const SCALE_Y: f64 = 3.0;

    #[test]
    fn text_metrics_include_padding_and_scale() {
        assert_eq!(
            scaled_text_size(TextMetrics {
                width: TEXT_WIDTH,
                height: TEXT_HEIGHT,
                scale_x: SCALE_X,
                scale_y: SCALE_Y,
            }),
            (
                (TEXT_WIDTH + TEXT_WIDTH_PADDING) * SCALE_X,
                TEXT_HEIGHT * SCALE_Y
            )
        );
    }

    #[test]
    fn empty_text_metrics_stay_bounded() {
        assert_eq!(
            scaled_text_size(TextMetrics {
                width: 0.0,
                height: TEXT_HEIGHT,
                scale_x: TEXT_DEFAULT_SCALE,
                scale_y: TEXT_DEFAULT_SCALE,
            }),
            (TEXT_WIDTH_PADDING, TEXT_HEIGHT)
        );
        assert!(!text_visual_state_changed(
            TEXT_DEFAULT_COLOUR,
            TEXT_DEFAULT_COLOUR
        ));
    }
}
