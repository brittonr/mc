# Tasks

- [x] [serial] Satisfy `triage_block`. r[mc_compatibility.roi_04_scenario_triage_receipts.triage_block]
  - Evidence: Runner `smoke_receipt_json()` emits a `triage` block with `first_missing_client_milestone`, `first_missing_server_milestone`, `first_forbidden_pattern`, `first_forbidden_source`, `client_log_paths`, `server_log_path`, and `suggested_boundary` (values: `preflight-or-server-startup`, `protocol-runtime`, `client-probe`, `server-correlation`, `runner-error`, `none`). The `suggested_triage_boundary()` function computes the boundary from error/client-evidence/forbidden-pattern/missing-milestone inputs.
  - Verification: `nix build .#checks.x86_64-linux.mc-compat-multi-client-scenario-dry-run` passes with receipt containing all triage fields.
  - Non-claims: triage is a local-debug aid; it does not strengthen correctness or semantic-equivalence claims.
- [x] [serial] Satisfy `triage_tests`. r[mc_compatibility.roi_04_scenario_triage_receipts.triage_tests]
  - Evidence: Two unit tests cover triage regression:
    - `scenario_receipt_records_actionable_failure_triage()` — verifies triage block with missing milestones (`render_tick`, `server_flag_or_score`), forbidden pattern (`unexpected_eof`), forbidden source (`client`), log paths, and `suggested_boundary: "protocol-runtime"`.
    - `failed_preflight_receipt_triages_before_client_evidence()` — verifies preflight error produces `first_missing_client_milestone: "protocol_detected"` and `suggested_boundary: "preflight-or-server-startup"`.
  - Verification: Tests are part of `tools/mc-compat-runner/src/main.rs` under the `tests` module.
- [x] [serial] Satisfy `docs_check`. r[mc_compatibility.roi_04_scenario_triage_receipts.docs_check]
  - Evidence: `README.md` line 145 documents triage fields and `suggested_boundary` values. Evidence file `docs/evidence/valence-ctf-latency-jitter-inventory.md` references triage output. Nix dry-run checks (`mc-compat-multi-client-scenario-dry-run`, `mc-compat-blue-flag-score-dry-run`) grep for all triage receipt fields as deterministic contract validation.
  - Non-claims: documentation reflects local harness behavior only.
