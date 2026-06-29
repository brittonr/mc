use instant::Duration;
use log::{error, info};

use crate::capture;
use crate::game_shell::startup::Opt;
use crate::Game;

pub const CAPTURE_START_MILLIS: u64 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartupRecordingOptions {
    pub frame_rate_hz: Option<u16>,
    pub max_frames: Option<u32>,
    pub max_duration_millis: Option<u64>,
}

pub fn capture_policy_from_opt(opt: &Opt) -> capture::CapturePolicy {
    match &opt.capture_dir {
        Some(capture_dir) => capture::CapturePolicy::local(capture_dir),
        None => capture::CapturePolicy::memory(),
    }
}

pub fn startup_recording_options_from_opt(opt: &Opt) -> StartupRecordingOptions {
    StartupRecordingOptions {
        frame_rate_hz: opt.capture_record_fps,
        max_frames: opt.capture_record_frames,
        max_duration_millis: opt.capture_record_duration_millis,
    }
}

pub fn startup_recording_request_from_opt(
    opt: &Opt,
    policy: &capture::CapturePolicy,
) -> Result<Option<capture::CaptureRequest>, capture::CaptureValidationError> {
    startup_recording_request(startup_recording_options_from_opt(opt), policy)
}

pub fn startup_recording_request(
    options: StartupRecordingOptions,
    policy: &capture::CapturePolicy,
) -> Result<Option<capture::CaptureRequest>, capture::CaptureValidationError> {
    if !startup_recording_requested(options) {
        return Ok(None);
    }

    let request = capture::CaptureRequest {
        mode: capture::CaptureMode::Recording,
        format: capture::CaptureFormat::Png,
        output: capture::CaptureOutput::Artifact {
            relative_path: capture::default_recording_relative_dir(
                capture::CAPTURE_SEQUENCE_INITIAL,
            ),
        },
        includes_ui: true,
        recording: startup_recording_bounds(options),
        sequence_id: Some(capture::CAPTURE_SEQUENCE_INITIAL),
    };
    capture::validate_capture_request(&request, policy)?;
    Ok(Some(request))
}

pub fn duration_to_millis_saturated(duration: Duration) -> u64 {
    let millis = duration.as_millis();
    if millis > u128::from(u64::MAX) {
        return u64::MAX;
    }
    millis as u64
}

fn startup_recording_requested(options: StartupRecordingOptions) -> bool {
    options.frame_rate_hz.is_some()
        || options.max_frames.is_some()
        || options.max_duration_millis.is_some()
}

fn startup_recording_bounds(options: StartupRecordingOptions) -> Option<capture::RecordingBounds> {
    options
        .frame_rate_hz
        .map(|frame_rate_hz| capture::RecordingBounds {
            frame_rate_hz,
            max_frames: options.max_frames,
            max_duration_millis: options.max_duration_millis,
        })
}

impl Game {
    pub fn service_pending_mcp_capture_requests(&mut self) -> usize {
        let Some(receiver) = self.mcp_capture_request_receiver.take() else {
            return 0;
        };
        let frame = self.current_capture_frame_context();
        let serviced = receiver.service_pending_one_shot_with_readback(
            &self.capture_policy,
            frame,
            capture::read_current_framebuffer_for_context,
        );
        self.mcp_capture_request_receiver = Some(receiver);
        serviced
    }

    pub fn service_active_capture_recording(&mut self) {
        let frame = self.current_capture_frame_context();
        let now_millis = duration_to_millis_saturated(self.capture_started_at.elapsed());
        let Some(recording) = self.active_capture_recording.as_mut() else {
            return;
        };
        let outcome = capture::service_recording_frame_with_readback(
            recording,
            &self.capture_policy,
            now_millis,
            frame,
            &mut capture::read_current_framebuffer_for_context,
        );
        match outcome {
            Ok(capture::RecordingServiceOutcome::Captured(metadata)) => info!(
                "Capture recording wrote {:?} digest={}",
                metadata.relative_path,
                metadata.blake3_digest.as_str()
            ),
            Ok(capture::RecordingServiceOutcome::Waiting) => {}
            Ok(capture::RecordingServiceOutcome::Complete) => {
                self.active_capture_recording = None;
                info!("Capture recording complete");
            }
            Err(err) => {
                self.active_capture_recording = None;
                error!("Capture recording stopped: {:?}", err);
            }
        }
    }

    fn current_capture_frame_context(&self) -> capture::CaptureFrameContext {
        self.renderer
            .capture_frame_context()
            .expect("renderer capture context should be available after frame setup")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RECORDING_FPS: u16 = 5;
    const TEST_RECORDING_FRAMES: u32 = 3;
    const TEST_RECORDING_DURATION_MILLIS: u64 = 250;

    fn requested_recording_options() -> StartupRecordingOptions {
        StartupRecordingOptions {
            frame_rate_hz: Some(TEST_RECORDING_FPS),
            max_frames: Some(TEST_RECORDING_FRAMES),
            max_duration_millis: Some(TEST_RECORDING_DURATION_MILLIS),
        }
    }

    #[test]
    fn default_startup_options_do_not_request_recording() {
        let policy = capture::CapturePolicy::memory();
        let options = StartupRecordingOptions {
            frame_rate_hz: None,
            max_frames: None,
            max_duration_millis: None,
        };

        assert_eq!(startup_recording_request(options, &policy), Ok(None));
    }

    #[test]
    fn startup_recording_plan_requires_local_capture_dir() {
        let policy = capture::CapturePolicy::memory();

        assert_eq!(
            startup_recording_request(requested_recording_options(), &policy),
            Err(capture::CaptureValidationError::MissingCaptureDir)
        );
    }

    #[test]
    fn startup_recording_plan_builds_bounded_recording_request() {
        let capture_dir = std::env::temp_dir().join("stevenarella-startup-recording-plan");
        let policy = capture::CapturePolicy::local(&capture_dir);

        let request = startup_recording_request(requested_recording_options(), &policy)
            .expect("valid recording request")
            .expect("recording requested");

        assert_eq!(request.mode, capture::CaptureMode::Recording);
        assert_eq!(request.format, capture::CaptureFormat::Png);
        assert_eq!(request.sequence_id, Some(capture::CAPTURE_SEQUENCE_INITIAL));
        assert_eq!(
            request.recording,
            Some(capture::RecordingBounds {
                frame_rate_hz: TEST_RECORDING_FPS,
                max_frames: Some(TEST_RECORDING_FRAMES),
                max_duration_millis: Some(TEST_RECORDING_DURATION_MILLIS),
            })
        );
    }

    #[test]
    fn startup_recording_plan_rejects_missing_frame_rate() {
        let capture_dir = std::env::temp_dir().join("stevenarella-startup-recording-missing-fps");
        let policy = capture::CapturePolicy::local(&capture_dir);
        let options = StartupRecordingOptions {
            frame_rate_hz: None,
            max_frames: Some(TEST_RECORDING_FRAMES),
            max_duration_millis: None,
        };

        assert_eq!(
            startup_recording_request(options, &policy),
            Err(capture::CaptureValidationError::RecordingBoundsRequired)
        );
    }

    #[test]
    fn duration_conversion_saturates_large_values() {
        assert_eq!(
            duration_to_millis_saturated(Duration::from_millis(TEST_RECORDING_DURATION_MILLIS)),
            TEST_RECORDING_DURATION_MILLIS
        );
        assert_eq!(
            duration_to_millis_saturated(Duration::from_secs(u64::MAX)),
            u64::MAX
        );
    }
}
