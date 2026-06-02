# Survival hunger/food task evidence oracle

## Question

Do the archived `survival-hunger-food-parity` contract and checker tasks remain reviewable when their primary cited evidence files predate the hunger/food implementation commit and may not appear in a narrow review path list?

## Inspected evidence

- `docs/evidence/survival-gap-cairns-2026-05-31.run.log` (`8cda8ae8321f33340fd25e4a68312886636e782330b5b50aacce613da7fb5935`) records `survival-hunger-food-parity` Cairn proposal, design, and tasks gates with `valid: true`, `verdict: "PASS"`, and `exit_status=0` for each stage.
- `docs/evidence/survival-gap-cairns-2026-05-31.b3` (`a79643c2d8261c580f5d6fbf746948490926b2c56e21a0c190d61c94b18dd8ed`) covers the gap Cairn validation log.
- `docs/evidence/survival-row-parity-checker-2026-06-01.run.log` (`0b7669f2e4cbef771be2249d39c22fd2f2d33c97982b2db6f863c77669ab4c7f`) records `rustfmt --check tools/check_survival_row_parity.rs`, `rustc --edition=2021 tools/check_survival_row_parity.rs`, and checker self-test output `survival row parity self-test ok: 6 row contracts exercised`, followed by `exit_status=0`.
- `docs/evidence/survival-row-parity-checker-2026-06-01.b3` (`61d0ae4cfc7378d658ef192d9c3ca393ae5b8655fec34b4d0e0f30386b95a4ff`) covers the checker log and `tools/check_survival_row_parity.rs`.
- `docs/evidence/survival-hunger-food-cairn-gates-2026-06-02.run.log` records a post-archive temporary active-copy gate replay for the archived proposal/design/tasks artifacts, with all three stages returning `valid: true`, `verdict: "PASS"`, and `exit_status=0`.

## Decision

The first two archived tasks may remain complete. The contract evidence is older but tracked and BLAKE3-backed, and the checker evidence exercised the shared survival-row parity checker across all six missing-row contracts before the hunger/food rail implementation. The post-archive gate replay confirms the archived artifacts still pass Cairn proposal/design/tasks gates.

This checkpoint does not expand the claim beyond the bounded Bread consume row.

## Owner

mc compatibility maintainer / current agent session.

## Next action

Keep this checkpoint and `docs/evidence/survival-hunger-food-cairn-gates-2026-06-02.run.log` in the validation manifest so future narrow reviews can verify the previously cited evidence without relying on unchanged-file discovery.
