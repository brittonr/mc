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

pub(crate) const BUTTON_TEXTURE_ATLAS_SIZE: f32 = 256.0;
pub(crate) const BUTTON_TEXTURE_WIDTH: f32 = 200.0;
pub(crate) const BUTTON_TEXTURE_HEIGHT: f32 = 20.0;
pub(crate) const BUTTON_DISABLED_TEXTURE_Y: f32 = 46.0;
pub(crate) const BUTTON_NORMAL_TEXTURE_Y: f32 = 66.0;
pub(crate) const BUTTON_HOVER_TEXTURE_Y: f32 = 86.0;
pub(crate) const BUTTON_HOVER_TEXT_BLUE: u8 = 160;
pub(crate) const BUTTON_NORMAL_TEXT_BLUE: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ButtonState {
    pub(crate) disabled: bool,
    pub(crate) hovered: bool,
}

pub(crate) fn texture_y_offset(state: ButtonState) -> f32 {
    match (state.disabled, state.hovered) {
        (true, _) => BUTTON_DISABLED_TEXTURE_Y,
        (false, true) => BUTTON_HOVER_TEXTURE_Y,
        (false, false) => BUTTON_NORMAL_TEXTURE_Y,
    }
}

pub(crate) fn hover_text_blue_channel(hovered: bool) -> u8 {
    if hovered {
        BUTTON_HOVER_TEXT_BLUE
    } else {
        BUTTON_NORMAL_TEXT_BLUE
    }
}

pub(crate) fn visual_state_changed(previous: ButtonState, current: ButtonState) -> bool {
    previous != current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_state_selects_normal_hover_and_disabled_texture_rows() {
        assert_eq!(
            texture_y_offset(ButtonState {
                disabled: false,
                hovered: false,
            }),
            BUTTON_NORMAL_TEXTURE_Y
        );
        assert_eq!(
            texture_y_offset(ButtonState {
                disabled: false,
                hovered: true,
            }),
            BUTTON_HOVER_TEXTURE_Y
        );
        assert_eq!(hover_text_blue_channel(true), BUTTON_HOVER_TEXT_BLUE);
    }

    #[test]
    fn disabled_widgets_ignore_hover_texture_and_detect_noop_state() {
        let disabled_hover = ButtonState {
            disabled: true,
            hovered: true,
        };
        let disabled_plain = ButtonState {
            disabled: true,
            hovered: false,
        };

        assert_eq!(texture_y_offset(disabled_hover), BUTTON_DISABLED_TEXTURE_Y);
        assert_eq!(texture_y_offset(disabled_plain), BUTTON_DISABLED_TEXTURE_Y);
        assert!(!visual_state_changed(disabled_plain, disabled_plain));
        assert!(visual_state_changed(disabled_plain, disabled_hover));
    }
}
