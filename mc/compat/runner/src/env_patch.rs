use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const ENV_KEY_UNDERSCORE: char = '_';

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnvPatch {
    entries: Vec<EnvPatchEntry>,
    removals: Vec<EnvPatchRemoval>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnvPatchEntry {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) source: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnvPatchRemoval {
    pub(crate) key: String,
    pub(crate) source: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EnvPatchDiagnosticKind {
    MalformedKey,
    ConflictingSet,
    SetRemoveConflict,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnvPatchDiagnostic {
    pub(crate) kind: EnvPatchDiagnosticKind,
    pub(crate) key: String,
    pub(crate) source: String,
    pub(crate) existing_source: Option<String>,
    pub(crate) detail: String,
}

impl EnvPatch {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
            removals: Vec::new(),
        }
    }

    pub(crate) fn set(
        mut self,
        source: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, EnvPatchDiagnostic> {
        self.push_set(source, key, value)?;
        Ok(self)
    }

    pub(crate) fn remove(
        mut self,
        source: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Self, EnvPatchDiagnostic> {
        self.push_remove(source, key)?;
        Ok(self)
    }

    pub(crate) fn push_set(
        &mut self,
        source: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), EnvPatchDiagnostic> {
        let source = source.into();
        let key = key.into();
        validate_env_key(&source, &key)?;
        self.entries.push(EnvPatchEntry {
            key,
            value: value.into(),
            source,
        });
        Ok(())
    }

    pub(crate) fn push_remove(
        &mut self,
        source: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<(), EnvPatchDiagnostic> {
        let source = source.into();
        let key = key.into();
        validate_env_key(&source, &key)?;
        self.removals.push(EnvPatchRemoval { key, source });
        Ok(())
    }

    pub(crate) fn compose(patches: &[EnvPatch]) -> Result<Self, EnvPatchDiagnostic> {
        let mut result = EnvPatch::new();
        let mut set_by_key: BTreeMap<String, (String, String)> = BTreeMap::new();
        let mut removed_by_key: BTreeMap<String, String> = BTreeMap::new();
        let mut emitted_removals: BTreeSet<String> = BTreeSet::new();

        for patch in patches {
            for removal in patch.removals() {
                validate_env_key(&removal.source, &removal.key)?;
                if let Some((_, set_source)) = set_by_key.get(&removal.key) {
                    return Err(EnvPatchDiagnostic::set_remove_conflict(
                        &removal.key,
                        &removal.source,
                        set_source,
                    ));
                }
                removed_by_key
                    .entry(removal.key.clone())
                    .or_insert_with(|| removal.source.clone());
                if emitted_removals.insert(removal.key.clone()) {
                    result.removals.push(removal.clone());
                }
            }

            for entry in patch.entries() {
                validate_env_key(&entry.source, &entry.key)?;
                if let Some(remove_source) = removed_by_key.get(&entry.key) {
                    return Err(EnvPatchDiagnostic::set_remove_conflict(
                        &entry.key,
                        &entry.source,
                        remove_source,
                    ));
                }
                if let Some((existing_value, existing_source)) = set_by_key.get(&entry.key) {
                    if existing_value != &entry.value {
                        return Err(EnvPatchDiagnostic::conflicting_set(
                            &entry.key,
                            &entry.source,
                            existing_source,
                        ));
                    }
                } else {
                    set_by_key.insert(
                        entry.key.clone(),
                        (entry.value.clone(), entry.source.clone()),
                    );
                    result.entries.push(entry.clone());
                }
            }
        }

        Ok(result)
    }

    pub(crate) fn entries(&self) -> &[EnvPatchEntry] {
        &self.entries
    }

    pub(crate) fn removals(&self) -> &[EnvPatchRemoval] {
        &self.removals
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.removals.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn as_map(&self) -> BTreeMap<String, String> {
        self.entries
            .iter()
            .map(|entry| (entry.key.clone(), entry.value.clone()))
            .collect()
    }
}

impl Default for EnvPatch {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvPatchDiagnostic {
    fn malformed_key(source: &str, key: &str, detail: impl Into<String>) -> Self {
        Self {
            kind: EnvPatchDiagnosticKind::MalformedKey,
            key: key.to_string(),
            source: source.to_string(),
            existing_source: None,
            detail: detail.into(),
        }
    }

    fn conflicting_set(key: &str, source: &str, existing_source: &str) -> Self {
        Self {
            kind: EnvPatchDiagnosticKind::ConflictingSet,
            key: key.to_string(),
            source: source.to_string(),
            existing_source: Some(existing_source.to_string()),
            detail: "conflicting set values".to_string(),
        }
    }

    fn set_remove_conflict(key: &str, source: &str, existing_source: &str) -> Self {
        Self {
            kind: EnvPatchDiagnosticKind::SetRemoveConflict,
            key: key.to_string(),
            source: source.to_string(),
            existing_source: Some(existing_source.to_string()),
            detail: "set/remove conflict".to_string(),
        }
    }
}

impl fmt::Display for EnvPatchDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.existing_source {
            Some(existing_source) => write!(
                f,
                "env patch {:?} for key {:?}: {} between source {:?} and source {:?}",
                self.kind, self.key, self.detail, existing_source, self.source
            ),
            None => write!(
                f,
                "env patch {:?} for key {:?} from source {:?}: {}",
                self.kind, self.key, self.source, self.detail
            ),
        }
    }
}

fn validate_env_key(source: &str, key: &str) -> Result<(), EnvPatchDiagnostic> {
    let mut chars = key.chars();
    let Some(first) = chars.next() else {
        return Err(EnvPatchDiagnostic::malformed_key(
            source,
            key,
            "key must not be empty",
        ));
    };
    if !is_env_key_initial_char(first) {
        return Err(EnvPatchDiagnostic::malformed_key(
            source,
            key,
            "key must start with an ASCII letter or underscore",
        ));
    }
    if chars.any(|ch| !is_env_key_char(ch)) {
        return Err(EnvPatchDiagnostic::malformed_key(
            source,
            key,
            "key must contain only ASCII letters, digits, or underscores",
        ));
    }
    Ok(())
}

fn is_env_key_initial_char(ch: char) -> bool {
    ch == ENV_KEY_UNDERSCORE || ch.is_ascii_alphabetic()
}

fn is_env_key_char(ch: char) -> bool {
    is_env_key_initial_char(ch) || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE_BASE: &str = "base";
    const SOURCE_FEATURE: &str = "feature";
    const SOURCE_REMOVE: &str = "remove";
    const TEST_KEY_A: &str = "MC_COMPAT_A";
    const TEST_KEY_B: &str = "MC_COMPAT_B";
    const TEST_KEY_C: &str = "MC_COMPAT_C";
    const TEST_VALUE_ONE: &str = "1";
    const TEST_VALUE_TWO: &str = "2";
    const FEATURE_ENTRY_INDEX: usize = 2;

    #[test]
    fn env_patch_composes_deterministically_and_keeps_sources() {
        let base = EnvPatch::new()
            .set(SOURCE_BASE, TEST_KEY_A, TEST_VALUE_ONE)
            .expect("base env key is valid")
            .set(SOURCE_BASE, TEST_KEY_B, TEST_VALUE_TWO)
            .expect("second base env key is valid");
        let feature = EnvPatch::new()
            .set(SOURCE_FEATURE, TEST_KEY_A, TEST_VALUE_ONE)
            .expect("idempotent duplicate key is valid")
            .set(SOURCE_FEATURE, TEST_KEY_C, TEST_VALUE_ONE)
            .expect("feature env key is valid");

        let composed = EnvPatch::compose(&[base, feature]).expect("compatible patches compose");

        assert_eq!(
            composed.as_map(),
            BTreeMap::from([
                (TEST_KEY_A.to_string(), TEST_VALUE_ONE.to_string()),
                (TEST_KEY_B.to_string(), TEST_VALUE_TWO.to_string()),
                (TEST_KEY_C.to_string(), TEST_VALUE_ONE.to_string()),
            ])
        );
        assert_eq!(composed.entries()[0].source, SOURCE_BASE);
        assert_eq!(
            composed.entries()[FEATURE_ENTRY_INDEX].source,
            SOURCE_FEATURE
        );
    }

    #[test]
    fn env_patch_rejects_conflicting_keys() {
        let base = EnvPatch::new()
            .set(SOURCE_BASE, TEST_KEY_A, TEST_VALUE_ONE)
            .expect("base env key is valid");
        let feature = EnvPatch::new()
            .set(SOURCE_FEATURE, TEST_KEY_A, TEST_VALUE_TWO)
            .expect("feature env key is valid");

        let err = EnvPatch::compose(&[base, feature]).expect_err("conflict fails closed");

        assert_eq!(err.kind, EnvPatchDiagnosticKind::ConflictingSet);
        assert_eq!(err.key, TEST_KEY_A);
        assert!(err.to_string().contains(SOURCE_FEATURE));
    }

    #[test]
    fn env_patch_rejects_malformed_keys() {
        let err = EnvPatch::new()
            .set(SOURCE_BASE, "MC-COMPAT-BAD", TEST_VALUE_ONE)
            .expect_err("malformed env key fails closed");

        assert_eq!(err.kind, EnvPatchDiagnosticKind::MalformedKey);
        assert!(err.to_string().contains("ASCII letters"));
    }

    #[test]
    fn env_patch_rejects_set_remove_conflicts() {
        let remove = EnvPatch::new()
            .remove(SOURCE_REMOVE, TEST_KEY_A)
            .expect("remove env key is valid");
        let set = EnvPatch::new()
            .set(SOURCE_FEATURE, TEST_KEY_A, TEST_VALUE_ONE)
            .expect("set env key is valid");

        let err = EnvPatch::compose(&[remove, set]).expect_err("set/remove conflict fails");

        assert_eq!(err.kind, EnvPatchDiagnosticKind::SetRemoveConflict);
        assert_eq!(err.key, TEST_KEY_A);
    }
}
