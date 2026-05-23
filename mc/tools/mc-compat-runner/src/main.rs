use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_VALENCE_REV: &str = "c86b828^";
const DEFAULT_VALENCE_EXAMPLE: &str = "terrain";
const DEFAULT_SERVER_VERSION: &str = "1.18.2";
const DEFAULT_SERVER_PROTOCOL: u32 = 758;
const DEFAULT_CLIENT_USERNAME: &str = "compatbot";
const DEFAULT_CLIENT_TIMEOUT_SECS: u64 = 20;
const DEFAULT_SUCCESS_PATTERN: &[&str] = &[
    "Detected server protocol version",
    "Dimension type:",
    "Received chat message",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    DryRun,
    Run,
    BuildClient,
    StatusOnly,
    Stop,
    CompareReceipts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ServerBackend {
    Valence,
    Paper,
}

#[derive(Debug)]
struct Config {
    root: PathBuf,
    client_dir: PathBuf,
    valence_repo: PathBuf,
    valence_rev: String,
    valence_worktree: PathBuf,
    valence_example: String,
    valence_log: PathBuf,
    valence_target_dir: PathBuf,
    valence_pid_file: PathBuf,
    server_backend: ServerBackend,
    target_dir: PathBuf,
    server_name: String,
    server_version: String,
    server_protocol: u32,
    server_port: u16,
    client_username: String,
    docker_image: String,
    mode: Mode,
    keep_server: bool,
    client_timeout: Duration,
    client_success_needles: Vec<String>,
    receipt_path: Option<PathBuf>,
    compare_receipts: Option<(PathBuf, PathBuf)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClientRunEvidence {
    log_path: Option<PathBuf>,
    exit_code: Option<i32>,
    classification: &'static str,
    matched_success_pattern: Option<String>,
}

struct ManagedServer {
    child: Option<Child>,
    pid_file: PathBuf,
    keep: bool,
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
    }
}

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[mc-compat] error: {err}");
            ExitCode::from(1)
        }
    }
}

fn real_main() -> Result<(), String> {
    let cfg = Config::from_env_and_args()?;
    let result = execute(&cfg);
    if cfg.receipt_path.is_some() {
        if let Err(receipt_err) = write_smoke_receipt(&cfg, result.as_ref()) {
            return match result {
                Ok(_) => Err(receipt_err),
                Err(err) => Err(format!(
                    "{err}; additionally failed to write receipt: {receipt_err}"
                )),
            };
        }
    }
    result.map(|_| ())
}

fn execute(cfg: &Config) -> Result<Option<ClientRunEvidence>, String> {
    if matches!(cfg.mode, Mode::DryRun | Mode::Run | Mode::BuildClient) {
        ensure_client_dir_ready(cfg)?;
    }
    if cfg.server_backend == ServerBackend::Valence && matches!(cfg.mode, Mode::DryRun | Mode::Run)
    {
        ensure_valence_repo_ready(cfg)?;
    }
    match cfg.mode {
        Mode::DryRun => {
            log(format_args!(
                "plan: build client, start {:?} server, wait for protocol {}, run client under isolated Xvfb/X11",
                cfg.server_backend, cfg.server_protocol
            ));
            build_client(&cfg)?;
            if cfg.server_backend == ServerBackend::Paper {
                log(format_args!(
                    "server start will set EULA=TRUE using recorded user acceptance"
                ));
            }
            let _server = start_server(cfg)?;
            probe_status(cfg)?;
            let client = run_client(cfg)?;
            Ok(Some(client))
        }
        Mode::BuildClient => {
            build_client(cfg)?;
            Ok(None)
        }
        Mode::StatusOnly => {
            probe_status(cfg)?;
            Ok(None)
        }
        Mode::Stop => {
            stop_server(cfg)?;
            Ok(None)
        }
        Mode::CompareReceipts => {
            compare_receipts(cfg)?;
            Ok(None)
        }
        Mode::Run => {
            build_client(cfg)?;
            let _server = start_server(cfg)?;
            probe_status(cfg)?;
            let client = run_client(cfg)?;
            Ok(Some(client))
        }
    }
}

impl Config {
    fn from_env_and_args() -> Result<Self, String> {
        Self::from_sources(
            env::current_dir().map_err(|e| format!("current dir: {e}"))?,
            |name| env::var(name).ok().filter(|s| !s.is_empty()),
            env::args().skip(1),
        )
    }

    fn from_sources<I, F>(current_dir: PathBuf, mut get_env: F, args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
        F: FnMut(&str) -> Option<String>,
    {
        let root = get_env("MC_COMPAT_ROOT")
            .or_else(|| get_env("ROOT"))
            .map(PathBuf::from)
            .unwrap_or(current_dir);
        let mut cfg = Config {
            client_dir: get_env("CLIENT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("stevenarella")),
            valence_repo: get_env("VALENCE_REPO")
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("valence")),
            valence_rev: get_env("VALENCE_REV").unwrap_or_else(|| DEFAULT_VALENCE_REV.to_string()),
            valence_worktree: get_env("VALENCE_WORKTREE")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp/valence-compat-758")),
            valence_example: get_env("VALENCE_EXAMPLE")
                .unwrap_or_else(|| DEFAULT_VALENCE_EXAMPLE.to_string()),
            valence_log: get_env("VALENCE_LOG")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp/mc-compat-valence.log")),
            valence_target_dir: get_env("VALENCE_TARGET_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp/valence-compat-758-target")),
            valence_pid_file: get_env("VALENCE_PID_FILE")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp/mc-compat-valence.pid")),
            server_backend: parse_backend(
                &get_env("SERVER_BACKEND").unwrap_or_else(|| "valence".to_string()),
            )?,
            target_dir: get_env("TARGET_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp/stevenarella-target2")),
            server_name: get_env("SERVER_NAME").unwrap_or_else(|| "mc-compat-1-18-2".to_string()),
            server_version: get_env("SERVER_VERSION")
                .unwrap_or_else(|| DEFAULT_SERVER_VERSION.to_string()),
            server_protocol: get_env("SERVER_PROTOCOL")
                .and_then(|s| s.parse().ok())
                .unwrap_or(DEFAULT_SERVER_PROTOCOL),
            server_port: 0,
            client_username: get_env("CLIENT_USERNAME")
                .unwrap_or_else(|| DEFAULT_CLIENT_USERNAME.to_string()),
            docker_image: get_env("DOCKER_IMAGE")
                .unwrap_or_else(|| "itzg/minecraft-server:java17".to_string()),
            mode: Mode::DryRun,
            keep_server: false,
            client_timeout: Duration::from_secs(
                get_env("CLIENT_TIMEOUT")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(DEFAULT_CLIENT_TIMEOUT_SECS),
            ),
            client_success_needles: get_env("CLIENT_SUCCESS_PATTERN")
                .map(|s| s.split('|').map(str::to_string).collect())
                .unwrap_or_else(|| {
                    DEFAULT_SUCCESS_PATTERN
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                }),
            receipt_path: get_env("SMOKE_RECEIPT").map(PathBuf::from),
            compare_receipts: None,
            root,
        };

        let server_port_was_set = get_env("SERVER_PORT").is_some();
        cfg.server_port = get_env("SERVER_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(match cfg.server_backend {
                ServerBackend::Valence => 25565,
                ServerBackend::Paper => 25566,
            });

        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--dry-run" => cfg.mode = Mode::DryRun,
                "--run" => cfg.mode = Mode::Run,
                "--build-client" => cfg.mode = Mode::BuildClient,
                "--status-only" => cfg.mode = Mode::StatusOnly,
                "--stop" => cfg.mode = Mode::Stop,
                "--compare-receipts" => {
                    let left = PathBuf::from(args.next().ok_or_else(|| {
                        "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT".to_string()
                    })?);
                    let right = PathBuf::from(args.next().ok_or_else(|| {
                        "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT".to_string()
                    })?);
                    cfg.mode = Mode::CompareReceipts;
                    cfg.compare_receipts = Some((left, right));
                }
                "--accept-eula" => {}
                "--keep-server" => cfg.keep_server = true,
                "--server-backend" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--server-backend requires valence or paper".to_string())?;
                    cfg.server_backend = parse_backend(&value)?;
                }
                "--client-dir" => {
                    cfg.client_dir = PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--client-dir requires a path".to_string())?,
                    );
                }
                "--receipt" => {
                    cfg.receipt_path = Some(PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--receipt requires a path".to_string())?,
                    ));
                }
                "--valence-repo" => {
                    cfg.valence_repo = PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--valence-repo requires a path".to_string())?,
                    );
                }
                "--valence-rev" => {
                    cfg.valence_rev = args
                        .next()
                        .ok_or_else(|| "--valence-rev requires a git revision".to_string())?;
                }
                "-h" | "--help" => {
                    print_usage(&cfg);
                    std::process::exit(0);
                }
                _ if arg.starts_with("--server-backend=") => {
                    cfg.server_backend = parse_backend(&arg[17..])?;
                }
                _ if arg.starts_with("--client-dir=") => {
                    cfg.client_dir = PathBuf::from(&arg[13..]);
                }
                _ if arg.starts_with("--receipt=") => {
                    cfg.receipt_path = Some(PathBuf::from(&arg[10..]));
                }
                _ if arg.starts_with("--valence-repo=") => {
                    cfg.valence_repo = PathBuf::from(&arg[15..]);
                }
                _ if arg.starts_with("--valence-rev=") => {
                    cfg.valence_rev = arg[14..].to_string();
                }
                _ => return Err(format!("unknown arg: {arg}")),
            }
        }

        if !server_port_was_set {
            cfg.server_port = match cfg.server_backend {
                ServerBackend::Valence => 25565,
                ServerBackend::Paper => 25566,
            };
        }
        Ok(cfg)
    }
}

fn print_usage(cfg: &Config) {
    println!(
        "Usage: mc-compat-runner [--dry-run|--run] [--build-client] [--status-only] [--stop] [--compare-receipts PAPER_RECEIPT VALENCE_RECEIPT] [--keep-server] [--server-backend valence|paper] [--client-dir PATH] [--receipt PATH] [--valence-repo PATH] [--valence-rev REV]\n\n\
Automates a local Stevenarella compatibility smoke against a Minecraft {} / protocol {} server.\n\
Default client checkout is the editable local Stevenarella sibling at ./stevenarella; pass --client-dir/CLIENT_DIR to use another checkout.\n\
Pass --receipt/SMOKE_RECEIPT to write a machine-readable mc.compat.smoke.receipt.v1 JSON receipt for Cairn/Octet evidence flows.\n\
Use --compare-receipts PAPER_RECEIPT VALENCE_RECEIPT to check the fallback/control and default-backend receipts agree on protocol and headless isolation.\n\
Default server backend is Valence, using an editable local Valence checkout plus an isolated protocol-758 worktree so the dirty/current checkout is untouched.\n\
If the Stevenarella or Valence checkout is missing, clone/fetch it or pass --client-dir/CLIENT_DIR and --valence-repo/VALENCE_REPO to editable checkouts.\n\
Client runs are forced through Xvfb/X11 with software GL and no inherited Wayland socket.\n\
Paper fallback runs set EULA=TRUE based on recorded user acceptance.\n\n\
Env: MC_COMPAT_ROOT={} CLIENT_DIR={} TARGET_DIR={} SMOKE_RECEIPT={} VALENCE_REPO={} VALENCE_REV={} VALENCE_WORKTREE={} VALENCE_TARGET_DIR={} CLIENT_TIMEOUT={}\n",
        cfg.server_version,
        cfg.server_protocol,
        cfg.root.display(),
        cfg.client_dir.display(),
        cfg.target_dir.display(),
        cfg.receipt_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.valence_repo.display(),
        cfg.valence_rev,
        cfg.valence_worktree.display(),
        cfg.valence_target_dir.display(),
        cfg.client_timeout.as_secs()
    );
}

fn build_client(cfg: &Config) -> Result<(), String> {
    ensure_client_dir_ready(cfg)?;
    log(format_args!("building Stevenarella client"));
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&cfg.client_dir)
        .arg("build")
        .arg("--bin")
        .arg("stevenarella");
    apply_build_env(&mut cmd, &cfg.target_dir);
    run_cmd(cfg, &mut cmd)
}

fn ensure_client_dir_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.client_dir.exists() {
        return Err(format!(
            "Stevenarella checkout not found at {}. Keep an editable sibling checkout with `git clone https://github.com/iceiix/stevenarella {}` or pass --client-dir/CLIENT_DIR to another checkout.",
            cfg.client_dir.display(),
            cfg.client_dir.display()
        ));
    }

    let manifest = cfg.client_dir.join("Cargo.toml");
    if !manifest.exists() {
        return Err(format!(
            "Stevenarella checkout {} is missing Cargo.toml. Point --client-dir/CLIENT_DIR at the Stevenarella repository root.",
            cfg.client_dir.display()
        ));
    }

    Ok(())
}

fn start_server(cfg: &Config) -> Result<ManagedServer, String> {
    match cfg.server_backend {
        ServerBackend::Valence => start_valence_server(cfg),
        ServerBackend::Paper => {
            start_paper_server(cfg)?;
            Ok(ManagedServer {
                child: None,
                pid_file: cfg.valence_pid_file.clone(),
                keep: true,
            })
        }
    }
}

fn stop_server(cfg: &Config) -> Result<(), String> {
    stop_valence_server(cfg)?;
    log(format_args!(
        "stopping managed Paper container {}",
        cfg.server_name
    ));
    let mut cmd = Command::new("docker");
    cmd.arg("rm").arg("-f").arg(&cfg.server_name);
    run_cmd(cfg, &mut cmd)
}

fn prepare_valence_worktree(cfg: &Config) -> Result<(), String> {
    ensure_valence_repo_ready(cfg)?;
    if !cfg.valence_worktree.join(".git").exists() {
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
        log(format_args!(
            "using existing Valence worktree {}",
            cfg.valence_worktree.display()
        ));
    }
    Ok(())
}

fn ensure_valence_repo_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.valence_repo.exists() {
        return Err(format!(
            "Valence checkout not found at {}. Keep an editable sibling checkout with `git clone https://github.com/valence-rs/valence {}` or pass --valence-repo/VALENCE_REPO to another checkout.",
            cfg.valence_repo.display(),
            cfg.valence_repo.display()
        ));
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
        .map_err(|e| format!("check Valence checkout {}: {e}", cfg.valence_repo.display()))?;

    if !status.success() {
        return Err(format!(
            "Valence checkout {} does not contain compatible revision {}. Run `git -C {} fetch --all --tags` or pass --valence-repo/VALENCE_REPO to an editable checkout that has it.",
            cfg.valence_repo.display(),
            cfg.valence_rev,
            cfg.valence_repo.display()
        ));
    }

    Ok(())
}

fn start_valence_server(cfg: &Config) -> Result<ManagedServer, String> {
    prepare_valence_worktree(cfg)?;
    log(format_args!(
        "starting Valence {} example '{}' on 127.0.0.1:{}; log: {}",
        cfg.valence_rev,
        cfg.valence_example,
        cfg.server_port,
        cfg.valence_log.display()
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!(
            "would run Valence example from {}",
            cfg.valence_worktree.display()
        ));
        return Ok(ManagedServer {
            child: None,
            pid_file: cfg.valence_pid_file.clone(),
            keep: true,
        });
    }
    if cfg.server_port != 25565 {
        log(format_args!(
            "warning: Valence revision {} defaults to 127.0.0.1:25565; SERVER_PORT={} may only work if the example overrides Config::address",
            cfg.valence_rev, cfg.server_port
        ));
    }
    stop_valence_server(cfg)?;
    let log_file = File::create(&cfg.valence_log)
        .map_err(|e| format!("create {}: {e}", cfg.valence_log.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone valence log handle: {e}"))?;
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&cfg.valence_worktree)
        .arg("run")
        .arg("--example")
        .arg(&cfg.valence_example)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    cmd.env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", &cfg.valence_target_dir);
    let child = cmd.spawn().map_err(|e| format!("spawn Valence: {e}"))?;
    fs::write(&cfg.valence_pid_file, child.id().to_string())
        .map_err(|e| format!("write {}: {e}", cfg.valence_pid_file.display()))?;
    Ok(ManagedServer {
        child: Some(child),
        pid_file: cfg.valence_pid_file.clone(),
        keep: cfg.keep_server,
    })
}

fn start_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "starting Paper {} server on 127.0.0.1:{} via {} with EULA=TRUE",
        cfg.server_version, cfg.server_port, cfg.docker_image
    ));
    let _ = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&cfg.server_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let mut cmd = Command::new("docker");
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
        .arg(&cfg.docker_image);
    run_cmd(cfg, &mut cmd)
}

fn stop_valence_server(cfg: &Config) -> Result<(), String> {
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

fn probe_status(cfg: &Config) -> Result<(), String> {
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

fn read_status(port: u16, protocol: u32) -> Result<String, String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    let mut payload = Vec::new();
    write_varint(protocol, &mut payload);
    write_string("127.0.0.1", &mut payload);
    payload.extend_from_slice(&port.to_be_bytes());
    write_varint(1, &mut payload);
    write_packet(0, &payload, &mut stream)?;
    write_packet(0, &[], &mut stream)?;
    let _packet_len = read_varint(&mut stream)?;
    let packet_id = read_varint(&mut stream)?;
    if packet_id != 0 {
        return Err(format!("unexpected status packet id {packet_id}"));
    }
    let string_len = read_varint(&mut stream)? as usize;
    let mut buf = vec![0; string_len];
    stream.read_exact(&mut buf).map_err(|e| e.to_string())?;
    String::from_utf8(buf).map_err(|e| e.to_string())
}

fn run_client(cfg: &Config) -> Result<ClientRunEvidence, String> {
    log(format_args!(
        "running Stevenarella headless smoke isolated from host Wayland compositor"
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Stevenarella under xvfb-run"));
        return Ok(ClientRunEvidence {
            log_path: None,
            exit_code: None,
            classification: "dry-run",
            matched_success_pattern: None,
        });
    }
    let client_log = env_path("CLIENT_LOG").unwrap_or_else(temp_client_log);
    let log_file =
        File::create(&client_log).map_err(|e| format!("create {}: {e}", client_log.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone client log handle: {e}"))?;
    let mut cmd = Command::new("timeout");
    cmd.arg(cfg.client_timeout.as_secs().to_string())
        .arg("xvfb-run")
        .arg("-a")
        .arg("-s")
        .arg("-screen 0 1280x720x24 +extension GLX +render -noreset")
        .arg(cfg.target_dir.join("debug/stevenarella"))
        .arg("--server")
        .arg(format!("127.0.0.1:{}", cfg.server_port))
        .arg("--username")
        .arg(&cfg.client_username)
        .arg("--default-protocol-version")
        .arg(cfg.server_protocol.to_string())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    apply_build_env(&mut cmd, &cfg.target_dir);
    apply_headless_env(&mut cmd);
    let status = cmd.status().map_err(|e| format!("run client: {e}"))?;
    let output = fs::read_to_string(&client_log)
        .map_err(|e| format!("read {}: {e}", client_log.display()))?;
    print!("{output}");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let matched_success_pattern = cfg
        .client_success_needles
        .iter()
        .find(|needle| output.contains(needle.as_str()))
        .cloned();
    if status.success() {
        log(format_args!(
            "client exited successfully; log: {}",
            client_log.display()
        ));
        Ok(ClientRunEvidence {
            log_path: Some(client_log),
            exit_code: status.code(),
            classification: "client-exited-success",
            matched_success_pattern,
        })
    } else if status.code() == Some(124) && matched_success_pattern.is_some() {
        log(format_args!(
            "bounded client smoke passed before timeout; log: {}",
            client_log.display()
        ));
        Ok(ClientRunEvidence {
            log_path: Some(client_log),
            exit_code: status.code(),
            classification: "timeout-success-evidence",
            matched_success_pattern,
        })
    } else {
        Err(format!(
            "client smoke failed with exit {:?}; log: {}",
            status.code(),
            client_log.display()
        ))
    }
}

fn write_smoke_receipt(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &String>,
) -> Result<(), String> {
    let Some(path) = &cfg.receipt_path else {
        return Ok(());
    };
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|e| format!("create receipt dir {}: {e}", parent.display()))?;
    }
    let json = smoke_receipt_json(cfg, result);
    fs::write(path, json).map_err(|e| format!("write receipt {}: {e}", path.display()))?;
    log(format_args!("wrote smoke receipt {}", path.display()));
    Ok(())
}

fn smoke_receipt_json(cfg: &Config, result: Result<&Option<ClientRunEvidence>, &String>) -> String {
    let status = if result.is_ok() { "pass" } else { "fail" };
    let error = result.err();
    let client = result.ok().and_then(|client| client.as_ref());
    let receipt_path = cfg
        .receipt_path
        .as_ref()
        .map(|path| path.display().to_string());
    let client_log_path = client
        .and_then(|evidence| evidence.log_path.as_ref())
        .map(|path| path.display().to_string());
    let matched_pattern = client.and_then(|evidence| evidence.matched_success_pattern.as_deref());
    let classification = client.map(|evidence| evidence.classification);
    let exit_code = client.and_then(|evidence| evidence.exit_code);
    let error_json = error
        .map(|err| json_string(err))
        .unwrap_or_else(|| "null".to_string());
    let receipt_path_json = json_optional_string(receipt_path.as_deref());
    let client_log_json = json_optional_string(client_log_path.as_deref());
    let matched_pattern_json = json_optional_string(matched_pattern);
    let classification_json = json_optional_string(classification);
    let exit_code_json = exit_code
        .map(|code| code.to_string())
        .unwrap_or_else(|| "null".to_string());

    format!(
        "{{\n  \"schema\": \"mc.compat.smoke.receipt.v1\",\n  \"status\": {status_json},\n  \"mode\": {mode_json},\n  \"dry_run\": {dry_run},\n  \"contract\": {{\n    \"cairn_contract\": \"mc.compat.smoke.receipt.v1\",\n    \"octet_producer_surface\": \"tools/mc-compat-runner/src/main.rs\",\n    \"claims_correctness\": false,\n    \"claims_semantic_equivalence\": false\n  }},\n  \"server\": {{\n    \"backend\": {backend_json},\n    \"version\": {version_json},\n    \"protocol\": {protocol},\n    \"port\": {port}\n  }},\n  \"client\": {{\n    \"dir\": {client_dir_json},\n    \"target_dir\": {target_dir_json},\n    \"username\": {username_json},\n    \"timeout_secs\": {timeout_secs},\n    \"headless_isolation\": {{\n      \"xvfb\": true,\n      \"x11_backend\": true,\n      \"software_gl\": true,\n      \"wayland_socket_inherited\": false\n    }},\n    \"log_path\": {client_log_json},\n    \"exit_code\": {exit_code_json},\n    \"classification\": {classification_json},\n    \"matched_success_pattern\": {matched_pattern_json}\n  }},\n  \"valence\": {{\n    \"repo\": {valence_repo_json},\n    \"rev\": {valence_rev_json},\n    \"worktree\": {valence_worktree_json},\n    \"example\": {valence_example_json},\n    \"log_path\": {valence_log_json}\n  }},\n  \"receipt_path\": {receipt_path_json},\n  \"error\": {error_json}\n}}\n",
        status_json = json_string(status),
        mode_json = json_string(mode_name(cfg.mode)),
        dry_run = cfg.mode == Mode::DryRun,
        backend_json = json_string(backend_name(cfg.server_backend)),
        version_json = json_string(&cfg.server_version),
        protocol = cfg.server_protocol,
        port = cfg.server_port,
        client_dir_json = json_string(&cfg.client_dir.display().to_string()),
        target_dir_json = json_string(&cfg.target_dir.display().to_string()),
        username_json = json_string(&cfg.client_username),
        timeout_secs = cfg.client_timeout.as_secs(),
        valence_repo_json = json_string(&cfg.valence_repo.display().to_string()),
        valence_rev_json = json_string(&cfg.valence_rev),
        valence_worktree_json = json_string(&cfg.valence_worktree.display().to_string()),
        valence_example_json = json_string(&cfg.valence_example),
        valence_log_json = json_string(&cfg.valence_log.display().to_string()),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptSummary {
    path: PathBuf,
    schema: String,
    status: String,
    backend: String,
    protocol: u32,
    port: u16,
    classification: String,
    matched_success_pattern: Option<String>,
    xvfb: bool,
    x11_backend: bool,
    software_gl: bool,
    wayland_socket_inherited: bool,
}

fn compare_receipts(cfg: &Config) -> Result<(), String> {
    let (left, right) = cfg
        .compare_receipts
        .as_ref()
        .ok_or_else(|| "compare-receipts mode requires two receipt paths".to_string())?;
    let left = read_receipt_summary(left)?;
    let right = read_receipt_summary(right)?;
    validate_receipt_pair(&left, &right)?;
    let paper = if left.backend == "paper" {
        &left
    } else {
        &right
    };
    let valence = if left.backend == "valence" {
        &left
    } else {
        &right
    };
    println!(
        "[mc-compat] receipt comparison passed: paper={} valence={} protocol={} headless=xvfb/x11/software-gl/no-wayland",
        paper.path.display(),
        valence.path.display(),
        paper.protocol
    );
    Ok(())
}

fn read_receipt_summary(path: &Path) -> Result<ReceiptSummary, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("read receipt {}: {e}", path.display()))?;
    read_receipt_summary_from_text(path.to_path_buf(), &text)
}

fn read_receipt_summary_from_text(path: PathBuf, text: &str) -> Result<ReceiptSummary, String> {
    Ok(ReceiptSummary {
        path,
        schema: json_string_field(text, "schema")?,
        status: json_string_field(text, "status")?,
        backend: json_object_string_field(text, "server", "backend")?,
        protocol: json_object_u32_field(text, "server", "protocol")?,
        port: json_object_u32_field(text, "server", "port")? as u16,
        classification: json_object_string_field(text, "client", "classification")?,
        matched_success_pattern: json_object_optional_string_field(
            text,
            "client",
            "matched_success_pattern",
        )?,
        xvfb: json_object_bool_field(text, "headless_isolation", "xvfb")?,
        x11_backend: json_object_bool_field(text, "headless_isolation", "x11_backend")?,
        software_gl: json_object_bool_field(text, "headless_isolation", "software_gl")?,
        wayland_socket_inherited: json_object_bool_field(
            text,
            "headless_isolation",
            "wayland_socket_inherited",
        )?,
    })
}

fn validate_receipt_pair(left: &ReceiptSummary, right: &ReceiptSummary) -> Result<(), String> {
    validate_receipt_summary(left)?;
    validate_receipt_summary(right)?;
    let backends = [left.backend.as_str(), right.backend.as_str()];
    if !(backends.contains(&"paper") && backends.contains(&"valence")) {
        return Err(format!(
            "expected one paper receipt and one valence receipt, got {} and {}",
            left.backend, right.backend
        ));
    }
    if left.protocol != right.protocol {
        return Err(format!(
            "receipt protocol mismatch: {} has {}, {} has {}",
            left.path.display(),
            left.protocol,
            right.path.display(),
            right.protocol
        ));
    }
    if left.protocol != DEFAULT_SERVER_PROTOCOL {
        return Err(format!(
            "expected protocol {}, got {}",
            DEFAULT_SERVER_PROTOCOL, left.protocol
        ));
    }
    for receipt in [left, right] {
        match receipt.backend.as_str() {
            "paper" if receipt.port != 25566 => {
                return Err(format!(
                    "paper receipt port must be 25566, got {}",
                    receipt.port
                ));
            }
            "valence" if receipt.port != 25565 => {
                return Err(format!(
                    "valence receipt port must be 25565, got {}",
                    receipt.port
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_receipt_summary(receipt: &ReceiptSummary) -> Result<(), String> {
    if receipt.schema != "mc.compat.smoke.receipt.v1" {
        return Err(format!(
            "{} has unexpected schema {}",
            receipt.path.display(),
            receipt.schema
        ));
    }
    if receipt.status != "pass" {
        return Err(format!(
            "{} did not pass; status={}",
            receipt.path.display(),
            receipt.status
        ));
    }
    if !matches!(
        receipt.classification.as_str(),
        "timeout-success-evidence" | "client-exited-success"
    ) {
        return Err(format!(
            "{} has unsupported client classification {}",
            receipt.path.display(),
            receipt.classification
        ));
    }
    if receipt.matched_success_pattern.is_none() {
        return Err(format!(
            "{} is missing matched client success pattern",
            receipt.path.display()
        ));
    }
    if !(receipt.xvfb && receipt.x11_backend && receipt.software_gl)
        || receipt.wayland_socket_inherited
    {
        return Err(format!(
            "{} does not prove niri-safe headless isolation",
            receipt.path.display()
        ));
    }
    Ok(())
}

fn json_object_string_field(text: &str, object: &str, key: &str) -> Result<String, String> {
    json_string_field(json_object_slice(text, object)?, key)
}

fn json_object_optional_string_field(
    text: &str,
    object: &str,
    key: &str,
) -> Result<Option<String>, String> {
    json_optional_string_field(json_object_slice(text, object)?, key)
}

fn json_object_u32_field(text: &str, object: &str, key: &str) -> Result<u32, String> {
    json_u32_field(json_object_slice(text, object)?, key)
}

fn json_object_bool_field(text: &str, object: &str, key: &str) -> Result<bool, String> {
    json_bool_field(json_object_slice(text, object)?, key)
}

fn json_object_slice<'a>(text: &'a str, object: &str) -> Result<&'a str, String> {
    let key = format!("\"{object}\"");
    let start = text
        .find(&key)
        .ok_or_else(|| format!("missing object {object}"))?;
    let after_key = &text[start + key.len()..];
    let brace_offset = after_key
        .find('{')
        .ok_or_else(|| format!("missing object body for {object}"))?;
    let body_start = start + key.len() + brace_offset;
    let mut depth = 0usize;
    for (offset, ch) in text[body_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&text[body_start..=body_start + offset]);
                }
            }
            _ => {}
        }
    }
    Err(format!("unterminated object {object}"))
}

fn json_string_field(text: &str, key: &str) -> Result<String, String> {
    let after_colon = json_field_value(text, key)?;
    parse_json_string(after_colon).map(|(value, _)| value)
}

fn json_optional_string_field(text: &str, key: &str) -> Result<Option<String>, String> {
    let after_colon = json_field_value(text, key)?;
    if after_colon.trim_start().starts_with("null") {
        Ok(None)
    } else {
        parse_json_string(after_colon).map(|(value, _)| Some(value))
    }
}

fn json_u32_field(text: &str, key: &str) -> Result<u32, String> {
    let value = json_field_value(text, key)?.trim_start();
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(format!("field {key} is not an unsigned integer"));
    }
    digits
        .parse()
        .map_err(|e| format!("parse field {key}: {e}"))
}

fn json_bool_field(text: &str, key: &str) -> Result<bool, String> {
    let value = json_field_value(text, key)?.trim_start();
    if value.starts_with("true") {
        Ok(true)
    } else if value.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field {key} is not a bool"))
    }
}

fn json_field_value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    let needle = format!("\"{key}\"");
    let start = text
        .find(&needle)
        .ok_or_else(|| format!("missing field {key}"))?;
    let after_key = &text[start + needle.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| format!("missing colon for field {key}"))?;
    Ok(&after_key[colon + 1..])
}

fn parse_json_string(text: &str) -> Result<(String, &str), String> {
    let text = text.trim_start();
    let mut chars = text.char_indices();
    match chars.next() {
        Some((_, '"')) => {}
        _ => return Err("expected JSON string".to_string()),
    }
    let mut out = String::new();
    let mut escape = false;
    for (idx, ch) in chars {
        if escape {
            match ch {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                other => out.push(other),
            }
            escape = false;
        } else if ch == '\\' {
            escape = true;
        } else if ch == '"' {
            return Ok((out, &text[idx + 1..]));
        } else {
            out.push(ch);
        }
    }
    Err("unterminated JSON string".to_string())
}

fn mode_name(mode: Mode) -> &'static str {
    match mode {
        Mode::DryRun => "dry-run",
        Mode::Run => "run",
        Mode::BuildClient => "build-client",
        Mode::StatusOnly => "status-only",
        Mode::Stop => "stop",
        Mode::CompareReceipts => "compare-receipts",
    }
}

fn backend_name(backend: ServerBackend) -> &'static str {
    match backend {
        ServerBackend::Valence => "valence",
        ServerBackend::Paper => "paper",
    }
}

fn json_optional_string(value: Option<&str>) -> String {
    value.map(json_string).unwrap_or_else(|| "null".to_string())
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn apply_build_env(cmd: &mut Command, target_dir: &Path) {
    cmd.env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CMAKE_POLICY_VERSION_MINIMUM", "3.5");
}

fn apply_headless_env(cmd: &mut Command) {
    cmd.env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .env_remove("XDG_CURRENT_DESKTOP")
        .env("XDG_SESSION_TYPE", "x11")
        .env("WINIT_UNIX_BACKEND", "x11")
        .env("GDK_BACKEND", "x11")
        .env("SDL_VIDEODRIVER", "x11")
        .env("LIBGL_ALWAYS_SOFTWARE", "1")
        .env("MESA_LOADER_DRIVER_OVERRIDE", "llvmpipe");
}

fn run_cmd(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    if cfg.mode == Mode::DryRun {
        println!("+ {cmd:?}");
        return Ok(());
    }
    let status = cmd.status().map_err(|e| format!("spawn {cmd:?}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command {cmd:?} failed with {status}"))
    }
}

fn write_packet(id: u32, payload: &[u8], out: &mut TcpStream) -> Result<(), String> {
    let mut body = Vec::new();
    write_varint(id, &mut body);
    body.extend_from_slice(payload);
    let mut packet = Vec::new();
    write_varint(body.len() as u32, &mut packet);
    packet.extend_from_slice(&body);
    out.write_all(&packet).map_err(|e| e.to_string())
}

fn write_string(s: &str, out: &mut Vec<u8>) {
    write_varint(s.len() as u32, out);
    out.extend_from_slice(s.as_bytes());
}

fn write_varint(mut value: u32, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn read_varint(input: &mut TcpStream) -> Result<u32, String> {
    let mut value = 0u32;
    for shift in (0..35).step_by(7) {
        let mut byte = [0u8; 1];
        input.read_exact(&mut byte).map_err(|e| e.to_string())?;
        value |= u32::from(byte[0] & 0x7f) << shift;
        if byte[0] & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err("varint too long".to_string())
}

fn parse_backend(value: &str) -> Result<ServerBackend, String> {
    match value {
        "valence" => Ok(ServerBackend::Valence),
        "paper" => Ok(ServerBackend::Paper),
        other => Err(format!("unknown server backend: {other}")),
    }
}

fn env_string(name: &str) -> Option<String> {
    env::var(name).ok().filter(|s| !s.is_empty())
}

fn env_path(name: &str) -> Option<PathBuf> {
    env_string(name).map(PathBuf::from)
}

fn temp_client_log() -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    PathBuf::from(format!("/tmp/mc-compat-client.{millis}.log"))
}

fn log(args: std::fmt::Arguments<'_>) {
    println!("[mc-compat] {args}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn test_config(args: &[&str], env: &[(&str, &str)]) -> Result<Config, String> {
        let env: BTreeMap<String, String> = env
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect();
        Config::from_sources(
            PathBuf::from("/workspace/mc"),
            |name| env.get(name).cloned(),
            args.iter().map(|arg| (*arg).to_string()),
        )
    }

    fn fake_stevenarella_checkout(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "mc-compat-stevenarella-{label}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create fake Stevenarella checkout");
        fs::write(
            dir.join("Cargo.toml"),
            "[package]\nname = \"stevenarella\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
        )
        .expect("write fake Stevenarella manifest");
        dir
    }

    #[test]
    fn defaults_to_valence_protocol_and_port() {
        let cfg = test_config(&[], &[]).expect("default config parses");

        assert_eq!(cfg.root, PathBuf::from("/workspace/mc"));
        assert_eq!(cfg.client_dir, PathBuf::from("/workspace/mc/stevenarella"));
        assert_eq!(cfg.valence_repo, PathBuf::from("/workspace/mc/valence"));
        assert_eq!(cfg.server_backend, ServerBackend::Valence);
        assert_eq!(cfg.server_protocol, DEFAULT_SERVER_PROTOCOL);
        assert_eq!(cfg.server_port, 25565);
        assert_eq!(cfg.valence_rev, DEFAULT_VALENCE_REV);
        assert_eq!(cfg.mode, Mode::DryRun);
    }

    #[test]
    fn cli_overrides_backend_client_dir_valence_repo_and_revision() {
        let cfg = test_config(
            &[
                "--run",
                "--server-backend",
                "paper",
                "--client-dir",
                "/tmp/editable-stevenarella",
                "--receipt=/tmp/mc-smoke.json",
                "--valence-repo",
                "/tmp/editable-valence",
                "--valence-rev=local-debug-rev",
            ],
            &[],
        )
        .expect("cli override config parses");

        assert_eq!(cfg.mode, Mode::Run);
        assert_eq!(cfg.server_backend, ServerBackend::Paper);
        assert_eq!(cfg.server_port, 25566);
        assert_eq!(cfg.client_dir, PathBuf::from("/tmp/editable-stevenarella"));
        assert_eq!(cfg.receipt_path, Some(PathBuf::from("/tmp/mc-smoke.json")));
        assert_eq!(cfg.valence_repo, PathBuf::from("/tmp/editable-valence"));
        assert_eq!(cfg.valence_rev, "local-debug-rev");
    }

    #[test]
    fn env_overrides_are_parsed_without_global_environment_mutation() {
        let cfg = test_config(
            &["--server-backend=paper"],
            &[
                ("MC_COMPAT_ROOT", "/repo/mc"),
                ("CLIENT_TIMEOUT", "8"),
                (
                    "CLIENT_SUCCESS_PATTERN",
                    "Detected server protocol version|Dimension type:",
                ),
                ("SERVER_PORT", "24444"),
                ("SMOKE_RECEIPT", "/repo/receipts/smoke.json"),
                ("CLIENT_DIR", "/repo/stevenarella-edit"),
                ("VALENCE_REPO", "/repo/valence-edit"),
                ("VALENCE_REV", "debug-rev"),
            ],
        )
        .expect("env override config parses");

        assert_eq!(cfg.root, PathBuf::from("/repo/mc"));
        assert_eq!(cfg.client_dir, PathBuf::from("/repo/stevenarella-edit"));
        assert_eq!(cfg.server_backend, ServerBackend::Paper);
        assert_eq!(cfg.server_port, 24444);
        assert_eq!(
            cfg.receipt_path,
            Some(PathBuf::from("/repo/receipts/smoke.json"))
        );
        assert_eq!(cfg.client_timeout, Duration::from_secs(8));
        assert_eq!(cfg.valence_repo, PathBuf::from("/repo/valence-edit"));
        assert_eq!(cfg.valence_rev, "debug-rev");
        assert_eq!(
            cfg.client_success_needles,
            vec![
                "Detected server protocol version".to_string(),
                "Dimension type:".to_string()
            ]
        );
    }

    #[test]
    fn invalid_backend_is_rejected() {
        let err = test_config(&["--server-backend", "spigot"], &[]).unwrap_err();
        assert!(err.contains("unknown server backend: spigot"), "{err}");
    }

    #[test]
    fn missing_valence_checkout_has_actionable_diagnostic() {
        let missing =
            std::env::temp_dir().join(format!("mc-compat-missing-valence-{}", std::process::id()));
        let cfg = test_config(&["--valence-repo", missing.to_str().unwrap()], &[])
            .expect("config with missing Valence repo parses");

        let err = ensure_valence_repo_ready(&cfg).unwrap_err();

        assert!(err.contains("Valence checkout not found"), "{err}");
        assert!(
            err.contains("git clone https://github.com/valence-rs/valence"),
            "{err}"
        );
        assert!(err.contains("--valence-repo/VALENCE_REPO"), "{err}");
    }

    #[test]
    fn missing_client_checkout_has_actionable_diagnostic() {
        let missing = std::env::temp_dir().join(format!(
            "mc-compat-missing-stevenarella-{}",
            std::process::id()
        ));
        let cfg = test_config(&["--client-dir", missing.to_str().unwrap()], &[])
            .expect("config with missing Stevenarella checkout parses");

        let err = ensure_client_dir_ready(&cfg).unwrap_err();

        assert!(err.contains("Stevenarella checkout not found"), "{err}");
        assert!(
            err.contains("git clone https://github.com/iceiix/stevenarella"),
            "{err}"
        );
        assert!(err.contains("--client-dir/CLIENT_DIR"), "{err}");
    }

    #[test]
    fn client_checkout_must_point_at_manifest_root() {
        let dir =
            std::env::temp_dir().join(format!("mc-compat-bad-stevenarella-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create bad Stevenarella checkout");
        let cfg = test_config(&["--client-dir", dir.to_str().unwrap()], &[])
            .expect("config with bad Stevenarella checkout parses");

        let err = ensure_client_dir_ready(&cfg).unwrap_err();

        assert!(err.contains("missing Cargo.toml"), "{err}");
        assert!(err.contains("Stevenarella repository root"), "{err}");
    }

    #[test]
    fn valid_client_checkout_preflight_passes() {
        let dir = fake_stevenarella_checkout("valid");
        let cfg = test_config(&["--client-dir", dir.to_str().unwrap()], &[])
            .expect("config with fake Stevenarella checkout parses");

        ensure_client_dir_ready(&cfg).expect("fake checkout has a manifest");
    }

    #[test]
    fn smoke_receipt_records_cairn_contract_and_octet_surface() {
        let mut cfg = test_config(
            &[
                "--server-backend=paper",
                "--receipt",
                "/tmp/receipt.json",
                "--client-dir",
                "/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        cfg.server_port = 25566;
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"schema\": \"mc.compat.smoke.receipt.v1\""),
            "{json}"
        );
        assert!(
            json.contains("\"cairn_contract\": \"mc.compat.smoke.receipt.v1\""),
            "{json}"
        );
        assert!(
            json.contains("\"octet_producer_surface\": \"tools/mc-compat-runner/src/main.rs\""),
            "{json}"
        );
        assert!(
            json.contains("\"classification\": \"timeout-success-evidence\""),
            "{json}"
        );
        assert!(
            json.contains("\"matched_success_pattern\": \"Detected server protocol version\""),
            "{json}"
        );
        assert!(
            json.contains("\"wayland_socket_inherited\": false"),
            "{json}"
        );
    }

    fn receipt_fixture(backend: &str, protocol: u32, port: u16) -> String {
        format!(
            "{{\n  \"schema\": \"mc.compat.smoke.receipt.v1\",\n  \"status\": \"pass\",\n  \"mode\": \"run\",\n  \"dry_run\": false,\n  \"contract\": {{\n    \"claims_correctness\": false,\n    \"claims_semantic_equivalence\": false\n  }},\n  \"server\": {{\n    \"backend\": \"{backend}\",\n    \"version\": \"1.18.2\",\n    \"protocol\": {protocol},\n    \"port\": {port}\n  }},\n  \"client\": {{\n    \"headless_isolation\": {{\n      \"xvfb\": true,\n      \"x11_backend\": true,\n      \"software_gl\": true,\n      \"wayland_socket_inherited\": false\n    }},\n    \"classification\": \"timeout-success-evidence\",\n    \"matched_success_pattern\": \"Detected server protocol version\"\n  }},\n  \"error\": null\n}}\n"
        )
    }

    #[test]
    fn compares_paper_and_valence_receipts() {
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", 758, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", 758, 25565),
        )
        .expect("valence fixture parses");

        validate_receipt_pair(&paper, &valence).expect("matching receipts compare");
    }

    #[test]
    fn rejects_receipt_protocol_mismatch() {
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", 758, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", 759, 25565),
        )
        .expect("valence fixture parses");

        let err = validate_receipt_pair(&paper, &valence).unwrap_err();
        assert!(err.contains("receipt protocol mismatch"), "{err}");
    }

    #[test]
    fn smoke_receipt_records_failures_without_success_claims() {
        let cfg =
            test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("receipt config parses");
        let err = "server status probe failed".to_string();

        let json = smoke_receipt_json(&cfg, Err(&err));

        assert!(json.contains("\"status\": \"fail\""), "{json}");
        assert!(json.contains("\"classification\": null"), "{json}");
        assert!(
            json.contains("\"error\": \"server status probe failed\""),
            "{json}"
        );
        assert!(json.contains("\"claims_correctness\": false"), "{json}");
        assert!(
            json.contains("\"claims_semantic_equivalence\": false"),
            "{json}"
        );
    }
}
