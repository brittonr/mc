use super::*;

pub(crate) fn start_server(cfg: &Config) -> Result<ManagedServer, String> {
    cfg.server_backend.runtime().start(cfg)
}

pub(crate) fn stop_server(cfg: &Config) -> Result<(), String> {
    cfg.server_backend.runtime().stop(cfg)
}

pub(crate) fn force_stop_server(cfg: &Config) -> Result<(), String> {
    cfg.server_backend.runtime().force_stop(cfg)
}

pub(crate) fn force_stop_valence_server(cfg: &Config) -> Result<(), String> {
    if let Ok(pid) = fs::read_to_string(&cfg.valence_pid_file) {
        let pid = pid.trim();
        if !pid.is_empty() {
            log(format_args!(
                "force stopping managed Valence server process {pid}"
            ));
            let _ = Command::new("kill").arg("-9").arg(pid).status();
        }
        fs::remove_file(&cfg.valence_pid_file)
            .map_err(|e| format!("remove {}: {e}", cfg.valence_pid_file.display()))?;
    }
    Ok(())
}

pub(crate) fn force_stop_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "force stopping managed Paper container {}",
        cfg.server_name
    ));
    let mut remove = Command::new("docker");
    remove.arg("rm").arg("-f").arg(&cfg.server_name);
    run_cmd(cfg, &mut remove)
}

pub(crate) fn stop_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "stopping managed Paper container {} with graceful timeout",
        cfg.server_name
    ));
    let mut stop = Command::new("docker");
    stop.arg("stop")
        .arg("--time")
        .arg(PAPER_GRACEFUL_STOP_TIMEOUT_SECS.to_string())
        .arg(&cfg.server_name);
    run_cmd(cfg, &mut stop)?;
    let mut remove = Command::new("docker");
    remove.arg("rm").arg(&cfg.server_name);
    run_cmd(cfg, &mut remove)
}

pub(crate) fn print_harness_status(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "harness status for server '{}'",
        cfg.server_name
    ));
    let docker = docker_container_status(&cfg.server_name)?;
    println!("paper_container={docker}");

    let pid_state = valence_pid_state(&cfg.valence_pid_file)?;
    println!("valence_pid={pid_state}");
    println!(
        "valence_worktree={} exists={}",
        cfg.valence_worktree.display(),
        cfg.valence_worktree.exists()
    );
    println!(
        "valence_target_dir={} exists={}",
        cfg.valence_target_dir.display(),
        cfg.valence_target_dir.exists()
    );
    println!(
        "valence_log={} exists={}",
        cfg.valence_log.display(),
        cfg.valence_log.exists()
    );
    let logs = client_log_paths()?;
    println!("client_logs={}", logs.len());
    for path in logs.iter().take(20) {
        println!("client_log={}", path.display());
    }
    if logs.len() > 20 {
        println!("client_logs_omitted={}", logs.len() - 20);
    }
    Ok(())
}

pub(crate) fn cleanup_harness_state(_cfg: &Config, plan: &CleanupPlan) -> Result<(), String> {
    let apply = plan.apply;
    if apply {
        log(format_args!("cleaning harness-owned state"));
    } else {
        log(format_args!(
            "cleanup dry-run; pass --cleanup --apply to remove harness-owned state"
        ));
    }

    cleanup_paper_container(&plan.paper_container, apply)?;
    cleanup_valence_pid(Path::new(&plan.valence_pid_file), apply)?;
    for action in &plan.path_actions {
        cleanup_path(&action.label, Path::new(&action.path), apply)?;
    }
    for path in client_log_paths()? {
        cleanup_path("client log", &path, apply)?;
    }
    Ok(())
}

pub(crate) fn docker_container_status(name: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--filter")
        .arg(format!("name={name}"))
        .arg("--format")
        .arg("{{.Names}} {{.Status}}")
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() {
                Ok("absent".to_string())
            } else {
                Ok(text)
            }
        }
        Ok(out) => Ok(format!(
            "unavailable: docker ps exited {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        )),
        Err(err) => Ok(format!("unavailable: {err}")),
    }
}

pub(crate) fn cleanup_paper_container(name: &str, apply: bool) -> Result<(), String> {
    let state = docker_container_status(name)?;
    if state == "absent" || state.starts_with("unavailable:") {
        println!("cleanup paper_container {name}: {state}");
        return Ok(());
    }
    if apply {
        log(format_args!("removing Paper container {name}"));
        let status = Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg(name)
            .status()
            .map_err(|e| format!("docker rm -f {name}: {e}"))?;
        if !status.success() {
            return Err(format!("docker rm -f {name} failed with {status}"));
        }
    } else {
        println!("would remove Paper container {name}: {state}");
    }
    Ok(())
}

pub(crate) fn valence_pid_state(pid_file: &Path) -> Result<String, String> {
    let pid = match fs::read_to_string(pid_file) {
        Ok(pid) => pid.trim().to_string(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok("absent".to_string()),
        Err(err) => return Err(format!("read {}: {err}", pid_file.display())),
    };
    if pid.is_empty() {
        return Ok(format!("empty pid file {}", pid_file.display()));
    }
    let alive = Command::new("kill")
        .arg("-0")
        .arg(&pid)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    Ok(format!(
        "pid={} alive={} file={}",
        pid,
        alive,
        pid_file.display()
    ))
}

pub(crate) fn cleanup_valence_pid(pid_file: &Path, apply: bool) -> Result<(), String> {
    let pid = match fs::read_to_string(pid_file) {
        Ok(pid) => pid.trim().to_string(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            println!("cleanup valence_pid_file {}: absent", pid_file.display());
            return Ok(());
        }
        Err(err) => return Err(format!("read {}: {err}", pid_file.display())),
    };
    if !pid.is_empty() {
        if apply {
            log(format_args!("stopping stale Valence process {pid}"));
            let _ = Command::new("kill").arg(&pid).status();
        } else {
            println!("would stop Valence process {pid}");
        }
    }
    if apply {
        fs::remove_file(pid_file).map_err(|e| format!("remove {}: {e}", pid_file.display()))?;
    } else {
        println!("would remove Valence pid file {}", pid_file.display());
    }
    Ok(())
}

pub(crate) fn cleanup_path(label: &str, path: &Path, apply: bool) -> Result<(), String> {
    if !path.exists() {
        println!("cleanup {label} {}: absent", path.display());
        return Ok(());
    }
    if apply {
        log(format_args!("removing {label} {}", path.display()));
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| format!("remove {}: {e}", path.display()))?;
        } else {
            fs::remove_file(path).map_err(|e| format!("remove {}: {e}", path.display()))?;
        }
    } else {
        println!("would remove {label} {}", path.display());
    }
    Ok(())
}

pub(crate) fn client_log_paths() -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    let entries = match fs::read_dir("/tmp") {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(paths),
        Err(err) => return Err(format!("read /tmp: {err}")),
    };
    for entry in entries {
        let entry = entry.map_err(|e| format!("read /tmp entry: {e}"))?;
        let name = entry.file_name();
        if is_mc_compat_client_log(&name.to_string_lossy()) {
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(paths)
}

pub(crate) fn is_mc_compat_client_log(name: &str) -> bool {
    name.starts_with("mc-compat-client.") && name.ends_with(".log")
}

pub(crate) fn prepare_valence_worktree(cfg: &Config) -> Result<(), String> {
    ensure_valence_repo_ready(cfg)?;
    if !cfg.valence_worktree.join(".git").exists() {
        prune_stale_valence_worktrees(cfg)?;
        log(format_args!(
            "creating isolated Valence worktree {} at {}",
            cfg.valence_worktree.display(),
            cfg.valence_rev
        ));
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(&cfg.valence_repo)
            .arg("worktree")
            .arg("add")
            .arg("--detach")
            .arg(&cfg.valence_worktree)
            .arg(&cfg.valence_rev);
        run_cmd(cfg, &mut cmd)?;
    } else {
        ensure_valence_worktree_at_requested_rev(cfg)?;
        log(format_args!(
            "using existing Valence worktree {}",
            cfg.valence_worktree.display()
        ));
    }
    Ok(())
}

pub(crate) fn ensure_valence_worktree_at_requested_rev(cfg: &Config) -> Result<(), String> {
    if cfg.mode == Mode::DryRun {
        return Ok(());
    }
    let current = git_rev_parse(&cfg.valence_worktree, GIT_HEAD_REV)?;
    let requested = git_rev_parse(
        &cfg.valence_repo,
        &format!("{}^{{commit}}", cfg.valence_rev),
    )?;
    if current == requested {
        return Ok(());
    }
    Err(format!(
        "Valence worktree {} is at {current}, but requested {} resolves to {requested}. Remove the stale worktree or pass VALENCE_WORKTREE to a fresh path.",
        cfg.valence_worktree.display(),
        cfg.valence_rev
    ))
}

pub(crate) fn git_rev_parse(repo: &Path, rev: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("rev-parse")
        .arg(rev)
        .output()
        .map_err(|e| format!("git rev-parse {rev} in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git rev-parse {rev} in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_string())
        .map_err(|e| {
            format!(
                "git rev-parse {rev} output in {} was not UTF-8: {e}",
                repo.display()
            )
        })
}

pub(crate) fn git_scoped_latest_commit(repo: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("log")
        .arg("-1")
        .arg(GIT_LOG_COMMIT_FORMAT)
        .arg("--")
        .arg(GIT_CURRENT_DIR_PATHSPEC)
        .output()
        .map_err(|e| format!("git scoped log in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git scoped log in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_string())
        .map_err(|e| {
            format!(
                "git scoped log output in {} was not UTF-8: {e}",
                repo.display()
            )
        })
        .and_then(|commit| {
            if commit.is_empty() {
                Err(format!(
                    "git scoped log in {} did not find a commit for {}",
                    repo.display(),
                    GIT_CURRENT_DIR_PATHSPEC
                ))
            } else {
                Ok(commit)
            }
        })
}

pub(crate) fn git_scoped_worktree_dirty(repo: &Path) -> Result<bool, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("status")
        .arg(GIT_STATUS_PORCELAIN_FLAG)
        .arg("--")
        .arg(GIT_CURRENT_DIR_PATHSPEC)
        .output()
        .map_err(|e| format!("git scoped status in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git scoped status in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| !text.trim().is_empty())
        .map_err(|e| {
            format!(
                "git scoped status output in {} was not UTF-8: {e}",
                repo.display()
            )
        })
}

pub(crate) fn build_git_revision_evidence(
    requested_rev: Option<&str>,
    resolved_rev: Result<String, String>,
    dirty: Result<bool, String>,
) -> GitRevisionEvidence {
    match (resolved_rev, dirty) {
        (Ok(resolved_rev), Ok(dirty)) => GitRevisionEvidence {
            requested_rev: requested_rev.map(str::to_string),
            resolved_rev: Some(resolved_rev),
            status: if dirty {
                GIT_STATUS_DIRTY
            } else {
                GIT_STATUS_CLEAN
            },
            dirty,
            diagnostics: Vec::new(),
        },
        (resolved_rev, dirty) => {
            let mut diagnostics = Vec::new();
            if let Err(err) = resolved_rev {
                diagnostics.push(err);
            }
            if let Err(err) = dirty {
                diagnostics.push(err);
            }
            GitRevisionEvidence {
                requested_rev: requested_rev.map(str::to_string),
                resolved_rev: None,
                status: GIT_STATUS_UNAVAILABLE,
                dirty: true,
                diagnostics,
            }
        }
    }
}

pub(crate) fn git_revision_evidence(
    repo: &Path,
    requested_rev: Option<&str>,
) -> GitRevisionEvidence {
    build_git_revision_evidence(
        requested_rev,
        git_scoped_latest_commit(repo),
        git_scoped_worktree_dirty(repo),
    )
}

pub(crate) fn valence_source_dir(cfg: &Config) -> Result<PathBuf, String> {
    resolve_valence_source_dir(&cfg.valence_worktree)
}

pub(crate) fn valence_revision_dir(cfg: &Config) -> Result<PathBuf, String> {
    if cfg.valence_worktree.exists() {
        valence_source_dir(cfg)
    } else {
        Ok(cfg.valence_repo.clone())
    }
}

pub(crate) fn git_revision_evidence_unavailable(
    requested_rev: Option<&str>,
    diagnostic: String,
) -> GitRevisionEvidence {
    build_git_revision_evidence(
        requested_rev,
        Err(diagnostic),
        Err(GIT_DIRTY_SKIPPED_LAYOUT_DIAGNOSTIC.to_string()),
    )
}

pub(crate) fn child_revision_evidence_for_receipt(cfg: &Config) -> ChildRevisionEvidence {
    if cfg.mode == Mode::DryRun {
        return ChildRevisionEvidence {
            client: GitRevisionEvidence::dry_run(None),
            valence: GitRevisionEvidence::dry_run(Some(cfg.valence_rev.clone())),
        };
    }
    let valence = match valence_revision_dir(cfg) {
        Ok(path) => git_revision_evidence(&path, Some(&cfg.valence_rev)),
        Err(err) => git_revision_evidence_unavailable(Some(&cfg.valence_rev), err),
    };
    ChildRevisionEvidence {
        client: git_revision_evidence(&cfg.client_dir, None),
        valence,
    }
}

pub(crate) fn prune_stale_valence_worktrees(cfg: &Config) -> Result<(), String> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(&cfg.valence_repo)
        .arg("worktree")
        .arg("prune");
    run_cmd(cfg, &mut cmd)
}

pub(crate) fn ensure_valence_repo_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.valence_repo.exists() {
        return Err(format!(
            "Valence source tree not found at {}. Keep the core server tree present at servers/valence or pass --valence-repo/VALENCE_REPO to another checkout.",
            cfg.valence_repo.display()
        ));
    }
    if cfg.mode == Mode::DryRun {
        return Ok(());
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(&cfg.valence_repo)
        .arg("rev-parse")
        .arg("--verify")
        .arg(format!("{}^{{commit}}", cfg.valence_rev))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| {
            format!(
                "check Valence source tree {}: {e}",
                cfg.valence_repo.display()
            )
        })?;

    if !status.success() {
        return Err(format!(
            "Valence source tree {} does not contain compatible revision {}. Fetch the parent repository history or pass --valence-repo/VALENCE_REPO to a checkout that has it.",
            cfg.valence_repo.display(),
            cfg.valence_rev
        ));
    }

    Ok(())
}

pub(crate) fn start_valence_server(cfg: &Config) -> Result<ManagedServer, String> {
    prepare_valence_worktree(cfg)?;
    log(format_args!(
        "starting Valence {} example '{}' on 127.0.0.1:{}; log: {}",
        cfg.valence_rev,
        cfg.valence_example,
        cfg.server_port,
        cfg.valence_log.display()
    ));
    if cfg.mode == Mode::DryRun {
        let source_dir = valence_source_dir(cfg)?;
        log(format_args!(
            "would run Valence example from {}",
            source_dir.display()
        ));
        return Ok(ManagedServer {
            child: None,
            pid_file: cfg.valence_pid_file.clone(),
            paper_container: None,
            keep: true,
        });
    }
    if cfg.server_port != VALENCE_DEFAULT_SERVER_PORT {
        log(format_args!(
            "warning: Valence revision {} defaults to 127.0.0.1:{}; SERVER_PORT={} may only work if the example overrides Config::address",
            cfg.valence_rev, VALENCE_DEFAULT_SERVER_PORT, cfg.server_port
        ));
    }
    stop_valence_server(cfg)?;
    let log_file = File::create(&cfg.valence_log)
        .map_err(|e| format!("create {}: {e}", cfg.valence_log.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone valence log handle: {e}"))?;
    let source_dir = valence_source_dir(cfg)?;
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&source_dir)
        .arg("run")
        .arg("--example")
        .arg(&cfg.valence_example)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    cmd.env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", &cfg.valence_target_dir);
    scenario_behavior(cfg.scenario).apply_valence_server_env(&mut cmd, cfg);
    if let Some(path) = &cfg.steel_config_path {
        cmd.env("MC_COMPAT_STEEL_CONFIG", path);
    }
    let child = cmd.spawn().map_err(|e| format!("spawn Valence: {e}"))?;
    fs::write(&cfg.valence_pid_file, child.id().to_string())
        .map_err(|e| format!("write {}: {e}", cfg.valence_pid_file.display()))?;
    Ok(ManagedServer {
        child: Some(child),
        pid_file: cfg.valence_pid_file.clone(),
        paper_container: None,
        keep: cfg.keep_server,
    })
}

pub(crate) fn start_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "starting Paper {} server on 127.0.0.1:{} via {} with EULA=TRUE",
        cfg.server_version, cfg.server_port, cfg.docker_image
    ));
    if cfg.mode == Mode::DryRun {
        let mut cmd = Command::new("docker");
        configure_paper_run_command(cfg, &mut cmd)?;
        return run_cmd(cfg, &mut cmd);
    }
    let _ = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&cfg.server_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let mut cmd = Command::new("docker");
    configure_paper_run_command(cfg, &mut cmd)?;
    run_cmd(cfg, &mut cmd)
}

pub(crate) fn configure_paper_run_command(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    cmd.arg("run")
        .arg("-d")
        .arg("--name")
        .arg(&cfg.server_name)
        .arg("-p")
        .arg(format!("127.0.0.1:{}:25565", cfg.server_port))
        .arg("-e")
        .arg("EULA=TRUE")
        .arg("-e")
        .arg("TYPE=PAPER")
        .arg("-e")
        .arg(format!("VERSION={}", cfg.server_version))
        .arg("-e")
        .arg("ONLINE_MODE=FALSE")
        .arg("-e")
        .arg("MEMORY=1G")
        .arg("-e")
        .arg(format!("VIEW_DISTANCE={PAPER_VIEW_DISTANCE}"))
        .arg("-e")
        .arg(format!("SIMULATION_DISTANCE={PAPER_SIMULATION_DISTANCE}"));
    scenario_behavior(cfg.scenario).apply_paper_server_env(cmd, cfg)?;
    add_paper_plugin_mount(cfg, cmd)?;
    cmd.arg(&cfg.docker_image);
    Ok(())
}

pub(crate) fn add_paper_plugin_mount(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    let Some(plugin_jar) = &cfg.paper_plugin_jar else {
        return Ok(());
    };
    let absolute_jar = fs::canonicalize(plugin_jar).map_err(|e| {
        format!(
            "canonicalize PAPER_PLUGIN_JAR {}: {e}",
            plugin_jar.display()
        )
    })?;
    let file_name = absolute_jar.file_name().ok_or_else(|| {
        format!(
            "PAPER_PLUGIN_JAR {} has no file name",
            absolute_jar.display()
        )
    })?;
    let container_path = Path::new(PAPER_PLUGIN_CONTAINER_DIR).join(file_name);
    cmd.arg("-v").arg(format!(
        "{}:{}:ro",
        absolute_jar.display(),
        container_path.display()
    ));
    Ok(())
}

pub(crate) fn stop_valence_server(cfg: &Config) -> Result<(), String> {
    if let Ok(pid) = fs::read_to_string(&cfg.valence_pid_file) {
        let pid = pid.trim();
        if !pid.is_empty() {
            log(format_args!(
                "stopping managed Valence server process {pid}"
            ));
            let _ = Command::new("kill").arg(pid).status();
        }
        fs::remove_file(&cfg.valence_pid_file)
            .map_err(|e| format!("remove {}: {e}", cfg.valence_pid_file.display()))?;
    }
    Ok(())
}

pub(crate) fn probe_status(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "probing status 127.0.0.1:{} expecting protocol {}",
        cfg.server_port, cfg.server_protocol
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Rust protocol status probe"));
        return Ok(());
    }
    let mut last = String::new();
    for _ in 0..90 {
        match read_status(cfg.server_port, cfg.server_protocol) {
            Ok(status) => {
                println!("{status}");
                let needle = format!("\"protocol\":{}", cfg.server_protocol);
                let spaced = format!("\"protocol\": {}", cfg.server_protocol);
                if status.contains(&needle) || status.contains(&spaced) {
                    assert_status_expectations(cfg, &status)?;
                    return Ok(());
                }
                return Err(format!(
                    "protocol mismatch in status response; expected {}",
                    cfg.server_protocol
                ));
            }
            Err(err) => last = err,
        }
        thread::sleep(Duration::from_secs(2));
    }
    Err(format!("server status probe failed: {last}"))
}

pub(crate) fn assert_status_expectations(cfg: &Config, status: &str) -> Result<(), String> {
    if let Some(expected) = &cfg.expected_status_description {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected description {expected:?}"
            ));
        }
    }
    if let Some(expected) = &cfg.expected_status_version_name {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected version {expected:?}"
            ));
        }
    }
    for expected in &cfg.expected_status_sample {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected sample {expected:?}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn read_status(port: u16, protocol: u32) -> Result<String, String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(STATUS_SOCKET_TIMEOUT_SECS)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(STATUS_SOCKET_TIMEOUT_SECS)))
        .map_err(|e| e.to_string())?;
    let mut payload = Vec::new();
    payload.write_varint(protocol)?;
    payload.write_mc_string(STATUS_LOCALHOST_ADDRESS)?;
    payload.extend_from_slice(&port.to_be_bytes());
    payload.write_varint(STATUS_HANDSHAKE_NEXT_STATE)?;
    stream.write_packet(STATUS_PACKET_ID, &payload)?;
    stream.write_packet(STATUS_PACKET_ID, &[])?;
    let _packet_len = stream.read_varint()?;
    let packet_id = stream.read_varint()?;
    if packet_id != STATUS_PACKET_ID {
        return Err(format!("unexpected status packet id {packet_id}"));
    }
    stream.read_mc_string()
}

#[cfg(test)]
#[path = "backend_shell_colocated_tests.rs"]
mod root_colocated_tests;
