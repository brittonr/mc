#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-component-registry-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const ROOT_FLAG: &str = "--root";
const REGISTRY_FLAG: &str = "--registry";
const SELF_TEST_FLAG: &str = "--self-test";
const DEFAULT_ROOT: &str = ".";
const REGISTRY_PATH: &str = "compat/config/component-registry.ncl";
const CHECKLIST_PATH: &str = "docs/layout-checklist.md";
const SUCCESS_MESSAGE: &str = "component registry checks passed";
const SELF_TEST_SUCCESS_MESSAGE: &str = "component registry self-test passed";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const PATH_SEPARATOR: char = '/';
const ROOT_PATH_SUFFIX: &str = "/";
const GIT_BOUNDARY_SUFFIX: &str = "/.git";
const GIT_ENTRY_NAME: &str = ".git";
const JJ_ENTRY_NAME: &str = ".jj";
const TARGET_DIR_NAME: &str = "target";
const RESULT_LINK_NAME: &str = "result";
const RESULT_LINK_PREFIX: &str = "result-";
const COMPONENTS_START: &str = "components = [";
const IGNORED_WALK_DIRS: &[&str] = &[".agent", ".direnv", ".pi"];
const ARRAY_END_TOKEN: &str = "]";
const ROW_START_TOKEN: &str = "{";
const ROW_END_TOKEN: &str = "}";
const ROW_END_COMMA_TOKEN: &str = "},";
const ASSIGNMENT_SEPARATOR: char = '=';
const QUOTE: char = '"';
const ESCAPE: char = '\\';
const COMMENT: char = '#';
const OPEN_BRACKET: char = '[';
const CLOSE_BRACKET: char = ']';
const PATH_FIELD: &str = "path";
const ROLE_FIELD: &str = "role";
const OWNER_FIELD: &str = "owner";
const VCS_BOUNDARY_FIELD: &str = "vcs_boundary";
const COMMANDS_FIELD: &str = "commands";
const DEFAULT_GATE_FIELD: &str = "default_gate_participation";
const EVIDENCE_POLICY_FIELD: &str = "evidence_policy";
const LOCAL_NOTES_FIELD: &str = "local_notes";
const NESTED_GIT_FIELD: &str = "nested_git_exception";
const TRUE_LITERAL: &str = "true";
const FALSE_LITERAL: &str = "false";
const INCLUDED_GATE: &str = "included";
const SELECTED_GATE: &str = "selected";
const EXCLUDED_GATE: &str = "excluded";
const CHECKLIST_HEADER: &str = "Path | Role | Ownership | Local notes | Default gates";
const CHECKLIST_SEPARATOR: &str = "---";
const CHECKLIST_EXPECTED_CELL_COUNT: usize = 5;
const CHECKLIST_PATH_CELL_INDEX: usize = 0;
const CHECKLIST_DEFAULT_GATES_CELL_INDEX: usize = 4;

const REQUIRED_COMPONENT_PATHS: &[&str] = &[
    "clients/stevenarella/",
    "servers/valence/",
    "compat/runner/",
    "compat/config/",
    "compat/fixtures/paper-survival/",
    "cairn/",
    "docs/evidence/",
    "Leafish/",
];

const OWNER_VALUES: &[&str] = &[
    "parent-repository",
    "independent-nested-repository",
    "reference-only-checkout",
];
const VCS_BOUNDARY_VALUES: &[&str] = &["parent-git", "nested-git-and-jj", "nested-git"];
const GATE_VALUES: &[&str] = &[INCLUDED_GATE, SELECTED_GATE, EXCLUDED_GATE];
const EVIDENCE_VALUES: &[&str] = &[
    "copy-review-artifacts",
    "generated-checked-in",
    "durable-evidence-root",
    "reference-excluded",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComponentRow {
    start_line: usize,
    path: Option<String>,
    role: Option<String>,
    owner: Option<String>,
    vcs_boundary: Option<String>,
    commands: Vec<String>,
    default_gate_participation: Option<String>,
    evidence_policy: Option<String>,
    local_notes: Vec<String>,
    nested_git_exception: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChecklistEntry {
    path: String,
    default_gates: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepositoryModel {
    paths: BTreeSet<String>,
    nested_git_roots: BTreeSet<String>,
    checklist: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    code: &'static str,
    path: String,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    diagnostics: Vec<Diagnostic>,
}

impl ComponentRow {
    fn new(start_line: usize) -> Self {
        Self {
            start_line,
            path: None,
            role: None,
            owner: None,
            vcs_boundary: None,
            commands: Vec::new(),
            default_gate_participation: None,
            evidence_policy: None,
            local_notes: Vec::new(),
            nested_git_exception: None,
        }
    }
}

fn diagnostic(
    code: &'static str,
    path: impl Into<String>,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic {
        code,
        path: path.into(),
        message: message.into(),
    }
}

fn validate_registry(rows: &[ComponentRow], model: &RepositoryModel) -> Report {
    let checklist_entries = parse_checklist_entries(&model.checklist);
    let mut diagnostics = Vec::new();

    diagnostics.extend(check_required_fields(rows));
    diagnostics.extend(check_path_safety(rows));
    diagnostics.extend(check_enum_values(rows));
    diagnostics.extend(check_unique_roles(rows));
    diagnostics.extend(check_unique_paths(rows));
    diagnostics.extend(check_required_inventory(rows));
    diagnostics.extend(check_observed_paths(rows, model));
    diagnostics.extend(check_local_notes(rows, model));
    diagnostics.extend(check_nested_git_boundaries(rows, model));
    diagnostics.extend(check_checklist_sync(rows, &checklist_entries));

    diagnostics.sort_by(|left, right| {
        left.code
            .cmp(right.code)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.message.cmp(&right.message))
    });

    Report { diagnostics }
}

fn check_required_fields(rows: &[ComponentRow]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for row in rows {
        let identity = row_identity(row);
        push_missing_option(&mut diagnostics, row, &identity, PATH_FIELD, &row.path);
        push_missing_option(&mut diagnostics, row, &identity, ROLE_FIELD, &row.role);
        push_missing_option(&mut diagnostics, row, &identity, OWNER_FIELD, &row.owner);
        push_missing_option(
            &mut diagnostics,
            row,
            &identity,
            VCS_BOUNDARY_FIELD,
            &row.vcs_boundary,
        );
        push_missing_array(&mut diagnostics, row, &identity, COMMANDS_FIELD, &row.commands);
        push_missing_option(
            &mut diagnostics,
            row,
            &identity,
            DEFAULT_GATE_FIELD,
            &row.default_gate_participation,
        );
        push_missing_option(
            &mut diagnostics,
            row,
            &identity,
            EVIDENCE_POLICY_FIELD,
            &row.evidence_policy,
        );
        push_missing_array(
            &mut diagnostics,
            row,
            &identity,
            LOCAL_NOTES_FIELD,
            &row.local_notes,
        );
        if row.nested_git_exception.is_none() {
            diagnostics.push(diagnostic(
                "missing_field",
                identity,
                format!(
                    "registry row starting at line {} is missing required field {NESTED_GIT_FIELD}",
                    row.start_line
                ),
            ));
        }
    }
    diagnostics
}

fn push_missing_option(
    diagnostics: &mut Vec<Diagnostic>,
    row: &ComponentRow,
    identity: &str,
    field: &'static str,
    value: &Option<String>,
) {
    if value.as_deref().map_or(true, str::is_empty) {
        diagnostics.push(diagnostic(
            "missing_field",
            identity,
            format!(
                "registry row starting at line {} is missing required field {field}",
                row.start_line
            ),
        ));
    }
}

fn push_missing_array(
    diagnostics: &mut Vec<Diagnostic>,
    row: &ComponentRow,
    identity: &str,
    field: &'static str,
    value: &[String],
) {
    if value.is_empty() {
        diagnostics.push(diagnostic(
            "missing_field",
            identity,
            format!(
                "registry row starting at line {} is missing required non-empty field {field}",
                row.start_line
            ),
        ));
    }
}

fn check_path_safety(rows: &[ComponentRow]) -> Vec<Diagnostic> {
    rows.iter()
        .filter_map(|row| {
            let path = row.path.as_deref()?;
            if is_safe_component_path(path) {
                None
            } else {
                Some(diagnostic(
                    "unsafe_path",
                    row_identity(row),
                    format!("component path {path:?} must be a safe repository-relative directory"),
                ))
            }
        })
        .collect()
}

fn check_enum_values(rows: &[ComponentRow]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for row in rows {
        push_invalid_enum(
            &mut diagnostics,
            row,
            OWNER_FIELD,
            &row.owner,
            OWNER_VALUES,
            "invalid_owner",
        );
        push_invalid_enum(
            &mut diagnostics,
            row,
            VCS_BOUNDARY_FIELD,
            &row.vcs_boundary,
            VCS_BOUNDARY_VALUES,
            "invalid_vcs_boundary",
        );
        push_invalid_enum(
            &mut diagnostics,
            row,
            DEFAULT_GATE_FIELD,
            &row.default_gate_participation,
            GATE_VALUES,
            "invalid_default_gate_participation",
        );
        push_invalid_enum(
            &mut diagnostics,
            row,
            EVIDENCE_POLICY_FIELD,
            &row.evidence_policy,
            EVIDENCE_VALUES,
            "invalid_evidence_policy",
        );
    }
    diagnostics
}

fn push_invalid_enum(
    diagnostics: &mut Vec<Diagnostic>,
    row: &ComponentRow,
    field: &'static str,
    value: &Option<String>,
    allowed: &[&str],
    code: &'static str,
) {
    let Some(value) = value.as_deref() else {
        return;
    };
    if allowed.iter().any(|allowed_value| *allowed_value == value) {
        return;
    }
    diagnostics.push(diagnostic(
        code,
        row_identity(row),
        format!("field {field} has invalid value {value:?}"),
    ));
}

fn check_unique_roles(rows: &[ComponentRow]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut first_path_by_role: BTreeMap<&str, String> = BTreeMap::new();
    for row in rows {
        let Some(role) = row.role.as_deref() else {
            continue;
        };
        let path = row_identity(row);
        if let Some(first_path) = first_path_by_role.insert(role, path.clone()) {
            diagnostics.push(diagnostic(
                "duplicate_role",
                path,
                format!("role {role:?} is already used by {first_path}"),
            ));
        }
    }
    diagnostics
}

fn check_unique_paths(rows: &[ComponentRow]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen_paths = BTreeSet::new();
    for row in rows {
        let Some(path) = row.path.as_deref() else {
            continue;
        };
        if !seen_paths.insert(path) {
            diagnostics.push(diagnostic(
                "duplicate_path",
                path,
                format!("component path {path:?} appears more than once"),
            ));
        }
    }
    diagnostics
}

fn check_required_inventory(rows: &[ComponentRow]) -> Vec<Diagnostic> {
    let registered = rows
        .iter()
        .filter_map(|row| row.path.as_deref())
        .collect::<BTreeSet<_>>();
    REQUIRED_COMPONENT_PATHS
        .iter()
        .copied()
        .filter(|required| !registered.contains(required))
        .map(|required| {
            diagnostic(
                "missing_current_inventory",
                required,
                format!("current workspace component {required} is absent from the registry"),
            )
        })
        .collect()
}

fn check_observed_paths(rows: &[ComponentRow], model: &RepositoryModel) -> Vec<Diagnostic> {
    rows.iter()
        .filter_map(|row| {
            let path = row.path.as_deref()?;
            if !is_safe_component_path(path)
                || model.paths.contains(trim_root_suffix(path))
                || !is_parent_owned(row)
            {
                return None;
            }
            Some(diagnostic(
                "missing_component_path",
                path,
                format!("registry component path {path} is missing from the current tree"),
            ))
        })
        .collect()
}

fn check_local_notes(rows: &[ComponentRow], model: &RepositoryModel) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for row in rows {
        let should_check_notes = is_parent_owned(row) || component_observed(row, model);
        for note in &row.local_notes {
            if !is_safe_note_path(note) {
                diagnostics.push(diagnostic(
                    "unsafe_local_note_path",
                    row_identity(row),
                    format!("local note path {note:?} is not a safe repository-relative file"),
                ));
                continue;
            }
            if should_check_notes && !model.paths.contains(note) {
                diagnostics.push(diagnostic(
                    "missing_local_note",
                    row_identity(row),
                    format!("local note path {note} is missing from the current tree"),
                ));
            }
        }
    }
    diagnostics
}

fn check_nested_git_boundaries(rows: &[ComponentRow], model: &RepositoryModel) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let allowed_nested_roots = rows
        .iter()
        .filter(|row| row.nested_git_exception == Some(true))
        .filter_map(|row| row.path.as_deref().map(trim_root_suffix))
        .collect::<BTreeSet<_>>();

    for nested_root in &model.nested_git_roots {
        if !allowed_nested_roots.contains(nested_root.as_str()) {
            diagnostics.push(diagnostic(
                "undocumented_nested_git",
                nested_root,
                format!("nested Git boundary {nested_root}/.git is not recorded as a registry exception"),
            ));
        }
    }

    for row in rows {
        let Some(path) = row.path.as_deref() else {
            continue;
        };
        let trimmed = trim_root_suffix(path);
        if row.nested_git_exception == Some(true)
            && model.paths.contains(trimmed)
            && !model.nested_git_roots.contains(trimmed)
        {
            diagnostics.push(diagnostic(
                "missing_nested_git_boundary",
                path,
                format!("registry row {path} declares a nested Git exception, but no .git boundary was observed"),
            ));
        }
        if is_parent_owned(row) && nested_git_under(trimmed, model) {
            diagnostics.push(diagnostic(
                "parent_owned_nested_git",
                path,
                format!("parent-owned component {path} contains an unexpected nested Git boundary"),
            ));
        }
    }

    diagnostics
}

fn nested_git_under(component_root: &str, model: &RepositoryModel) -> bool {
    model
        .nested_git_roots
        .iter()
        .any(|nested_root| nested_root == component_root || nested_root.starts_with(&format!("{component_root}{PATH_SEPARATOR}")))
}

fn is_parent_owned(row: &ComponentRow) -> bool {
    row.owner.as_deref() == Some("parent-repository")
}

fn component_observed(row: &ComponentRow, model: &RepositoryModel) -> bool {
    row.path
        .as_deref()
        .is_some_and(|path| model.paths.contains(trim_root_suffix(path)))
}

fn check_checklist_sync(
    rows: &[ComponentRow],
    checklist_entries: &BTreeMap<String, ChecklistEntry>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let registered_paths = rows
        .iter()
        .filter_map(|row| row.path.as_deref())
        .collect::<BTreeSet<_>>();

    for row in rows {
        let Some(path) = row.path.as_deref() else {
            continue;
        };
        match checklist_entries.get(path) {
            Some(entry) => {
                if let Some(gate) = row.default_gate_participation.as_deref() {
                    let expected_keyword = checklist_gate_keyword(gate);
                    if !entry
                        .default_gates
                        .to_ascii_lowercase()
                        .contains(expected_keyword)
                    {
                        diagnostics.push(diagnostic(
                            "checklist_gate_drift",
                            path,
                            format!(
                                "checklist default-gates cell {:?} does not reflect registry gate {gate:?}",
                                entry.default_gates
                            ),
                        ));
                    }
                }
            }
            None => diagnostics.push(diagnostic(
                "checklist_missing_registry_row",
                path,
                format!("docs/layout-checklist.md is missing registry summary row {path}"),
            )),
        }
    }

    for path in checklist_entries.keys() {
        if !registered_paths.contains(path.as_str()) {
            diagnostics.push(diagnostic(
                "checklist_extra_registry_row",
                path,
                format!("docs/layout-checklist.md has summary row {path} not present in component registry"),
            ));
        }
    }

    diagnostics
}

fn checklist_gate_keyword(gate: &str) -> &str {
    match gate {
        INCLUDED_GATE => "included",
        SELECTED_GATE => "selected",
        EXCLUDED_GATE => "excluded",
        _ => gate,
    }
}

fn is_safe_component_path(path: &str) -> bool {
    if path.is_empty()
        || !path.ends_with(ROOT_PATH_SUFFIX)
        || path.starts_with(PATH_SEPARATOR)
        || path.contains(ESCAPE)
        || path.contains("//")
    {
        return false;
    }
    let trimmed = trim_root_suffix(path);
    !trimmed.is_empty() && trimmed.split(PATH_SEPARATOR).all(is_safe_path_segment)
}

fn is_safe_note_path(path: &str) -> bool {
    if path.is_empty()
        || path.ends_with(ROOT_PATH_SUFFIX)
        || path.starts_with(PATH_SEPARATOR)
        || path.contains(ESCAPE)
        || path.contains("//")
    {
        return false;
    }
    path.split(PATH_SEPARATOR).all(is_safe_path_segment)
}

fn is_safe_path_segment(segment: &str) -> bool {
    !segment.is_empty() && segment != "." && segment != ".." && segment != GIT_ENTRY_NAME
}

fn trim_root_suffix(path: &str) -> &str {
    path.trim_end_matches(ROOT_PATH_SUFFIX)
}

fn row_identity(row: &ComponentRow) -> String {
    row.path
        .clone()
        .unwrap_or_else(|| format!("line:{}", row.start_line))
}

fn parse_registry_source(text: &str) -> Result<Vec<ComponentRow>, Vec<String>> {
    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut in_components = false;
    let mut current_row: Option<ComponentRow> = None;
    let mut current_array_field: Option<String> = None;

    for (line_index, raw_line) in text.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed_owned = strip_comment(raw_line).trim().to_string();
        let trimmed = trimmed_owned.as_str();
        if trimmed.is_empty() {
            continue;
        }

        if !in_components {
            if trimmed.contains(COMPONENTS_START) {
                in_components = true;
            }
            continue;
        }

        if let Some(field) = current_array_field.clone() {
            if let Some(row) = current_row.as_mut() {
                push_array_values(row, &field, extract_quoted_strings(trimmed));
            }
            if trimmed.contains(CLOSE_BRACKET) {
                current_array_field = None;
            }
            continue;
        }

        if current_row.is_none() {
            if trimmed.starts_with(ARRAY_END_TOKEN) {
                break;
            }
            if trimmed.starts_with(ROW_START_TOKEN) {
                current_row = Some(ComponentRow::new(line_number));
            }
            continue;
        }

        if trimmed == ROW_END_TOKEN || trimmed == ROW_END_COMMA_TOKEN {
            if let Some(row) = current_row.take() {
                rows.push(row);
            }
            continue;
        }

        let Some((key, value)) = split_assignment(trimmed) else {
            continue;
        };
        if let Some(row) = current_row.as_mut() {
            if value.starts_with(OPEN_BRACKET) {
                push_array_values(row, key, extract_quoted_strings(value));
                if !value.contains(CLOSE_BRACKET) {
                    current_array_field = Some(key.to_string());
                }
            } else if let Some(string_value) = extract_first_quoted_string(value) {
                set_string_field(row, key, string_value);
            } else if value.starts_with(TRUE_LITERAL) || value.starts_with(FALSE_LITERAL) {
                set_bool_field(row, key, value.starts_with(TRUE_LITERAL));
            }
        }
    }

    if current_row.is_some() {
        errors.push(String::from("registry parse ended before closing the current component row"));
    }
    if rows.is_empty() {
        errors.push(String::from("registry parse found no component rows"));
    }

    if errors.is_empty() {
        Ok(rows)
    } else {
        Err(errors)
    }
}

fn strip_comment(line: &str) -> String {
    let mut in_string = false;
    let mut escaped = false;
    let mut result = String::new();
    for character in line.chars() {
        if escaped {
            result.push(character);
            escaped = false;
            continue;
        }
        if character == ESCAPE && in_string {
            result.push(character);
            escaped = true;
            continue;
        }
        if character == QUOTE {
            in_string = !in_string;
            result.push(character);
            continue;
        }
        if character == COMMENT && !in_string {
            break;
        }
        result.push(character);
    }
    result
}

fn split_assignment(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(ASSIGNMENT_SEPARATOR)?;
    Some((key.trim(), value.trim().trim_end_matches(',')))
}

fn extract_first_quoted_string(text: &str) -> Option<String> {
    extract_quoted_strings(text).into_iter().next()
}

fn extract_quoted_strings(text: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut escaped = false;
    for character in text.chars() {
        if escaped {
            if in_string {
                current.push(character);
            }
            escaped = false;
            continue;
        }
        if character == ESCAPE && in_string {
            escaped = true;
            continue;
        }
        if character == QUOTE {
            if in_string {
                values.push(current.clone());
                current.clear();
            }
            in_string = !in_string;
            continue;
        }
        if in_string {
            current.push(character);
        }
    }
    values
}

fn push_array_values(row: &mut ComponentRow, field: &str, values: Vec<String>) {
    match field {
        COMMANDS_FIELD => row.commands.extend(values),
        LOCAL_NOTES_FIELD => row.local_notes.extend(values),
        _ => {}
    }
}

fn set_string_field(row: &mut ComponentRow, field: &str, value: String) {
    match field {
        PATH_FIELD => row.path = Some(value),
        ROLE_FIELD => row.role = Some(value),
        OWNER_FIELD => row.owner = Some(value),
        VCS_BOUNDARY_FIELD => row.vcs_boundary = Some(value),
        DEFAULT_GATE_FIELD => row.default_gate_participation = Some(value),
        EVIDENCE_POLICY_FIELD => row.evidence_policy = Some(value),
        _ => {}
    }
}

fn set_bool_field(row: &mut ComponentRow, field: &str, value: bool) {
    if field == NESTED_GIT_FIELD {
        row.nested_git_exception = Some(value);
    }
}

fn parse_checklist_entries(checklist: &str) -> BTreeMap<String, ChecklistEntry> {
    let mut entries = BTreeMap::new();
    for line in checklist.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|')
            || trimmed.contains(CHECKLIST_SEPARATOR)
            || trimmed.contains(CHECKLIST_HEADER)
        {
            continue;
        }
        let cells = trimmed
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect::<Vec<_>>();
        if cells.len() < CHECKLIST_EXPECTED_CELL_COUNT {
            continue;
        }
        let Some(path) = extract_first_backticked_path(&cells[CHECKLIST_PATH_CELL_INDEX]) else {
            continue;
        };
        entries.insert(
            path.clone(),
            ChecklistEntry {
                path,
                default_gates: cells[CHECKLIST_DEFAULT_GATES_CELL_INDEX].clone(),
            },
        );
    }
    entries
}

fn extract_first_backticked_path(text: &str) -> Option<String> {
    let (_, after_open) = text.split_once('`')?;
    let (path, _) = after_open.split_once('`')?;
    if path.ends_with(ROOT_PATH_SUFFIX) {
        Some(path.to_string())
    } else {
        None
    }
}

fn load_repository_model(root: &Path) -> Result<RepositoryModel, String> {
    let checklist_path = root.join(CHECKLIST_PATH);
    let checklist = fs::read_to_string(&checklist_path)
        .map_err(|error| format!("failed to read {}: {error}", checklist_path.display()))?;
    let mut paths = BTreeSet::new();
    let mut nested_git_roots = BTreeSet::new();
    collect_tree_entries(root, root, &mut paths, &mut nested_git_roots)?;
    Ok(RepositoryModel {
        paths,
        nested_git_roots,
        checklist,
    })
}

fn collect_tree_entries(
    root: &Path,
    current: &Path,
    paths: &mut BTreeSet<String>,
    nested_git_roots: &mut BTreeSet<String>,
) -> Result<(), String> {
    let entries = fs::read_dir(current)
        .map_err(|error| format!("failed to read directory {}: {error}", current.display()))?;
    for entry_result in entries {
        let entry = entry_result.map_err(|error| {
            format!(
                "failed to read directory entry {}: {error}",
                current.display()
            )
        })?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
        let relative = relative_path(root, &path)?;
        paths.insert(relative.clone());

        if is_git_boundary(&relative) {
            if let Some(owner_root) = git_boundary_owner_root(&relative) {
                nested_git_roots.insert(owner_root);
            }
            continue;
        }

        if metadata.is_dir() && !should_skip_walk(&relative) {
            collect_tree_entries(root, &path, paths, nested_git_roots)?;
        }
    }
    Ok(())
}

fn relative_path(root: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(root)
        .map_err(|error| format!("failed to relativize {}: {error}", path.display()))?
        .to_str()
        .map(|text| text.replace('\\', "/"))
        .ok_or_else(|| format!("{} is not valid UTF-8", path.display()))
}

fn should_skip_walk(relative: &str) -> bool {
    if relative == TARGET_DIR_NAME
        || relative == RESULT_LINK_NAME
        || relative == JJ_ENTRY_NAME
        || relative.starts_with(RESULT_LINK_PREFIX)
    {
        return true;
    }
    if relative.starts_with(&format!("{TARGET_DIR_NAME}{PATH_SEPARATOR}"))
        || relative.starts_with(&format!("{JJ_ENTRY_NAME}{PATH_SEPARATOR}"))
    {
        return true;
    }
    IGNORED_WALK_DIRS.iter().any(|ignored| {
        relative == *ignored || relative.starts_with(&format!("{ignored}{PATH_SEPARATOR}"))
    })
}

fn is_git_boundary(path: &str) -> bool {
    path == GIT_ENTRY_NAME || path.ends_with(GIT_BOUNDARY_SUFFIX)
}

fn git_boundary_owner_root(path: &str) -> Option<String> {
    path.strip_suffix(GIT_BOUNDARY_SUFFIX)
        .map(ToString::to_string)
}

fn report_has_errors(report: &Report) -> bool {
    !report.diagnostics.is_empty()
}

fn render_report(report: &Report) -> String {
    report
        .diagnostics
        .iter()
        .map(render_diagnostic)
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_diagnostic(diagnostic: &Diagnostic) -> String {
    format!(
        "code={} path={} message={}",
        diagnostic.code, diagnostic.path, diagnostic.message
    )
}

fn fixture_component(path: &str, role: &str) -> ComponentRow {
    ComponentRow {
        start_line: 1,
        path: Some(path.to_string()),
        role: Some(role.to_string()),
        owner: Some("parent-repository".to_string()),
        vcs_boundary: Some("parent-git".to_string()),
        commands: vec!["check command".to_string()],
        default_gate_participation: Some(INCLUDED_GATE.to_string()),
        evidence_policy: Some("copy-review-artifacts".to_string()),
        local_notes: vec!["README.md".to_string()],
        nested_git_exception: Some(false),
    }
}

fn fixture_rows() -> Vec<ComponentRow> {
    let mut rows = Vec::new();
    rows.push(fixture_component("clients/stevenarella/", "core-client"));
    rows.push(fixture_component("servers/valence/", "core-server"));
    rows.push(fixture_component("compat/runner/", "compatibility-runner"));
    rows.push(fixture_component("compat/config/", "compatibility-config"));
    rows.push(fixture_component(
        "compat/fixtures/paper-survival/",
        "paper-reference-fixture",
    ));
    rows.push(fixture_component("cairn/", "cairn-lifecycle"));
    rows.push(fixture_component("docs/evidence/", "durable-evidence"));
    let mut leafish = fixture_component("Leafish/", "reference-client");
    leafish.owner = Some("reference-only-checkout".to_string());
    leafish.vcs_boundary = Some("nested-git".to_string());
    leafish.default_gate_participation = Some(EXCLUDED_GATE.to_string());
    leafish.evidence_policy = Some("reference-excluded".to_string());
    leafish.nested_git_exception = Some(true);
    rows.push(leafish);
    rows
}

fn fixture_model(rows: &[ComponentRow]) -> RepositoryModel {
    let mut paths = BTreeSet::from(["README.md".to_string(), CHECKLIST_PATH.to_string()]);
    for row in rows {
        if let Some(path) = row.path.as_deref() {
            paths.insert(trim_root_suffix(path).to_string());
        }
        for note in &row.local_notes {
            paths.insert(note.clone());
        }
    }
    RepositoryModel {
        paths,
        nested_git_roots: BTreeSet::from(["Leafish".to_string()]),
        checklist: fixture_checklist(rows),
    }
}

fn fixture_checklist(rows: &[ComponentRow]) -> String {
    let mut text = String::from(
        "| Path | Role | Ownership | Local notes | Default gates |\n| --- | --- | --- | --- | --- |\n",
    );
    for row in rows {
        let path = row.path.as_deref().unwrap_or("missing/");
        let gate = row
            .default_gate_participation
            .as_deref()
            .map(checklist_gate_keyword)
            .unwrap_or("missing");
        text.push_str(&format!(
            "| `{path}` | role | owner | `README.md` | {gate} |\n"
        ));
    }
    text
}

fn assert_report_has_code(report: &Report, code: &str) -> Result<(), String> {
    if report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == code)
    {
        Ok(())
    } else {
        Err(format!(
            "expected diagnostic code {code}; got {:?}",
            report.diagnostics
        ))
    }
}

fn run_self_test() -> Result<(), String> {
    let valid_rows = fixture_rows();
    let valid_model = fixture_model(&valid_rows);
    let valid_report = validate_registry(&valid_rows, &valid_model);
    if report_has_errors(&valid_report) {
        return Err(format!(
            "positive fixture unexpectedly failed: {:?}",
            valid_report.diagnostics
        ));
    }

    let mut missing_owner = valid_rows.clone();
    missing_owner[0].owner = None;
    assert_report_has_code(
        &validate_registry(&missing_owner, &valid_model),
        "missing_field",
    )?;

    let mut duplicate_role = valid_rows.clone();
    duplicate_role[1].role = duplicate_role[0].role.clone();
    assert_report_has_code(
        &validate_registry(&duplicate_role, &valid_model),
        "duplicate_role",
    )?;

    let mut unsafe_path = valid_rows.clone();
    unsafe_path[0].path = Some("../escape/".to_string());
    assert_report_has_code(&validate_registry(&unsafe_path, &valid_model), "unsafe_path")?;

    let mut undocumented_model = valid_model.clone();
    undocumented_model
        .nested_git_roots
        .insert("vendor".to_string());
    assert_report_has_code(
        &validate_registry(&valid_rows, &undocumented_model),
        "undocumented_nested_git",
    )?;

    let mut invalid_gate = valid_rows.clone();
    invalid_gate[0].default_gate_participation = Some("always".to_string());
    assert_report_has_code(
        &validate_registry(&invalid_gate, &valid_model),
        "invalid_default_gate_participation",
    )?;

    let mut missing_summary_model = valid_model.clone();
    missing_summary_model.checklist = String::from(
        "| Path | Role | Ownership | Local notes | Default gates |\n| --- | --- | --- | --- | --- |\n",
    );
    assert_report_has_code(
        &validate_registry(&valid_rows, &missing_summary_model),
        "checklist_missing_registry_row",
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Command {
    root: PathBuf,
    registry_path: String,
    self_test: bool,
}

fn parse_args() -> Result<Command, String> {
    let mut args = env::args().skip(1);
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut registry_path = REGISTRY_PATH.to_string();
    let mut self_test = false;

    while let Some(arg) = args.next() {
        if arg == ROOT_FLAG {
            let value = args
                .next()
                .ok_or_else(|| format!("{ROOT_FLAG} requires a path"))?;
            root = PathBuf::from(value);
        } else if arg == REGISTRY_FLAG {
            registry_path = args
                .next()
                .ok_or_else(|| format!("{REGISTRY_FLAG} requires a path"))?;
        } else if arg == SELF_TEST_FLAG {
            self_test = true;
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }

    Ok(Command {
        root,
        registry_path,
        self_test,
    })
}

fn run(command: Command) -> Result<String, String> {
    if command.self_test {
        run_self_test()?;
        return Ok(String::from(SELF_TEST_SUCCESS_MESSAGE));
    }

    let registry_path = command.root.join(&command.registry_path);
    let registry_text = fs::read_to_string(&registry_path)
        .map_err(|error| format!("failed to read {}: {error}", registry_path.display()))?;
    let rows = parse_registry_source(&registry_text).map_err(|errors| errors.join("\n"))?;
    let model = load_repository_model(&command.root)?;
    let report = validate_registry(&rows, &model);
    if report_has_errors(&report) {
        Err(render_report(&report))
    } else {
        Ok(String::from(SUCCESS_MESSAGE))
    }
}

fn main() -> ExitCode {
    let command = match parse_args() {
        Ok(command) => command,
        Err(error) => {
            eprintln!("{error}");
            return FAILURE;
        }
    };

    match run(command) {
        Ok(message) => {
            println!("{message}");
            SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            FAILURE
        }
    }
}
