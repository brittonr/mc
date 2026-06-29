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

const BACKSPACE_CHAR: char = '\x08';
const DELETE_CHAR: char = '\x7f';
const PASSWORD_MASK_CHAR: char = '*';
pub(crate) const CURSOR_CHAR: char = '|';
pub(crate) const CURSOR_BLINK_PERIOD: f64 = 3000.0;
const CURSOR_BLINK_PHASE: f64 = 30.0;
const CURSOR_VISIBLE_PHASES: i32 = 2;
const CURSOR_VISIBLE_REMAINDER: i32 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TextEditOutcome {
    Inserted,
    RemovedLastChar,
    IgnoredEmptyDelete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ClipboardPath {
    Supported,
    Unsupported,
}

pub(crate) fn transformed_input(input: &str, password: bool) -> String {
    if password {
        std::iter::repeat(PASSWORD_MASK_CHAR)
            .take(input.chars().count())
            .collect()
    } else {
        input.to_owned()
    }
}

pub(crate) fn apply_typed_char(input: &mut String, c: char) -> TextEditOutcome {
    if c == DELETE_CHAR || c == BACKSPACE_CHAR {
        if input.pop().is_some() {
            return TextEditOutcome::RemovedLastChar;
        }
        return TextEditOutcome::IgnoredEmptyDelete;
    }

    input.push(c);
    TextEditOutcome::Inserted
}

pub(crate) fn clamp_cursor_position(input: &str, requested_position: usize) -> usize {
    requested_position.min(input.chars().count())
}

pub(crate) fn advance_cursor_tick(current_tick: f64, delta: f64) -> f64 {
    if !current_tick.is_finite() || !delta.is_finite() {
        return 0.0;
    }
    let mut next_tick = current_tick + delta;
    if next_tick > CURSOR_BLINK_PERIOD {
        next_tick -= CURSOR_BLINK_PERIOD;
    }
    if next_tick < 0.0 {
        return 0.0;
    }
    next_tick
}

pub(crate) fn cursor_is_visible(focused: bool, cursor_tick: f64) -> bool {
    focused
        && ((cursor_tick / CURSOR_BLINK_PHASE) as i32) % CURSOR_VISIBLE_PHASES
            == CURSOR_VISIBLE_REMAINDER
}

pub(crate) fn clipboard_paste_allowed(ctrl_pressed: bool, path: ClipboardPath) -> bool {
    ctrl_pressed && matches!(path, ClipboardPath::Supported)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VISIBLE_TEXT: &str = "steven";
    const EDITED_TEXT: &str = "steve";
    const INSERT_CHAR: char = 'n';
    const TOO_LARGE_CURSOR_POSITION: usize = 64;
    const VALID_CURSOR_POSITION: usize = 3;
    const CURSOR_TICK_BEFORE_WRAP: f64 = 2990.0;
    const CURSOR_TICK_DELTA: f64 = 20.0;
    const CURSOR_TICK_AFTER_WRAP: f64 = 10.0;

    #[test]
    fn textbox_editing_inserts_removes_and_masks_text() {
        let mut input = EDITED_TEXT.to_owned();

        assert_eq!(
            apply_typed_char(&mut input, INSERT_CHAR),
            TextEditOutcome::Inserted
        );
        assert_eq!(input, VISIBLE_TEXT);
        assert_eq!(transformed_input(&input, false), VISIBLE_TEXT);
        assert_eq!(transformed_input(&input, true), "******");
        assert_eq!(
            apply_typed_char(&mut input, BACKSPACE_CHAR),
            TextEditOutcome::RemovedLastChar
        );
        assert_eq!(input, EDITED_TEXT);
    }

    #[test]
    fn empty_text_invalid_cursor_and_unsupported_clipboard_fail_closed() {
        let mut input = String::new();

        assert_eq!(transformed_input(&input, false), "");
        assert_eq!(
            apply_typed_char(&mut input, DELETE_CHAR),
            TextEditOutcome::IgnoredEmptyDelete
        );
        assert_eq!(
            clamp_cursor_position(VISIBLE_TEXT, TOO_LARGE_CURSOR_POSITION),
            VISIBLE_TEXT.len()
        );
        assert_eq!(
            clamp_cursor_position(VISIBLE_TEXT, VALID_CURSOR_POSITION),
            VALID_CURSOR_POSITION
        );
        assert!(!clipboard_paste_allowed(true, ClipboardPath::Unsupported));
        assert!(!clipboard_paste_allowed(false, ClipboardPath::Supported));
        assert_eq!(
            advance_cursor_tick(CURSOR_TICK_BEFORE_WRAP, CURSOR_TICK_DELTA),
            CURSOR_TICK_AFTER_WRAP
        );
        assert_eq!(advance_cursor_tick(f64::NAN, CURSOR_TICK_DELTA), 0.0);
        assert!(!cursor_is_visible(false, 0.0));
    }
}
