use std::path::{Path, PathBuf};

const CARGO_MANIFEST_FILE: &str = "Cargo.toml";
const GIT_DIR: &str = ".git";
const MONOREPO_MC_ROOT_REL: &str = "mc";
const MONOREPO_VALENCE_ROLE_REL: &str = "mc/servers/valence";
const MONOREPO_VALENCE_TRANSITION_REL: &str = "mc/valence";

pub const CLIENT_ROLE_REL: &str = "clients/stevenarella";
pub const CLIENT_TRANSITION_REL: &str = "stevenarella";
pub const VALENCE_ROLE_REL: &str = "servers/valence";
pub const VALENCE_TRANSITION_REL: &str = "valence";
pub const COMPAT_RUNNER_ROLE_REL: &str = "compat/runner";
pub const COMPAT_RUNNER_TRANSITION_REL: &str = "tools/mc-compat-runner";
pub const COMPAT_CONFIG_ROLE_REL: &str = "compat/config";
pub const COMPAT_CONFIG_TRANSITION_REL: &str = "config/mc-compat";
pub const PAPER_SURVIVAL_FIXTURE_ROLE_REL: &str = "compat/fixtures/paper-survival";
pub const PAPER_SURVIVAL_FIXTURE_TRANSITION_REL: &str = "tools/paper-survival-fixture";

const CLIENT_COMPONENT: &str = "client";
const VALENCE_COMPONENT: &str = "valence-server";
const COMPAT_RUNNER_COMPONENT: &str = "compat-runner";
const COMPAT_CONFIG_COMPONENT: &str = "compat-config";
const PAPER_SURVIVAL_FIXTURE_COMPONENT: &str = "paper-survival-fixture";

const CLIENT_MIGRATION_ACTION: &str =
    "move the client tree to clients/stevenarella or pass --client-dir/CLIENT_DIR";
const VALENCE_MIGRATION_ACTION: &str =
    "move the server tree to servers/valence or pass --valence-repo/VALENCE_REPO";
const COMPAT_RUNNER_MIGRATION_ACTION: &str = "move the runner tree to compat/runner";
const COMPAT_CONFIG_MIGRATION_ACTION: &str = "move checked configuration to compat/config";
const PAPER_SURVIVAL_FIXTURE_MIGRATION_ACTION: &str =
    "move the Paper survival fixture to compat/fixtures/paper-survival";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutResolutionMode {
    AllowMissing,
    RequireExisting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutKind {
    RoleBased,
    Transition,
    MissingRoleBasedDefault,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentRoot {
    pub name: &'static str,
    pub path: PathBuf,
    pub relative_path: &'static str,
    pub kind: LayoutKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLayout {
    pub client: ComponentRoot,
    pub valence: ComponentRoot,
    pub compat_runner: ComponentRoot,
    pub compat_config: ComponentRoot,
    pub paper_survival_fixture: ComponentRoot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ComponentSpec {
    name: &'static str,
    role_relative: &'static str,
    transition_relative: &'static str,
    migration_action: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ComponentProbe {
    spec: ComponentSpec,
    role_exists: bool,
    transition_exists: bool,
    role_nested_git: bool,
    transition_nested_git: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ComponentSelection {
    relative_path: &'static str,
    kind: LayoutKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValenceSourceRootKind {
    RoleBased,
    Transition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ValenceSourceCandidate {
    path: PathBuf,
    relative_path: &'static str,
    kind: ValenceSourceRootKind,
    nested_git: bool,
}

const CLIENT_SPEC: ComponentSpec = ComponentSpec {
    name: CLIENT_COMPONENT,
    role_relative: CLIENT_ROLE_REL,
    transition_relative: CLIENT_TRANSITION_REL,
    migration_action: CLIENT_MIGRATION_ACTION,
};
const VALENCE_SPEC: ComponentSpec = ComponentSpec {
    name: VALENCE_COMPONENT,
    role_relative: VALENCE_ROLE_REL,
    transition_relative: VALENCE_TRANSITION_REL,
    migration_action: VALENCE_MIGRATION_ACTION,
};
const COMPAT_RUNNER_SPEC: ComponentSpec = ComponentSpec {
    name: COMPAT_RUNNER_COMPONENT,
    role_relative: COMPAT_RUNNER_ROLE_REL,
    transition_relative: COMPAT_RUNNER_TRANSITION_REL,
    migration_action: COMPAT_RUNNER_MIGRATION_ACTION,
};
const COMPAT_CONFIG_SPEC: ComponentSpec = ComponentSpec {
    name: COMPAT_CONFIG_COMPONENT,
    role_relative: COMPAT_CONFIG_ROLE_REL,
    transition_relative: COMPAT_CONFIG_TRANSITION_REL,
    migration_action: COMPAT_CONFIG_MIGRATION_ACTION,
};
const PAPER_SURVIVAL_FIXTURE_SPEC: ComponentSpec = ComponentSpec {
    name: PAPER_SURVIVAL_FIXTURE_COMPONENT,
    role_relative: PAPER_SURVIVAL_FIXTURE_ROLE_REL,
    transition_relative: PAPER_SURVIVAL_FIXTURE_TRANSITION_REL,
    migration_action: PAPER_SURVIVAL_FIXTURE_MIGRATION_ACTION,
};

pub fn resolve_repository_layout(
    root: &Path,
    mode: LayoutResolutionMode,
) -> Result<SourceLayout, String> {
    Ok(SourceLayout {
        client: resolve_component_root(root, component_probe(root, CLIENT_SPEC), mode)?,
        valence: resolve_component_root(root, component_probe(root, VALENCE_SPEC), mode)?,
        compat_runner: resolve_component_root(
            root,
            component_probe(root, COMPAT_RUNNER_SPEC),
            mode,
        )?,
        compat_config: resolve_component_root(
            root,
            component_probe(root, COMPAT_CONFIG_SPEC),
            mode,
        )?,
        paper_survival_fixture: resolve_component_root(
            root,
            component_probe(root, PAPER_SURVIVAL_FIXTURE_SPEC),
            mode,
        )?,
    })
}

pub fn resolve_valence_source_dir(worktree: &Path) -> Result<PathBuf, String> {
    if worktree.join(CARGO_MANIFEST_FILE).exists() {
        return Ok(worktree.to_path_buf());
    }

    let candidates = valence_source_candidates(worktree);
    resolve_valence_source_selection(worktree, &candidates)
}

fn valence_source_candidates(worktree: &Path) -> Vec<ValenceSourceCandidate> {
    let candidates = [
        (
            worktree.join(MONOREPO_VALENCE_ROLE_REL),
            MONOREPO_VALENCE_ROLE_REL,
            ValenceSourceRootKind::RoleBased,
        ),
        (
            worktree.join(MONOREPO_VALENCE_TRANSITION_REL),
            MONOREPO_VALENCE_TRANSITION_REL,
            ValenceSourceRootKind::Transition,
        ),
        (
            worktree.join(VALENCE_ROLE_REL),
            VALENCE_ROLE_REL,
            ValenceSourceRootKind::RoleBased,
        ),
        (
            worktree.join(VALENCE_TRANSITION_REL),
            VALENCE_TRANSITION_REL,
            ValenceSourceRootKind::Transition,
        ),
    ];

    candidates
        .into_iter()
        .filter_map(|(path, relative_path, kind)| {
            if path.join(CARGO_MANIFEST_FILE).exists() {
                Some(ValenceSourceCandidate {
                    nested_git: path.join(GIT_DIR).exists(),
                    path,
                    relative_path,
                    kind,
                })
            } else {
                None
            }
        })
        .collect()
}

fn resolve_valence_source_selection(
    worktree: &Path,
    candidates: &[ValenceSourceCandidate],
) -> Result<PathBuf, String> {
    for candidate in candidates {
        if candidate.nested_git {
            return Err(format!(
                "Valence source root {} contains nested Git directory {}; expected canonical role path {}; migration action: {}",
                candidate.path.display(),
                candidate.path.join(GIT_DIR).display(),
                MONOREPO_VALENCE_ROLE_REL,
                VALENCE_MIGRATION_ACTION
            ));
        }
    }

    let role_matches = candidates
        .iter()
        .filter(|candidate| candidate.kind == ValenceSourceRootKind::RoleBased)
        .collect::<Vec<_>>();
    let transition_matches = candidates
        .iter()
        .filter(|candidate| candidate.kind == ValenceSourceRootKind::Transition)
        .collect::<Vec<_>>();

    if !role_matches.is_empty() && !transition_matches.is_empty() {
        return Err(format!(
            "ambiguous Valence source roots under {}: canonical role path(s) {} and legacy transition path(s) {}; migration action: remove legacy transition roots and keep {}",
            worktree.display(),
            join_valence_candidate_paths(&role_matches),
            join_valence_candidate_paths(&transition_matches),
            MONOREPO_VALENCE_ROLE_REL
        ));
    }

    match role_matches.len() {
        0 => {}
        1 => return Ok(role_matches[0].path.clone()),
        _ => {
            return Err(format!(
                "ambiguous canonical Valence source roots under {}: {}",
                worktree.display(),
                join_valence_candidate_paths(&role_matches)
            ));
        }
    }

    if !transition_matches.is_empty() {
        return Err(format!(
            "legacy Valence source root {} is no longer accepted; expected canonical role path {}; migration action: {}",
            join_valence_candidate_paths(&transition_matches),
            MONOREPO_VALENCE_ROLE_REL,
            VALENCE_MIGRATION_ACTION
        ));
    }

    Ok(worktree.to_path_buf())
}

fn join_valence_candidate_paths(candidates: &[&ValenceSourceCandidate]) -> String {
    candidates
        .iter()
        .map(|candidate| format!("{} ({})", candidate.path.display(), candidate.relative_path))
        .collect::<Vec<_>>()
        .join(", ")
}

fn component_probe(root: &Path, spec: ComponentSpec) -> ComponentProbe {
    let role_path = root.join(spec.role_relative);
    let transition_path = root.join(spec.transition_relative);
    ComponentProbe {
        spec,
        role_exists: role_path.exists(),
        transition_exists: transition_path.exists(),
        role_nested_git: role_path.join(GIT_DIR).exists(),
        transition_nested_git: transition_path.join(GIT_DIR).exists(),
    }
}

fn resolve_component_root(
    root: &Path,
    probe: ComponentProbe,
    mode: LayoutResolutionMode,
) -> Result<ComponentRoot, String> {
    let selection = resolve_component_selection(probe, mode)?;
    Ok(ComponentRoot {
        name: probe.spec.name,
        path: root.join(selection.relative_path),
        relative_path: selection.relative_path,
        kind: selection.kind,
    })
}

fn resolve_component_selection(
    probe: ComponentProbe,
    mode: LayoutResolutionMode,
) -> Result<ComponentSelection, String> {
    if probe.role_exists && probe.transition_exists {
        return Err(format!(
            "ambiguous {} roots: both canonical role path {} and legacy transition path {} exist; migration action: remove {} or move its contents under {}",
            probe.spec.name,
            probe.spec.role_relative,
            probe.spec.transition_relative,
            probe.spec.transition_relative,
            probe.spec.role_relative
        ));
    }
    if probe.role_exists && probe.role_nested_git {
        return Err(format!(
            "{} role-based root {} contains nested Git directory",
            probe.spec.name, probe.spec.role_relative
        ));
    }
    if probe.transition_exists && probe.transition_nested_git {
        return Err(format!(
            "{} legacy transition root {} contains nested Git directory; expected canonical role path {}; migration action: {}",
            probe.spec.name,
            probe.spec.transition_relative,
            probe.spec.role_relative,
            probe.spec.migration_action
        ));
    }
    if probe.role_exists {
        return Ok(ComponentSelection {
            relative_path: probe.spec.role_relative,
            kind: LayoutKind::RoleBased,
        });
    }
    if probe.transition_exists {
        return Err(format!(
            "legacy {} root {} is no longer accepted; expected canonical role path {}; migration action: {}",
            probe.spec.name,
            probe.spec.transition_relative,
            probe.spec.role_relative,
            probe.spec.migration_action
        ));
    }
    match mode {
        LayoutResolutionMode::AllowMissing => Ok(ComponentSelection {
            relative_path: probe.spec.role_relative,
            kind: LayoutKind::MissingRoleBasedDefault,
        }),
        LayoutResolutionMode::RequireExisting => Err(format!(
            "missing {} root: expected canonical role path {}; migration action: {}",
            probe.spec.name, probe.spec.role_relative, probe.spec.migration_action
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    const TEST_VALENCE_MANIFEST: &str = "[package]\nname = \"valence\"\n";

    fn temp_layout_root(label: &str) -> PathBuf {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is after Unix epoch")
            .as_millis();
        let root = std::env::temp_dir().join(format!(
            "mc-compat-layout-{label}-{}-{millis}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create layout root");
        root
    }

    fn create_dir(root: &Path, rel: &str) {
        fs::create_dir_all(root.join(rel)).expect("create layout component dir");
    }

    fn create_manifest(root: &Path, rel: &str, manifest: &str) {
        create_dir(root, rel);
        fs::write(root.join(rel).join(CARGO_MANIFEST_FILE), manifest).expect("write manifest");
    }

    #[test]
    fn repository_layout_prefers_role_paths_when_present() {
        let root = temp_layout_root("role");
        create_dir(&root, CLIENT_ROLE_REL);
        create_dir(&root, VALENCE_ROLE_REL);
        create_dir(&root, COMPAT_RUNNER_ROLE_REL);
        create_dir(&root, COMPAT_CONFIG_ROLE_REL);
        create_dir(&root, PAPER_SURVIVAL_FIXTURE_ROLE_REL);

        let layout = resolve_repository_layout(&root, LayoutResolutionMode::RequireExisting)
            .expect("role layout resolves");

        assert_eq!(layout.client.relative_path, CLIENT_ROLE_REL);
        assert_eq!(layout.client.kind, LayoutKind::RoleBased);
        assert_eq!(layout.valence.relative_path, VALENCE_ROLE_REL);
        assert_eq!(layout.compat_runner.relative_path, COMPAT_RUNNER_ROLE_REL);
        assert_eq!(layout.compat_config.relative_path, COMPAT_CONFIG_ROLE_REL);
        assert_eq!(
            layout.paper_survival_fixture.relative_path,
            PAPER_SURVIVAL_FIXTURE_ROLE_REL
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn repository_layout_rejects_transition_paths_with_migration_diagnostic() {
        let root = temp_layout_root("transition");
        create_dir(&root, CLIENT_TRANSITION_REL);
        create_dir(&root, VALENCE_TRANSITION_REL);
        create_dir(&root, COMPAT_RUNNER_TRANSITION_REL);
        create_dir(&root, COMPAT_CONFIG_TRANSITION_REL);
        create_dir(&root, PAPER_SURVIVAL_FIXTURE_TRANSITION_REL);

        let required_err = resolve_repository_layout(&root, LayoutResolutionMode::RequireExisting)
            .expect_err("transition layout is no longer accepted");
        assert!(
            required_err.contains("legacy client root"),
            "{required_err}"
        );
        assert!(
            required_err.contains(CLIENT_TRANSITION_REL),
            "{required_err}"
        );
        assert!(required_err.contains(CLIENT_ROLE_REL), "{required_err}");
        assert!(required_err.contains("migration action"), "{required_err}");

        let default_err = resolve_repository_layout(&root, LayoutResolutionMode::AllowMissing)
            .expect_err("allow-missing defaults still reject transition roots");
        assert!(default_err.contains("legacy client root"), "{default_err}");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn component_selection_rejects_each_transition_root() {
        let cases = [
            CLIENT_SPEC,
            VALENCE_SPEC,
            COMPAT_RUNNER_SPEC,
            COMPAT_CONFIG_SPEC,
            PAPER_SURVIVAL_FIXTURE_SPEC,
        ];

        for spec in cases {
            let err = resolve_component_selection(
                ComponentProbe {
                    spec,
                    role_exists: false,
                    transition_exists: true,
                    role_nested_git: false,
                    transition_nested_git: false,
                },
                LayoutResolutionMode::RequireExisting,
            )
            .expect_err("transition root fails closed");
            assert!(err.contains("legacy"), "{err}");
            assert!(err.contains(spec.transition_relative), "{err}");
            assert!(err.contains(spec.role_relative), "{err}");
            assert!(err.contains("migration action"), "{err}");
        }
    }

    #[test]
    fn repository_layout_fails_closed_for_missing_ambiguous_and_nested_git_roots() {
        let missing = temp_layout_root("missing");
        let err = resolve_repository_layout(&missing, LayoutResolutionMode::RequireExisting)
            .expect_err("missing roots fail");
        assert!(err.contains("missing client root"), "{err}");
        assert!(err.contains(CLIENT_ROLE_REL), "{err}");
        assert!(err.contains("migration action"), "{err}");
        assert!(!err.contains(" or stevenarella"), "{err}");
        let _ = fs::remove_dir_all(&missing);

        let ambiguous = temp_layout_root("ambiguous");
        create_dir(&ambiguous, CLIENT_ROLE_REL);
        create_dir(&ambiguous, CLIENT_TRANSITION_REL);
        create_dir(&ambiguous, VALENCE_ROLE_REL);
        create_dir(&ambiguous, COMPAT_RUNNER_ROLE_REL);
        create_dir(&ambiguous, COMPAT_CONFIG_ROLE_REL);
        create_dir(&ambiguous, PAPER_SURVIVAL_FIXTURE_ROLE_REL);
        let err = resolve_repository_layout(&ambiguous, LayoutResolutionMode::RequireExisting)
            .expect_err("duplicate roots fail");
        assert!(err.contains("ambiguous client roots"), "{err}");
        assert!(err.contains("canonical role path"), "{err}");
        assert!(err.contains(CLIENT_TRANSITION_REL), "{err}");
        let _ = fs::remove_dir_all(&ambiguous);

        let nested = temp_layout_root("nested");
        create_dir(&nested, CLIENT_ROLE_REL);
        create_dir(&nested, VALENCE_ROLE_REL);
        create_dir(&nested, COMPAT_RUNNER_ROLE_REL);
        create_dir(&nested, COMPAT_CONFIG_ROLE_REL);
        create_dir(&nested, PAPER_SURVIVAL_FIXTURE_ROLE_REL);
        create_dir(&nested, &format!("{CLIENT_ROLE_REL}/{GIT_DIR}"));
        let err = resolve_repository_layout(&nested, LayoutResolutionMode::RequireExisting)
            .expect_err("nested Git roots fail");
        assert!(err.contains("nested Git directory"), "{err}");
        let _ = fs::remove_dir_all(&nested);
    }

    #[test]
    fn repository_layout_allows_missing_defaults_to_role_paths() {
        let root = temp_layout_root("allow-missing");

        let layout = resolve_repository_layout(&root, LayoutResolutionMode::AllowMissing)
            .expect("allow-missing layout resolves");

        assert_eq!(layout.client.relative_path, CLIENT_ROLE_REL);
        assert_eq!(layout.client.kind, LayoutKind::MissingRoleBasedDefault);
        assert_eq!(layout.valence.relative_path, VALENCE_ROLE_REL);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn valence_source_dir_detects_direct_and_role_worktree_shapes() {
        let root = temp_layout_root("valence-source");
        let direct = root.join("direct-valence");
        create_manifest(&direct, "", TEST_VALENCE_MANIFEST);
        assert_eq!(resolve_valence_source_dir(&direct).unwrap(), direct);

        let monorepo_role = root.join("monorepo-role");
        create_manifest(
            &monorepo_role,
            &format!("{MONOREPO_MC_ROOT_REL}/{VALENCE_ROLE_REL}"),
            TEST_VALENCE_MANIFEST,
        );
        assert_eq!(
            resolve_valence_source_dir(&monorepo_role).unwrap(),
            monorepo_role
                .join(MONOREPO_MC_ROOT_REL)
                .join(VALENCE_ROLE_REL)
        );

        let mc_root_role = root.join("mc-root-role");
        create_manifest(&mc_root_role, VALENCE_ROLE_REL, TEST_VALENCE_MANIFEST);
        assert_eq!(
            resolve_valence_source_dir(&mc_root_role).unwrap(),
            mc_root_role.join(VALENCE_ROLE_REL)
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn valence_source_dir_rejects_transition_worktree_shapes() {
        let root = temp_layout_root("valence-transition-source");
        let monorepo_transition = root.join("monorepo-transition");
        create_manifest(
            &monorepo_transition,
            &format!("{MONOREPO_MC_ROOT_REL}/{VALENCE_TRANSITION_REL}"),
            TEST_VALENCE_MANIFEST,
        );
        let err = resolve_valence_source_dir(&monorepo_transition)
            .expect_err("monorepo transition root fails closed");
        assert!(err.contains("legacy Valence source root"), "{err}");
        assert!(err.contains(MONOREPO_VALENCE_TRANSITION_REL), "{err}");
        assert!(err.contains(MONOREPO_VALENCE_ROLE_REL), "{err}");
        assert!(err.contains("migration action"), "{err}");

        let mc_root_transition = root.join("mc-root-transition");
        create_manifest(
            &mc_root_transition,
            VALENCE_TRANSITION_REL,
            TEST_VALENCE_MANIFEST,
        );
        let err = resolve_valence_source_dir(&mc_root_transition)
            .expect_err("mc-root transition root fails closed");
        assert!(err.contains(VALENCE_TRANSITION_REL), "{err}");
        assert!(err.contains(MONOREPO_VALENCE_ROLE_REL), "{err}");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn valence_source_dir_fails_for_ambiguous_or_nested_component_roots() {
        let root = temp_layout_root("valence-negative");
        let ambiguous = root.join("ambiguous");
        create_manifest(
            &ambiguous,
            &format!("{MONOREPO_MC_ROOT_REL}/{VALENCE_ROLE_REL}"),
            TEST_VALENCE_MANIFEST,
        );
        create_manifest(
            &ambiguous,
            &format!("{MONOREPO_MC_ROOT_REL}/{VALENCE_TRANSITION_REL}"),
            TEST_VALENCE_MANIFEST,
        );
        let err = resolve_valence_source_dir(&ambiguous).expect_err("ambiguous roots fail");
        assert!(err.contains("ambiguous Valence source roots"), "{err}");

        let nested = root.join("nested");
        create_manifest(
            &nested,
            &format!("{MONOREPO_MC_ROOT_REL}/{VALENCE_ROLE_REL}"),
            TEST_VALENCE_MANIFEST,
        );
        create_dir(
            &nested,
            &format!("{MONOREPO_MC_ROOT_REL}/{VALENCE_ROLE_REL}/{GIT_DIR}"),
        );
        let err = resolve_valence_source_dir(&nested).expect_err("nested roots fail");
        assert!(err.contains("nested Git directory"), "{err}");
        let _ = fs::remove_dir_all(&root);
    }
}
