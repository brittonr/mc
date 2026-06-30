use crate::evidence_types::{
    FrameArtifactsReceiptEvidence, McpControlRunEvidence, ProjectileDamageCausalityEvidence,
    ProjectileTravelCollisionEvidence, ScenarioEvidence, ServerScenarioEvidence,
};
use crate::runtime_config;
use crate::scenario_core::Scenario;
use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    DryRun,
    Run,
    RunMatrix,
    BuildClient,
    StatusOnly,
    HarnessStatus,
    Cleanup,
    Stop,
    CompareReceipts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ServerBackend {
    Valence,
    Paper,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScenarioRouteRequest {
    pub(crate) scenario: Scenario,
    pub(crate) backend: ServerBackend,
    pub(crate) mode: Mode,
    pub(crate) receipt_path: Option<PathBuf>,
    pub(crate) timeout_secs: Option<u64>,
    pub(crate) packet_capture_summary: bool,
    pub(crate) proxy_route: Option<String>,
    pub(crate) proxy_forwarding_mode: Option<String>,
    pub(crate) failure_bundle_path: Option<PathBuf>,
    pub(crate) passthrough_args: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) root: PathBuf,
    pub(crate) client_dir: PathBuf,
    pub(crate) valence_repo: PathBuf,
    pub(crate) valence_rev: String,
    pub(crate) valence_worktree: PathBuf,
    pub(crate) valence_example: String,
    pub(crate) valence_log: PathBuf,
    pub(crate) valence_target_dir: PathBuf,
    pub(crate) valence_pid_file: PathBuf,
    pub(crate) server_backend: ServerBackend,
    pub(crate) target_dir: PathBuf,
    pub(crate) server_name: String,
    pub(crate) server_version: String,
    pub(crate) server_protocol: u32,
    pub(crate) server_port: u16,
    pub(crate) client_username: String,
    pub(crate) docker_image: String,
    pub(crate) paper_plugin_jar: Option<PathBuf>,
    pub(crate) mode: Mode,
    pub(crate) keep_server: bool,
    pub(crate) client_timeout: Duration,
    pub(crate) client_success_needles: Vec<String>,
    pub(crate) scenario: Scenario,
    pub(crate) expected_status_description: Option<String>,
    pub(crate) expected_status_version_name: Option<String>,
    pub(crate) expected_status_sample: Vec<String>,
    pub(crate) packet_capture_summary: bool,
    pub(crate) proxy_route: Option<String>,
    pub(crate) proxy_forwarding_mode: Option<String>,
    pub(crate) receipt_path: Option<PathBuf>,
    pub(crate) receipt_dir: Option<PathBuf>,
    pub(crate) failure_bundle_path: Option<PathBuf>,
    pub(crate) compare_receipts: Option<(PathBuf, PathBuf)>,
    pub(crate) config_path: Option<PathBuf>,
    pub(crate) steel_config_path: Option<PathBuf>,
    pub(crate) matrix_dry_run: bool,
    pub(crate) cleanup_apply: bool,
    pub(crate) negative_public_target: bool,
    pub(crate) negative_external_authorized: bool,
    pub(crate) arrow_damage_policy: runtime_config::ArrowDamagePolicy,
    pub(crate) scenario_route: Option<ScenarioRouteRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClientRunEvidence {
    pub(crate) log_path: Option<PathBuf>,
    pub(crate) log_paths: Vec<PathBuf>,
    pub(crate) usernames: Vec<String>,
    pub(crate) exit_code: Option<i32>,
    pub(crate) classification: &'static str,
    pub(crate) matched_success_pattern: Option<String>,
    pub(crate) scenario: Option<ScenarioEvidence>,
    pub(crate) server_scenario: Option<ServerScenarioEvidence>,
    pub(crate) projectile_damage_causality: Option<ProjectileDamageCausalityEvidence>,
    pub(crate) projectile_travel_collision: Option<ProjectileTravelCollisionEvidence>,
    pub(crate) mcp_control: Option<McpControlRunEvidence>,
    pub(crate) frame_artifacts: Option<FrameArtifactsReceiptEvidence>,
}

pub(crate) struct ManagedServer {
    pub(crate) child: Option<Child>,
    pub(crate) pid_file: PathBuf,
    pub(crate) paper_container: Option<String>,
    pub(crate) keep: bool,
}

impl Drop for ManagedServer {
    fn drop(&mut self) {
        if self.keep {
            return;
        }
        if let Some(mut child) = self.child.take() {
            eprintln!(
                "[mc-compat] stopping managed Valence server process {}",
                child.id()
            );
            let _ = child.kill();
            let _ = child.wait();
            let _ = fs::remove_file(&self.pid_file);
        }
        if let Some(container) = self.paper_container.take() {
            eprintln!("[mc-compat] stopping managed Paper container {container}");
            let _ = Command::new("docker")
                .arg("rm")
                .arg("-f")
                .arg(container)
                .status();
        }
    }
}
