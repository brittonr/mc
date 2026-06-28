use crate::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) const TEST_GIT_USER_EMAIL: &str = "mc-compat@example.invalid";
pub(crate) const TEST_GIT_USER_NAME: &str = "mc-compat";
pub(crate) const TEST_STEVENARELLA_SUBTREE_DIR: &str = "mc/clients/stevenarella";
pub(crate) const TEST_SESSION_ID: &str = "s1";
pub(crate) const TEST_USERNAME: &str = "compatbot";
pub(crate) const TEST_CLIENT_DIR: &str = "/tmp/stevenarella";
pub(crate) const TEST_ATTACKER_USERNAME: &str = "compatbota";
pub(crate) const TEST_VICTIM_USERNAME: &str = "compatbotb";

pub(crate) fn test_config(args: &[&str], env: &[(&str, &str)]) -> Result<Config, String> {
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

pub(crate) fn fake_stevenarella_checkout(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mc-compat-stevenarella-{label}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create fake Stevenarella checkout");
    fs::write(
        dir.join(CARGO_MANIFEST_FILE),
        "[package]\nname = \"stevenarella\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("write fake Stevenarella manifest");
    dir
}

pub(crate) fn git_fixture_root(label: &str) -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after Unix epoch")
        .as_millis();
    let dir = std::env::temp_dir().join(format!(
        "mc-compat-git-fixture-{label}-{}-{millis}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create git fixture root");
    dir
}

pub(crate) fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub(crate) fn run_git_fixture(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("git command starts");
    assert!(
        output.status.success(),
        "git {:?} failed with {}\nstdout={}\nstderr={}",
        args,
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("git stdout is UTF-8")
        .trim()
        .to_string()
}

pub(crate) fn typed_event_fixture_lines() -> Vec<&'static str> {
    vec![
        "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=1 event=protocol_detected",
        "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=2 event=join_game",
        "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=3 event=render_tick",
    ]
}

pub(crate) fn typed_event_fixture() -> Vec<TypedEvent> {
    typed_event_fixture_lines()
        .into_iter()
        .map(|line| parse_typed_event_line(line).expect("typed event parses"))
        .collect()
}
