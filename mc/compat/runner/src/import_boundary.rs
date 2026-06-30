#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SourceFile {
    pub(crate) path: &'static str,
    pub(crate) text: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ImportBoundaryDiagnostic {
    pub(crate) path: &'static str,
    pub(crate) line: usize,
    pub(crate) import: &'static str,
    pub(crate) message: &'static str,
}

const ROOT_WILDCARD_IMPORT: &str = "use super::*;";
const CFG_TEST_ATTR: &str = "#[cfg(test)]";
const ATTRIBUTE_PREFIX: &str = "#[";
const MODULE_DECL_PREFIX: &str = "mod ";
const OPEN_BRACE: char = '{';
const CLOSE_BRACE: char = '}';
const LINE_NUMBER_BASE: usize = 1;
const ROOT_WILDCARD_MESSAGE: &str =
    "production runner modules must import dependencies explicitly instead of using `use super::*`";

pub(crate) fn production_root_wildcard_diagnostics(
    files: &[SourceFile],
) -> Vec<ImportBoundaryDiagnostic> {
    let mut diagnostics = Vec::new();
    for file in files {
        diagnostics.extend(root_wildcard_diagnostics(file));
    }
    diagnostics
}

fn root_wildcard_diagnostics(file: &SourceFile) -> Vec<ImportBoundaryDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut brace_depth = 0usize;
    let mut pending_test_cfg = false;
    let mut test_module_depth = None;

    for (line_index, line) in file.text.lines().enumerate() {
        let trimmed = line.trim();
        let starts_test_module = pending_test_cfg && starts_module_block(trimmed);
        if starts_test_module {
            test_module_depth = Some(brace_depth + line_open_brace_count(line));
        }

        if trimmed == ROOT_WILDCARD_IMPORT && test_module_depth.is_none() {
            diagnostics.push(ImportBoundaryDiagnostic {
                path: file.path,
                line: line_index + LINE_NUMBER_BASE,
                import: ROOT_WILDCARD_IMPORT,
                message: ROOT_WILDCARD_MESSAGE,
            });
        }

        brace_depth = next_brace_depth(brace_depth, line);
        if let Some(depth) = test_module_depth {
            if brace_depth < depth {
                test_module_depth = None;
            }
        }
        pending_test_cfg = next_pending_test_cfg(trimmed, pending_test_cfg, starts_test_module);
    }

    diagnostics
}

fn starts_module_block(trimmed: &str) -> bool {
    trimmed.starts_with(MODULE_DECL_PREFIX) && trimmed.contains(OPEN_BRACE)
}

fn next_pending_test_cfg(trimmed: &str, pending_test_cfg: bool, starts_test_module: bool) -> bool {
    if trimmed == CFG_TEST_ATTR {
        return true;
    }
    pending_test_cfg && trimmed.starts_with(ATTRIBUTE_PREFIX) && !starts_test_module
}

fn next_brace_depth(current: usize, line: &str) -> usize {
    let opens = line_open_brace_count(line);
    let closes = line_close_brace_count(line);
    current.saturating_add(opens).saturating_sub(closes)
}

fn line_open_brace_count(line: &str) -> usize {
    line.chars().filter(|ch| *ch == OPEN_BRACE).count()
}

fn line_close_brace_count(line: &str) -> usize {
    line.chars().filter(|ch| *ch == CLOSE_BRACE).count()
}

const PRODUCTION_SOURCES: &[SourceFile] = &[
    SourceFile {
        path: "compat/runner/src/backend_shell.rs",
        text: include_str!("backend_shell.rs"),
    },
    SourceFile {
        path: "compat/runner/src/client_driver.rs",
        text: include_str!("client_driver.rs"),
    },
    SourceFile {
        path: "compat/runner/src/config_patches.rs",
        text: include_str!("config_patches.rs"),
    },
    SourceFile {
        path: "compat/runner/src/env_patch.rs",
        text: include_str!("env_patch.rs"),
    },
    SourceFile {
        path: "compat/runner/src/evidence_bundle.rs",
        text: include_str!("evidence_bundle.rs"),
    },
    SourceFile {
        path: "compat/runner/src/evidence_core.rs",
        text: include_str!("evidence_core.rs"),
    },
    SourceFile {
        path: "compat/runner/src/evidence_receipts.rs",
        text: include_str!("evidence_receipts.rs"),
    },
    SourceFile {
        path: "compat/runner/src/evidence_types.rs",
        text: include_str!("evidence_types.rs"),
    },
    SourceFile {
        path: "compat/runner/src/json_support.rs",
        text: include_str!("json_support.rs"),
    },
    SourceFile {
        path: "compat/runner/src/layout.rs",
        text: include_str!("layout.rs"),
    },
    SourceFile {
        path: "compat/runner/src/main.rs",
        text: include_str!("main.rs"),
    },
    SourceFile {
        path: "compat/runner/src/planning.rs",
        text: include_str!("planning.rs"),
    },
    SourceFile {
        path: "compat/runner/src/receipt_validation.rs",
        text: include_str!("receipt_validation.rs"),
    },
    SourceFile {
        path: "compat/runner/src/receipts.rs",
        text: include_str!("receipts.rs"),
    },
    SourceFile {
        path: "compat/runner/src/runner_config.rs",
        text: include_str!("runner_config.rs"),
    },
    SourceFile {
        path: "compat/runner/src/runtime_config.rs",
        text: include_str!("runtime_config.rs"),
    },
    SourceFile {
        path: "compat/runner/src/scenario_behavior_metadata.rs",
        text: include_str!("scenario_behavior_metadata.rs"),
    },
    SourceFile {
        path: "compat/runner/src/scenario_catalog.rs",
        text: include_str!("scenario_catalog.rs"),
    },
    SourceFile {
        path: "compat/runner/src/scenario_contracts_generated.rs",
        text: include_str!("scenario_contracts_generated.rs"),
    },
    SourceFile {
        path: "compat/runner/src/scenario_core.rs",
        text: include_str!("scenario_core.rs"),
    },
    SourceFile {
        path: "compat/runner/src/scenario_manifest_generated.rs",
        text: include_str!("scenario_manifest_generated.rs"),
    },
    SourceFile {
        path: "compat/runner/src/wire.rs",
        text: include_str!("wire.rs"),
    },
];

#[test]
fn current_production_runner_modules_use_explicit_imports() {
    let diagnostics = production_root_wildcard_diagnostics(PRODUCTION_SOURCES);
    assert_eq!(diagnostics, Vec::new(), "{diagnostics:?}");
}

#[test]
fn import_boundary_accepts_explicit_production_and_scoped_test_imports() {
    let file = SourceFile {
        path: "fixture/allowed.rs",
        text: r#"
use crate::runner_config::Config;

fn production_dependency(_: &Config) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoped_test_import_is_allowed() {
        let _ = production_dependency;
    }
}
"#,
    };

    let diagnostics = production_root_wildcard_diagnostics(&[file]);
    assert_eq!(diagnostics, Vec::new(), "{diagnostics:?}");
}

#[test]
fn import_boundary_rejects_production_root_wildcard_imports() {
    const PRODUCTION_WILDCARD_LINE: usize = 2;
    let file = SourceFile {
        path: "fixture/regression.rs",
        text: r#"
use super::*;

fn production_regression() {}
"#,
    };

    let diagnostics = production_root_wildcard_diagnostics(&[file]);
    assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
    assert_eq!(diagnostics[0].path, "fixture/regression.rs");
    assert_eq!(diagnostics[0].line, PRODUCTION_WILDCARD_LINE);
    assert!(
        diagnostics[0]
            .message
            .contains("import dependencies explicitly"),
        "{diagnostics:?}"
    );
}
