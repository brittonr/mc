use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

#[cfg(test)]
use super::default_arrow_damage_policy;
use super::json_support::{
    json_optional_bool_field, json_optional_string_array_field, json_optional_string_field,
    json_optional_u32_field,
};
use super::runtime_config;
use super::{default_port, parse_backend, parse_scenario, Config, Mode, Scenario, ServerBackend};

const FIELD_CLIENT_DIR: &str = "client_dir";
const FIELD_VALENCE_REPO: &str = "valence_repo";
const FIELD_VALENCE_REV: &str = "valence_rev";
const FIELD_VALENCE_WORKTREE: &str = "valence_worktree";
const FIELD_VALENCE_EXAMPLE: &str = "valence_example";
const FIELD_VALENCE_LOG: &str = "valence_log";
const FIELD_VALENCE_TARGET_DIR: &str = "valence_target_dir";
const FIELD_VALENCE_PID_FILE: &str = "valence_pid_file";
const FIELD_SERVER_BACKEND: &str = "server_backend";
const FIELD_TARGET_DIR: &str = "target_dir";
const FIELD_SERVER_NAME: &str = "server_name";
const FIELD_SERVER_VERSION: &str = "server_version";
const FIELD_SERVER_PROTOCOL: &str = "server_protocol";
const FIELD_SERVER_PORT: &str = "server_port";
const FIELD_CLIENT_USERNAME: &str = "client_username";
const FIELD_DOCKER_IMAGE: &str = "docker_image";
const FIELD_PAPER_PLUGIN_JAR: &str = "paper_plugin_jar";
const FIELD_MODE: &str = "mode";
const FIELD_KEEP_SERVER: &str = "keep_server";
const FIELD_CLIENT_TIMEOUT: &str = "client_timeout";
const FIELD_CLIENT_SUCCESS_NEEDLES: &str = "client_success_needles";
const FIELD_SCENARIO: &str = "scenario";
const FIELD_EXPECTED_STATUS_DESCRIPTION: &str = "expected_status_description";
const FIELD_EXPECTED_STATUS_VERSION_NAME: &str = "expected_status_version_name";
const FIELD_EXPECTED_STATUS_SAMPLE: &str = "expected_status_sample";
const FIELD_PACKET_CAPTURE_SUMMARY: &str = "packet_capture_summary";
const FIELD_PROXY_ROUTE: &str = "proxy_route";
const FIELD_PROXY_FORWARDING_MODE: &str = "proxy_forwarding_mode";
const FIELD_RECEIPT_PATH: &str = "receipt_path";
const FIELD_RECEIPT_DIR: &str = "receipt_dir";
const FIELD_FAILURE_BUNDLE_PATH: &str = "failure_bundle_path";
const FIELD_COMPARE_RECEIPTS: &str = "compare_receipts";
const FIELD_CONFIG_PATH: &str = "config_path";
const FIELD_STEEL_CONFIG_PATH: &str = "steel_config_path";
const FIELD_MATRIX_DRY_RUN: &str = "matrix_dry_run";
const FIELD_CLEANUP_APPLY: &str = "cleanup_apply";
const FIELD_NEGATIVE_PUBLIC_TARGET: &str = "negative_public_target";
const FIELD_NEGATIVE_EXTERNAL_AUTHORIZED: &str = "negative_external_authorized";
const FIELD_ARROW_DAMAGE_POLICY: &str = "arrow_damage_policy";

const TIMEOUT_GREATER_THAN_ZERO_MESSAGE: &str = "client timeout must be greater than zero";
const SERVER_PORT_GREATER_THAN_ZERO_MESSAGE: &str = "server port must be greater than zero";
const OUTPUT_PATH_EMPTY_MESSAGE: &str = "output path must not be empty";
const OUTPUT_PATH_PARENT_TRAVERSAL_MESSAGE: &str = "output path must not contain parent traversal";
const RUN_MATRIX_RECEIPT_CONFLICT_MESSAGE: &str =
    "--run-matrix writes backend receipts under --receipt-dir; do not combine it with --receipt/SMOKE_RECEIPT";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ConfigSourceKind {
    Defaults,
    NickelJsonConfig,
    SteelConfig,
    Environment,
    Cli,
    Validation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConfigSource {
    pub(crate) kind: ConfigSourceKind,
    pub(crate) label: String,
}

impl ConfigSource {
    pub(crate) fn defaults() -> Self {
        Self::new(ConfigSourceKind::Defaults, "defaults")
    }

    pub(crate) fn nickel_json(label: impl Into<String>) -> Self {
        Self::new(ConfigSourceKind::NickelJsonConfig, label)
    }

    pub(crate) fn steel(label: impl Into<String>) -> Self {
        Self::new(ConfigSourceKind::SteelConfig, label)
    }

    pub(crate) fn environment() -> Self {
        Self::new(ConfigSourceKind::Environment, "environment")
    }

    pub(crate) fn cli(label: impl Into<String>) -> Self {
        Self::new(ConfigSourceKind::Cli, label)
    }

    fn validation() -> Self {
        Self::new(ConfigSourceKind::Validation, "post-resolution validation")
    }

    fn new(kind: ConfigSourceKind, label: impl Into<String>) -> Self {
        Self {
            kind,
            label: label.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConfigPatch {
    pub(crate) source: ConfigSource,
    pub(crate) client_dir: Option<PathBuf>,
    pub(crate) valence_repo: Option<PathBuf>,
    pub(crate) valence_rev: Option<String>,
    pub(crate) valence_worktree: Option<PathBuf>,
    pub(crate) valence_example: Option<String>,
    pub(crate) valence_log: Option<PathBuf>,
    pub(crate) valence_target_dir: Option<PathBuf>,
    pub(crate) valence_pid_file: Option<PathBuf>,
    pub(crate) server_backend: Option<ServerBackend>,
    pub(crate) target_dir: Option<PathBuf>,
    pub(crate) server_name: Option<String>,
    pub(crate) server_version: Option<String>,
    pub(crate) server_protocol: Option<u32>,
    pub(crate) server_port: Option<u16>,
    pub(crate) client_username: Option<String>,
    pub(crate) docker_image: Option<String>,
    pub(crate) paper_plugin_jar: Option<PathBuf>,
    pub(crate) mode: Option<Mode>,
    pub(crate) keep_server: Option<bool>,
    pub(crate) client_timeout: Option<Duration>,
    pub(crate) client_success_needles: Option<Vec<String>>,
    pub(crate) scenario: Option<Scenario>,
    pub(crate) expected_status_description: Option<String>,
    pub(crate) expected_status_version_name: Option<String>,
    pub(crate) expected_status_sample: Option<Vec<String>>,
    pub(crate) packet_capture_summary: Option<bool>,
    pub(crate) proxy_route: Option<String>,
    pub(crate) proxy_forwarding_mode: Option<String>,
    pub(crate) receipt_path: Option<PathBuf>,
    pub(crate) receipt_dir: Option<PathBuf>,
    pub(crate) failure_bundle_path: Option<PathBuf>,
    pub(crate) compare_receipts: Option<(PathBuf, PathBuf)>,
    pub(crate) config_path: Option<PathBuf>,
    pub(crate) steel_config_path: Option<PathBuf>,
    pub(crate) matrix_dry_run: Option<bool>,
    pub(crate) cleanup_apply: Option<bool>,
    pub(crate) negative_public_target: Option<bool>,
    pub(crate) negative_external_authorized: Option<bool>,
    pub(crate) arrow_damage_policy: Option<runtime_config::ArrowDamagePolicy>,
}

impl ConfigPatch {
    pub(crate) fn new(source: ConfigSource) -> Self {
        Self {
            source,
            client_dir: None,
            valence_repo: None,
            valence_rev: None,
            valence_worktree: None,
            valence_example: None,
            valence_log: None,
            valence_target_dir: None,
            valence_pid_file: None,
            server_backend: None,
            target_dir: None,
            server_name: None,
            server_version: None,
            server_protocol: None,
            server_port: None,
            client_username: None,
            docker_image: None,
            paper_plugin_jar: None,
            mode: None,
            keep_server: None,
            client_timeout: None,
            client_success_needles: None,
            scenario: None,
            expected_status_description: None,
            expected_status_version_name: None,
            expected_status_sample: None,
            packet_capture_summary: None,
            proxy_route: None,
            proxy_forwarding_mode: None,
            receipt_path: None,
            receipt_dir: None,
            failure_bundle_path: None,
            compare_receipts: None,
            config_path: None,
            steel_config_path: None,
            matrix_dry_run: None,
            cleanup_apply: None,
            negative_public_target: None,
            negative_external_authorized: None,
            arrow_damage_policy: None,
        }
    }

    pub(crate) fn has_updates(&self) -> bool {
        self.client_dir.is_some()
            || self.valence_repo.is_some()
            || self.valence_rev.is_some()
            || self.valence_worktree.is_some()
            || self.valence_example.is_some()
            || self.valence_log.is_some()
            || self.valence_target_dir.is_some()
            || self.valence_pid_file.is_some()
            || self.server_backend.is_some()
            || self.target_dir.is_some()
            || self.server_name.is_some()
            || self.server_version.is_some()
            || self.server_protocol.is_some()
            || self.server_port.is_some()
            || self.client_username.is_some()
            || self.docker_image.is_some()
            || self.paper_plugin_jar.is_some()
            || self.mode.is_some()
            || self.keep_server.is_some()
            || self.client_timeout.is_some()
            || self.client_success_needles.is_some()
            || self.scenario.is_some()
            || self.expected_status_description.is_some()
            || self.expected_status_version_name.is_some()
            || self.expected_status_sample.is_some()
            || self.packet_capture_summary.is_some()
            || self.proxy_route.is_some()
            || self.proxy_forwarding_mode.is_some()
            || self.receipt_path.is_some()
            || self.receipt_dir.is_some()
            || self.failure_bundle_path.is_some()
            || self.compare_receipts.is_some()
            || self.config_path.is_some()
            || self.steel_config_path.is_some()
            || self.matrix_dry_run.is_some()
            || self.cleanup_apply.is_some()
            || self.negative_public_target.is_some()
            || self.negative_external_authorized.is_some()
            || self.arrow_damage_policy.is_some()
    }

    pub(crate) fn sets_server_port(&self) -> bool {
        self.server_port.is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConfigValidationDiagnostic {
    pub(crate) field: &'static str,
    pub(crate) source: ConfigSource,
    pub(crate) message: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ConfigResolution {
    pub(crate) config: Config,
    pub(crate) applied_sources: Vec<ConfigSource>,
    pub(crate) field_sources: BTreeMap<&'static str, ConfigSource>,
}

pub(crate) fn config_source_order() -> &'static [ConfigSourceKind] {
    &[
        ConfigSourceKind::Defaults,
        ConfigSourceKind::NickelJsonConfig,
        ConfigSourceKind::SteelConfig,
        ConfigSourceKind::Environment,
        ConfigSourceKind::Cli,
        ConfigSourceKind::Validation,
    ]
}

pub(crate) fn config_json_patch(source: ConfigSource, text: &str) -> Result<ConfigPatch, String> {
    let mut patch = ConfigPatch::new(source);
    if let Some(value) = json_optional_string_field(text, FIELD_CLIENT_DIR)? {
        patch.client_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_VALENCE_REPO)? {
        patch.valence_repo = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_VALENCE_REV)? {
        patch.valence_rev = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_VALENCE_WORKTREE)? {
        patch.valence_worktree = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_VALENCE_EXAMPLE)? {
        patch.valence_example = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_VALENCE_LOG)? {
        patch.valence_log = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_VALENCE_TARGET_DIR)? {
        patch.valence_target_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_VALENCE_PID_FILE)? {
        patch.valence_pid_file = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_SERVER_BACKEND)? {
        patch.server_backend = Some(parse_backend(&value)?);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_TARGET_DIR)? {
        patch.target_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_SERVER_NAME)? {
        patch.server_name = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_SERVER_VERSION)? {
        patch.server_version = Some(value);
    }
    if let Some(value) = json_optional_u32_field(text, FIELD_SERVER_PROTOCOL)? {
        patch.server_protocol = Some(value);
    }
    if let Some(value) = json_optional_u32_field(text, FIELD_SERVER_PORT)? {
        patch.server_port =
            Some(u16::try_from(value).map_err(|_| format!("server_port {value} exceeds u16"))?);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_CLIENT_USERNAME)? {
        patch.client_username = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_DOCKER_IMAGE)? {
        patch.docker_image = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_PAPER_PLUGIN_JAR)? {
        patch.paper_plugin_jar = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_u32_field(text, "client_timeout_secs")? {
        patch.client_timeout = Some(Duration::from_secs(u64::from(value)));
    }
    if let Some(value) = json_optional_string_array_field(text, "client_success_patterns")? {
        patch.client_success_needles = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_SCENARIO)? {
        patch.scenario = Some(parse_scenario(&value)?);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_EXPECTED_STATUS_DESCRIPTION)? {
        patch.expected_status_description = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_EXPECTED_STATUS_VERSION_NAME)? {
        patch.expected_status_version_name = Some(value);
    }
    if let Some(value) = json_optional_string_array_field(text, FIELD_EXPECTED_STATUS_SAMPLE)? {
        patch.expected_status_sample = Some(value);
    }
    if let Some(value) = json_optional_bool_field(text, FIELD_PACKET_CAPTURE_SUMMARY)? {
        patch.packet_capture_summary = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_PROXY_ROUTE)? {
        patch.proxy_route = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_PROXY_FORWARDING_MODE)? {
        patch.proxy_forwarding_mode = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, FIELD_RECEIPT_PATH)? {
        patch.receipt_path = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_RECEIPT_DIR)? {
        patch.receipt_dir = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, FIELD_FAILURE_BUNDLE_PATH)? {
        patch.failure_bundle_path = Some(PathBuf::from(value));
    }
    Ok(patch)
}

pub(crate) fn steel_snapshot_patch(
    source: ConfigSource,
    snapshot: runtime_config::RuntimeConfigSnapshot,
) -> Result<ConfigPatch, String> {
    let mut patch = ConfigPatch::new(source);
    patch.server_backend = Some(parse_backend(&snapshot.server_backend)?);
    patch.server_version = Some(snapshot.server_version);
    patch.server_protocol = Some(snapshot.server_protocol);
    patch.server_port = Some(snapshot.server_port);
    patch.valence_rev = Some(snapshot.valence_rev);
    patch.valence_example = Some(snapshot.valence_example);
    patch.valence_worktree = Some(PathBuf::from(snapshot.valence_worktree));
    patch.valence_target_dir = Some(PathBuf::from(snapshot.valence_target_dir));
    patch.valence_log = Some(PathBuf::from(snapshot.valence_log));
    patch.valence_pid_file = Some(PathBuf::from(snapshot.valence_pid_file));
    patch.client_username = Some(snapshot.client_username);
    patch.client_timeout = Some(Duration::from_secs(u64::from(snapshot.client_timeout_secs)));
    patch.client_success_needles = Some(snapshot.client_success_patterns);
    patch.receipt_dir = Some(PathBuf::from(snapshot.receipt_dir));
    patch.scenario = Some(parse_scenario(&snapshot.scenario)?);
    patch.arrow_damage_policy = Some(snapshot.arrow_damage);
    Ok(patch)
}

pub(crate) fn resolve_config(
    mut config: Config,
    patches: &[ConfigPatch],
) -> Result<ConfigResolution, Vec<ConfigValidationDiagnostic>> {
    let mut field_sources = BTreeMap::new();
    let mut applied_sources = vec![ConfigSource::defaults()];
    let mut server_port_was_set = false;
    for patch in patches {
        if !patch.has_updates() {
            continue;
        }
        server_port_was_set |= patch.sets_server_port();
        apply_patch(&mut config, patch, &mut field_sources);
        applied_sources.push(patch.source.clone());
    }
    if !server_port_was_set {
        config.server_port = default_port(config.server_backend);
        let source = field_sources
            .get(FIELD_SERVER_BACKEND)
            .cloned()
            .unwrap_or_else(ConfigSource::defaults);
        field_sources.insert(FIELD_SERVER_PORT, source);
    }
    let diagnostics = validate_resolved_config(&config, &field_sources);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    applied_sources.push(ConfigSource::validation());
    Ok(ConfigResolution {
        config,
        applied_sources,
        field_sources,
    })
}

pub(crate) fn apply_patch_for_legacy_mutation(config: &mut Config, patch: &ConfigPatch) {
    let mut field_sources = BTreeMap::new();
    let default_port_after_backend_change =
        patch.server_backend.is_some() && patch.server_port.is_none();
    apply_patch(config, patch, &mut field_sources);
    if default_port_after_backend_change {
        config.server_port = default_port(config.server_backend);
    }
}

pub(crate) fn format_validation_diagnostics(diagnostics: &[ConfigValidationDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{} from {}: {}",
                diagnostic.field, diagnostic.source.label, diagnostic.message
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn apply_patch(
    config: &mut Config,
    patch: &ConfigPatch,
    field_sources: &mut BTreeMap<&'static str, ConfigSource>,
) {
    let source = &patch.source;
    if let Some(value) = &patch.client_dir {
        config.client_dir = value.clone();
        record_source(field_sources, FIELD_CLIENT_DIR, source);
    }
    if let Some(value) = &patch.valence_repo {
        config.valence_repo = value.clone();
        record_source(field_sources, FIELD_VALENCE_REPO, source);
    }
    if let Some(value) = &patch.valence_rev {
        config.valence_rev = value.clone();
        record_source(field_sources, FIELD_VALENCE_REV, source);
    }
    if let Some(value) = &patch.valence_worktree {
        config.valence_worktree = value.clone();
        record_source(field_sources, FIELD_VALENCE_WORKTREE, source);
    }
    if let Some(value) = &patch.valence_example {
        config.valence_example = value.clone();
        record_source(field_sources, FIELD_VALENCE_EXAMPLE, source);
    }
    if let Some(value) = &patch.valence_log {
        config.valence_log = value.clone();
        record_source(field_sources, FIELD_VALENCE_LOG, source);
    }
    if let Some(value) = &patch.valence_target_dir {
        config.valence_target_dir = value.clone();
        record_source(field_sources, FIELD_VALENCE_TARGET_DIR, source);
    }
    if let Some(value) = &patch.valence_pid_file {
        config.valence_pid_file = value.clone();
        record_source(field_sources, FIELD_VALENCE_PID_FILE, source);
    }
    if let Some(value) = patch.server_backend {
        config.server_backend = value;
        record_source(field_sources, FIELD_SERVER_BACKEND, source);
    }
    if let Some(value) = &patch.target_dir {
        config.target_dir = value.clone();
        record_source(field_sources, FIELD_TARGET_DIR, source);
    }
    if let Some(value) = &patch.server_name {
        config.server_name = value.clone();
        record_source(field_sources, FIELD_SERVER_NAME, source);
    }
    if let Some(value) = &patch.server_version {
        config.server_version = value.clone();
        record_source(field_sources, FIELD_SERVER_VERSION, source);
    }
    if let Some(value) = patch.server_protocol {
        config.server_protocol = value;
        record_source(field_sources, FIELD_SERVER_PROTOCOL, source);
    }
    if let Some(value) = patch.server_port {
        config.server_port = value;
        record_source(field_sources, FIELD_SERVER_PORT, source);
    }
    if let Some(value) = &patch.client_username {
        config.client_username = value.clone();
        record_source(field_sources, FIELD_CLIENT_USERNAME, source);
    }
    if let Some(value) = &patch.docker_image {
        config.docker_image = value.clone();
        record_source(field_sources, FIELD_DOCKER_IMAGE, source);
    }
    if let Some(value) = &patch.paper_plugin_jar {
        config.paper_plugin_jar = Some(value.clone());
        record_source(field_sources, FIELD_PAPER_PLUGIN_JAR, source);
    }
    if let Some(value) = patch.mode {
        config.mode = value;
        record_source(field_sources, FIELD_MODE, source);
    }
    if let Some(value) = patch.keep_server {
        config.keep_server = value;
        record_source(field_sources, FIELD_KEEP_SERVER, source);
    }
    if let Some(value) = patch.client_timeout {
        config.client_timeout = value;
        record_source(field_sources, FIELD_CLIENT_TIMEOUT, source);
    }
    if let Some(value) = &patch.client_success_needles {
        config.client_success_needles = value.clone();
        record_source(field_sources, FIELD_CLIENT_SUCCESS_NEEDLES, source);
    }
    if let Some(value) = patch.scenario {
        config.scenario = value;
        record_source(field_sources, FIELD_SCENARIO, source);
    }
    if let Some(value) = &patch.expected_status_description {
        config.expected_status_description = Some(value.clone());
        record_source(field_sources, FIELD_EXPECTED_STATUS_DESCRIPTION, source);
    }
    if let Some(value) = &patch.expected_status_version_name {
        config.expected_status_version_name = Some(value.clone());
        record_source(field_sources, FIELD_EXPECTED_STATUS_VERSION_NAME, source);
    }
    if let Some(value) = &patch.expected_status_sample {
        config.expected_status_sample = value.clone();
        record_source(field_sources, FIELD_EXPECTED_STATUS_SAMPLE, source);
    }
    if let Some(value) = patch.packet_capture_summary {
        config.packet_capture_summary = value;
        record_source(field_sources, FIELD_PACKET_CAPTURE_SUMMARY, source);
    }
    if let Some(value) = &patch.proxy_route {
        config.proxy_route = Some(value.clone());
        record_source(field_sources, FIELD_PROXY_ROUTE, source);
    }
    if let Some(value) = &patch.proxy_forwarding_mode {
        config.proxy_forwarding_mode = Some(value.clone());
        record_source(field_sources, FIELD_PROXY_FORWARDING_MODE, source);
    }
    if let Some(value) = &patch.receipt_path {
        config.receipt_path = Some(value.clone());
        record_source(field_sources, FIELD_RECEIPT_PATH, source);
    }
    if let Some(value) = &patch.receipt_dir {
        config.receipt_dir = Some(value.clone());
        record_source(field_sources, FIELD_RECEIPT_DIR, source);
    }
    if let Some(value) = &patch.failure_bundle_path {
        config.failure_bundle_path = Some(value.clone());
        record_source(field_sources, FIELD_FAILURE_BUNDLE_PATH, source);
    }
    if let Some(value) = &patch.compare_receipts {
        config.compare_receipts = Some(value.clone());
        record_source(field_sources, FIELD_COMPARE_RECEIPTS, source);
    }
    if let Some(value) = &patch.config_path {
        config.config_path = Some(value.clone());
        record_source(field_sources, FIELD_CONFIG_PATH, source);
    }
    if let Some(value) = &patch.steel_config_path {
        config.steel_config_path = Some(value.clone());
        record_source(field_sources, FIELD_STEEL_CONFIG_PATH, source);
    }
    if let Some(value) = patch.matrix_dry_run {
        config.matrix_dry_run = value;
        record_source(field_sources, FIELD_MATRIX_DRY_RUN, source);
    }
    if let Some(value) = patch.cleanup_apply {
        config.cleanup_apply = value;
        record_source(field_sources, FIELD_CLEANUP_APPLY, source);
    }
    if let Some(value) = patch.negative_public_target {
        config.negative_public_target = value;
        record_source(field_sources, FIELD_NEGATIVE_PUBLIC_TARGET, source);
    }
    if let Some(value) = patch.negative_external_authorized {
        config.negative_external_authorized = value;
        record_source(field_sources, FIELD_NEGATIVE_EXTERNAL_AUTHORIZED, source);
    }
    if let Some(value) = &patch.arrow_damage_policy {
        config.arrow_damage_policy = value.clone();
        record_source(field_sources, FIELD_ARROW_DAMAGE_POLICY, source);
    }
}

fn record_source(
    field_sources: &mut BTreeMap<&'static str, ConfigSource>,
    field: &'static str,
    source: &ConfigSource,
) {
    field_sources.insert(field, source.clone());
}

fn validate_resolved_config(
    config: &Config,
    field_sources: &BTreeMap<&'static str, ConfigSource>,
) -> Vec<ConfigValidationDiagnostic> {
    let mut diagnostics = Vec::new();
    if config.client_timeout.is_zero() {
        diagnostics.push(validation_diagnostic(
            FIELD_CLIENT_TIMEOUT,
            field_sources,
            TIMEOUT_GREATER_THAN_ZERO_MESSAGE,
        ));
    }
    if config.server_port == 0 {
        diagnostics.push(validation_diagnostic(
            FIELD_SERVER_PORT,
            field_sources,
            SERVER_PORT_GREATER_THAN_ZERO_MESSAGE,
        ));
    }
    push_output_path_diagnostics(
        &mut diagnostics,
        FIELD_RECEIPT_PATH,
        config.receipt_path.as_deref(),
        field_sources,
    );
    push_output_path_diagnostics(
        &mut diagnostics,
        FIELD_RECEIPT_DIR,
        config.receipt_dir.as_deref(),
        field_sources,
    );
    push_output_path_diagnostics(
        &mut diagnostics,
        FIELD_FAILURE_BUNDLE_PATH,
        config.failure_bundle_path.as_deref(),
        field_sources,
    );
    if config.mode == Mode::RunMatrix && config.receipt_path.is_some() {
        diagnostics.push(validation_diagnostic(
            FIELD_RECEIPT_PATH,
            field_sources,
            RUN_MATRIX_RECEIPT_CONFLICT_MESSAGE,
        ));
    }
    diagnostics
}

fn push_output_path_diagnostics(
    diagnostics: &mut Vec<ConfigValidationDiagnostic>,
    field: &'static str,
    path: Option<&Path>,
    field_sources: &BTreeMap<&'static str, ConfigSource>,
) {
    let Some(path) = path else {
        return;
    };
    if path.as_os_str().is_empty() {
        diagnostics.push(validation_diagnostic(
            field,
            field_sources,
            OUTPUT_PATH_EMPTY_MESSAGE,
        ));
        return;
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        diagnostics.push(validation_diagnostic(
            field,
            field_sources,
            OUTPUT_PATH_PARENT_TRAVERSAL_MESSAGE,
        ));
    }
}

fn validation_diagnostic(
    field: &'static str,
    field_sources: &BTreeMap<&'static str, ConfigSource>,
    message: &str,
) -> ConfigValidationDiagnostic {
    let source = field_sources
        .get(field)
        .cloned()
        .unwrap_or_else(ConfigSource::defaults);
    ConfigValidationDiagnostic {
        field,
        source,
        message: message.to_string(),
    }
}

#[cfg(test)]
pub(crate) fn default_patch_for_tests() -> ConfigPatch {
    let mut patch = ConfigPatch::new(ConfigSource::defaults());
    patch.server_backend = Some(ServerBackend::Valence);
    patch.server_port = Some(default_port(ServerBackend::Valence));
    patch.client_timeout = Some(Duration::from_secs(super::DEFAULT_CLIENT_TIMEOUT_SECS));
    patch.scenario = Some(Scenario::Smoke);
    patch.arrow_damage_policy = Some(default_arrow_damage_policy());
    patch
}
