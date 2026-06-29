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

pub(crate) const SCALED_WIDTH: f64 = 854.0;
pub(crate) const SCALED_HEIGHT: f64 = 480.0;
const HALF_DIVISOR: f64 = 2.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Scaled,
    Unscaled(f64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VAttach {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAttach {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Region {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) w: f64,
    pub(crate) h: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LayoutBoundsError {
    NonFinite,
    NegativeSize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WidgetGeometry {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) v_attach: VAttach,
    pub(crate) h_attach: HAttach,
}

pub(crate) const SCREEN: Region = Region {
    x: 0.0,
    y: 0.0,
    w: SCALED_WIDTH,
    h: SCALED_HEIGHT,
};

impl Region {
    pub(crate) fn checked(x: f64, y: f64, w: f64, h: f64) -> Result<Self, LayoutBoundsError> {
        validate_region_numbers(x, y, w, h)?;
        Ok(Region { x, y, w, h })
    }

    pub(crate) fn intersects(&self, other: &Region) -> bool {
        !(self.x + self.w < other.x
            || self.x > other.x + other.w
            || self.y + self.h < other.y
            || self.y > other.y + other.h)
    }

    pub(crate) fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

pub(crate) fn validate_region_numbers(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), LayoutBoundsError> {
    if !x.is_finite() || !y.is_finite() || !width.is_finite() || !height.is_finite() {
        return Err(LayoutBoundsError::NonFinite);
    }
    if width < 0.0 || height < 0.0 {
        return Err(LayoutBoundsError::NegativeSize);
    }
    Ok(())
}

pub(crate) fn scale_for_mode(mode: Mode, width: f64, height: f64) -> (f64, f64) {
    match mode {
        Mode::Scaled => (SCALED_WIDTH / width, SCALED_HEIGHT / height),
        Mode::Unscaled(scale) => (scale, scale),
    }
}

pub(crate) fn mouse_to_scaled(x: f64, y: f64, width: f64, height: f64) -> (f64, f64) {
    ((x / width) * SCALED_WIDTH, (y / height) * SCALED_HEIGHT)
}

pub(crate) fn compute_draw_region(
    geometry: WidgetGeometry,
    scale: (f64, f64),
    super_region: &Region,
) -> Region {
    let (scale_x, scale_y) = scale;
    let width = geometry.width * scale_x;
    let height = geometry.height * scale_y;
    let x = match geometry.h_attach {
        HAttach::Left => geometry.x * scale_x,
        HAttach::Center => {
            (super_region.w / HALF_DIVISOR) - (width / HALF_DIVISOR) + geometry.x * scale_x
        }
        HAttach::Right => super_region.w - geometry.x * scale_x - width,
    };
    let y = match geometry.v_attach {
        VAttach::Top => geometry.y * scale_y,
        VAttach::Middle => {
            (super_region.h / HALF_DIVISOR) - (height / HALF_DIVISOR) + geometry.y * scale_y
        }
        VAttach::Bottom => super_region.h - geometry.y * scale_y - height,
    };

    Region {
        x: x + super_region.x,
        y: y + super_region.y,
        w: width,
        h: height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT_X: f64 = 10.0;
    const ROOT_Y: f64 = 20.0;
    const ROOT_WIDTH: f64 = 200.0;
    const ROOT_HEIGHT: f64 = 100.0;
    const WIDGET_X: f64 = 5.0;
    const WIDGET_Y: f64 = 7.0;
    const WIDGET_WIDTH: f64 = 50.0;
    const WIDGET_HEIGHT: f64 = 20.0;
    const SCALE_X: f64 = 2.0;
    const SCALE_Y: f64 = 3.0;
    const NEGATIVE_WIDTH: f64 = -1.0;

    fn root_region() -> Region {
        Region::checked(ROOT_X, ROOT_Y, ROOT_WIDTH, ROOT_HEIGHT).unwrap()
    }

    fn geometry(v_attach: VAttach, h_attach: HAttach) -> WidgetGeometry {
        WidgetGeometry {
            x: WIDGET_X,
            y: WIDGET_Y,
            width: WIDGET_WIDTH,
            height: WIDGET_HEIGHT,
            v_attach,
            h_attach,
        }
    }

    #[test]
    fn layout_regions_compute_center_and_middle_attachment() {
        let region = compute_draw_region(
            geometry(VAttach::Middle, HAttach::Center),
            (SCALE_X, SCALE_Y),
            &root_region(),
        );

        assert_eq!(
            region,
            Region {
                x: ROOT_X + (ROOT_WIDTH / HALF_DIVISOR) - ((WIDGET_WIDTH * SCALE_X) / HALF_DIVISOR)
                    + WIDGET_X * SCALE_X,
                y: ROOT_Y + (ROOT_HEIGHT / HALF_DIVISOR)
                    - ((WIDGET_HEIGHT * SCALE_Y) / HALF_DIVISOR)
                    + WIDGET_Y * SCALE_Y,
                w: WIDGET_WIDTH * SCALE_X,
                h: WIDGET_HEIGHT * SCALE_Y,
            }
        );
        assert!(root_region().intersects(&region));
    }

    #[test]
    fn attachment_calculations_handle_right_and_bottom_edges() {
        let region = compute_draw_region(
            geometry(VAttach::Bottom, HAttach::Right),
            (SCALE_X, SCALE_Y),
            &root_region(),
        );

        assert_eq!(
            region,
            Region {
                x: ROOT_X + ROOT_WIDTH - WIDGET_X * SCALE_X - WIDGET_WIDTH * SCALE_X,
                y: ROOT_Y + ROOT_HEIGHT - WIDGET_Y * SCALE_Y - WIDGET_HEIGHT * SCALE_Y,
                w: WIDGET_WIDTH * SCALE_X,
                h: WIDGET_HEIGHT * SCALE_Y,
            }
        );
        assert!(region.contains(region.x, region.y));
    }

    #[test]
    fn invalid_layout_bounds_are_rejected() {
        assert_eq!(
            Region::checked(ROOT_X, ROOT_Y, NEGATIVE_WIDTH, ROOT_HEIGHT),
            Err(LayoutBoundsError::NegativeSize)
        );
        assert_eq!(
            Region::checked(ROOT_X, f64::NAN, ROOT_WIDTH, ROOT_HEIGHT),
            Err(LayoutBoundsError::NonFinite)
        );
    }
}
