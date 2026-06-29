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

const FIRST_PRINTABLE_CHAR: char = ' ';
const BACKSPACE_CHAR: char = '\x08';

pub(crate) fn next_focus_index(last_focus: Option<usize>, focusable_len: usize) -> Option<usize> {
    if focusable_len == 0 {
        return None;
    }
    Some(last_focus.map_or(0, |index| index + 1) % focusable_len)
}

pub(crate) fn should_auto_focus(focusable_len: usize, any_focused: bool) -> bool {
    focusable_len != 0 && !any_focused
}

pub(crate) fn accepts_typed_char(c: char) -> bool {
    c >= FIRST_PRINTABLE_CHAR || c == BACKSPACE_CHAR
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOCUSABLE_COUNT: usize = 3;
    const FIRST_INDEX: usize = 0;
    const SECOND_INDEX: usize = 1;
    const LAST_INDEX: usize = 2;
    const PRINTABLE_CHAR: char = 'a';
    const CONTROL_CHAR: char = '\u{0007}';

    #[test]
    fn focus_changes_advance_and_wrap() {
        assert_eq!(next_focus_index(None, FOCUSABLE_COUNT), Some(FIRST_INDEX));
        assert_eq!(
            next_focus_index(Some(FIRST_INDEX), FOCUSABLE_COUNT),
            Some(SECOND_INDEX)
        );
        assert_eq!(
            next_focus_index(Some(LAST_INDEX), FOCUSABLE_COUNT),
            Some(FIRST_INDEX)
        );
        assert!(should_auto_focus(FOCUSABLE_COUNT, false));
    }

    #[test]
    fn focus_loss_and_control_input_fail_closed() {
        assert_eq!(next_focus_index(Some(FIRST_INDEX), 0), None);
        assert!(!should_auto_focus(0, false));
        assert!(!should_auto_focus(FOCUSABLE_COUNT, true));
        assert!(accepts_typed_char(PRINTABLE_CHAR));
        assert!(accepts_typed_char(BACKSPACE_CHAR));
        assert!(!accepts_typed_char(CONTROL_CHAR));
    }
}
