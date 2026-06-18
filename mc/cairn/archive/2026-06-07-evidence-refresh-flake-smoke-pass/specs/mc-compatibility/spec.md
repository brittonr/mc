# mc-compatibility Change Spec: Evidence refresh and flake smoke pass

## Requirements

### Requirement: Evidence refresh smoke contract

r[mc_compatibility.evidence_refresh_flake_smoke.contract] The evidence refresh pass MUST define a bounded smoke set, command scope, runtime limits, evidence paths, and non-claims before running checks.

#### Scenario: Smoke scope is explicit

r[mc_compatibility.evidence_refresh_flake_smoke.contract.scope]
- GIVEN evidence refresh work starts
- WHEN reviewers inspect the smoke contract
- THEN it names selected Cairn checks, targeted packet checks, scenario manifest checks, representative flake dry-runs or smokes, expected evidence outputs, runtime limits, and non-claims
- AND live gameplay parity, public-server safety, WAN behavior, production readiness, and new packet/gameplay coverage remain explicit non-claims.

### Requirement: Evidence refresh baseline

r[mc_compatibility.evidence_refresh_flake_smoke.baseline] The evidence refresh pass MUST run selected non-mutating baseline checks before manifest refresh.

#### Scenario: Baseline runs before evidence mutation

r[mc_compatibility.evidence_refresh_flake_smoke.baseline.recorded]
- GIVEN selected checks and dry-runs can reveal stale metadata
- WHEN baseline checks run
- THEN logs record command shape, selected scope, success or fail-closed diagnostics, and non-claim status before BLAKE3 manifests are refreshed.

### Requirement: Evidence refresh logs

r[mc_compatibility.evidence_refresh_flake_smoke.logs] Review-critical smoke output MUST be recorded under `docs/evidence/` with explicit successful exit statuses or blocker notes.

#### Scenario: Run logs are citeable

r[mc_compatibility.evidence_refresh_flake_smoke.logs.reviewable]
- GIVEN a smoke or check output is cited by tasks or closeout notes
- WHEN reviewers inspect the evidence
- THEN the cited `.run.log` contains an `exit_status=0` line for successful checks
- AND any failed or skipped check is represented as a blocker note with owner and next action rather than a passing claim.

### Requirement: Evidence refresh manifests

r[mc_compatibility.evidence_refresh_flake_smoke.manifests] BLAKE3 manifests MUST be refreshed only for changed tracked evidence files and rerun to a deterministic fixpoint.

#### Scenario: Manifest refresh is deterministic

r[mc_compatibility.evidence_refresh_flake_smoke.manifests.fixpoint]
- GIVEN smoke logs or evidence sidecars changed
- WHEN manifest refresh runs
- THEN only digest fields for existing in-repository evidence files are updated
- AND evidence-manifest checks pass after refresh or report a fail-closed blocker.

### Requirement: Evidence refresh validation

r[mc_compatibility.evidence_refresh_flake_smoke.validation] The evidence refresh pass MUST record task-evidence checks, Cairn gates, sync/archive checks, and post-archive validation before closeout.

#### Scenario: Closeout validation is complete

r[mc_compatibility.evidence_refresh_flake_smoke.validation.logs]
- GIVEN smoke logs and manifests are refreshed or blockers are recorded
- WHEN the change is archived
- THEN reviewable logs show selected smoke checks, manifest refresh/checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing without promoting new compatibility rows.
