# Tasks

- [x] [serial] Audit whether the Hyperion checkout is parent-tracked and whether the nested checkout still has live repo state. r[valence_hyperion_integration.hyperion_checkout_retirement.audit]
  - Evidence: `docs/evidence/run-logs/2026-07-01/hyperion-retirement-readiness.audit.run.log`, `docs/evidence/oracles/2026-07-01/hyperion-retirement-readiness.md`, and `docs/evidence/manifests/2026-07-01/hyperion-retirement-readiness.b3`.
- [x] [depends:audit] Audit accepted specs, docs/config, and promoted evidence references that still depend on Hyperion-local ownership. r[valence_hyperion_integration.hyperion_checkout_retirement.audit]
  - Evidence: `docs/evidence/run-logs/2026-07-01/hyperion-retirement-readiness.audit.run.log`, `docs/evidence/oracles/2026-07-01/hyperion-retirement-readiness.md`, and `docs/evidence/manifests/2026-07-01/hyperion-retirement-readiness.b3`.
- [x] [depends:audit] Preserve the Hyperion checkout and record deletion as blocked while live references and nested work remain. r[valence_hyperion_integration.hyperion_checkout_retirement.blockers]
  - Evidence: `docs/evidence/run-logs/2026-07-01/hyperion-retirement-readiness.audit.run.log`, `docs/evidence/oracles/2026-07-01/hyperion-retirement-readiness.md`, and `docs/evidence/manifests/2026-07-01/hyperion-retirement-readiness.b3`.
- [x] [depends:blockers] Define the future deletion mechanism gate so physical removal is reviewable as a parent-tracked diff, local cleanup, or nested-repo archival action. r[valence_hyperion_integration.hyperion_checkout_retirement.mechanism]
  - Evidence: `docs/evidence/run-logs/2026-07-01/hyperion-retirement-readiness.audit.run.log`, `docs/evidence/oracles/2026-07-01/hyperion-retirement-readiness.md`, `cairn/changes/gate-hyperion-checkout-retirement/specs/valence-hyperion-integration/spec.md`, and `docs/evidence/manifests/2026-07-01/hyperion-retirement-readiness.b3`.
- [x] [depends:mechanism] Run Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest validation with promoted logs. r[valence_hyperion_integration.hyperion_checkout_retirement.validation]
  - Evidence: `docs/evidence/run-logs/2026-07-01/hyperion-retirement-readiness.cairn-gates.run.log` and `docs/evidence/manifests/2026-07-01/hyperion-retirement-readiness.b3`.
