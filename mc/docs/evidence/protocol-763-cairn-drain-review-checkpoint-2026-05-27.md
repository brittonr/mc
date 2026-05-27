# Protocol-763 Cairn drain review checkpoint — 2026-05-27

## Purpose

Review follow-up for the same-family review failure on `cf5c0a7`. The failure was caused by citing untracked `target/...` live receipts as proof. This checkpoint promotes review-critical receipt artifacts into `docs/evidence/` and records an oracle decision for artifacts that were historical or untracked.

## ROI 05 current-head receipt oracle

- Change: `cairn/archive/2026-05-27-roi-05-current-head-live-refresh`.
- Source receipt: `target/mc-compat-current-head-live-refresh/projectile-hit-current-head.json`.
- Reviewable receipt: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.receipt.json`.
- Reviewable run log: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.run.log`.
- Hash manifest: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.b3`.

Independent BLAKE3 command output:

```text
756b6f732e71ae370808b2a653d1310baa88875f2c3345a1c87444fcffb51c6c  docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.receipt.json
05429930472e764a6a2b140ce9c0a7652552659210b4bb1407d93d0d2cd7fada  docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.run.log
```

Receipt oracle fields:

```text
status=pass mode=run dry_run=false scenario=projectile-hit scenario.passed=true
scenario.missing_milestones=[] server.missing_milestones=[] triage.suggested_boundary=none
```

Decision: ROI 05 is reviewable from tracked evidence now. Reviewer can recompute the receipt/log BLAKE3 and inspect the copied JSON/log without relying on `target/...`.

## ROI 01–04 archive checkpoint

These changes were already task-complete before the archive commit. This follow-up adds reviewable copies where local receipts were still available and records the residual historical boundary where they were not.

| Change | Reviewable checkpoint | BLAKE3 / gate evidence | Decision |
| --- | --- | --- | --- |
| ROI 01 armor/equipment mitigation | `docs/evidence/protocol-763-roi-01-armor-equipment-mitigation-2026-05-27.receipt.json`; `docs/evidence/protocol-763-roi-01-armor-equipment-mitigation-2026-05-27.dry-run.receipt.json` | live `176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765`; dry-run `819b042dd37d52a54e0079d28e9535bcc553f0059f77756cb48d83d6705af6db`; dry-run Nix gate rerun in pueue task `14` | Reviewable from copied receipts plus gate. |
| ROI 02 equipment update rail | `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json`; `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.run.log`; `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.dry-run.receipt.json`; `docs/evidence/protocol-763-runner-verification-2026-05-27.log` | live `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d`; run log `184223509605c1dc9b89db3d8c77ff0b8b7a1c103bec8031de75515d4777dc16`; regenerated dry-run `44383cc9cdb5fe0eaab3ce7da11280eb217a448c81b44b37087b7c1bd5b3696b`; runner verification log `6ddd9f6256e06fb9b2d8afb7cd35e92cf5f7a87d4a1e9819fb0afb46a3098a4f`; dry-run Nix gate and marker checks rerun in pueue task `32`; live rerun passed in pueue task `25` | Reviewable from copied receipt/log plus gate. |
| ROI 03 projectile hit rail | `docs/evidence/protocol-763-roi-03-projectile-hit-2026-05-27.receipt.json`; `docs/evidence/protocol-763-roi-03-projectile-hit-2026-05-27.dry-run.receipt.json` | live `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b`; dry-run `50d0709a192435a7efc3ade64abd4d06f01b31dccedaf9cda35439ced114ae0b`; dry-run Nix gate rerun in pueue task `14` | Reviewable from copied receipts plus gate. |
| ROI 04 scenario triage receipts | Archived task file plus dry-run Nix gate rerun in pueue task `14` | `nix build .#checks.x86_64-linux.mc-compat-multi-client-scenario-dry-run --no-link -L --no-update-lock-file --option builders ''` passed | Reviewable as deterministic receipt-shape/code gate; no live receipt is part of this change. |

## Human checkpoint

- Unresolved question from review: previous commit did not include reviewable live receipt artifacts.
- Inspected evidence: tracked receipt copies above, BLAKE3 manifests, archived tasks, current bundle, Cairn validation, pueue task `14` dry-run gate success, pueue task `25` ROI 02 live rerun success, and pueue task `32` post-fix fmt/test/dry-run/gate success.
- Decision owner: reviewer/maintainer can accept tracked copied receipts or request another live rerun for any historical rail.
- Next action: rerun final matrix/bundle/Cairn validation before commit.
