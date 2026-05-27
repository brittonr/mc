# Tasks

- [x] [serial] Satisfy `claim_demoted`. r[mc_compatibility.roi_09_demote_projectile_damage_claim.claim_demoted]
- [x] [serial] Satisfy `blocker_checkpoint`. r[mc_compatibility.roi_09_demote_projectile_damage_claim.blocker_checkpoint]
- [x] [serial] Satisfy `checker_alignment`. r[mc_compatibility.roi_09_demote_projectile_damage_claim.checker_alignment]

## Evidence

- Acceptance matrix demoted projectile damage attribution and now passes with `acceptance matrix ok: 14 seams, 14 hashes`.
- Current evidence bundle demoted projectile damage attribution and now passes with `current evidence bundle ok: 14 seams`.
- Residual combat catalog lists projectile damage attribution as the next blocked seam, requiring pinned Valence instrumentation and causal client/server receipt proof.
- Blocker checkpoint: `docs/evidence/protocol-763-roi-09-projectile-damage-demotion-checkpoint-2026-05-27.md`; BLAKE3 `a76eafd8601b6d73480eb522f918719f689ff9f3f8a8890a07b7336969f5eda5`.
- ROI 08 summary now marks artifacts experimental/demoted; BLAKE3 `c4659cb52dc66b4037f7f62d1ecd822a661593064ab798dd44ba7004f3a892a0`.
- Evidence manifest check passes with `evidence manifests ok: 6 manifests, 20 entries, 32 receipts scanned`.
