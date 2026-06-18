# Evidence refresh flake smoke contract (2026-06-07)

## Scope

Bounded operational refresh for Cairn change `evidence-refresh-flake-smoke-pass`.

Selected checks:
- Cairn validation for `/home/brittonr/git/mc` with the repo-pinned `.#cairn` app.
- Cairn proposal/design/tasks gates for `evidence-refresh-flake-smoke-pass`.
- Targeted packet promotion check: `checks.x86_64-linux.mc-compat-targeted-packet-promotions-check`.
- Scenario manifest check: `checks.x86_64-linux.mc-compat-scenario-manifest`.
- Representative dry-run smoke: `checks.x86_64-linux.mc-compat-dry-run`.
- Evidence manifest and task-evidence checks after refresh.

## Runtime and side-effect bounds

Commands are non-mutating except for writing review logs and refreshing in-repository BLAKE3 evidence manifests. Nix builds use repo-pinned flake outputs and `--option substituters ""` where practical. Live servers, WAN access, public servers, destructive tests, and production deployments are out of scope.

## Expected evidence outputs

- `docs/evidence/evidence-refresh-flake-smoke-contract-2026-06-07.md`
- `docs/evidence/evidence-refresh-flake-smoke-baseline-2026-06-07.run.log`
- `docs/evidence/evidence-refresh-flake-smoke-manifest-refresh-2026-06-07.run.log`
- `docs/evidence/evidence-refresh-flake-smoke-closeout-2026-06-07.run.log`
- `docs/evidence/evidence-refresh-flake-smoke-archive-2026-06-07.run.log`
- `docs/evidence/evidence-refresh-flake-smoke-post-archive-2026-06-07.run.log`
- `docs/evidence/evidence-refresh-flake-smoke-2026-06-07.b3`

## Non-claims

This refresh does not add live gameplay parity, public-server safety, WAN behavior, production readiness, semantic equivalence, new packet rows, new scenario coverage, or new acceptance-matrix promotions. Failed or skipped smokes must remain fail-closed blockers with owner and next action rather than compatibility claims.
