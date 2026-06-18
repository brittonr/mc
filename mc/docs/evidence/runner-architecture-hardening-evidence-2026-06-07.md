# Runner architecture hardening evidence (2026-06-07)

## Selected seam

The pass hardens `tools/check_scenario_manifest.rs` live-capability registry validation. The public checker still validates the same `tools/mc-compat-runner/src/scenario_core.rs` tokens, but token presence is now evaluated by the pure `evaluate_live_capability_registry_surface` core and converted to diagnostics by the thin `live_capability_registry_diagnostics` shell.

## Public output preservation

Baseline and post-refactor logs show:
- The checker compiles with `rustc --edition=2021`.
- `--self-test` passes before and after the refactor.
- The repository scenario manifest check passes before and after the refactor.
- The flake scenario-manifest check passes after the refactor.

## Positive and negative coverage

Self-tests now cover:
- Positive complete live-capability registry surface.
- Negative missing-token registry surface.
- Negative malformed/unknown registry surface with fail-closed diagnostic for a required token.
- Negative stale-revision registry surface that fails closed when required live capability tokens are absent.
- Negative overclaim registry surface that fails closed when required live capability tokens are absent.
- Existing manifest negative fixtures: duplicate rows, missing alias, missing milestone, invalid wrapper, and unsupported migration state.

## Non-claims

No compatibility rows, packet rows, gameplay semantics, receipt schemas, scenario names, milestone IDs, backend names, public-server claims, production-readiness claims, or acceptance-matrix classifications were changed by this hardening pass.
