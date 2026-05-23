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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ServerBackend {
    Valence,
    Paper,
}

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
            let _server = start_server(&cfg)?;
            probe_status(&cfg)?;
            run_client(&cfg)?;
        }
        Mode::BuildClient => build_client(&cfg)?,
        Mode::StatusOnly => probe_status(&cfg)?,
        Mode::Stop => stop_server(&cfg)?,
        Mode::Run => {
            build_client(&cfg)?;
            let _server = start_server(&cfg)?;
            probe_status(&cfg)?;
            run_client(&cfg)?;
        }
    }
    Ok(())
}

impl Config {
    fn from_env_and_args() -> Result<Self, String> {
        let root = env_path("MC_COMPAT_ROOT")
            .or_else(|| env_path("ROOT"))
            .unwrap_or(env::current_dir().map_err(|e| format!("current dir: {e}"))?);
        let mut cfg = Config {
            client_dir: env_path("CLIENT_DIR").unwrap_or_else(|| root.join("stevenarella")),
            valence_repo: env_path("VALENCE_REPO").unwrap_or_else(|| root.join("valence")),
            valence_rev: env_string("VALENCE_REV")
                .unwrap_or_else(|| DEFAULT_VALENCE_REV.to_string()),
            valence_worktree: env_path("VALENCE_WORKTREE")
                .unwrap_or_else(|| PathBuf::from("/tmp/valence-compat-758")),
            valence_example: env_string("VALENCE_EXAMPLE")
                .unwrap_or_else(|| DEFAULT_VALENCE_EXAMPLE.to_string()),
            valence_log: env_path("VALENCE_LOG")
                .unwrap_or_else(|| PathBuf::from("/tmp/mc-compat-valence.log")),
            valence_target_dir: env_path("VALENCE_TARGET_DIR")
                .unwrap_or_else(|| PathBuf::from("/tmp/valence-compat-758-target")),
            valence_pid_file: env_path("VALENCE_PID_FILE")
                .unwrap_or_else(|| PathBuf::from("/tmp/mc-compat-valence.pid")),
            server_backend: parse_backend(
                &env_string("SERVER_BACKEND").unwrap_or_else(|| "valence".to_string()),
            )?,
            target_dir: env_path("TARGET_DIR")
                .unwrap_or_else(|| PathBuf::from("/tmp/stevenarella-target2")),
            server_name: env_string("SERVER_NAME")
                .unwrap_or_else(|| "mc-compat-1-18-2".to_string()),
            server_version: env_string("SERVER_VERSION")
                .unwrap_or_else(|| DEFAULT_SERVER_VERSION.to_string()),
            server_protocol: env_string("SERVER_PROTOCOL")
                .and_then(|s| s.parse().ok())
                .unwrap_or(DEFAULT_SERVER_PROTOCOL),
            server_port: 0,
            client_username: env_string("CLIENT_USERNAME")
                .unwrap_or_else(|| DEFAULT_CLIENT_USERNAME.to_string()),
            docker_image: env_string("DOCKER_IMAGE")
                .unwrap_or_else(|| "itzg/minecraft-server:java17".to_string()),
            mode: Mode::DryRun,
            keep_server: false,
            client_timeout: Duration::from_secs(
                env_string("CLIENT_TIMEOUT")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(DEFAULT_CLIENT_TIMEOUT_SECS),
            ),
            client_success_needles: env_string("CLIENT_SUCCESS_PATTERN")
                .map(|s| s.split('|').map(str::to_string).collect())
                .unwrap_or_else(|| {
                    DEFAULT_SUCCESS_PATTERN
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                }),
            root,
        };

        let server_port_was_set = env_string("SERVER_PORT").is_some();
        cfg.server_port = env_string("SERVER_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(match cfg.server_backend {
                ServerBackend::Valence => 25565,
                ServerBackend::Paper => 25566,
            });

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--dry-run" => cfg.mode = Mode::DryRun,
                "--run" => cfg.mode = Mode::Run,
                "--build-client" => cfg.mode = Mode::BuildClient,
                "--status-only" => cfg.mode = Mode::StatusOnly,
                "--stop" => cfg.mode = Mode::Stop,
                "--accept-eula" => {}
                "--keep-server" => cfg.keep_server = true,
                "--server-backend" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--server-backend requires valence or paper".to_string())?;
                    cfg.server_backend = parse_backend(&value)?;
                }
                "-h" | "--help" => {
                    print_usage(&cfg);
                    std::process::exit(0);
                }
                _ if arg.starts_with("--server-backend=") => {
                    cfg.server_backend = parse_backend(&arg[17..])?;
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
        "Usage: mc-compat-runner [--dry-run|--run] [--build-client] [--status-only] [--stop] [--keep-server] [--server-backend valence|paper]\n\n\
Automates a local Stevenarella compatibility smoke against a Minecraft {} / protocol {} server.\n\
Default server backend is Valence, using an isolated protocol-758 worktree so the dirty/current Valence checkout is untouched.\n\
Client runs are forced through Xvfb/X11 with software GL and no inherited Wayland socket.\n\
Paper fallback runs set EULA=TRUE based on recorded user acceptance.\n\n\
Env: MC_COMPAT_ROOT={} CLIENT_DIR={} TARGET_DIR={} VALENCE_REPO={} VALENCE_REV={} VALENCE_WORKTREE={} VALENCE_TARGET_DIR={} CLIENT_TIMEOUT={}\n",
        cfg.server_version,
        cfg.server_protocol,
        cfg.root.display(),
        cfg.client_dir.display(),
        cfg.target_dir.display(),
        cfg.valence_repo.display(),
        cfg.valence_rev,
        cfg.valence_worktree.display(),
        cfg.valence_target_dir.display(),
        cfg.client_timeout.as_secs()
    );
}

fn build_client(cfg: &Config) -> Result<(), String> {
    log(format_args!("building Stevenarella client"));
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&cfg.client_dir)
        .arg("build")
        .arg("--bin")
        .arg("stevenarella");
    apply_build_env(&mut cmd, &cfg.target_dir);
    run_cmd(cfg, &mut cmd)
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

fn run_client(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "running Stevenarella headless smoke isolated from host Wayland compositor"
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Stevenarella under xvfb-run"));
        return Ok(());
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
    if status.success() {
        log(format_args!(
            "client exited successfully; log: {}",
            client_log.display()
        ));
        Ok(())
    } else if status.code() == Some(124)
        && cfg
            .client_success_needles
            .iter()
            .any(|needle| output.contains(needle))
    {
        log(format_args!(
            "bounded client smoke passed before timeout; log: {}",
            client_log.display()
        ));
        Ok(())
    } else {
        Err(format!(
            "client smoke failed with exit {:?}; log: {}",
            status.code(),
            client_log.display()
        ))
    }
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
