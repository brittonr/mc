use super::*;

pub(crate) fn harness_plan_from_config(
    cfg: &Config,
) -> Result<HarnessPlan, Vec<PlanningDiagnostic>> {
    let mut diagnostics = Vec::new();
    let receipt = receipt_output_plan_from_config(cfg, &mut diagnostics);
    let artifacts = artifact_collection_plan_from_config(cfg, &mut diagnostics);
    let cleanup = cleanup_plan_from_config(cfg, &mut diagnostics);
    let matrix = matrix_plan_from_config(cfg, &mut diagnostics);
    let scenario_route = scenario_route_plan_from_config(cfg);
    let plan = HarnessPlan {
        server: server_startup_plan_from_config(cfg),
        client_sessions: client_session_plans_from_config(cfg),
        receipt,
        artifacts,
        cleanup,
        matrix,
        scenario_route,
        non_claims: harness_plan_non_claims(),
    };
    if diagnostics.is_empty() {
        Ok(plan)
    } else {
        Err(diagnostics)
    }
}

fn server_startup_plan_from_config(cfg: &Config) -> ServerStartupPlan {
    let (valence_worktree, valence_log, docker_image) = match cfg.server_backend {
        ServerBackend::Valence => (
            Some(cfg.valence_worktree.display().to_string()),
            Some(cfg.valence_log.display().to_string()),
            None,
        ),
        ServerBackend::Paper => (None, None, Some(cfg.docker_image.clone())),
    };
    ServerStartupPlan {
        backend: backend_name(cfg.server_backend).to_string(),
        protocol: cfg.server_protocol,
        port: cfg.server_port,
        server_name: cfg.server_name.clone(),
        keep_server: cfg.keep_server || cfg.mode == Mode::DryRun,
        eula_acceptance_required: cfg.server_backend == ServerBackend::Paper,
        valence_worktree,
        valence_log,
        docker_image,
    }
}

fn client_session_plans_from_config(cfg: &Config) -> Vec<ClientSessionPlan> {
    let usernames = planned_client_usernames(cfg);
    let session_count = planned_client_session_count(cfg);
    usernames
        .into_iter()
        .enumerate()
        .map(|(index, username)| ClientSessionPlan {
            index,
            username,
            timeout_secs: client_timeout_secs(cfg, index),
            scenario: scenario_name(cfg.scenario).to_string(),
            session_count,
            log_path_strategy: client_log_path_strategy(cfg),
        })
        .collect()
}

fn planned_client_session_count(cfg: &Config) -> usize {
    if scenario_behavior(cfg.scenario).run_strategy() == ScenarioRunStrategy::ReconnectSequence {
        RECONNECT_SEQUENCE_SESSION_COUNT
    } else {
        1
    }
}

fn client_log_path_strategy(cfg: &Config) -> String {
    match scenario_behavior(cfg.scenario).run_strategy() {
        ScenarioRunStrategy::ReconnectSequence => PLAN_CLIENT_LOG_RECONNECT_TEMP.to_string(),
        ScenarioRunStrategy::MultiClient => PLAN_CLIENT_LOG_TEMP.to_string(),
        ScenarioRunStrategy::SingleClient => PLAN_CLIENT_LOG_ENV_OR_TEMP.to_string(),
    }
}

fn receipt_output_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> ReceiptOutputPlan {
    let receipt_path = cfg.receipt_path.as_ref().map(|path| display_path(path));
    let receipt_dir = cfg.receipt_dir.as_ref().map(|path| display_path(path));
    let failure_bundle_path = cfg
        .failure_bundle_path
        .as_ref()
        .map(|path| display_path(path));
    if cfg.failure_bundle_path.is_some() && cfg.receipt_path.is_none() {
        push_plan_diagnostic(
            diagnostics,
            "failure_bundle_path",
            "failure bundle planning requires a receipt path for reviewable artifact identity",
        );
    }
    if let Some(path) = &cfg.failure_bundle_path {
        validate_reviewable_plan_path(
            diagnostics,
            "failure_bundle_path",
            &cfg.root,
            path,
            "failure bundle output",
        );
    }
    ReceiptOutputPlan {
        receipt_path,
        receipt_dir,
        failure_bundle_path,
        schema: SCENARIO_RECEIPT_SCHEMA.to_string(),
    }
}

fn artifact_collection_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> ArtifactCollectionPlan {
    let typed_event_log_path = cfg
        .receipt_path
        .as_ref()
        .map(|path| display_path(&typed_event_log_path_for_receipt(path)));
    let failure_bundle_path = cfg
        .failure_bundle_path
        .as_ref()
        .map(|path| display_path(path));
    let mut failure_artifact_candidates = Vec::new();
    for (kind, path) in failure_bundle_artifact_candidates(cfg) {
        if cfg.failure_bundle_path.is_some() && kind != FAILURE_BUNDLE_ARTIFACT_SERVER_LOG {
            validate_reviewable_plan_path(
                diagnostics,
                "failure_artifact_candidate",
                &cfg.root,
                &path,
                kind,
            );
        }
        failure_artifact_candidates.push(ArtifactCandidatePlan {
            kind: kind.to_string(),
            path: display_path(&path),
        });
    }
    ArtifactCollectionPlan {
        typed_event_log_path,
        failure_bundle_path,
        failure_artifact_candidates,
    }
}

fn cleanup_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> CleanupPlan {
    if cfg.mode == Mode::Cleanup {
        validate_cleanup_plan_path(
            diagnostics,
            &cfg.root,
            "valence pid file",
            &cfg.valence_pid_file,
        );
        validate_cleanup_plan_path(
            diagnostics,
            &cfg.root,
            "valence target dir",
            &cfg.valence_target_dir,
        );
        validate_cleanup_plan_path(diagnostics, &cfg.root, "valence log", &cfg.valence_log);
    }
    CleanupPlan {
        apply: cfg.cleanup_apply,
        paper_container: cfg.server_name.clone(),
        valence_pid_file: display_path(&cfg.valence_pid_file),
        path_actions: vec![
            CleanupPathPlan {
                label: "valence target dir".to_string(),
                path: display_path(&cfg.valence_target_dir),
            },
            CleanupPathPlan {
                label: "valence log".to_string(),
                path: display_path(&cfg.valence_log),
            },
        ],
        client_log_discovery: PLAN_CLEANUP_CLIENT_LOG_DISCOVERY.to_string(),
    }
}

fn matrix_plan_from_config(
    cfg: &Config,
    diagnostics: &mut Vec<PlanningDiagnostic>,
) -> Option<MatrixPlan> {
    if cfg.mode != Mode::RunMatrix {
        return None;
    }
    if cfg.receipt_path.is_some() {
        push_plan_diagnostic(
            diagnostics,
            "receipt_path",
            "run-matrix planning writes backend receipts under receipt_dir and rejects a single receipt path",
        );
    }
    let receipt_dir = cfg
        .receipt_dir
        .clone()
        .unwrap_or_else(|| cfg.root.join(DEFAULT_MATRIX_RECEIPT_DIR));
    let paper_receipt = receipt_dir.join("paper.json");
    let valence_receipt = receipt_dir.join("valence.json");
    Some(MatrixPlan {
        dry_run: cfg.matrix_dry_run,
        matrix_mode: if cfg.matrix_dry_run { "dry-run" } else { "run" }.to_string(),
        receipt_dir: display_path(&receipt_dir),
        paper_receipt: display_path(&paper_receipt),
        valence_receipt: display_path(&valence_receipt),
    })
}

fn scenario_route_plan_from_config(cfg: &Config) -> Option<ScenarioRoutePlan> {
    cfg.scenario_route.as_ref()?;
    Some(ScenarioRoutePlan {
        scenario: scenario_name(cfg.scenario).to_string(),
        backend: backend_name(cfg.server_backend).to_string(),
        mode: mode_name(cfg.mode).to_string(),
        receipt_path: cfg.receipt_path.as_ref().map(|path| display_path(path)),
        timeout_secs: cfg.client_timeout.as_secs(),
        packet_capture_summary: cfg.packet_capture_summary,
        proxy_route: cfg.proxy_route.clone(),
        proxy_forwarding_mode: cfg.proxy_forwarding_mode.clone(),
        failure_bundle_path: cfg
            .failure_bundle_path
            .as_ref()
            .map(|path| display_path(path)),
        non_claims: scenario_route_non_claims(),
    })
}

pub(crate) fn scenario_route_non_claims() -> Vec<String> {
    SCENARIO_ROUTER_NON_CLAIMS
        .iter()
        .map(|claim| (*claim).to_string())
        .collect()
}

fn harness_plan_non_claims() -> Vec<String> {
    let mut non_claims = vec![PLAN_NON_CLAIM_ARCHITECTURE_ONLY.to_string()];
    non_claims.extend(scenario_route_non_claims());
    non_claims
}

fn validate_reviewable_plan_path(
    diagnostics: &mut Vec<PlanningDiagnostic>,
    field: &str,
    root: &Path,
    path: &Path,
    label: &str,
) {
    let Some(review_path) = plan_reviewable_path(root, path) else {
        push_plan_diagnostic(
            diagnostics,
            field,
            &format!("{label} path must be under docs/evidence for review"),
        );
        return;
    };
    if let Err(err) = validate_failure_bundle_artifact_path(&review_path) {
        push_plan_diagnostic(diagnostics, field, &err);
    }
}

fn plan_reviewable_path(root: &Path, path: &Path) -> Option<String> {
    if path.is_absolute() {
        let relative = path.strip_prefix(root).ok()?;
        return path_to_forward_slashes(relative);
    }
    path_to_forward_slashes(path)
}

fn validate_cleanup_plan_path(
    diagnostics: &mut Vec<PlanningDiagnostic>,
    root: &Path,
    label: &str,
    path: &Path,
) {
    if path.as_os_str().is_empty() {
        push_plan_diagnostic(diagnostics, "cleanup", &format!("{label} path is empty"));
        return;
    }
    if path == Path::new(CLEANUP_ROOT_PATH)
        || cleanup_component_count(path) < CLEANUP_MIN_SAFE_COMPONENTS
    {
        push_plan_diagnostic(
            diagnostics,
            "cleanup",
            &format!("{label} path is too broad for cleanup: {}", path.display()),
        );
        return;
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        push_plan_diagnostic(
            diagnostics,
            "cleanup",
            &format!("{label} path contains parent traversal: {}", path.display()),
        );
        return;
    }
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    let target_root = root.join("target");
    if !absolute_path.starts_with(Path::new(HARNESS_TEMP_ROOT))
        && !absolute_path.starts_with(target_root)
    {
        push_plan_diagnostic(
            diagnostics,
            "cleanup",
            &format!(
                "{label} path is outside harness-owned cleanup roots: {}",
                path.display()
            ),
        );
    }
}

fn cleanup_component_count(path: &Path) -> usize {
    path.components()
        .filter(|component| matches!(component, std::path::Component::Normal(_)))
        .count()
}

fn push_plan_diagnostic(diagnostics: &mut Vec<PlanningDiagnostic>, field: &str, message: &str) {
    diagnostics.push(PlanningDiagnostic {
        field: field.to_string(),
        message: message.to_string(),
    });
}

pub(crate) fn format_plan_diagnostics(diagnostics: Vec<PlanningDiagnostic>) -> String {
    let rendered = diagnostics
        .into_iter()
        .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
        .collect::<Vec<_>>();
    format!("harness planning failed: {}", rendered.join("; "))
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}

pub(crate) fn log_harness_plan(plan: &HarnessPlan) {
    log(format_args!(
        "plan: build client, start {} server, wait for protocol {}, run {} client session plan(s) under isolated Xvfb/X11",
        plan_backend_display_name(&plan.server.backend),
        plan.server.protocol,
        plan.client_sessions.len()
    ));
    if let Some(route) = &plan.scenario_route {
        log(format_args!(
            "typed scenario route: scenario '{}' backend {} mode {} receipt {} timeout {}s packet_capture_summary={} proxy_route={} proxy_forwarding_mode={} non_claims={}",
            route.scenario,
            route.backend,
            route.mode,
            route
                .receipt_path
                .as_deref()
                .unwrap_or("<unset>"),
            route.timeout_secs,
            route.packet_capture_summary,
            route.proxy_route.as_deref().unwrap_or("<unset>"),
            route
                .proxy_forwarding_mode
                .as_deref()
                .unwrap_or("<unset>"),
            route.non_claims.join(",")
        ));
    }
}

fn plan_backend_display_name(backend: &str) -> &str {
    match backend {
        "paper" => "Paper",
        "valence" => "Valence",
        _ => backend,
    }
}

#[cfg(test)]
#[path = "planning_colocated_tests.rs"]
mod root_colocated_tests;
