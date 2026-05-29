# Tasks

- [x] [serial] Build canonical passing client/server fixture builders for every maintained scenario without starting external processes. r[mc_compatibility.oracle_property_tests.fixtures]
  - Evidence: `tools/mc-compat-runner/src/main.rs` test helpers `ALL_TEST_SCENARIOS`, `passing_client_lines`, `passing_server_lines`, `output_from_lines`, and `server_fixture_line_for` build in-memory fixtures only.
- [x] [depends:fixtures] Add positive oracle tests proving each complete client/server fixture passes with empty missing and forbidden lists. r[mc_compatibility.oracle_property_tests.positive]
  - Evidence: `scenario_oracle_property_all_required_client_milestones_matter` and `scenario_oracle_property_all_required_server_milestones_matter` first assert complete fixtures pass for every maintained scenario.
- [x] [depends:fixtures] Add missing-client-milestone mutation tests for every scenario and assert the exact missing milestone diagnostic. r[mc_compatibility.oracle_property_tests.missing_client]
  - Evidence: `scenario_oracle_property_all_required_client_milestones_matter` removes each required client milestone one at a time and asserts the exact `missing_milestones` entry.
- [x] [depends:fixtures] Add missing-server-milestone mutation tests for every scenario with server requirements and assert the exact missing milestone diagnostic. r[mc_compatibility.oracle_property_tests.missing_server]
  - Evidence: `scenario_oracle_property_all_required_server_milestones_matter` removes each required server milestone one at a time; username-only correlation is mutated to a different username so `server_username_seen` cannot pass via other lines.
- [x] [depends:fixtures] Add forbidden-pattern mutation tests for every scenario and assert the exact forbidden marker diagnostic/source. r[mc_compatibility.oracle_property_tests.forbidden]
  - Evidence: `scenario_oracle_property_forbidden_markers_fail` inserts every forbidden marker for every maintained scenario and asserts the exact forbidden diagnostic.
- [x] [depends:fixtures] Add ordered-causality mutation tests for projectile damage and future typed event ordered edges. r[mc_compatibility.oracle_property_tests.causality]
  - Evidence: existing `projectile_damage_attribution_scenario_tracks_client_and_server_evidence` covers the positive causality fixture, out-of-order projectile-use/projectile-hit failure, and missing victim health update failure.
- [x] [depends:fixtures] Add receipt summary and load/network safety mutation tests for missing status, wrong protocol/backend/port, missing headless isolation, unsafe public target, missing telemetry, and missing live receipt. r[mc_compatibility.oracle_property_tests.receipt_mutations]
  - Evidence: `receipt_summary_mutations_fail_closed`, `rejects_receipt_protocol_mismatch`, `rejects_receipts_that_do_not_match_configured_protocol`, and `load_network_safety_envelope_fails_closed_for_unsafe_inputs` cover receipt/status/headless/protocol and safety-envelope mutations.
- [x] [depends:receipt_mutations] Record focused test output and update README/evidence docs to explain oracle property coverage and remaining non-claims. r[mc_compatibility.oracle_property_tests.validation]
  - Evidence: `docs/evidence/protocol-763-oracle-triage-drain-runner-tests-2026-05-29.run.log` records `cargo test --manifest-path tools/mc-compat-runner/Cargo.toml`; BLAKE3 sidecar `docs/evidence/protocol-763-oracle-triage-drain-runner-tests-2026-05-29.b3` records the log digest. README triage text preserves failure/non-claim semantics.
