use super::*;

#[derive(Clone, Debug)]
struct SingleClientRun {
    username: String,
    log_path: PathBuf,
    exit_code: Option<i32>,
    output: String,
    matched_success_pattern: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ClientDryRunEvidenceMode {
    Generic,
    McpControlled,
    ProjectileTravelCollision,
    ProjectileDamage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ClientLogPathStrategy {
    EnvClientLogOrTemp,
    TempClientLog,
    ReconnectSessionTempLog,
}

impl ClientLogPathStrategy {
    fn plan_label(self) -> &'static str {
        match self {
            Self::EnvClientLogOrTemp => PLAN_CLIENT_LOG_ENV_OR_TEMP,
            Self::TempClientLog => PLAN_CLIENT_LOG_TEMP,
            Self::ReconnectSessionTempLog => PLAN_CLIENT_LOG_RECONNECT_TEMP,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ClientProcessRunPlan {
    username: String,
    client_index: usize,
    session_number: usize,
    session_count: usize,
    timeout_secs: u64,
    log_path_strategy: ClientLogPathStrategy,
    restart_after_session: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ClientRunPlan {
    run_strategy: ScenarioRunStrategy,
    usernames: Vec<String>,
    sessions: Vec<ClientProcessRunPlan>,
    dry_run_evidence_mode: Option<ClientDryRunEvidenceMode>,
    requires_projectile_server_log: bool,
    requires_server_correlation: bool,
}

#[derive(Debug)]
struct ClientClassificationOutcome {
    combined_output: String,
    evidence: ClientRunEvidence,
}

const CLIENT_LOG_ENV_VAR: &str = "CLIENT_LOG";
const DRY_RUN_CLASSIFICATION: &str = "dry-run";
const CLIENT_EXITED_SUCCESS_CLASSIFICATION: &str = "client-exited-success";
const TIMEOUT_SUCCESS_CLASSIFICATION: &str = "timeout-success-evidence";
const MULTI_CLIENT_LOAD_CLASSIFICATION: &str = "multi-client-load-evidence";
const COMMAND_TIMEOUT_EXIT_CODE: i32 = 124;

pub(crate) fn run_client(cfg: &Config) -> Result<ClientRunEvidence, String> {
    log(format_args!(
        "running Stevenarella headless scenario '{}' isolated from host Wayland compositor",
        scenario_name(cfg.scenario)
    ));

    let plan = client_run_plan_from_config(cfg);
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Stevenarella under xvfb-run"));
        return Ok(dry_run_client_evidence(cfg, &plan));
    }

    if scenario_behavior(cfg.scenario).is_mcp_controlled_smoke() {
        return run_mcp_controlled_live_client(cfg);
    }

    let runs = run_client_sessions(cfg, &plan)?;
    let server_log = cfg.server_backend.runtime().read_log(cfg)?;
    let pre_restart_server_log = if uses_isolated_restart_storage(cfg.scenario) {
        read_world_persistence_pre_restart_server_log(cfg)?
    } else {
        String::new()
    };
    let projectile_server_log = if plan.requires_projectile_server_log {
        Some(read_valence_log(cfg)?)
    } else {
        None
    };
    let outcome = classify_client_runs(
        cfg,
        &plan,
        &runs,
        &server_log,
        &pre_restart_server_log,
        projectile_server_log.as_deref(),
    )?;

    print!("{}", outcome.combined_output);
    io::stdout().flush().map_err(|e| e.to_string())?;
    for run in &runs {
        log(format_args!(
            "client {} finished {:?}; log: {}",
            run.username,
            run.exit_code,
            run.log_path.display()
        ));
    }
    Ok(outcome.evidence)
}

pub(crate) fn client_run_plan_from_config(cfg: &Config) -> ClientRunPlan {
    let behavior = scenario_behavior(cfg.scenario);
    let run_strategy = behavior.run_strategy();
    let usernames = planned_client_usernames_for(run_strategy, &cfg.client_username);
    let session_count = planned_client_session_count_for(run_strategy);
    let log_path_strategy = client_log_path_strategy_for(run_strategy);
    let sessions = client_process_run_plans(
        cfg,
        run_strategy,
        &usernames,
        session_count,
        log_path_strategy,
    );
    ClientRunPlan {
        run_strategy,
        usernames,
        sessions,
        dry_run_evidence_mode: (cfg.mode == Mode::DryRun).then(|| dry_run_evidence_mode_for(cfg)),
        requires_projectile_server_log: behavior.uses_dynamic_projectile_health()
            || cfg.scenario == Scenario::ProjectileHit,
        requires_server_correlation: behavior.requires_server_correlation(),
    }
}

fn client_process_run_plans(
    cfg: &Config,
    run_strategy: ScenarioRunStrategy,
    usernames: &[String],
    session_count: usize,
    log_path_strategy: ClientLogPathStrategy,
) -> Vec<ClientProcessRunPlan> {
    match run_strategy {
        ScenarioRunStrategy::ReconnectSequence => (FIRST_CLIENT_INDEX..session_count)
            .map(|client_index| ClientProcessRunPlan {
                username: cfg.client_username.clone(),
                client_index,
                session_number: client_index + SESSION_INDEX_ENV_OFFSET,
                session_count,
                timeout_secs: client_timeout_secs(cfg, client_index),
                log_path_strategy,
                restart_after_session: uses_isolated_restart_storage(cfg.scenario)
                    && client_index == FIRST_CLIENT_INDEX,
            })
            .collect(),
        ScenarioRunStrategy::MultiClient | ScenarioRunStrategy::SingleClient => usernames
            .iter()
            .enumerate()
            .map(|(client_index, username)| ClientProcessRunPlan {
                username: username.clone(),
                client_index,
                session_number: SESSION_INDEX_ENV_OFFSET,
                session_count,
                timeout_secs: client_timeout_secs(cfg, client_index),
                log_path_strategy,
                restart_after_session: false,
            })
            .collect(),
    }
}

fn dry_run_evidence_mode_for(cfg: &Config) -> ClientDryRunEvidenceMode {
    let behavior = scenario_behavior(cfg.scenario);
    if behavior.is_mcp_controlled_smoke() {
        ClientDryRunEvidenceMode::McpControlled
    } else if cfg.scenario == Scenario::ProjectileHit {
        ClientDryRunEvidenceMode::ProjectileTravelCollision
    } else if behavior.uses_dynamic_projectile_health() {
        ClientDryRunEvidenceMode::ProjectileDamage
    } else {
        ClientDryRunEvidenceMode::Generic
    }
}

fn dry_run_client_evidence(cfg: &Config, plan: &ClientRunPlan) -> ClientRunEvidence {
    match plan
        .dry_run_evidence_mode
        .expect("dry-run plan carries an evidence mode")
    {
        ClientDryRunEvidenceMode::Generic => generic_dry_run_evidence(cfg, plan),
        ClientDryRunEvidenceMode::McpControlled => mcp_controlled_dry_run_evidence(cfg),
        ClientDryRunEvidenceMode::ProjectileTravelCollision => {
            projectile_travel_collision_dry_run_evidence(cfg)
        }
        ClientDryRunEvidenceMode::ProjectileDamage => projectile_damage_dry_run_evidence(cfg),
    }
}

fn generic_dry_run_evidence(cfg: &Config, plan: &ClientRunPlan) -> ClientRunEvidence {
    let scenario = evaluate_scenario_for_config(cfg, "");
    let server_scenario = Some(evaluate_server_scenario(
        cfg.scenario,
        "",
        &cfg.client_username,
    ));
    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: plan.usernames.clone(),
        exit_code: None,
        classification: DRY_RUN_CLASSIFICATION,
        matched_success_pattern: None,
        scenario: Some(scenario),
        server_scenario,
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    }
}

fn classify_client_runs(
    cfg: &Config,
    plan: &ClientRunPlan,
    runs: &[SingleClientRun],
    server_log: &str,
    pre_restart_server_log: &str,
    projectile_server_log: Option<&str>,
) -> Result<ClientClassificationOutcome, String> {
    let combined_output = combined_client_output(cfg.scenario, runs);
    let scenario = evaluate_scenario_for_config(cfg, &combined_output);
    if cfg.scenario != Scenario::Smoke && !scenario.passed {
        return Err(format!(
            "scenario {} failed: missing={:?} forbidden={:?}; logs={}",
            scenario_name(cfg.scenario),
            scenario.missing_milestones,
            scenario.forbidden_matches,
            run_log_labels(runs)
        ));
    }

    let server_scenario = Some(evaluate_server_scenario_from_logs(
        cfg,
        runs,
        server_log,
        pre_restart_server_log,
    ));
    if plan.requires_server_correlation {
        if let Some(server) = &server_scenario {
            if !server.passed {
                return Err(format!(
                    "server correlation for scenario {} failed: missing={:?} forbidden={:?}; log={}",
                    scenario_name(cfg.scenario),
                    server.missing_milestones,
                    server.forbidden_matches,
                    server_log_label(cfg)
                ));
            }
        }
    }

    let projectile_client_logs = projectile_client_log_slices(runs);
    let projectile_damage_causality = classify_projectile_damage_causality(
        cfg,
        runs,
        &projectile_client_logs,
        projectile_server_log,
    )?;
    let projectile_travel_collision = classify_projectile_travel_collision(
        cfg,
        runs,
        &projectile_client_logs,
        projectile_server_log,
    )?;
    let classification = classify_run_exit_state(plan.run_strategy, runs)?;
    let evidence = client_run_evidence_from_parts(
        runs,
        classification,
        matched_success_pattern_for_output(cfg, &combined_output),
        scenario,
        server_scenario,
        projectile_damage_causality,
        projectile_travel_collision,
    );
    validate_typed_event_oracle_for_migrated_scenario(cfg, &evidence)?;
    Ok(ClientClassificationOutcome {
        combined_output,
        evidence,
    })
}

fn combined_client_output(scenario: Scenario, runs: &[SingleClientRun]) -> String {
    let behavior = scenario_behavior(scenario);
    let mut combined_output = String::new();
    behavior.append_client_count_markers(runs.len(), &mut combined_output);
    if behavior.uses_reconnect_session_marker() {
        append_count_marker(&mut combined_output, RECONNECT_SESSION_COUNT_NEEDLE);
    }
    for run in runs {
        combined_output.push_str(&run.output);
        if !combined_output.ends_with('\n') {
            combined_output.push('\n');
        }
    }
    if behavior.uses_crash_recovery_restart() {
        combined_output.push_str(&derive_survival_crash_recovery_client_milestones(
            &combined_output,
        ));
    }
    combined_output
}

fn evaluate_server_scenario_from_logs(
    cfg: &Config,
    runs: &[SingleClientRun],
    server_log: &str,
    pre_restart_server_log: &str,
) -> ServerScenarioEvidence {
    let mut correlation_log = server_log.to_string();
    if uses_isolated_restart_storage(cfg.scenario) {
        correlation_log.push('\n');
        correlation_log.push_str(pre_restart_server_log);
    }
    for run in runs {
        correlation_log.push('\n');
        correlation_log.push_str(&run.output);
    }
    if scenario_behavior(cfg.scenario).uses_crash_recovery_restart() {
        let derived = derive_survival_crash_recovery_server_milestones(&correlation_log);
        correlation_log.push_str(&derived);
    }
    evaluate_server_scenario(cfg.scenario, &correlation_log, &cfg.client_username)
}

fn classify_projectile_damage_causality(
    cfg: &Config,
    runs: &[SingleClientRun],
    projectile_client_logs: &[ClientLogSlice<'_>],
    projectile_server_log: Option<&str>,
) -> Result<Option<ProjectileDamageCausalityEvidence>, String> {
    if !scenario_behavior(cfg.scenario).uses_dynamic_projectile_health() {
        return Ok(None);
    }
    let server_log = projectile_server_log.ok_or_else(|| {
        format!(
            "projectile damage causality missing server log; client_logs={}; server_log={}",
            run_log_labels(runs),
            cfg.valence_log.display()
        )
    })?;
    let expected_damage = projectile_damage_amount_needle(cfg);
    let causality = evaluate_projectile_damage_causality_for_damage(
        projectile_client_logs,
        server_log,
        &cfg.client_username,
        &expected_damage,
    );
    if !causality.passed {
        return Err(format!(
            "projectile damage causality failed: missing={:?} order_violations={:?}; client_logs={}; server_log={}",
            causality.missing_steps,
            causality.order_violations,
            run_log_labels(runs),
            cfg.valence_log.display()
        ));
    }
    Ok(Some(causality))
}

fn classify_projectile_travel_collision(
    cfg: &Config,
    runs: &[SingleClientRun],
    projectile_client_logs: &[ClientLogSlice<'_>],
    projectile_server_log: Option<&str>,
) -> Result<Option<ProjectileTravelCollisionEvidence>, String> {
    if cfg.scenario != Scenario::ProjectileHit {
        return Ok(None);
    }
    let server_log = projectile_server_log.ok_or_else(|| {
        format!(
            "projectile travel/collision rail missing server log; client_logs={}; server_log={}",
            run_log_labels(runs),
            cfg.valence_log.display()
        )
    })?;
    let evidence = evaluate_projectile_travel_collision(
        projectile_client_logs,
        server_log,
        &cfg.client_username,
    );
    if !evidence.passed {
        return Err(format!(
            "projectile travel/collision rail failed: missing={:?} order_violations={:?} identity_violations={:?}; client_logs={}; server_log={}",
            evidence.missing_steps,
            evidence.order_violations,
            evidence.identity_violations,
            run_log_labels(runs),
            cfg.valence_log.display()
        ));
    }
    Ok(Some(evidence))
}

fn classify_run_exit_state(
    run_strategy: ScenarioRunStrategy,
    runs: &[SingleClientRun],
) -> Result<&'static str, String> {
    let all_success = runs.iter().all(|run| run.exit_code == Some(0));
    let timeout_success = runs.iter().all(|run| {
        run.exit_code == Some(COMMAND_TIMEOUT_EXIT_CODE) && run.matched_success_pattern.is_some()
    });
    let mixed_success = runs.iter().all(|run| {
        run.exit_code == Some(0)
            || (run.exit_code == Some(COMMAND_TIMEOUT_EXIT_CODE)
                && run.matched_success_pattern.is_some())
    });
    if run_strategy != ScenarioRunStrategy::SingleClient && mixed_success {
        Ok(MULTI_CLIENT_LOAD_CLASSIFICATION)
    } else if all_success {
        Ok(CLIENT_EXITED_SUCCESS_CLASSIFICATION)
    } else if timeout_success {
        Ok(TIMEOUT_SUCCESS_CLASSIFICATION)
    } else {
        Err(format!(
            "client scenario failed; exits={:?}; logs={}",
            runs.iter().map(|run| run.exit_code).collect::<Vec<_>>(),
            run_log_labels(runs)
        ))
    }
}

fn client_run_evidence_from_parts(
    runs: &[SingleClientRun],
    classification: &'static str,
    matched_success_pattern: Option<String>,
    scenario: ScenarioEvidence,
    server_scenario: Option<ServerScenarioEvidence>,
    projectile_damage_causality: Option<ProjectileDamageCausalityEvidence>,
    projectile_travel_collision: Option<ProjectileTravelCollisionEvidence>,
) -> ClientRunEvidence {
    let log_paths = runs
        .iter()
        .map(|run| run.log_path.clone())
        .collect::<Vec<_>>();
    let usernames = runs
        .iter()
        .map(|run| run.username.clone())
        .collect::<Vec<_>>();
    ClientRunEvidence {
        log_path: log_paths.first().cloned(),
        log_paths,
        usernames,
        exit_code: runs.first().and_then(|run| run.exit_code),
        classification,
        matched_success_pattern,
        scenario: Some(scenario),
        server_scenario,
        projectile_damage_causality,
        projectile_travel_collision,
        mcp_control: None,
        frame_artifacts: None,
    }
}

fn projectile_client_log_slices(runs: &[SingleClientRun]) -> Vec<ClientLogSlice<'_>> {
    runs.iter()
        .map(|run| ClientLogSlice {
            username: &run.username,
            output: &run.output,
        })
        .collect()
}

fn matched_success_pattern_for_output(cfg: &Config, output: &str) -> Option<String> {
    cfg.client_success_needles
        .iter()
        .find(|needle| output.contains(needle.as_str()))
        .cloned()
}

fn run_log_labels(runs: &[SingleClientRun]) -> String {
    runs.iter()
        .map(|run| run.log_path.display().to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn mcp_controlled_dry_run_evidence(cfg: &Config) -> ClientRunEvidence {
    let output = mcp_controlled_success_output();
    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: planned_client_usernames(cfg),
        exit_code: None,
        classification: DRY_RUN_CLASSIFICATION,
        matched_success_pattern: None,
        scenario: Some(evaluate_scenario_for_config(cfg, &output)),
        server_scenario: Some(evaluate_server_scenario(
            cfg.scenario,
            "",
            &cfg.client_username,
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: Some(mcp_control_dry_run_control_evidence()),
        frame_artifacts: Some(evaluate_frame_artifacts_receipt(cfg, None)),
    }
}

pub(crate) fn mcp_controlled_success_output() -> String {
    [
        "mcp_control_dry_run",
        "mcp_initialize",
        "mcp_tools_list",
        "mcp_status_call",
        "mcp_command_outcomes",
    ]
    .join("\n")
        + "\n"
}

pub(crate) fn mcp_control_dry_run_control_evidence() -> McpControlRunEvidence {
    McpControlRunEvidence {
        handshake_success: true,
        tool_list_digest: mcp_control_tool_list_digest(),
        tool_names: MCP_CONTROL_TOOL_NAMES.to_vec(),
        calls_attempted: MCP_CONTROL_REQUIRED_CALLS.to_vec(),
        calls_succeeded: MCP_CONTROL_REQUIRED_CALLS.to_vec(),
        first_failure: None,
        stdout_clean: true,
        command_outcome_ids: MCP_CONTROL_REQUIRED_OUTCOME_IDS.to_vec(),
    }
}

#[derive(Debug)]
struct McpControlledLivePaths {
    stderr_log_path: PathBuf,
    transcript_log_path: PathBuf,
    capture_dir: PathBuf,
}

struct McpJsonRpcSession {
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
    transcript: File,
    stdout_clean: bool,
}

struct KillOnDropChild {
    child: Child,
}

impl Drop for KillOnDropChild {
    fn drop(&mut self) {
        let process_group = format!("-{}", self.child.id());
        let _ = Command::new(MCP_CONTROL_TERMINATE_COMMAND)
            .arg(MCP_CONTROL_TERMINATE_SIGNAL)
            .arg(&process_group)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        thread::sleep(Duration::from_millis(MCP_CONTROL_TERMINATE_GRACE_MILLIS));
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = Command::new(MCP_CONTROL_TERMINATE_COMMAND)
            .arg(MCP_CONTROL_KILL_SIGNAL)
            .arg(&process_group)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

impl McpJsonRpcSession {
    fn request(&mut self, id: &str, request: &str) -> Result<String, String> {
        writeln!(self.transcript, "> {request}")
            .map_err(|err| format!("write transcript: {err}"))?;
        self.stdin
            .write_all(request.as_bytes())
            .map_err(|err| format!("write MCP request {id}: {err}"))?;
        self.stdin
            .write_all(b"\n")
            .map_err(|err| format!("write MCP request newline {id}: {err}"))?;
        self.stdin
            .flush()
            .map_err(|err| format!("flush MCP request {id}: {err}"))?;

        loop {
            let mut line = String::new();
            let bytes = self
                .stdout
                .read_line(&mut line)
                .map_err(|err| format!("read MCP response {id}: {err}"))?;
            if bytes == 0 {
                return Err(format!("MCP response stream closed before id {id}"));
            }
            let trimmed = line.trim_end_matches(['\r', '\n']);
            writeln!(self.transcript, "< {trimmed}")
                .map_err(|err| format!("write transcript: {err}"))?;
            if !mcp_stdout_line_is_clean_jsonrpc(trimmed) {
                self.stdout_clean = false;
                continue;
            }
            if mcp_response_matches_id(trimmed, id) {
                return Ok(trimmed.to_string());
            }
        }
    }
}

fn mcp_controlled_live_paths(cfg: &Config) -> Result<McpControlledLivePaths, String> {
    let (base_dir, stem) = match &cfg.receipt_path {
        Some(receipt_path) => {
            let parent = receipt_path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or_else(|| Path::new("."));
            let stem = receipt_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or(MCP_CONTROLLED_SMOKE_SCENARIO)
                .to_string();
            (parent.to_path_buf(), stem)
        }
        None => (
            cfg.target_dir.join(MCP_CONTROLLED_SMOKE_SCENARIO),
            MCP_CONTROLLED_SMOKE_SCENARIO.to_string(),
        ),
    };
    let base_dir = absolute_child_path(&cfg.root, &base_dir);
    fs::create_dir_all(&base_dir)
        .map_err(|err| format!("create MCP evidence dir {}: {err}", base_dir.display()))?;
    let capture_dir = base_dir.join(format!("{stem}-{MCP_CONTROL_LIVE_CAPTURE_DIR_SUFFIX}"));
    fs::create_dir_all(&capture_dir)
        .map_err(|err| format!("create MCP capture dir {}: {err}", capture_dir.display()))?;
    Ok(McpControlledLivePaths {
        stderr_log_path: base_dir.join(format!("{stem}.{MCP_CONTROL_LIVE_STDERR_LOG_EXTENSION}")),
        transcript_log_path: base_dir
            .join(format!("{stem}.{MCP_CONTROL_LIVE_TRANSCRIPT_EXTENSION}")),
        capture_dir,
    })
}

fn absolute_child_path(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    root.join(path)
}

pub(crate) fn uses_isolated_restart_storage(scenario: Scenario) -> bool {
    scenario_behavior(scenario).uses_isolated_restart_storage()
}

pub(crate) fn world_persistence_artifact_dir_name(scenario: Scenario) -> &'static str {
    scenario_behavior(scenario).world_persistence_artifact_dir_name()
}

pub(crate) fn world_persistence_state_dir(cfg: &Config, backend: ServerBackend) -> PathBuf {
    let backend_name = backend_name(backend);
    cfg.root
        .join("target")
        .join(world_persistence_artifact_dir_name(cfg.scenario))
        .join(backend_name)
}

pub(crate) fn world_persistence_restart_phase_path(cfg: &Config) -> PathBuf {
    let backend_name = backend_name(cfg.server_backend);
    cfg.root
        .join("target")
        .join(format!(
            "{}-pre-restart",
            world_persistence_artifact_dir_name(cfg.scenario)
        ))
        .join(format!("{backend_name}.phase"))
}

pub(crate) fn world_persistence_phase_value(cfg: &Config) -> &'static str {
    if world_persistence_restart_phase_path(cfg).exists() {
        SURVIVAL_WORLD_PERSISTENCE_POST_RESTART_PHASE
    } else {
        SURVIVAL_WORLD_PERSISTENCE_INITIAL_PHASE
    }
}

fn mark_world_persistence_post_restart_phase(cfg: &Config) -> Result<(), String> {
    let path = world_persistence_restart_phase_path(cfg);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    fs::write(&path, SURVIVAL_WORLD_PERSISTENCE_POST_RESTART_PHASE)
        .map_err(|e| format!("write {}: {e}", path.display()))
}

fn run_mcp_controlled_live_client(cfg: &Config) -> Result<ClientRunEvidence, String> {
    let paths = mcp_controlled_live_paths(cfg)?;
    let mut child = KillOnDropChild {
        child: spawn_mcp_controlled_client_process(cfg, &paths)?,
    };
    let stdin = child
        .child
        .stdin
        .take()
        .ok_or_else(|| "MCP client stdin pipe missing".to_string())?;
    let stdout = child
        .child
        .stdout
        .take()
        .ok_or_else(|| "MCP client stdout pipe missing".to_string())?;
    let transcript = File::create(&paths.transcript_log_path).map_err(|err| {
        format!(
            "create MCP transcript {}: {err}",
            paths.transcript_log_path.display()
        )
    })?;
    let mut session = McpJsonRpcSession {
        stdin,
        stdout: BufReader::new(stdout),
        transcript,
        stdout_clean: true,
    };
    let mut control = McpControlRunEvidence {
        handshake_success: false,
        tool_list_digest: mcp_control_tool_list_digest(),
        tool_names: MCP_CONTROL_TOOL_NAMES.to_vec(),
        calls_attempted: Vec::new(),
        calls_succeeded: Vec::new(),
        first_failure: None,
        stdout_clean: true,
        command_outcome_ids: Vec::new(),
    };

    control.calls_attempted.push("initialize");
    let initialize = session
        .request(
            MCP_CONTROL_INITIALIZE_ID,
            &mcp_jsonrpc_request(MCP_CONTROL_INITIALIZE_ID, "initialize", "{}"),
        )
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_HANDSHAKE, err))?;
    if !mcp_response_has_result(&initialize) {
        return Err(mcp_live_failure(
            &mut control,
            MCP_CONTROL_FAILURE_HANDSHAKE,
            initialize,
        ));
    }
    control.handshake_success = true;
    control.calls_succeeded.push("initialize");

    control.calls_attempted.push("tools/list");
    let tools = session
        .request(
            MCP_CONTROL_TOOLS_LIST_ID,
            &mcp_jsonrpc_request(MCP_CONTROL_TOOLS_LIST_ID, "tools/list", "{}"),
        )
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_TOOLS_LIST, err))?;
    if !mcp_tools_list_contains_required_tools(&tools) {
        return Err(mcp_live_failure(
            &mut control,
            MCP_CONTROL_FAILURE_TOOLS_LIST,
            tools,
        ));
    }
    control.calls_succeeded.push("tools/list");

    wait_for_mcp_connected_status(&mut session, &mut control)?;
    run_mcp_control_command(
        &mut session,
        &mut control,
        MCP_CONTROL_LOOK_ID,
        "tools/call look",
        "look.applied",
        r#"{"action":"look","yaw_delta":0.0,"pitch_delta":0.0}"#,
    )?;
    run_mcp_control_command(
        &mut session,
        &mut control,
        MCP_CONTROL_KEY_ID,
        "tools/call key",
        "key.applied",
        r#"{"action":"key","key":"jump","down":false}"#,
    )?;
    run_mcp_control_command(
        &mut session,
        &mut control,
        MCP_CONTROL_CHAT_ID,
        "tools/call chat",
        "chat.applied",
        r#"{"action":"chat","message":"mcp controlled smoke"}"#,
    )?;

    control
        .calls_attempted
        .push("tools/call capture_latest_frame");
    let capture_response = session
        .request(
            MCP_CONTROL_CAPTURE_ID,
            &mcp_capture_latest_frame_request(MCP_CONTROL_CAPTURE_ID),
        )
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_FRAME_CAPTURE, err))?;
    let artifact = mcp_frame_artifact_from_response(&capture_response, &paths.capture_dir)
        .map_err(|err| mcp_live_failure(&mut control, MCP_CONTROL_FAILURE_FRAME_CAPTURE, err))?;
    control
        .calls_succeeded
        .push("tools/call capture_latest_frame");
    control
        .command_outcome_ids
        .push("capture_latest_frame.captured");
    control.stdout_clean = session.stdout_clean;
    if !control.stdout_clean {
        control.first_failure = Some(MCP_CONTROL_FAILURE_HANDSHAKE);
        return Err("MCP stdio stdout was contaminated by non-JSON-RPC output".to_string());
    }

    let output = mcp_controlled_success_output();
    let frame_artifacts = FrameArtifactsReceiptEvidence {
        selected: true,
        capture_requested: true,
        artifact_count: 1,
        artifacts: vec![artifact],
        missing_digests: Vec::new(),
        path_containment_checked: true,
        promotion_ready: paths
            .capture_dir
            .display()
            .to_string()
            .contains("docs/evidence/"),
        non_claims: FRAME_ARTIFACT_NON_CLAIMS.to_vec(),
    };
    Ok(ClientRunEvidence {
        log_path: Some(paths.transcript_log_path.clone()),
        log_paths: vec![paths.transcript_log_path, paths.stderr_log_path],
        usernames: planned_client_usernames(cfg),
        exit_code: None,
        classification: "mcp-controlled-live-evidence",
        matched_success_pattern: Some("mcp_command_outcomes".to_string()),
        scenario: Some(evaluate_scenario_for_config(cfg, &output)),
        server_scenario: Some(evaluate_server_scenario(
            cfg.scenario,
            "",
            &cfg.client_username,
        )),
        projectile_damage_causality: None,
        projectile_travel_collision: None,
        mcp_control: Some(control),
        frame_artifacts: Some(frame_artifacts),
    })
}

fn spawn_mcp_controlled_client_process(
    cfg: &Config,
    paths: &McpControlledLivePaths,
) -> Result<Child, String> {
    let err_file = File::create(&paths.stderr_log_path)
        .map_err(|err| format!("create {}: {err}", paths.stderr_log_path.display()))?;
    let mut cmd = Command::new(MCP_CONTROL_PROCESS_GROUP_COMMAND);
    cmd.arg("timeout")
        .arg(cfg.client_timeout.as_secs().to_string())
        .arg("xvfb-run")
        .arg("-a")
        .arg("-s")
        .arg(XVFB_SERVER_ARGS)
        .arg(cfg.target_dir.join("debug/stevenarella"))
        .arg("--server")
        .arg(format!("127.0.0.1:{}", cfg.server_port))
        .arg("--username")
        .arg(&cfg.client_username)
        .arg("--default-protocol-version")
        .arg(cfg.server_protocol.to_string())
        .arg("--mcp-stdio")
        .arg("--capture-dir")
        .arg(&paths.capture_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::from(err_file));
    apply_build_env(&mut cmd, &cfg.target_dir)?;
    apply_headless_env(&mut cmd)?;
    apply_scenario_probe_env(
        &mut cmd,
        cfg.scenario,
        FIRST_CLIENT_INDEX,
        cfg.server_backend,
    )?;
    cmd.spawn()
        .map_err(|err| format!("run MCP-controlled client {}: {err}", cfg.client_username))
}

fn mcp_live_failure(
    control: &mut McpControlRunEvidence,
    first_failure: &'static str,
    detail: String,
) -> String {
    control.first_failure = Some(first_failure);
    format!("{first_failure}: {detail}")
}

fn mcp_jsonrpc_request(id: &str, method: &str, params_json: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":{method},"params":{params}}}"#,
        id = json_string(id),
        method = json_string(method),
        params = params_json,
    )
}

fn mcp_control_tool_call_request(id: &str, command_json: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"stevenarella.enqueue_control","arguments":{{"command":{command}}}}}}}"#,
        id = json_string(id),
        command = command_json,
    )
}

fn mcp_capture_latest_frame_request(id: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"stevenarella.capture_latest_frame","arguments":{{"output":"artifact","format":"png","relative_path":{relative_path},"include_ui":true}}}}}}"#,
        id = json_string(id),
        relative_path = json_string(MCP_CONTROL_LIVE_CAPTURE_RELATIVE_PATH),
    )
}

fn wait_for_mcp_connected_status(
    session: &mut McpJsonRpcSession,
    control: &mut McpControlRunEvidence,
) -> Result<(), String> {
    control.calls_attempted.push("tools/call status");
    for poll in 0..MCP_CONTROL_MAX_STATUS_POLLS {
        let id = format!("{MCP_CONTROL_STATUS_ID_PREFIX}-{poll}");
        let response = session
            .request(
                &id,
                &mcp_control_tool_call_request(&id, r#"{"action":"status"}"#),
            )
            .map_err(|err| mcp_live_failure(control, MCP_CONTROL_FAILURE_COMMAND, err))?;
        if mcp_control_response_applied(&response) {
            if !control.calls_succeeded.contains(&"tools/call status") {
                control.calls_succeeded.push("tools/call status");
                control.command_outcome_ids.push("status.applied");
            }
            if response.contains(MCP_CONTROL_CONNECTED_TOKEN) {
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(MCP_CONTROL_STATUS_POLL_MILLIS));
    }
    Err(mcp_live_failure(
        control,
        MCP_CONTROL_FAILURE_STATUS_TIMEOUT,
        "status never reported connected=true".to_string(),
    ))
}

fn run_mcp_control_command(
    session: &mut McpJsonRpcSession,
    control: &mut McpControlRunEvidence,
    id: &'static str,
    call_label: &'static str,
    outcome_id: &'static str,
    command_json: &str,
) -> Result<(), String> {
    control.calls_attempted.push(call_label);
    let response = session
        .request(id, &mcp_control_tool_call_request(id, command_json))
        .map_err(|err| mcp_live_failure(control, MCP_CONTROL_FAILURE_COMMAND, err))?;
    if !mcp_control_response_applied(&response) {
        return Err(mcp_live_failure(
            control,
            MCP_CONTROL_FAILURE_COMMAND,
            response,
        ));
    }
    control.calls_succeeded.push(call_label);
    control.command_outcome_ids.push(outcome_id);
    Ok(())
}

fn mcp_stdout_line_is_clean_jsonrpc(line: &str) -> bool {
    line.starts_with('{') && line.contains(MCP_CONTROL_JSONRPC_VERSION_NEEDLE)
}

fn mcp_response_matches_id(line: &str, id: &str) -> bool {
    line.contains(&format!("\"id\":{}", json_string(id)))
}

fn mcp_response_has_result(line: &str) -> bool {
    line.contains(MCP_CONTROL_RESULT_NEEDLE) && !line.contains("\"error\"")
}

fn mcp_tools_list_contains_required_tools(line: &str) -> bool {
    mcp_response_has_result(line)
        && line.contains(MCP_CONTROL_TOOLS_ARRAY_NEEDLE)
        && MCP_CONTROL_TOOL_NAMES
            .iter()
            .all(|tool| line.contains(tool))
}

fn mcp_control_response_applied(line: &str) -> bool {
    mcp_response_has_result(line) && line.contains(MCP_CONTROL_OUTCOME_APPLIED_ESCAPED)
}

fn mcp_frame_artifact_from_response(
    response: &str,
    capture_dir: &Path,
) -> Result<FrameArtifactReceiptItem, String> {
    if !mcp_response_has_result(response) {
        return Err(format!("capture response was not successful: {response}"));
    }
    let metadata = json_string_field(response, "text")?;
    let relative_path = json_string_field(&metadata, "relative_path")?;
    let relative = PathBuf::from(&relative_path);
    if !relative_artifact_path_is_contained(&relative) {
        return Err(format!(
            "capture artifact path escapes capture dir: {relative_path}"
        ));
    }
    let artifact_path = capture_dir.join(&relative);
    let artifact_bytes = fs::read(&artifact_path)
        .map_err(|err| format!("read capture artifact {}: {err}", artifact_path.display()))?;
    let actual_digest = blake3::hash(&artifact_bytes).to_hex().to_string();
    let recorded_digest = json_string_field(&metadata, "blake3_digest")?;
    if recorded_digest != actual_digest {
        return Err(format!(
            "capture artifact digest mismatch for {}: metadata={} actual={}",
            artifact_path.display(),
            recorded_digest,
            actual_digest
        ));
    }
    Ok(FrameArtifactReceiptItem {
        path: artifact_path.display().to_string(),
        relative_path,
        format: json_string_field(&metadata, "format")?,
        width_px: json_u32_field(&metadata, "width_px")?,
        height_px: json_u32_field(&metadata, "height_px")?,
        frame_id: json_u64_field(&metadata, "frame_id")?,
        sequence_id: json_u64_field(&metadata, "sequence_id")?,
        byte_len: json_u64_field(&metadata, "byte_len")?,
        blake3: recorded_digest,
        redaction: json_string_field(&metadata, "redaction")?,
        includes_ui: json_bool_field(&metadata, "includes_ui")?,
    })
}

fn relative_artifact_path_is_contained(path: &Path) -> bool {
    let mut saw_component = false;
    for component in path.components() {
        match component {
            std::path::Component::Normal(name) if !name.is_empty() => saw_component = true,
            _ => return false,
        }
    }
    saw_component
}

pub(crate) fn projectile_travel_collision_dry_run_evidence(cfg: &Config) -> ClientRunEvidence {
    let attacker_username = format!(
        "{}{}",
        cfg.client_username, PROJECTILE_DAMAGE_ATTACKER_SUFFIX
    );
    let target_username = format!("{}{}", cfg.client_username, PROJECTILE_DAMAGE_VICTIM_SUFFIX);
    let attacker_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team RED!\nremote_player_spawn\n{} hand=main {} projectile_id={} weapon={}\n{} projectile_id={} weapon={} proof_basis={}\n{} hand=main projectile_id={}\n{} projectile_id={} proof_basis={}\n",
        cfg.server_protocol,
        PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_TRAVEL_COLLISION_CLIENT_SPAWN_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS,
        PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_CLIENT_TRAVEL_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS
    );
    let target_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team BLUE!\nremote_player_spawn\n",
        cfg.server_protocol
    );
    let server_log = format!(
        "{attacker_username} joined\n{target_username} joined\nMC-COMPAT-MILESTONE projectile_loadout username={attacker_username} slot=0 item=Bow arrows={}\n{} attacker={attacker_username} victim={target_username} hand=Main {} projectile_id={} weapon={} {} policy={} generation={} clamped=false\n{} attacker={attacker_username} target={target_username} {} projectile_id={} weapon={} sample={} sample_index={} proof_basis={}\n{} attacker={attacker_username} target={target_username} {} projectile_id={} weapon={} collision={} proof_basis={}\n{} attacker={attacker_username} victim={target_username} {} projectile_id={} weapon={} {} victim_health_before={:.1} victim_health_after={:.1} policy={} generation={} clamped=false\n",
        PROJECTILE_TRAVEL_COLLISION_LOADOUT_ARROW_COUNT,
        PROJECTILE_DAMAGE_SERVER_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_DAMAGE_AMOUNT_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_POLICY_ID,
        PROJECTILE_TRAVEL_COLLISION_POLICY_GENERATION,
        PROJECTILE_TRAVEL_COLLISION_SERVER_TRAVEL_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_TRAVEL_COLLISION_SAMPLE_KIND,
        PROJECTILE_TRAVEL_COLLISION_SAMPLE_INDEX,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS,
        PROJECTILE_TRAVEL_COLLISION_SERVER_COLLISION_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_TRAVEL_COLLISION_COLLISION_KIND,
        PROJECTILE_TRAVEL_COLLISION_PROOF_BASIS,
        PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_TRAVEL_COLLISION_PROJECTILE_ID,
        PROJECTILE_TRAVEL_COLLISION_WEAPON,
        PROJECTILE_DAMAGE_AMOUNT_NEEDLE,
        PROJECTILE_DAMAGE_VICTIM_START_HEALTH,
        PROJECTILE_TRAVEL_COLLISION_VICTIM_END_HEALTH,
        PROJECTILE_TRAVEL_COLLISION_POLICY_ID,
        PROJECTILE_TRAVEL_COLLISION_POLICY_GENERATION
    );
    let combined_output =
        format!("mc_compat_projectile_hit_client_count=2\n{attacker_log}{target_log}");
    let client_logs = [
        ClientLogSlice {
            username: &attacker_username,
            output: &attacker_log,
        },
        ClientLogSlice {
            username: &target_username,
            output: &target_log,
        },
    ];
    let scenario = evaluate_scenario_for_config(cfg, &combined_output);
    let server_scenario = evaluate_server_scenario(cfg.scenario, &server_log, &cfg.client_username);
    let projectile_travel_collision =
        evaluate_projectile_travel_collision(&client_logs, &server_log, &cfg.client_username);
    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![attacker_username, target_username],
        exit_code: None,
        classification: DRY_RUN_CLASSIFICATION,
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(scenario),
        server_scenario: Some(server_scenario),
        projectile_damage_causality: None,
        projectile_travel_collision: Some(projectile_travel_collision),
        mcp_control: None,
        frame_artifacts: None,
    }
}

pub(crate) fn projectile_damage_dry_run_evidence(cfg: &Config) -> ClientRunEvidence {
    let attacker_username = format!(
        "{}{}",
        cfg.client_username, PROJECTILE_DAMAGE_ATTACKER_SUFFIX
    );
    let victim_username = format!("{}{}", cfg.client_username, PROJECTILE_DAMAGE_VICTIM_SUFFIX);
    let attacker_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team RED!\nremote_player_spawn\n{} hand=main {}\n{} hand=main\n",
        cfg.server_protocol,
        PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE
    );
    let client_health_needle = projectile_damage_client_health_needle(cfg);
    let server_damage_needle = projectile_damage_amount_needle(cfg);
    let server_health_after_needle = projectile_damage_server_health_after_needle(cfg);
    let victim_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team BLUE!\nremote_player_spawn\n{}\n",
        cfg.server_protocol,
        client_health_needle
    );
    let server_log = format!(
        "{attacker_username} joined\n{victim_username} joined\nMC-COMPAT-MILESTONE projectile_loadout username={attacker_username} slot=0 item=Bow arrows=16\n{} attacker={attacker_username} victim={victim_username} hand=Main {} {}\n{} attacker={attacker_username} victim={victim_username} victim_health_before=20.0 {}\n",
        PROJECTILE_DAMAGE_SERVER_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        server_damage_needle,
        PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE,
        server_health_after_needle
    );
    let combined_output =
        format!("mc_compat_projectile_damage_client_count=2\n{attacker_log}{victim_log}");
    let client_logs = [
        ClientLogSlice {
            username: &attacker_username,
            output: &attacker_log,
        },
        ClientLogSlice {
            username: &victim_username,
            output: &victim_log,
        },
    ];
    let scenario = evaluate_scenario_for_config(cfg, &combined_output);
    let server_scenario = evaluate_server_scenario(cfg.scenario, &server_log, &cfg.client_username);
    let projectile_damage_causality = evaluate_projectile_damage_causality_for_damage(
        &client_logs,
        &server_log,
        &cfg.client_username,
        &server_damage_needle,
    );
    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![attacker_username, victim_username],
        exit_code: None,
        classification: DRY_RUN_CLASSIFICATION,
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(scenario),
        server_scenario: Some(server_scenario),
        projectile_damage_causality: Some(projectile_damage_causality),
        projectile_travel_collision: None,
        mcp_control: None,
        frame_artifacts: None,
    }
}

fn run_client_sessions(cfg: &Config, plan: &ClientRunPlan) -> Result<Vec<SingleClientRun>, String> {
    validate_client_run_plan(plan)?;
    match plan.run_strategy {
        ScenarioRunStrategy::ReconnectSequence => run_reconnect_sequence_scenario(cfg, plan),
        ScenarioRunStrategy::MultiClient => run_multi_client_load_scenario(cfg, plan),
        ScenarioRunStrategy::SingleClient => run_single_client_scenario(cfg, plan),
    }
}

fn validate_client_run_plan(plan: &ClientRunPlan) -> Result<(), String> {
    if plan.sessions.is_empty() {
        return Err("client run plan has no sessions".to_string());
    }
    for session in &plan.sessions {
        if session.session_count == 0 {
            return Err(format!(
                "client run plan for {} has zero session count",
                session.username
            ));
        }
        if session.session_number == 0 {
            return Err(format!(
                "client run plan for {} has zero session number",
                session.username
            ));
        }
        if session.timeout_secs == 0 {
            return Err(format!(
                "client run plan for {} has zero timeout",
                session.username
            ));
        }
        let _strategy_label = session.log_path_strategy.plan_label();
    }
    Ok(())
}

fn run_reconnect_sequence_scenario(
    cfg: &Config,
    plan: &ClientRunPlan,
) -> Result<Vec<SingleClientRun>, String> {
    let mut runs = Vec::new();
    let mut restarted_server: Option<ManagedServer> = None;
    for session in &plan.sessions {
        let log_path = client_log_path_for_session(cfg, session);
        let run = run_client_session(cfg, session, &log_path)?;
        runs.push(run);
        if session.restart_after_session {
            restarted_server = Some(run_world_persistence_restart_transition(cfg)?);
        }
        thread::sleep(Duration::from_secs(RECONNECT_SEQUENCE_PAUSE_SECS));
    }
    if restarted_server.is_some() {
        append_world_persistence_post_restart_server_log(cfg)?;
    }
    drop(restarted_server);
    Ok(runs)
}

fn run_world_persistence_restart_transition(cfg: &Config) -> Result<ManagedServer, String> {
    let behavior = scenario_behavior(cfg.scenario);
    write_world_persistence_pre_restart_server_log(cfg)?;
    if behavior.uses_crash_recovery_restart() {
        force_stop_server(cfg)?;
        append_world_persistence_orchestration_milestone(
            cfg,
            SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE,
        )?;
    } else {
        stop_server(cfg)?;
        append_world_persistence_orchestration_milestone(cfg, restart_clean_milestone(behavior))?;
    }
    mark_world_persistence_post_restart_phase(cfg)?;
    let restarted_server = start_server(cfg)?;
    probe_status(cfg)?;
    if behavior.uses_crash_recovery_restart() {
        append_world_persistence_orchestration_milestone(
            cfg,
            SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE,
        )?;
    } else {
        append_world_persistence_orchestration_milestone(cfg, restart_backend_milestone(behavior))?;
    }
    Ok(restarted_server)
}

fn restart_clean_milestone(behavior: &'static dyn ScenarioBehavior) -> &'static str {
    if behavior.uses_block_entity_persistence_storage() {
        SURVIVAL_BLOCK_ENTITY_SERVER_CLEAN_NEEDLE
    } else if behavior.uses_world_multichunk_storage() {
        SURVIVAL_WORLD_MULTICHUNK_SERVER_CLEAN_NEEDLE
    } else {
        SURVIVAL_WORLD_PERSISTENCE_SERVER_CLEAN_NEEDLE
    }
}

fn restart_backend_milestone(behavior: &'static dyn ScenarioBehavior) -> &'static str {
    if behavior.uses_block_entity_persistence_storage() {
        SURVIVAL_BLOCK_ENTITY_SERVER_RESTART_NEEDLE
    } else if behavior.uses_world_multichunk_storage() {
        SURVIVAL_WORLD_MULTICHUNK_SERVER_RESTART_NEEDLE
    } else {
        SURVIVAL_WORLD_PERSISTENCE_SERVER_RESTART_NEEDLE
    }
}

fn run_multi_client_load_scenario(
    cfg: &Config,
    plan: &ClientRunPlan,
) -> Result<Vec<SingleClientRun>, String> {
    let mut children = Vec::new();
    for session in &plan.sessions {
        let log_path = client_log_path_for_session(cfg, session);
        let child = spawn_client_process(
            cfg,
            &session.username,
            session.client_index,
            session.timeout_secs,
            &log_path,
        )?;
        children.push((session.clone(), log_path, child));
        if cfg.scenario != Scenario::CtfSimultaneousPickupCaptureRace {
            thread::sleep(Duration::from_secs(MULTI_CLIENT_START_STAGGER_SECS));
        }
    }
    let mut runs = Vec::new();
    for (session, log_path, mut child) in children {
        let status = child
            .wait()
            .map_err(|e| format!("wait client {}: {e}", session.username))?;
        runs.push(read_completed_client_run(
            cfg,
            &session,
            log_path,
            status.code(),
        )?);
    }
    Ok(runs)
}

fn run_single_client_scenario(
    cfg: &Config,
    plan: &ClientRunPlan,
) -> Result<Vec<SingleClientRun>, String> {
    let session = plan
        .sessions
        .first()
        .ok_or_else(|| "single-client run plan has no session".to_string())?;
    let log_path = client_log_path_for_session(cfg, session);
    Ok(vec![run_client_session(cfg, session, &log_path)?])
}

fn run_client_session(
    cfg: &Config,
    session: &ClientProcessRunPlan,
    log_path: &Path,
) -> Result<SingleClientRun, String> {
    let mut child = spawn_client_process(
        cfg,
        &session.username,
        session.client_index,
        session.timeout_secs,
        log_path,
    )?;
    let scenario = scenario_name(cfg.scenario);
    let status = child.wait().map_err(|e| {
        format!(
            "wait {scenario} client {} session {}: {e}",
            session.username, session.session_number
        )
    })?;
    read_completed_client_run(cfg, session, log_path.to_path_buf(), status.code())
}

fn read_completed_client_run(
    cfg: &Config,
    session: &ClientProcessRunPlan,
    log_path: PathBuf,
    exit_code: Option<i32>,
) -> Result<SingleClientRun, String> {
    let output =
        fs::read_to_string(&log_path).map_err(|e| format!("read {}: {e}", log_path.display()))?;
    let matched_success_pattern = matched_success_pattern_for_output(cfg, &output);
    Ok(SingleClientRun {
        username: session.username.clone(),
        log_path,
        exit_code,
        output,
        matched_success_pattern,
    })
}

fn client_log_path_for_session(cfg: &Config, session: &ClientProcessRunPlan) -> PathBuf {
    match session.log_path_strategy {
        ClientLogPathStrategy::EnvClientLogOrTemp => {
            env_path(CLIENT_LOG_ENV_VAR).unwrap_or_else(|| temp_client_log_for(&session.username))
        }
        ClientLogPathStrategy::TempClientLog => temp_client_log_for(&session.username),
        ClientLogPathStrategy::ReconnectSessionTempLog => reconnect_session_log_path(cfg, session),
    }
}

fn reconnect_session_log_path(cfg: &Config, session: &ClientProcessRunPlan) -> PathBuf {
    std::env::temp_dir().join(format!(
        "mc-compat-client.{}.{}-session-{}.{}.log",
        session.username,
        scenario_name(cfg.scenario),
        session.session_number,
        std::process::id()
    ))
}

pub(crate) fn derive_survival_crash_recovery_client_milestones(output: &str) -> String {
    let mut derived = String::new();
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_MUTATION_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_MUTATION_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_PRE_RESTART_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_PRE_CRASH_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_RECONNECT_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_RECONNECT_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        output,
        SURVIVAL_WORLD_PERSISTENCE_CLIENT_POST_RESTART_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_CLIENT_POST_CRASH_NEEDLE,
    );
    derived
}

pub(crate) fn derive_survival_crash_recovery_server_milestones(log: &str) -> String {
    let mut derived = String::new();
    append_derived_line_if_contains(
        &mut derived,
        log,
        SURVIVAL_WORLD_PERSISTENCE_SERVER_MUTATION_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_SERVER_MUTATION_NEEDLE,
    );
    append_derived_line_if_contains(
        &mut derived,
        log,
        SURVIVAL_WORLD_PERSISTENCE_SERVER_POST_NEEDLE,
        SURVIVAL_CRASH_RECOVERY_SERVER_POST_NEEDLE,
    );
    if log.contains(SURVIVAL_WORLD_PERSISTENCE_SERVER_STATE_NEEDLE)
        && log.contains(SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE)
        && log.contains(SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE)
    {
        append_derived_line(&mut derived, SURVIVAL_CRASH_RECOVERY_SERVER_STATE_NEEDLE);
    }
    derived
}

fn append_derived_line_if_contains(
    output: &mut String,
    haystack: &str,
    source_needle: &str,
    derived_line: &str,
) {
    if haystack.contains(source_needle) {
        append_derived_line(output, derived_line);
    }
}

fn append_derived_line(output: &mut String, line: &str) {
    output.push_str(line);
    output.push('\n');
}

fn spawn_client_process(
    cfg: &Config,
    username: &str,
    client_index: usize,
    timeout_secs: u64,
    log_path: &Path,
) -> Result<Child, String> {
    let log_file =
        File::create(log_path).map_err(|e| format!("create {}: {e}", log_path.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone client log handle: {e}"))?;
    let mut cmd = Command::new("timeout");
    cmd.arg(timeout_secs.to_string())
        .arg("xvfb-run")
        .arg("-a")
        .arg("-s")
        .arg(XVFB_SERVER_ARGS)
        .arg(cfg.target_dir.join("debug/stevenarella"))
        .arg("--server")
        .arg(format!("127.0.0.1:{}", cfg.server_port))
        .arg("--username")
        .arg(username)
        .arg("--default-protocol-version")
        .arg(cfg.server_protocol.to_string())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    apply_build_env(&mut cmd, &cfg.target_dir)?;
    apply_headless_env(&mut cmd)?;
    apply_scenario_probe_env(&mut cmd, cfg.scenario, client_index, cfg.server_backend)?;
    cmd.spawn()
        .map_err(|e| format!("run client {username}: {e}"))
}

pub(crate) fn client_timeout_secs(cfg: &Config, client_index: usize) -> u64 {
    client_timeout_secs_for(cfg.scenario, cfg.client_timeout.as_secs(), client_index)
}

fn client_timeout_secs_for(
    scenario: Scenario,
    configured_timeout_secs: u64,
    client_index: usize,
) -> u64 {
    if scenario == Scenario::MultiClientLoadScore && client_index > FIRST_CLIENT_INDEX {
        configured_timeout_secs.min(MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS)
    } else {
        configured_timeout_secs
    }
}

fn apply_scenario_probe_env(
    cmd: &mut Command,
    scenario: Scenario,
    client_index: usize,
    server_backend: ServerBackend,
) -> Result<(), String> {
    let patch = scenario_behavior(scenario).client_probe_env_patch(client_index, server_backend)?;
    apply_env_patch_to_command(cmd, &patch);
    Ok(())
}

pub(crate) fn planned_client_usernames(cfg: &Config) -> Vec<String> {
    planned_client_usernames_for(
        scenario_behavior(cfg.scenario).run_strategy(),
        &cfg.client_username,
    )
}

fn planned_client_usernames_for(
    run_strategy: ScenarioRunStrategy,
    client_username: &str,
) -> Vec<String> {
    if run_strategy == ScenarioRunStrategy::MultiClient {
        vec![format!("{client_username}a"), format!("{client_username}b")]
    } else {
        vec![client_username.to_string()]
    }
}

fn planned_client_session_count_for(run_strategy: ScenarioRunStrategy) -> usize {
    if run_strategy == ScenarioRunStrategy::ReconnectSequence {
        RECONNECT_SEQUENCE_SESSION_COUNT
    } else {
        SAFETY_SINGLE_SESSION_COUNT
    }
}

fn client_log_path_strategy_for(run_strategy: ScenarioRunStrategy) -> ClientLogPathStrategy {
    match run_strategy {
        ScenarioRunStrategy::ReconnectSequence => ClientLogPathStrategy::ReconnectSessionTempLog,
        ScenarioRunStrategy::MultiClient => ClientLogPathStrategy::TempClientLog,
        ScenarioRunStrategy::SingleClient => ClientLogPathStrategy::EnvClientLogOrTemp,
    }
}

pub(crate) fn server_log_label(cfg: &Config) -> String {
    cfg.server_backend.runtime().log_label(cfg)
}

pub(crate) fn world_persistence_pre_restart_server_log_path(cfg: &Config) -> PathBuf {
    let backend_name = backend_name(cfg.server_backend);
    cfg.root
        .join("target")
        .join(format!(
            "{}-pre-restart",
            world_persistence_artifact_dir_name(cfg.scenario)
        ))
        .join(format!("{backend_name}.log"))
}

fn write_world_persistence_pre_restart_server_log(cfg: &Config) -> Result<(), String> {
    let text = cfg.server_backend.runtime().read_log(cfg)?;
    let path = world_persistence_pre_restart_server_log_path(cfg);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    fs::write(&path, text).map_err(|e| format!("write {}: {e}", path.display()))
}

fn append_world_persistence_orchestration_milestone(
    cfg: &Config,
    milestone: &str,
) -> Result<(), String> {
    append_world_persistence_pre_restart_server_log(
        cfg,
        &format!("MC-COMPAT-MILESTONE {milestone}\n"),
    )
}

fn append_world_persistence_post_restart_server_log(cfg: &Config) -> Result<(), String> {
    let text = cfg.server_backend.runtime().read_log(cfg)?;
    append_world_persistence_pre_restart_server_log(cfg, &text)
}

fn append_world_persistence_pre_restart_server_log(cfg: &Config, text: &str) -> Result<(), String> {
    let path = world_persistence_pre_restart_server_log_path(cfg);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let mut options = fs::OpenOptions::new();
    options.create(true).append(true);
    options
        .open(&path)
        .and_then(|mut file| file.write_all(text.as_bytes()))
        .map_err(|e| format!("append {}: {e}", path.display()))
}

fn read_world_persistence_pre_restart_server_log(cfg: &Config) -> Result<String, String> {
    let path = world_persistence_pre_restart_server_log_path(cfg);
    match fs::read_to_string(&path) {
        Ok(text) => Ok(text),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(format!("read {}: {err}", path.display())),
    }
}

pub(crate) fn read_valence_log(cfg: &Config) -> Result<String, String> {
    match fs::read_to_string(&cfg.valence_log) {
        Ok(text) => Ok(text),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(format!("read {}: {err}", cfg.valence_log.display())),
    }
}

pub(crate) fn read_paper_log(cfg: &Config) -> Result<String, String> {
    if cfg.mode == Mode::DryRun {
        return Ok(String::new());
    }
    let output = Command::new("docker")
        .arg("logs")
        .arg(&cfg.server_name)
        .output()
        .map_err(|e| format!("docker logs {}: {e}", cfg.server_name))?;
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(text)
}

pub(crate) fn requires_server_correlation(cfg: &Config) -> bool {
    scenario_behavior(cfg.scenario).requires_server_correlation()
}

#[cfg(test)]
#[path = "client_driver_colocated_tests.rs"]
mod root_colocated_tests;
