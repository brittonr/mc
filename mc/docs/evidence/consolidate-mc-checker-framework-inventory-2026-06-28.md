# consolidate-mc-checker-framework evidence inventory

## Requirement scope

- r[mc_compatibility.checker_framework.shared_components]
- r[mc_compatibility.checker_framework.checker_parity]
- r[mc_compatibility.checker_framework.python_migration]
- r[mc_compatibility.checker_framework.positive_tests]
- r[mc_compatibility.checker_framework.negative_tests]
- r[mc_compatibility.checker_framework.validation]

## Owner subtree

The implemented scope is the root `tools/` checker subtree plus `nix/checks.nix` and `docs/check-tiers.md` gate documentation. No Hyperion, Valence, Stevenarella, or Leafish behavior was used or changed.

## Duplication inventory

The baseline checker inventory found repeated key/value evidence parsing and validation helpers in representative packet-family checkers. The selected representative migrations were:

- `tools/check_scoreboard_team_packet_family.rs`: migrated fully to `tools/checker_framework.rs` for key/value parsing, exact/ok/true field checks, concrete child revision checks, truthy overclaim rejection, self-test fixture execution, and stable CLI shell.
- `tools/check_movement_packet_family.rs`: migrated key/value evidence parsing and common validators to `tools/checker_framework.rs`; the TSV packet-inventory parser remains local because it is checker-specific and is covered by existing positive/negative inventory fixtures.

`tools/check_checker_framework_usage.rs` now guards those representative migrations against reintroducing local `struct Evidence`/`BTreeMap<String, String>` key/value records or direct `split_once('=')` parsing.

## Python migration outcome

No Python checker was touched by this consolidation. The current legacy Python checker list is inventoried in `tools/check_checker_framework_usage.rs` and remains explicitly outside this change scope until the next owner-driven change selects one for migration.

## Validation evidence

- Preflight Cairn gates and validation: `docs/evidence/consolidate-mc-checker-framework-preflight-2026-06-28.run.log`
- Baseline framework/checker commands before core edits: `docs/evidence/consolidate-mc-checker-framework-baseline-2026-06-28.run.log`
- Focused framework tests, usage guard, and representative checker parity: `docs/evidence/consolidate-mc-checker-framework-focused-2026-06-28.run.log`
- New focused flake check: `docs/evidence/consolidate-mc-checker-framework-flake-2026-06-28.run.log`
- Existing affected checker flake checks and check-tier freshness: `docs/evidence/consolidate-mc-checker-framework-affected-checks-2026-06-28.run.log`
- Flake output inventory after adding the checker-framework check and the already-present paired-reference dry-run-shapes check to the allowlist: `docs/evidence/consolidate-mc-checker-framework-flake-inventory-2026-06-28.run.log`

## Non-claims

This consolidation is checker maintainability and gate-parity evidence only. It does not promote new Minecraft compatibility evidence and does not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness.
