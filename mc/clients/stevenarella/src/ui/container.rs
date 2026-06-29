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

use super::layout::Region;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TraversalPlanError {
    DrawIndexMissing,
}

pub(crate) fn draw_order(draw_indices: &[isize]) -> Vec<usize> {
    let mut order = (0..draw_indices.len()).collect::<Vec<_>>();
    order.sort_by_key(|index| draw_indices[*index]);
    order
}

pub(crate) fn sort_by_draw_index<T, F>(items: &mut [T], draw_index: F)
where
    F: FnMut(&T) -> isize,
{
    items.sort_by_key(draw_index);
}

pub(crate) fn clicked_index(regions: &[Region], x: f64, y: f64) -> Option<usize> {
    regions.iter().position(|region| region.contains(x, y))
}

pub(crate) fn checked_draw_order(
    draw_indices: &[Option<isize>],
) -> Result<Vec<usize>, TraversalPlanError> {
    let mut concrete = Vec::with_capacity(draw_indices.len());
    for draw_index in draw_indices {
        match draw_index {
            Some(value) => concrete.push(*value),
            None => return Err(TraversalPlanError::DrawIndexMissing),
        }
    }
    Ok(draw_order(&concrete))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_DRAW_INDEX: isize = 4;
    const SECOND_DRAW_INDEX: isize = -2;
    const THIRD_DRAW_INDEX: isize = 1;
    const FIRST_VISIT: usize = 1;
    const SECOND_VISIT: usize = 2;
    const THIRD_VISIT: usize = 0;
    const REGION_WIDTH: f64 = 10.0;
    const REGION_HEIGHT: f64 = 10.0;
    const INSIDE_X: f64 = 5.0;
    const INSIDE_Y: f64 = 5.0;
    const OUTSIDE_X: f64 = 50.0;
    const OUTSIDE_Y: f64 = 50.0;

    fn region() -> Region {
        Region::checked(0.0, 0.0, REGION_WIDTH, REGION_HEIGHT).unwrap()
    }

    #[test]
    fn container_traversal_visits_draw_indices_in_order() {
        assert_eq!(
            draw_order(&[FIRST_DRAW_INDEX, SECOND_DRAW_INDEX, THIRD_DRAW_INDEX]),
            vec![FIRST_VISIT, SECOND_VISIT, THIRD_VISIT]
        );
        assert_eq!(
            checked_draw_order(&[Some(FIRST_DRAW_INDEX)]).unwrap(),
            vec![0]
        );
        assert_eq!(clicked_index(&[region()], INSIDE_X, INSIDE_Y), Some(0));
    }

    #[test]
    fn missing_draw_index_and_outside_click_fail_closed() {
        assert_eq!(
            checked_draw_order(&[Some(FIRST_DRAW_INDEX), None]),
            Err(TraversalPlanError::DrawIndexMissing)
        );
        assert_eq!(clicked_index(&[region()], OUTSIDE_X, OUTSIDE_Y), None);
    }
}
