use std::collections::BTreeMap;

pub const OK_VALUE: &str = "ok";
pub const TRUE_VALUE: &str = "true";
pub const CLEAN_STATUS: &str = "clean";
pub const DRY_RUN_REV: &str = "dry-run";
pub const UNKNOWN_REV: &str = "unknown";
const FIRST_LINE_NUMBER: usize = 1;

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
            let Some((key, value)) = line.split_once('=') else {
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

pub type ValidationResult = Result<(), Vec<String>>;
pub type Validator = fn(&KeyValueRecord) -> ValidationResult;

pub trait KeyValueChecker {
    fn usage(&self) -> &'static str;
    fn validate(&self, evidence: &KeyValueRecord) -> ValidationResult;
    fn self_test(&self) -> Result<(), String>;
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

pub fn require_ok<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    key: &str,
) {
    require_exact(evidence, diagnostics, key, OK_VALUE);
}

pub fn require_clean_child_revision<E: KeyValueEvidence + ?Sized>(
    evidence: &E,
    diagnostics: &mut Vec<String>,
    rev_key: &str,
    status_key: &str,
) {
    match evidence.value(rev_key) {
        Some(rev) if !rev.is_empty() && rev != DRY_RUN_REV && rev != UNKNOWN_REV => {}
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

pub fn assert_self_test_fixtures(
    valid_text: &str,
    validator: Validator,
    negative_fixtures: &[(&str, String, &str)],
) -> Result<(), String> {
    let valid = KeyValueRecord::parse(valid_text)?;
    validator(&valid).map_err(|diagnostics| diagnostics.join("; "))?;

    for (name, text, expected) in negative_fixtures {
        let evidence =
            KeyValueRecord::parse(text).map_err(|err| format!("{name}: parse: {err}"))?;
        match validator(&evidence) {
            Ok(()) => return Err(format!("{name}: unexpectedly passed")),
            Err(diagnostics) => {
                let rendered = diagnostics.join("; ");
                if !rendered.contains(expected) {
                    return Err(format!(
                        "{name}: expected diagnostic containing {expected:?}, got {rendered}"
                    ));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_REV_KEY: &str = "child.sample.rev";
    const SAMPLE_STATUS_KEY: &str = "child.sample.status";
    const CLAIM_KEY: &str = "claim.full";
    const SAMPLE_VALID_RECORD: &str =
        "child.sample.rev=abcdef\nchild.sample.status=clean\nmetric=ok\n";

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
        assert!(all_at_least(&values, 0));
        assert!(!all_at_least(&values, 1));

        let err = parse_i32_sequence("0,,2").unwrap_err();
        assert_eq!(err, "empty state id");
    }
}
