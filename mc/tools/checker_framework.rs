#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process;

pub const OK_VALUE: &str = "ok";
pub const TRUE_VALUE: &str = "true";
pub const CLEAN_STATUS: &str = "clean";
pub const DRY_RUN_REV: &str = "dry-run";
pub const UNKNOWN_REV: &str = "unknown";
pub const PENDING_REVIEW_REV: &str = "pending-review";
pub const DOCS_EVIDENCE_PREFIX: &str = "docs/evidence/";
pub const MANIFEST_SEPARATOR: &str = "  ";
pub const BLAKE3_HEX_LENGTH: usize = 64;
const FIRST_LINE_NUMBER: usize = 1;
const KEY_VALUE_SEPARATOR: char = '=';
const JSON_NAME_DELIMITER: char = '"';
const JSON_FIELD_SEPARATOR: char = ':';
const JSON_ESCAPE_PREFIX: char = '\\';
const JSON_ESCAPE_QUOTE: char = '"';
const JSON_ESCAPE_REVERSE_SOLIDUS: char = '\\';
const JSON_ESCAPE_SOLIDUS: char = '/';
const JSON_ESCAPE_BACKSPACE: char = 'b';
const JSON_ESCAPE_FORM_FEED: char = 'f';
const JSON_ESCAPE_NEWLINE: char = 'n';
const JSON_ESCAPE_CARRIAGE_RETURN: char = 'r';
const JSON_ESCAPE_TAB: char = 't';
const JSON_ESCAPE_UNICODE: char = 'u';
const JSON_UNICODE_ESCAPE_DIGITS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectedField {
    pub key: &'static str,
    pub value: &'static str,
}

impl ExpectedField {
    pub const fn new(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }
}

pub trait KeyValueEvidence {
    fn value(&self, key: &str) -> Option<&str>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValueRecord {
    values: BTreeMap<String, String>,
}

impl KeyValueRecord {
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut values = BTreeMap::new();
        for (index, raw_line) in text.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once(KEY_VALUE_SEPARATOR) else {
                return Err(format!(
                    "line {} is not key=value",
                    index + FIRST_LINE_NUMBER
                ));
            };
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() {
                return Err(format!("line {} has empty key", index + FIRST_LINE_NUMBER));
            }
            if values.insert(key.to_string(), value.to_string()).is_some() {
                return Err(format!("duplicate key {key}"));
            }
        }
        Ok(Self { values })
    }
}

impl KeyValueEvidence for KeyValueRecord {
    fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostics {
    messages: Vec<String>,
    seen: BTreeSet<String>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            seen: BTreeSet::new(),
        }
    }

    pub fn push(&mut self, message: impl Into<String>) {
        let message = message.into();
        if self.seen.insert(message.clone()) {
            self.messages.push(message);
        }
    }

    pub fn extend(&mut self, messages: impl IntoIterator<Item = String>) {
        for message in messages {
            self.push(message);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn as_slice(&self) -> &[String] {
        &self.messages
    }

    pub fn into_vec(self) -> Vec<String> {
        self.messages
    }

    pub fn into_result(self) -> ValidationResult {
        if self.messages.is_empty() {
            Ok(())
        } else {
            Err(self.messages)
        }
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoLayout {
    root: PathBuf,
}

impl RepoLayout {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn safe_path(&self, relative: &str) -> Result<PathBuf, String> {
        validate_repo_relative_path(relative)?;
        Ok(self.root.join(relative))
    }

    pub fn docs_evidence_path(&self, relative: &str) -> Result<PathBuf, String> {
        require_docs_evidence_path(relative)?;
        self.safe_path(relative)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEntry {
    pub digest: String,
    pub path: String,
}

pub fn validate_repo_relative_path(relative: &str) -> Result<(), String> {
    if relative.is_empty() {
        return Err("path is empty".to_string());
    }
    let path = Path::new(relative);
    if path.is_absolute() {
        return Err(format!(
            "unsafe path {relative}: absolute paths are not allowed"
        ));
    }

    let mut has_normal_component = false;
    for component in path.components() {
        match component {
            Component::Normal(_) => has_normal_component = true,
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(format!(
                    "unsafe path {relative}: parent traversal is not allowed"
                ));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(format!(
                    "unsafe path {relative}: rooted paths are not allowed"
                ));
            }
        }
    }

    if has_normal_component {
        Ok(())
    } else {
        Err(format!("unsafe path {relative}: no file component"))
    }
}

pub fn require_docs_evidence_path(relative: &str) -> Result<(), String> {
    validate_repo_relative_path(relative)?;
    if relative.starts_with(DOCS_EVIDENCE_PREFIX) && relative != DOCS_EVIDENCE_PREFIX {
        Ok(())
    } else {
        Err(format!(
            "path {relative} must be under {DOCS_EVIDENCE_PREFIX}"
        ))
    }
}

pub fn extract_json_string_field(text: &str, key: &str) -> Result<String, String> {
    if key.is_empty() {
        return Err("JSON field key is empty".to_string());
    }

    let field_name = format!("{JSON_NAME_DELIMITER}{key}{JSON_NAME_DELIMITER}");
    let Some(field_start) = text.find(&field_name) else {
        return Err(format!("missing JSON field {key}"));
    };
    let after_name = &text[field_start + field_name.len()..];
    let Some(separator_index) = after_name.find(JSON_FIELD_SEPARATOR) else {
        return Err(format!("JSON field {key} missing ':' separator"));
    };
    let after_separator =
        after_name[separator_index + JSON_FIELD_SEPARATOR.len_utf8()..].trim_start();
    let Some(after_opening_quote) = after_separator.strip_prefix(JSON_NAME_DELIMITER) else {
        return Err(format!("JSON field {key} is not a string"));
    };

    parse_json_string_value(after_opening_quote, key)
}

fn parse_json_string_value(after_opening_quote: &str, key: &str) -> Result<String, String> {
    let mut value = String::new();
    let mut chars = after_opening_quote.chars();
    while let Some(character) = chars.next() {
        match character {
            JSON_NAME_DELIMITER => return Ok(value),
            JSON_ESCAPE_PREFIX => match chars.next() {
                Some(JSON_ESCAPE_QUOTE) => value.push(JSON_ESCAPE_QUOTE),
                Some(JSON_ESCAPE_REVERSE_SOLIDUS) => value.push(JSON_ESCAPE_REVERSE_SOLIDUS),
                Some(JSON_ESCAPE_SOLIDUS) => value.push(JSON_ESCAPE_SOLIDUS),
                Some(JSON_ESCAPE_BACKSPACE) => value.push('\u{0008}'),
                Some(JSON_ESCAPE_FORM_FEED) => value.push('\u{000c}'),
                Some(JSON_ESCAPE_NEWLINE) => value.push('\n'),
                Some(JSON_ESCAPE_CARRIAGE_RETURN) => value.push('\r'),
                Some(JSON_ESCAPE_TAB) => value.push('\t'),
                Some(JSON_ESCAPE_UNICODE) => {
                    let mut escape = String::new();
                    for _ in 0..JSON_UNICODE_ESCAPE_DIGITS {
                        let Some(hex) = chars.next() else {
                            return Err(format!("JSON field {key} has incomplete unicode escape"));
                        };
                        if !hex.is_ascii_hexdigit() {
                            return Err(format!("JSON field {key} has invalid unicode escape"));
                        }
                        escape.push(hex);
                    }
                    value.push(JSON_ESCAPE_PREFIX);
                    value.push(JSON_ESCAPE_UNICODE);
                    value.push_str(&escape);
                }
                Some(other) => {
                    return Err(format!("JSON field {key} has unsupported escape {other}"));
                }
                None => return Err(format!("JSON field {key} has trailing escape")),
            },
            control if control.is_control() => {
                return Err(format!("JSON field {key} contains a control character"));
            }
            other => value.push(other),
        }
    }

    Err(format!("JSON field {key} has unterminated string"))
}

pub fn parse_blake3_manifest(text: &str) -> Result<Vec<ManifestEntry>, Vec<String>> {
    let mut entries = Vec::new();
    let mut paths = BTreeSet::new();
    let mut diagnostics = Vec::new();

    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line_number = line_index + FIRST_LINE_NUMBER;
        let Some((digest, path)) = line.split_once(MANIFEST_SEPARATOR) else {
            diagnostics.push(format!(
                "manifest line {line_number} missing two-space separator"
            ));
            continue;
        };
        let digest = digest.trim();
        let path = path.trim();
        if !is_blake3_hex_digest(digest) {
            diagnostics.push(format!(
                "manifest line {line_number} has invalid BLAKE3 digest"
            ));
        }
        if let Err(error) = require_docs_evidence_path(path) {
            diagnostics.push(format!("manifest line {line_number}: {error}"));
        }
        if !paths.insert(path.to_string()) {
            diagnostics.push(format!(
                "manifest line {line_number} duplicates path {path}"
            ));
        }
        entries.push(ManifestEntry {
            digest: digest.to_string(),
            path: path.to_string(),
        });
    }

    if entries.is_empty() {
        diagnostics.push("manifest has no entries".to_string());
    }

    if diagnostics.is_empty() {
        Ok(entries)
    } else {
        Err(diagnostics)
    }
}

pub fn manifest_covers_path(entries: &[ManifestEntry], path: &str) -> bool {
    entries.iter().any(|entry| entry.path == path)
}

pub fn is_blake3_hex_digest(digest: &str) -> bool {
    digest.len() == BLAKE3_HEX_LENGTH
        && digest
            .chars()
            .all(|character| character.is_ascii_hexdigit())
}

pub fn require_exact<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    key: &str,
    expected: &str,
) {
    match evidence.value(key) {
        Some(actual) if actual == expected => {}
        Some(actual) => diagnostics.push(format!("{key} expected {expected}, got {actual}")),
        None => diagnostics.push(format!("missing {key}")),
    }
}

pub fn require_exact_fields<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    fields: &[ExpectedField],
) {
    for field in fields {
        require_exact(evidence, diagnostics, field.key, field.value);
    }
}

pub fn require_ok<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    key: &str,
) {
    require_exact(evidence, diagnostics, key, OK_VALUE);
}

pub fn require_ok_fields<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    keys: &[&str],
) {
    for key in keys {
        require_ok(evidence, diagnostics, key);
    }
}

pub fn require_true_fields<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    keys: &[&str],
) {
    for key in keys {
        require_exact(evidence, diagnostics, key, TRUE_VALUE);
    }
}

pub fn require_clean_child_revision<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    rev_key: &str,
    status_key: &str,
) {
    match evidence.value(rev_key) {
        Some(rev)
            if !rev.is_empty()
                && rev != DRY_RUN_REV
                && rev != UNKNOWN_REV
                && rev != PENDING_REVIEW_REV => {}
        Some(rev) => diagnostics.push(format!("{rev_key} must be concrete, got {rev}")),
        None => diagnostics.push(format!("missing {rev_key}")),
    }
    require_exact(evidence, diagnostics, status_key, CLEAN_STATUS);
}

pub fn reject_truthy_overclaims<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    keys: &[&str],
    truthy_values: &[&str],
) {
    for key in keys {
        if let Some(value) = evidence.value(key) {
            if truthy_values
                .iter()
                .any(|truthy| value.eq_ignore_ascii_case(truthy))
            {
                diagnostics.push(format!("broad overclaim {key}={value}"));
            }
        }
    }
}

pub fn parse_i32_sequence(raw: &str) -> Result<Vec<i32>, String> {
    let mut values = Vec::new();
    for part in raw.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            return Err("empty state id".to_string());
        }
        let parsed = trimmed
            .parse::<i32>()
            .map_err(|err| format!("invalid state id {trimmed}: {err}"))?;
        values.push(parsed);
    }
    Ok(values)
}

pub fn all_at_least(values: &[i32], minimum: i32) -> bool {
    values.iter().all(|value| *value >= minimum)
}

pub type ValidationResult = Result<(), Vec<String>>;
pub type Validator = fn(&KeyValueRecord) -> ValidationResult;

pub fn assert_self_test_fixtures(
    valid_text: &str,
    validator: Validator,
    negative_fixtures: &[(&str, String, &str)],
) -> Result<(), String> {
    let valid = KeyValueRecord::parse(valid_text)?;
    validator(&valid).map_err(|diagnostics| diagnostics.join("; "))?;

    for (name, text, expected) in negative_fixtures {
        assert_key_value_validation_error(name, text, validator, expected)?;
    }

    Ok(())
}

pub fn assert_key_value_validation_error(
    name: &str,
    text: &str,
    validator: Validator,
    expected: &str,
) -> Result<(), String> {
    let evidence = KeyValueRecord::parse(text).map_err(|err| format!("{name}: parse: {err}"))?;
    match validator(&evidence) {
        Ok(()) => Err(format!("{name}: unexpectedly passed")),
        Err(diagnostics) => {
            let rendered = diagnostics.join("; ");
            if rendered.contains(expected) {
                Ok(())
            } else {
                Err(format!(
                    "{name}: expected diagnostic containing {expected:?}, got {rendered}"
                ))
            }
        }
    }
}

pub trait Checker {
    fn usage(&self) -> &'static str;
    fn validate(&self, evidence: &KeyValueRecord) -> ValidationResult;
    fn self_test(&self) -> Result<(), String>;

    fn run_path(&self, path: &str) -> Result<(), String> {
        let text = fs::read_to_string(path).map_err(|err| format!("read {path}: {err}"))?;
        let evidence = KeyValueRecord::parse(&text)?;
        self.validate(&evidence)
            .map_err(|diagnostics| diagnostics.join("\n"))
    }
}

pub fn run_checker(checker: &impl Checker) {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = if args.iter().any(|arg| arg == "--self-test") {
        checker.self_test().map(|()| "self-test ok".to_string())
    } else if let Some(path) = args.first() {
        checker.run_path(path).map(|()| format!("{path}: ok"))
    } else {
        Err(checker.usage().to_string())
    };

    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_REV_KEY: &str = "child.sample.rev";
    const SAMPLE_STATUS_KEY: &str = "child.sample.status";
    const CLAIM_KEY: &str = "claim.full";
    const SAMPLE_VALID_RECORD: &str =
        "child.sample.rev=abcdef\nchild.sample.status=clean\nmetric=ok\n";
    const SAMPLE_DOCS_EVIDENCE_PATH: &str = "docs/evidence/sample.run.log";
    const SAMPLE_DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const SAMPLE_SEQUENCE_MINIMUM_ZERO: i32 = 0;
    const SAMPLE_SEQUENCE_MINIMUM_ONE: i32 = 1;

    fn sample_validator(record: &KeyValueRecord) -> ValidationResult {
        let mut diagnostics = Vec::new();
        require_clean_child_revision(record, &mut diagnostics, SAMPLE_REV_KEY, SAMPLE_STATUS_KEY);
        require_ok(record, &mut diagnostics, "metric");
        reject_truthy_overclaims(record, &mut diagnostics, &[CLAIM_KEY], &["true", "yes"]);
        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }

    #[test]
    fn key_value_record_parses_valid_rows_and_ignores_comments() {
        let record = KeyValueRecord::parse("# comment\n alpha = one \n\n beta=two\n")
            .expect("valid record parses");

        assert_eq!(record.value("alpha"), Some("one"));
        assert_eq!(record.value("beta"), Some("two"));
        assert_eq!(record.value("missing"), None);
    }

    #[test]
    fn key_value_record_rejects_malformed_empty_and_duplicate_keys() {
        let malformed = KeyValueRecord::parse("missing separator").unwrap_err();
        assert!(malformed.contains("line 1 is not key=value"), "{malformed}");

        let empty = KeyValueRecord::parse("=value").unwrap_err();
        assert!(empty.contains("line 1 has empty key"), "{empty}");

        let duplicate = KeyValueRecord::parse("a=1\na=2").unwrap_err();
        assert!(duplicate.contains("duplicate key a"), "{duplicate}");
    }

    #[test]
    fn helpers_accept_valid_records_and_reject_weak_evidence() {
        let negative = [
            (
                "missing metric",
                SAMPLE_VALID_RECORD.replace("metric=ok\n", ""),
                "missing metric",
            ),
            (
                "dry run revision",
                SAMPLE_VALID_RECORD.replace("child.sample.rev=abcdef", "child.sample.rev=dry-run"),
                "child.sample.rev must be concrete",
            ),
            (
                "pending review revision",
                SAMPLE_VALID_RECORD
                    .replace("child.sample.rev=abcdef", "child.sample.rev=pending-review"),
                "child.sample.rev must be concrete",
            ),
            (
                "overclaim",
                format!("{SAMPLE_VALID_RECORD}{CLAIM_KEY}=true\n"),
                "broad overclaim claim.full=true",
            ),
        ];

        assert_self_test_fixtures(SAMPLE_VALID_RECORD, sample_validator, &negative)
            .expect("positive and negative fixtures pass");
    }

    #[test]
    fn sequence_helpers_parse_and_reject_invalid_state_ids() {
        let values = parse_i32_sequence("0, 1, 2").expect("valid sequence parses");
        assert!(all_at_least(&values, SAMPLE_SEQUENCE_MINIMUM_ZERO));
        assert!(!all_at_least(&values, SAMPLE_SEQUENCE_MINIMUM_ONE));

        let err = parse_i32_sequence("0,,2").unwrap_err();
        assert_eq!(err, "empty state id");
    }

    #[test]
    fn repo_layout_accepts_safe_paths_and_rejects_traversal() {
        let layout = RepoLayout::new("/repo");
        assert_eq!(
            layout
                .docs_evidence_path(SAMPLE_DOCS_EVIDENCE_PATH)
                .expect("docs evidence path is safe"),
            PathBuf::from("/repo").join(SAMPLE_DOCS_EVIDENCE_PATH)
        );

        let absolute = validate_repo_relative_path("/tmp/evidence.log").unwrap_err();
        assert!(absolute.contains("absolute paths"), "{absolute}");
        let traversal = validate_repo_relative_path("docs/../secret").unwrap_err();
        assert!(traversal.contains("parent traversal"), "{traversal}");
        let wrong_root = require_docs_evidence_path("target/evidence.run.log").unwrap_err();
        assert!(wrong_root.contains(DOCS_EVIDENCE_PREFIX), "{wrong_root}");
    }

    #[test]
    fn json_receipt_fields_extract_strings_and_reject_malformed_receipts() {
        let receipt = r#"{"schema":"mc.compat.scenario.receipt.v2","path":"docs/evidence/example.receipt.json"}"#;
        assert_eq!(
            extract_json_string_field(receipt, "schema").expect("schema field parses"),
            "mc.compat.scenario.receipt.v2"
        );
        assert_eq!(
            extract_json_string_field(receipt, "path").expect("path field parses"),
            "docs/evidence/example.receipt.json"
        );

        let missing = extract_json_string_field(receipt, "missing").unwrap_err();
        assert!(missing.contains("missing JSON field"), "{missing}");
        let non_string = extract_json_string_field(r#"{"schema": false}"#, "schema").unwrap_err();
        assert!(non_string.contains("not a string"), "{non_string}");
        let unterminated = extract_json_string_field(r#"{"schema":"open}"#, "schema").unwrap_err();
        assert!(unterminated.contains("unterminated"), "{unterminated}");
    }

    #[test]
    fn manifests_load_entries_and_reject_stale_or_unsafe_rows() {
        let manifest = format!("{SAMPLE_DIGEST}{MANIFEST_SEPARATOR}{SAMPLE_DOCS_EVIDENCE_PATH}\n");
        let entries = parse_blake3_manifest(&manifest).expect("manifest parses");
        assert!(manifest_covers_path(&entries, SAMPLE_DOCS_EVIDENCE_PATH));
        assert!(!manifest_covers_path(
            &entries,
            "docs/evidence/missing.run.log"
        ));

        let stale_digest = parse_blake3_manifest(&manifest.replace(SAMPLE_DIGEST, "abc"))
            .expect_err("short digest is rejected");
        assert!(
            stale_digest
                .iter()
                .any(|diagnostic| diagnostic.contains("invalid BLAKE3 digest")),
            "{stale_digest:?}"
        );
        let unsafe_path =
            parse_blake3_manifest(&manifest.replace(SAMPLE_DOCS_EVIDENCE_PATH, "../log"))
                .expect_err("unsafe path is rejected");
        assert!(
            unsafe_path
                .iter()
                .any(|diagnostic| diagnostic.contains("parent traversal")),
            "{unsafe_path:?}"
        );
        let duplicate_manifest = format!(
            "{SAMPLE_DIGEST}{MANIFEST_SEPARATOR}{SAMPLE_DOCS_EVIDENCE_PATH}\n{SAMPLE_DIGEST}{MANIFEST_SEPARATOR}{SAMPLE_DOCS_EVIDENCE_PATH}\n"
        );
        let duplicate = parse_blake3_manifest(&duplicate_manifest)
            .expect_err("duplicate manifest path is rejected");
        assert!(
            duplicate
                .iter()
                .any(|diagnostic| diagnostic.contains("duplicates path")),
            "{duplicate:?}"
        );
    }

    #[test]
    fn diagnostics_deduplicate_messages_without_hiding_unique_errors() {
        let mut diagnostics = Diagnostics::new();
        diagnostics.push("missing row.id");
        diagnostics.push("missing row.id");
        diagnostics.push("missing evidence.receipt");

        assert_eq!(
            diagnostics.as_slice(),
            &[
                "missing row.id".to_string(),
                "missing evidence.receipt".to_string()
            ]
        );
        assert_eq!(
            diagnostics.into_result().unwrap_err(),
            vec![
                "missing row.id".to_string(),
                "missing evidence.receipt".to_string()
            ]
        );
    }
}
