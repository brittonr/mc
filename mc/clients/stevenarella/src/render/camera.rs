use cgmath::prelude::*;

const DEFAULT_CAMERA_PITCH_RADIANS: f64 = std::f64::consts::PI;
const VIEW_YAW_OFFSET_RADIANS: f64 = std::f64::consts::FRAC_PI_2;
const DEFAULT_FOV_DEGREES: f32 = 90.0;
const MIN_ASPECT_RATIO: f32 = 1.0;
const NEAR_CLIP_PLANE: f32 = 0.1;
const FAR_CLIP_PLANE: f32 = 500.0;
const CAMERA_AXIS_FLIP: f32 = -1.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub pos: cgmath::Point3<f64>,
    pub yaw: f64,
    pub pitch: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: cgmath::Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: DEFAULT_CAMERA_PITCH_RADIANS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CameraPlanError {
    InvalidViewport { width: u32, height: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CameraViewFacts {
    pub(crate) position: cgmath::Point3<f64>,
    pub(crate) yaw: f64,
    pub(crate) pitch: f64,
    pub(crate) view_vector: cgmath::Vector3<f32>,
}

pub(super) struct CameraMatrices {
    pub(super) camera_matrix: cgmath::Matrix4<f32>,
    pub(super) frustum: collision::Frustum<f32>,
    pub(super) view_vector: cgmath::Vector3<f32>,
}

pub(super) fn perspective_matrix(
    width: u32,
    height: u32,
) -> Result<cgmath::Matrix4<f32>, CameraPlanError> {
    if width == 0 || height == 0 {
        return Err(CameraPlanError::InvalidViewport { width, height });
    }

    let fovy = cgmath::Rad::from(cgmath::Deg(DEFAULT_FOV_DEGREES));
    let aspect = ((width as f32) / (height as f32)).max(MIN_ASPECT_RATIO);
    Ok(cgmath::Matrix4::from(cgmath::PerspectiveFov {
        fovy,
        aspect,
        near: NEAR_CLIP_PLANE,
        far: FAR_CLIP_PLANE,
    }))
}

pub(crate) fn camera_view_facts(camera: &Camera) -> CameraViewFacts {
    CameraViewFacts {
        position: camera.pos,
        yaw: camera.yaw,
        pitch: camera.pitch,
        view_vector: view_vector(camera),
    }
}

pub(super) fn camera_matrices(
    camera: &Camera,
    perspective_matrix: cgmath::Matrix4<f32>,
) -> CameraMatrices {
    let view_vector = view_vector(camera);
    let camera_pos = cgmath::Point3::new(
        -camera.pos.x as f32,
        -camera.pos.y as f32,
        camera.pos.z as f32,
    );
    let camera_matrix = cgmath::Matrix4::look_at(
        camera_pos,
        camera_pos + cgmath::Point3::new(-view_vector.x, -view_vector.y, view_vector.z).to_vec(),
        cgmath::Vector3::new(0.0, CAMERA_AXIS_FLIP, 0.0),
    ) * cgmath::Matrix4::from_nonuniform_scale(CAMERA_AXIS_FLIP, 1.0, 1.0);
    let frustum = collision::Frustum::from_matrix4(perspective_matrix * camera_matrix)
        .expect("camera matrices should produce a valid frustum");

    CameraMatrices {
        camera_matrix,
        frustum,
        view_vector,
    }
}

fn view_vector(camera: &Camera) -> cgmath::Vector3<f32> {
    cgmath::Vector3::new(
        ((camera.yaw - VIEW_YAW_OFFSET_RADIANS).cos() * -camera.pitch.cos()) as f32,
        (-camera.pitch.sin()) as f32,
        (-(camera.yaw - VIEW_YAW_OFFSET_RADIANS).sin() * -camera.pitch.cos()) as f32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VIEWPORT_WIDTH: u32 = 1280;
    const TEST_VIEWPORT_HEIGHT: u32 = 720;
    const TEST_CAMERA_X: f64 = 4.0;
    const TEST_CAMERA_Y: f64 = 65.0;
    const TEST_CAMERA_Z: f64 = -2.0;
    const TEST_YAW: f64 = std::f64::consts::FRAC_PI_2;
    const TEST_PITCH: f64 = 0.0;
    const EXPECTED_VIEW_X: f32 = -1.0;
    const FLOAT_EPSILON: f32 = 0.000_001;

    #[test]
    fn camera_view_facts_include_position_angles_and_view_vector() {
        let camera = Camera {
            pos: cgmath::Point3::new(TEST_CAMERA_X, TEST_CAMERA_Y, TEST_CAMERA_Z),
            yaw: TEST_YAW,
            pitch: TEST_PITCH,
        };

        let facts = camera_view_facts(&camera);

        assert_eq!(facts.position, camera.pos);
        assert_eq!(facts.yaw, TEST_YAW);
        assert_eq!(facts.pitch, TEST_PITCH);
        assert!((facts.view_vector.x - EXPECTED_VIEW_X).abs() <= FLOAT_EPSILON);
        assert!(facts.view_vector.y.abs() <= FLOAT_EPSILON);
        assert!(facts.view_vector.z.abs() <= FLOAT_EPSILON);
    }

    #[test]
    fn viewport_perspective_rejects_empty_dimensions() {
        assert_eq!(
            perspective_matrix(0, TEST_VIEWPORT_HEIGHT),
            Err(CameraPlanError::InvalidViewport {
                width: 0,
                height: TEST_VIEWPORT_HEIGHT,
            })
        );
        assert_eq!(
            perspective_matrix(TEST_VIEWPORT_WIDTH, 0),
            Err(CameraPlanError::InvalidViewport {
                width: TEST_VIEWPORT_WIDTH,
                height: 0,
            })
        );
    }
}
