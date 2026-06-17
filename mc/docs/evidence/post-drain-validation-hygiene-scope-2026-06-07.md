# Post-drain validation hygiene scope — 2026-06-07

## Question

Can the workspace start the next Cairn drain from a clean validation/evidence baseline without changing compatibility behavior?

## Commands in scope

- `nix run path:/home/brittonr/git/mc#cairn -- validate --root /home/brittonr/git/mc`
- `nix run path:/home/brittonr/git/mc#cairn -- gate proposal|design|tasks <active-change> --root /home/brittonr/git/mc` for each active change package present at baseline time.
- `nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-evidence-manifests --no-link -L`
- `nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-cairn-task-evidence --no-link -L`
- `nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-acceptance-matrix --no-link -L`
- `nix build path:/home/brittonr/git/mc#checks.x86_64-linux.mc-compat-current-evidence-bundle --no-link -L`

## Classification vocabulary

Diagnostics are classified as metadata drift, evidence freshness drift, task citation drift, policy/schema drift, implementation defect, or blocker. Deterministic remediation is limited to review metadata such as BLAKE3 manifests, run logs, and stale drain notes.

## Non-claim contract

This hygiene pass does not add or broaden gameplay coverage, protocol coverage, public-server safety, production readiness, semantic equivalence, packet inventory behavior, runner scenario semantics, or current-bundle compatibility claims.

## Owner / next action

Owner: local Cairn drain agent. Next action: run the non-mutating baseline, record diagnostics, and repair only deterministic metadata drift if the baseline identifies any.
