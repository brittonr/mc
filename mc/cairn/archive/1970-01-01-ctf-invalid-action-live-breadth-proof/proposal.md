# Proposal: Add live proof for CTF invalid-action breadth

## Why

The current `opponent-base-return-drop-without-carrier` invalid-action breadth row is deterministic fixture-only evidence. It is useful as a contract, but it does not prove the live Valence CTF path rejects the same bounded invalid action with client/server correlation.

Adding a live owned-local rail for that row upgrades the evidence class for one invalid-action breadth case while preserving all broad invalid-action and full CTF non-claims.

## What Changes

- Add or enable a live owned-local CTF scenario for opponent-base return/drop without carrier ownership.
- Record client attempt evidence, Valence rejection evidence, unchanged flag state, unchanged score state, and forbidden mutation absence in a receipt and typed-event log.
- Extend the invalid-action breadth checker to accept the live row only when actor, team, flag, attempted action, rejection, unchanged state, non-claims, and tracked evidence artifacts are present.
- Update acceptance matrix/current bundle rows from fixture-only to the new bounded live evidence class only for this row.
- Preserve non-claims for all invalid actions, all flag permutations, adversarial security, public-server safety, production readiness, broad Minecraft compatibility, and vanilla/reference parity.

## Impact

- **Files**: CTF runner scenario code, Valence CTF fixture instrumentation if needed, `tools/check_ctf_invalid_action_breadth.rs`, evidence docs, matrix/current bundle, BLAKE3 manifests, and Cairn lifecycle files.
- **Testing**: checker positive/negative fixtures, focused dry-run/live rail validation, evidence manifest validation, matrix/current-bundle validation, Cairn gates, and Cairn validation.
- **Non-claims**: this promotes only one bounded live invalid-action row. It does not claim full CTF correctness, live invalid-action breadth beyond the configured case, adversarial security, public-server safety, production readiness, broad Minecraft compatibility, or vanilla/reference parity.
