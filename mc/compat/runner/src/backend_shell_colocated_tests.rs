#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[test]
fn git_revision_evidence_core_reports_clean_dirty_and_unavailable() {
    let clean_evidence =
        build_git_revision_evidence(Some("HEAD"), Ok("abc123".to_string()), Ok(false));
    assert_eq!(clean_evidence.status, GIT_STATUS_CLEAN);
    assert!(!clean_evidence.dirty);
    assert_eq!(clean_evidence.requested_rev.as_deref(), Some("HEAD"));
    assert_eq!(clean_evidence.resolved_rev.as_deref(), Some("abc123"));
    assert!(clean_evidence.diagnostics.is_empty(), "{clean_evidence:?}");

    let dirty_evidence =
        build_git_revision_evidence(Some("HEAD"), Ok("abc123".to_string()), Ok(true));
    assert_eq!(dirty_evidence.status, GIT_STATUS_DIRTY);
    assert!(dirty_evidence.dirty);
    assert_eq!(dirty_evidence.resolved_rev.as_deref(), Some("abc123"));

    let unavailable_evidence = build_git_revision_evidence(
        None,
        Err("missing rev".to_string()),
        Err("missing status".to_string()),
    );
    assert_eq!(unavailable_evidence.status, GIT_STATUS_UNAVAILABLE);
    assert!(unavailable_evidence.dirty);
    assert!(unavailable_evidence.resolved_rev.is_none());
    let expected_diagnostic_count = 2;
    assert_eq!(
        unavailable_evidence.diagnostics.len(),
        expected_diagnostic_count
    );
}

#[test]
fn git_revision_evidence_scopes_to_core_source_directory() {
    if !git_available() {
        return;
    }
    let root = git_fixture_root("scoped-revision");
    run_git_fixture(&root, &["init"]);
    run_git_fixture(&root, &["config", "user.email", TEST_GIT_USER_EMAIL]);
    run_git_fixture(&root, &["config", "user.name", TEST_GIT_USER_NAME]);
    let source_dir = root.join(TEST_STEVENARELLA_SUBTREE_DIR);
    fs::create_dir_all(&source_dir).expect("create core source dir");
    fs::write(
        source_dir.join(CARGO_MANIFEST_FILE),
        "[package]\nname = \"stevenarella\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("write core manifest");
    run_git_fixture(&root, &["add", TEST_STEVENARELLA_SUBTREE_DIR]);
    run_git_fixture(&root, &["commit", "-m", "add core client"]);
    let source_commit = git_rev_parse(&root, GIT_HEAD_REV).expect("source commit resolves");

    fs::write(root.join("README.md"), "parent docs\n").expect("write parent docs");
    run_git_fixture(&root, &["add", "README.md"]);
    run_git_fixture(&root, &["commit", "-m", "update parent docs"]);
    let parent_commit = git_rev_parse(&root, GIT_HEAD_REV).expect("parent commit resolves");
    assert_ne!(source_commit, parent_commit);

    fs::write(root.join("UNTRACKED_PARENT.txt"), "outside subtree\n")
        .expect("write unrelated parent dirt");
    let clean_evidence = git_revision_evidence(&source_dir, None);
    assert_eq!(clean_evidence.status, GIT_STATUS_CLEAN);
    assert!(!clean_evidence.dirty);
    assert_eq!(
        clean_evidence.resolved_rev.as_deref(),
        Some(source_commit.as_str())
    );

    fs::write(source_dir.join("UNTRACKED_SOURCE.txt"), "inside subtree\n")
        .expect("write source dirt");
    let dirty_evidence = git_revision_evidence(&source_dir, None);
    assert_eq!(dirty_evidence.status, GIT_STATUS_DIRTY);
    assert!(dirty_evidence.dirty);
    assert_eq!(
        dirty_evidence.resolved_rev.as_deref(),
        Some(source_commit.as_str())
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn valence_source_dir_uses_canonical_worktrees_and_rejects_transition_shapes() {
    let root = git_fixture_root("valence-source-dir");
    let direct = root.join("direct-valence");
    fs::create_dir_all(&direct).expect("create direct Valence worktree");
    fs::write(
        direct.join(CARGO_MANIFEST_FILE),
        "[package]\nname = \"valence\"\n",
    )
    .expect("write direct manifest");
    let mut direct_cfg = test_config(&[], &[]).expect("default config parses");
    direct_cfg.valence_worktree = direct.clone();
    assert_eq!(valence_source_dir(&direct_cfg).unwrap(), direct);

    let monorepo_transition = root.join("monorepo-transition-worktree");
    let transition_source = monorepo_transition
        .join("mc")
        .join(layout::VALENCE_TRANSITION_REL);
    fs::create_dir_all(&transition_source).expect("create transition Valence subtree");
    fs::write(
        transition_source.join(CARGO_MANIFEST_FILE),
        "[package]\nname = \"valence\"\n",
    )
    .expect("write transition manifest");
    let mut transition_cfg = test_config(&[], &[]).expect("default config parses");
    transition_cfg.valence_worktree = monorepo_transition;
    let err = valence_source_dir(&transition_cfg).expect_err("transition source root fails");
    assert!(err.contains("legacy Valence source root"), "{err}");
    assert!(err.contains("mc/valence"), "{err}");
    assert!(err.contains("mc/servers/valence"), "{err}");
    assert!(err.contains("migration action"), "{err}");

    let monorepo_role = root.join("monorepo-role-worktree");
    let role_source = monorepo_role.join("mc").join(layout::VALENCE_ROLE_REL);
    fs::create_dir_all(&role_source).expect("create role-based Valence subtree");
    fs::write(
        role_source.join(CARGO_MANIFEST_FILE),
        "[package]\nname = \"valence\"\n",
    )
    .expect("write role-based manifest");
    let mut role_cfg = test_config(&[], &[]).expect("default config parses");
    role_cfg.valence_worktree = monorepo_role;
    assert_eq!(valence_source_dir(&role_cfg).unwrap(), role_source);

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn backend_runtime_dispatch_preserves_pure_facts_and_log_labels() {
    let valence = test_config(&[], &[]).expect("default config parses");
    let paper = test_config(&["--server-backend=paper"], &[]).expect("paper backend config parses");

    assert_eq!(backend_name(ServerBackend::Valence), "valence");
    assert_eq!(backend_name(ServerBackend::Paper), "paper");
    assert_eq!(
        default_port(ServerBackend::Valence),
        VALENCE_DEFAULT_SERVER_PORT
    );
    assert_eq!(
        default_port(ServerBackend::Paper),
        PAPER_DEFAULT_SERVER_PORT
    );
    assert_eq!(
        server_log_label(&valence),
        valence.valence_log.display().to_string()
    );
    assert_eq!(
        server_log_label(&paper),
        format!("docker logs {}", paper.server_name)
    );
    assert!(
        world_persistence_state_dir(&valence, ServerBackend::Valence)
            .display()
            .to_string()
            .contains("valence"),
        "Valence persistence path uses stable backend name"
    );
    assert!(
        world_persistence_state_dir(&paper, ServerBackend::Paper)
            .display()
            .to_string()
            .contains("paper"),
        "Paper persistence path uses stable backend name"
    );
}

#[test]
fn backend_runtime_dry_run_lifecycle_uses_expected_managed_server_shape() {
    let temp_root =
        std::env::temp_dir().join(format!("mc-compat-backend-runtime-{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create backend runtime temp root");

    let mut valence = test_config(&[], &[]).expect("default config parses");
    valence.valence_repo = temp_root.join("valence-repo");
    valence.valence_worktree = temp_root.join("valence-worktree");
    valence.valence_pid_file = temp_root.join("valence.pid");
    fs::create_dir_all(&valence.valence_repo).expect("create fake Valence repo");
    let valence_server = start_server(&valence).expect("Valence dry-run lifecycle starts");
    assert!(valence_server.child.is_none());
    assert!(valence_server.paper_container.is_none());
    assert_eq!(valence_server.pid_file, valence.valence_pid_file);
    assert!(valence_server.keep);

    let mut paper =
        test_config(&["--server-backend=paper"], &[]).expect("paper backend config parses");
    paper.valence_pid_file = temp_root.join("paper.pid");
    let paper_server = start_server(&paper).expect("Paper dry-run lifecycle starts");
    assert!(paper_server.child.is_none());
    assert_eq!(
        paper_server.paper_container.as_deref(),
        Some(paper.server_name.as_str())
    );
    assert_eq!(paper_server.pid_file, paper.valence_pid_file);
    assert!(paper_server.keep);

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn cleanup_client_log_match_is_narrow() {
    assert!(is_mc_compat_client_log("mc-compat-client.123.log"));
    assert!(!is_mc_compat_client_log("mc-compat-client.123.txt"));
    assert!(!is_mc_compat_client_log("other-mc-compat-client.123.log"));
}

#[test]
fn cleanup_path_dry_run_preserves_existing_files() {
    let dir =
        std::env::temp_dir().join(format!("mc-compat-cleanup-dry-run-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create cleanup dry-run fixture");
    let file = dir.join("artifact.log");
    fs::write(&file, "keep me").expect("write cleanup fixture");

    cleanup_path("test artifact", &file, false).expect("dry-run cleanup succeeds");
    assert!(file.exists(), "dry-run cleanup must not remove files");

    cleanup_path("test artifact", &file, true).expect("apply cleanup removes file");
    assert!(!file.exists(), "apply cleanup removes files");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn missing_valence_checkout_has_actionable_diagnostic() {
    let missing =
        std::env::temp_dir().join(format!("mc-compat-missing-valence-{}", std::process::id()));
    let cfg = test_config(&["--valence-repo", missing.to_str().unwrap()], &[])
        .expect("config with missing Valence repo parses");

    let err = ensure_valence_repo_ready(&cfg).unwrap_err();

    assert!(err.contains("Valence source tree not found"), "{err}");
    assert!(err.contains("core server tree"), "{err}");
    assert!(err.contains("--valence-repo/VALENCE_REPO"), "{err}");
    assert!(!err.contains("transition"), "{err}");
}
