# Agent Notes

## Scope
- Stevenarella is the core Rust Minecraft client used by mc-compat rails and manual client checks.
- This tree is owned by the parent `/home/brittonr/git` repository. It is not a nested Git repo or submodule.
- Keep client changes scoped to `clients/stevenarella/` unless the Cairn task explicitly requires runner, fixture, or server updates.

## Workflow
- Read `README.md` before editing client behavior or protocol support.
- Run Cargo through the mc devshell from this directory so native UI, OpenSSL, font, and X11 dependencies are present:
  - `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo check`
  - `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test world::tests -- --nocapture`
  - `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test protocol -- --nocapture`
- Use the root flake wrappers for compatibility probes rather than ad hoc client launch scripts:
  - `nix run /home/brittonr/git/mc#stevenarella -- --dry-run`
  - `nix run /home/brittonr/git/mc#mc-compat-smoke -- --dry-run --server-backend valence --scenario smoke`

## Compatibility instrumentation
- Compat probes and typed-event hooks are test harness surfaces. Keep them explicit, bounded, and easy to distinguish from normal client behavior.
- Do not broaden a probe into a gameplay/parity claim without paired evidence and a Cairn requirement that names the claim boundary.
- On protocol `1.20.1`, under-map CTF symptoms can come from missing dimension-codec bounds. JoinGame supplies `dimension_codec` plus `dimension_type_name`; apply the selected type's `min_y` and `height` before parsing `ChunkData_AndLight` sections.

## VCS and evidence
- Use parent-repository status scoped to `mc/clients/stevenarella/` when checking cleanliness or source evidence.
- Do not cite raw `clients/stevenarella/` paths as durable Cairn task evidence. Copy reviewable logs, receipts, and oracle notes under `docs/evidence/` and record BLAKE3 manifests before citing them.
- If a receipt does not machine-record the Stevenarella source-tree revision being claimed, add a `docs/evidence/*oracle*` checkpoint with inspected evidence, decision, owner, and next action.

## Checks
- For docs-only or guidance-only changes in this file, run the repository layout checker and Cairn validation.
- For client code changes, run the smallest relevant Cargo test first, then the affected mc-compat dry-run or live rail required by the Cairn task.
- Include both positive and negative tests for new client logic whenever a test suite is added or extended.
