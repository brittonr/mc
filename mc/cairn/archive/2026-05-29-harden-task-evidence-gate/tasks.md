# Tasks

- [x] [serial] Add the task-evidence closeout requirement to the mc compatibility spec. r[mc_compatibility.task_evidence_gate.active_closeout]
  - Evidence: `docs/evidence/task-evidence-gate-validation-2026-05-29.run.log` records Cairn proposal/design/tasks gates and validation after adding the delta spec; BLAKE3 manifest `docs/evidence/task-evidence-gate-validation-2026-05-29.b3`.
- [x] [depends:mc_compatibility.task_evidence_gate.active_closeout] Implement the Rust checker with pure parsing/validation core, positive fixtures, and negative fixtures for missing label, copied evidence, run log, BLAKE3, and artifact existence. r[mc_compatibility.task_evidence_gate.fail_closed]
  - Evidence: `docs/evidence/task-evidence-gate-validation-2026-05-29.run.log` records `check_cairn_task_evidence.rs --self-test` and active scan output; BLAKE3 manifest `docs/evidence/task-evidence-gate-validation-2026-05-29.b3`.
- [x] [depends:mc_compatibility.task_evidence_gate.fail_closed] Wire the checker into `flake.nix`, the maintained aggregate, and README operator workflow. r[mc_compatibility.task_evidence_gate.flake_workflow]
  - Evidence: `docs/evidence/task-evidence-gate-validation-2026-05-29.run.log` records `nix build .#checks.x86_64-linux.mc-compat-cairn-task-evidence --no-link -L`; BLAKE3 manifest `docs/evidence/task-evidence-gate-validation-2026-05-29.b3`.
- [x] [depends:mc_compatibility.task_evidence_gate.flake_workflow] Record validation output under `docs/evidence/`, run Cairn validation/gates, and archive the change. r[mc_compatibility.task_evidence_gate.validation_evidence]
  - Evidence: `docs/evidence/task-evidence-gate-validation-2026-05-29.run.log` records Cairn validation/gates; `docs/evidence/task-evidence-gate-manifest-validation-2026-05-29.run.log` records local evidence manifest self-test/full scan. BLAKE3 manifests: `docs/evidence/task-evidence-gate-validation-2026-05-29.b3`, `docs/evidence/task-evidence-gate-manifest-validation-2026-05-29.b3`.
