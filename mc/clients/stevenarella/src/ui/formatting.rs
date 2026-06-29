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

use crate::format;

use super::text::{TEXT_DEFAULT_HEIGHT, TEXT_WIDTH_PADDING};

pub(crate) const FORMATTED_NO_WRAP_WIDTH: f64 = -1.0;
const FORMATTED_TEXT_Y_OFFSET: f64 = 1.0;
const ENABLED_MAX_WIDTH_MINIMUM: f64 = 0.0;
const MINIMUM_GLYPH_WIDTH: f64 = 0.0;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormattedTextRun {
    pub(crate) text: String,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) color: format::Color,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormattedTextPlan {
    pub(crate) runs: Vec<FormattedTextRun>,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

pub(crate) fn plan_formatted_text<F>(
    component: &format::Component,
    max_width: f64,
    char_width: F,
) -> FormattedTextPlan
where
    F: FnMut(char) -> f64,
{
    let mut state = PlanningState {
        max_width,
        lines: 0,
        offset: 0.0,
        width: 0.0,
        runs: Vec::new(),
        char_width,
    };
    state.build(component, format::Color::White);
    state.into_plan()
}

pub(crate) fn selected_color(
    modifier: &format::Modifier,
    inherited: format::Color,
) -> format::Color {
    modifier.color.unwrap_or(inherited)
}

struct PlanningState<F> {
    max_width: f64,
    lines: usize,
    offset: f64,
    width: f64,
    runs: Vec<FormattedTextRun>,
    char_width: F,
}

impl<F> PlanningState<F>
where
    F: FnMut(char) -> f64,
{
    fn into_plan(self) -> FormattedTextPlan {
        FormattedTextPlan {
            runs: self.runs,
            width: self.width + TEXT_WIDTH_PADDING,
            height: (self.lines + 1) as f64 * TEXT_DEFAULT_HEIGHT,
        }
    }

    fn build(&mut self, component: &format::Component, color: format::Color) {
        match *component {
            format::Component::Text(ref text) => {
                let selected = selected_color(&text.modifier, color);
                self.append_text(&text.text, selected);
                if let Some(ref extra) = text.modifier.extra {
                    for child in extra {
                        self.build(child, selected);
                    }
                }
            }
        }
    }

    fn append_text(&mut self, text: &str, color: format::Color) {
        let mut width = 0.0;
        let mut last = 0;
        for (index, character) in text.char_indices() {
            let size = self.char_advance(character);
            if self.should_wrap(width, size) || character == '\n' {
                self.push_run(&text[last..index], color);
                last = index;
                if character == '\n' {
                    last += character.len_utf8();
                }
                self.offset = 0.0;
                self.lines += 1;
                width = 0.0;
            }
            width += size;
            let next_width = self.offset + width;
            if next_width > self.width {
                self.width = next_width;
            }
        }

        if last != text.len() {
            self.push_run(&text[last..], color);
            self.offset += self.text_advance(&text[last..]);
            if self.offset > self.width {
                self.width = self.offset;
            }
        }
    }

    fn should_wrap(&self, current_width: f64, next_size: f64) -> bool {
        self.max_width.is_finite()
            && self.max_width > ENABLED_MAX_WIDTH_MINIMUM
            && self.offset + current_width + next_size > self.max_width
    }

    fn push_run(&mut self, text: &str, color: format::Color) {
        if text.is_empty() {
            return;
        }
        self.runs.push(FormattedTextRun {
            text: text.to_owned(),
            x: self.offset,
            y: self.lines as f64 * TEXT_DEFAULT_HEIGHT + FORMATTED_TEXT_Y_OFFSET,
            color,
        });
    }

    fn text_advance(&mut self, text: &str) -> f64 {
        text.chars()
            .map(|character| self.char_advance(character))
            .sum()
    }

    fn char_advance(&mut self, character: char) -> f64 {
        let width = (self.char_width)(character);
        if width.is_finite() && width > MINIMUM_GLYPH_WIDTH {
            width + TEXT_WIDTH_PADDING
        } else {
            TEXT_WIDTH_PADDING
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WRAPPED_LINE_COUNT: f64 = 3.0;
    const TEST_GLYPH_WIDTH: f64 = 4.0;
    const TEST_GLYPH_ADVANCE: f64 = TEST_GLYPH_WIDTH + TEXT_WIDTH_PADDING;
    const WRAP_AFTER_FIRST_GLYPH: f64 = TEST_GLYPH_ADVANCE;
    const RED_GREEN_JSON: &str =
        r#"{"text":"ab","color":"red","extra":[{"text":"c","color":"green"}]}"#;
    const MALFORMED_JSON: &str = "{not-json";

    fn fixed_width(_: char) -> f64 {
        TEST_GLYPH_WIDTH
    }

    #[test]
    fn formatted_text_plans_colored_runs_and_wrapping() {
        let component = format::Component::from_string(RED_GREEN_JSON);
        let plan = plan_formatted_text(&component, WRAP_AFTER_FIRST_GLYPH, fixed_width);

        assert_eq!(plan.runs.len(), 3);
        assert_eq!(plan.runs[0].text, "a");
        assert_eq!(plan.runs[0].color, format::Color::Red);
        assert_eq!(plan.runs[1].text, "b");
        assert_eq!(plan.runs[2].text, "c");
        assert_eq!(plan.runs[2].color, format::Color::Green);
        assert_eq!(plan.height, TEXT_DEFAULT_HEIGHT * WRAPPED_LINE_COUNT);
    }

    #[test]
    fn empty_and_malformed_formatting_are_contained() {
        let empty = format::Component::from_string("");
        let empty_plan = plan_formatted_text(&empty, WRAP_AFTER_FIRST_GLYPH, fixed_width);
        let malformed = format::Component::from_string(MALFORMED_JSON);
        let malformed_plan = plan_formatted_text(&malformed, f64::NAN, |_| f64::NAN);

        assert!(empty_plan.runs.is_empty());
        assert_eq!(empty_plan.height, TEXT_DEFAULT_HEIGHT);
        assert_eq!(malformed_plan.runs.len(), 1);
        assert_eq!(malformed_plan.runs[0].text, MALFORMED_JSON);
        assert_eq!(malformed_plan.runs[0].color, format::Color::White);
        assert_eq!(
            malformed_plan.width,
            MALFORMED_JSON.chars().count() as f64 * TEXT_WIDTH_PADDING + TEXT_WIDTH_PADDING
        );
    }
}
