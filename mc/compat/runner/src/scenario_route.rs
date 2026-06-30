use crate::runner_config::{Mode, ScenarioRouteRequest, ServerBackend};
use crate::scenario_core::{parse_scenario, scenario_name, Scenario};
use crate::{backend_name, mode_name};
use std::path::{Component, Path, PathBuf};

pub(crate) const SCENARIO_ROUTER_COMMAND: &str = "scenario";
pub(crate) const SCENARIO_ROUTER_RUN_SUBCOMMAND: &str = "run";
pub(crate) const SCENARIO_ROUTER_BACKEND_FLAG: &str = "--backend";
pub(crate) const SCENARIO_ROUTER_SERVER_BACKEND_FLAG: &str = "--server-backend";
pub(crate) const SCENARIO_ROUTER_RECEIPT_FLAG: &str = "--receipt";
pub(crate) const SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG: &str = "--failure-bundle";
pub(crate) const SCENARIO_ROUTER_TIMEOUT_FLAG: &str = "--timeout";
pub(crate) const SCENARIO_ROUTER_PACKET_CAPTURE_FLAG: &str = "--packet-capture-summary";
pub(crate) const SCENARIO_ROUTER_PROXY_ROUTE_FLAG: &str = "--proxy-route";
pub(crate) const SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG: &str = "--proxy-forwarding-mode";
pub(crate) const SCENARIO_ROUTER_DRY_RUN_FLAG: &str = "--dry-run";
pub(crate) const SCENARIO_ROUTER_RUN_FLAG: &str = "--run";
pub(crate) const SCENARIO_ROUTER_LIVE_FLAG: &str = "--live";
pub(crate) const SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX: &str = "--receipt=";
pub(crate) const SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX: &str = "--failure-bundle=";
pub(crate) const SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX: &str = "--timeout=";
pub(crate) const SCENARIO_ROUTER_BACKEND_EQUALS_PREFIX: &str = "--backend=";
pub(crate) const SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX: &str = "--server-backend=";
pub(crate) const SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX: &str = "--proxy-route=";
pub(crate) const SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX: &str =
    "--proxy-forwarding-mode=";
pub(crate) const SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG: &str = "--scenario";
pub(crate) const SCENARIO_ROUTER_LEGACY_SCENARIO_EQUALS_PREFIX: &str = "--scenario=";
const SCENARIO_ROUTER_SUBCOMMAND_INDEX: usize = 1;
const SCENARIO_ROUTER_SCENARIO_INDEX: usize = 2;
const SCENARIO_ROUTER_OPTION_START_INDEX: usize = 3;
const SCENARIO_ROUTER_MISSING_VALUE: &str = "missing value";
const SCENARIO_ROUTER_NON_CLAIM_BROAD_COMPATIBILITY: &str = "broad_minecraft_compatibility_false";
const SCENARIO_ROUTER_NON_CLAIM_PRODUCTION_READINESS: &str = "production_readiness_false";
const SCENARIO_ROUTER_NON_CLAIM_SEMANTIC_EQUIVALENCE: &str = "semantic_equivalence_false";
pub(crate) const SCENARIO_ROUTER_NON_CLAIMS: &[&str] = &[
    SCENARIO_ROUTER_NON_CLAIM_BROAD_COMPATIBILITY,
    SCENARIO_ROUTER_NON_CLAIM_PRODUCTION_READINESS,
    SCENARIO_ROUTER_NON_CLAIM_SEMANTIC_EQUIVALENCE,
];
const SCENARIO_ROUTER_BLOCKED_COMMAND_FLAGS: &[&str] = &[
    "--run-matrix",
    "--build-client",
    "--status-only",
    "--status",
    "--cleanup",
    "--stop",
    "--compare-receipts",
];
const SCENARIO_ROUTER_BLOCKED_OVERCLAIM_FLAGS: &[&str] = &[
    "--claim-full-compatibility",
    "--claim-production-readiness",
    "--claim-semantic-equivalence",
];

pub(crate) fn parse_scenario_route_request(
    args: &[String],
) -> Result<Option<ScenarioRouteRequest>, String> {
    let Some(command) = args.first() else {
        return Ok(None);
    };
    if command != SCENARIO_ROUTER_COMMAND {
        return Ok(None);
    }
    let subcommand = args
        .get(SCENARIO_ROUTER_SUBCOMMAND_INDEX)
        .ok_or_else(|| scenario_router_usage_error("missing subcommand"))?;
    if subcommand != SCENARIO_ROUTER_RUN_SUBCOMMAND {
        return Err(scenario_router_usage_error(&format!(
            "unknown subcommand: {subcommand}"
        )));
    }
    let scenario_value = args
        .get(SCENARIO_ROUTER_SCENARIO_INDEX)
        .ok_or_else(|| scenario_router_usage_error("missing scenario"))?;
    let scenario = parse_scenario(scenario_value)?;
    parse_scenario_route_options(scenario, &args[SCENARIO_ROUTER_OPTION_START_INDEX..])
}

fn parse_scenario_route_options(
    scenario: Scenario,
    args: &[String],
) -> Result<Option<ScenarioRouteRequest>, String> {
    let mut request = ScenarioRouteRequest {
        scenario,
        backend: ServerBackend::Valence,
        mode: Mode::DryRun,
        receipt_path: None,
        timeout_secs: None,
        packet_capture_summary: false,
        proxy_route: None,
        proxy_forwarding_mode: None,
        failure_bundle_path: None,
        passthrough_args: Vec::new(),
    };
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if let Some(overclaim) = blocked_scenario_route_overclaim_flag(arg) {
            return Err(format!(
                "scenario router rejects overclaiming option {overclaim}; broad compatibility, production readiness, and semantic equivalence claims remain false"
            ));
        }
        if SCENARIO_ROUTER_BLOCKED_COMMAND_FLAGS.contains(&arg.as_str()) {
            return Err(format!(
                "scenario router blocks non-scenario command option {arg}; use mc-compat-runner {SCENARIO_ROUTER_COMMAND} {SCENARIO_ROUTER_RUN_SUBCOMMAND} <scenario>"
            ));
        }
        match arg.as_str() {
            SCENARIO_ROUTER_DRY_RUN_FLAG => request.mode = Mode::DryRun,
            SCENARIO_ROUTER_RUN_FLAG | SCENARIO_ROUTER_LIVE_FLAG => request.mode = Mode::Run,
            SCENARIO_ROUTER_BACKEND_FLAG | SCENARIO_ROUTER_SERVER_BACKEND_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.backend = parse_backend(value)?;
            }
            SCENARIO_ROUTER_RECEIPT_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.receipt_path = Some(parse_scenario_route_receipt_path(value)?);
            }
            SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.failure_bundle_path =
                    Some(parse_scenario_route_failure_bundle_path(value)?);
            }
            SCENARIO_ROUTER_TIMEOUT_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.timeout_secs = Some(parse_client_timeout_secs(
                    value,
                    SCENARIO_ROUTER_TIMEOUT_FLAG,
                )?);
            }
            SCENARIO_ROUTER_PACKET_CAPTURE_FLAG => request.packet_capture_summary = true,
            SCENARIO_ROUTER_PROXY_ROUTE_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.proxy_route = Some(value.to_string());
            }
            SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG => {
                let value = scenario_route_option_value(arg, iter.next())?;
                request.proxy_forwarding_mode = Some(value.to_string());
            }
            SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG => {
                return Err("scenario router takes the scenario as a positional argument; do not also pass --scenario".to_string());
            }
            _ if arg.starts_with(SCENARIO_ROUTER_LEGACY_SCENARIO_EQUALS_PREFIX) => {
                return Err("scenario router takes the scenario as a positional argument; do not also pass --scenario".to_string());
            }
            _ if arg.starts_with(SCENARIO_ROUTER_BACKEND_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_BACKEND_EQUALS_PREFIX.len()..];
                request.backend = parse_backend(value)?;
            }
            _ if arg.starts_with(SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_SERVER_BACKEND_EQUALS_PREFIX.len()..];
                request.backend = parse_backend(value)?;
            }
            _ if arg.starts_with(SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_RECEIPT_EQUALS_PREFIX.len()..];
                request.receipt_path = Some(parse_scenario_route_receipt_path(value)?);
            }
            _ if arg.starts_with(SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_FAILURE_BUNDLE_EQUALS_PREFIX.len()..];
                request.failure_bundle_path =
                    Some(parse_scenario_route_failure_bundle_path(value)?);
            }
            _ if arg.starts_with(SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX) => {
                let value = &arg[SCENARIO_ROUTER_TIMEOUT_EQUALS_PREFIX.len()..];
                request.timeout_secs = Some(parse_client_timeout_secs(
                    value,
                    SCENARIO_ROUTER_TIMEOUT_FLAG,
                )?);
            }
            _ if arg.starts_with(SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX) => {
                request.proxy_route =
                    Some(arg[SCENARIO_ROUTER_PROXY_ROUTE_EQUALS_PREFIX.len()..].to_string());
            }
            _ if arg.starts_with(SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX) => {
                request.proxy_forwarding_mode = Some(
                    arg[SCENARIO_ROUTER_PROXY_FORWARDING_MODE_EQUALS_PREFIX.len()..].to_string(),
                );
            }
            _ => request.passthrough_args.push(arg.clone()),
        }
    }
    Ok(Some(request))
}

fn scenario_route_option_value<'a>(
    flag: &str,
    value: Option<&'a String>,
) -> Result<&'a str, String> {
    value
        .map(String::as_str)
        .ok_or_else(|| format!("{flag} requires a value; got {SCENARIO_ROUTER_MISSING_VALUE}"))
}

fn parse_scenario_route_receipt_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    validate_scenario_route_output_path(&path, "receipt path")?;
    Ok(path)
}

fn parse_scenario_route_failure_bundle_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    validate_scenario_route_output_path(&path, "failure bundle path")?;
    Ok(path)
}

pub(crate) fn validate_scenario_route_output_path(path: &Path, label: &str) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return Err(format!("scenario router {label} is empty"));
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(format!(
            "scenario router {label} contains parent traversal: {}",
            path.display()
        ));
    }
    Ok(())
}

fn blocked_scenario_route_overclaim_flag(arg: &str) -> Option<&'static str> {
    SCENARIO_ROUTER_BLOCKED_OVERCLAIM_FLAGS
        .iter()
        .copied()
        .find(|flag| {
            arg == *flag
                || arg
                    .strip_prefix(*flag)
                    .is_some_and(|rest| rest.starts_with('='))
        })
}

fn scenario_router_usage_error(reason: &str) -> String {
    format!(
        "scenario router usage error: {reason}; expected mc-compat-runner {SCENARIO_ROUTER_COMMAND} {SCENARIO_ROUTER_RUN_SUBCOMMAND} <scenario> [{SCENARIO_ROUTER_DRY_RUN_FLAG}|{SCENARIO_ROUTER_RUN_FLAG}|{SCENARIO_ROUTER_LIVE_FLAG}] [{SCENARIO_ROUTER_BACKEND_FLAG} valence|paper] [{SCENARIO_ROUTER_RECEIPT_FLAG} PATH] [{SCENARIO_ROUTER_TIMEOUT_FLAG} SECS]"
    )
}

pub(crate) fn scenario_route_legacy_args(route: &ScenarioRouteRequest) -> Vec<String> {
    let mut args = Vec::new();
    args.push(format!("--{}", mode_name(route.mode)));
    args.push(SCENARIO_ROUTER_SERVER_BACKEND_FLAG.to_string());
    args.push(backend_name(route.backend).to_string());
    args.push(SCENARIO_ROUTER_LEGACY_SCENARIO_FLAG.to_string());
    args.push(scenario_name(route.scenario).to_string());
    if let Some(path) = &route.receipt_path {
        args.push(SCENARIO_ROUTER_RECEIPT_FLAG.to_string());
        args.push(path.display().to_string());
    }
    if let Some(path) = &route.failure_bundle_path {
        args.push(SCENARIO_ROUTER_FAILURE_BUNDLE_FLAG.to_string());
        args.push(path.display().to_string());
    }
    if let Some(timeout_secs) = route.timeout_secs {
        args.push(SCENARIO_ROUTER_TIMEOUT_FLAG.to_string());
        args.push(timeout_secs.to_string());
    }
    if route.packet_capture_summary {
        args.push(SCENARIO_ROUTER_PACKET_CAPTURE_FLAG.to_string());
    }
    if let Some(proxy_route) = &route.proxy_route {
        args.push(SCENARIO_ROUTER_PROXY_ROUTE_FLAG.to_string());
        args.push(proxy_route.clone());
    }
    if let Some(proxy_forwarding_mode) = &route.proxy_forwarding_mode {
        args.push(SCENARIO_ROUTER_PROXY_FORWARDING_MODE_FLAG.to_string());
        args.push(proxy_forwarding_mode.clone());
    }
    args.extend(route.passthrough_args.clone());
    args
}

pub(crate) fn parse_client_timeout_secs(value: &str, flag: &str) -> Result<u64, String> {
    let timeout_secs = value
        .parse::<u64>()
        .map_err(|err| format!("parse {flag}: {err}"))?;
    if timeout_secs == 0 {
        return Err(format!("{flag} must be greater than zero"));
    }
    Ok(timeout_secs)
}

pub(crate) fn parse_backend(value: &str) -> Result<ServerBackend, String> {
    match value {
        "valence" => Ok(ServerBackend::Valence),
        "paper" => Ok(ServerBackend::Paper),
        other => Err(format!("unknown server backend: {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_route_output_path_accepts_reviewable_relative_paths() {
        validate_scenario_route_output_path(Path::new("target/receipt.json"), "receipt path")
            .expect("relative path without traversal accepted");
    }

    #[test]
    fn scenario_route_output_path_rejects_parent_traversal() {
        let err = validate_scenario_route_output_path(Path::new("../receipt.json"), "receipt path")
            .unwrap_err();
        assert!(err.contains("parent traversal"), "{err}");
    }
}
