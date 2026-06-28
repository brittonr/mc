#![allow(unused_imports)]

use crate::test_support::*;
use crate::*;
use std::fs;

const TEST_FAILURE_BUNDLE_FIRST_FAILURE: &str = "scenario missing required milestone";

#[test]
fn failure_bundle_shell_writes_reviewable_bundle_for_failed_result() {
    let temp_root =
        std::env::temp_dir().join(format!("mc-compat-failure-bundle-{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_root);
    let evidence_dir = temp_root.join("docs/evidence");
    fs::create_dir_all(&evidence_dir).expect("create evidence dir");
    let receipt_path = evidence_dir.join("failed-receipt.json");
    let bundle_path = evidence_dir.join("failed-bundle.json");
    fs::write(&receipt_path, "receipt bytes").expect("write receipt artifact");

    let mut cfg =
        test_config(&["--run", "--scenario=smoke"], &[]).expect("failure bundle config parses");
    cfg.root = temp_root.clone();
    cfg.receipt_path = Some(receipt_path);
    cfg.failure_bundle_path = Some(bundle_path.clone());
    cfg.valence_log = evidence_dir.join("valence.log");
    let first_failure = TEST_FAILURE_BUNDLE_FIRST_FAILURE.to_string();
    let result: Result<&Option<ClientRunEvidence>, &String> = Err(&first_failure);

    write_failure_evidence_bundle(&cfg, result).expect("failure bundle writes");
    let json = fs::read_to_string(&bundle_path).expect("read failure bundle");

    assert!(json.contains(FAILURE_BUNDLE_SCHEMA));
    assert!(json.contains(TEST_FAILURE_BUNDLE_FIRST_FAILURE));
    assert!(json.contains("docs/evidence/failed-receipt.json"));
    assert!(json.contains(&blake3::hash(b"receipt bytes").to_hex().to_string()));

    fs::remove_dir_all(&temp_root).expect("remove temp failure bundle root");
}

#[test]
fn runner_result_preserves_original_failure_when_follow_up_fails() {
    let err = combine_runner_result(
        Err("original failure".to_string()),
        vec!["failed to write failure bundle: validation failed".to_string()],
    )
    .expect_err("combined failure remains failing");

    assert!(err.contains("original failure"));
    assert!(err.contains("failed to write failure bundle"));
}
