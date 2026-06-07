# Tasks

- [ ] [serial] Record the local resource-pack status contract: actor, local fixture identity, offer metadata, expected status response, packet rows, no-external-fetch guarantee, redaction policy, and non-claims. r[mc_compatibility.resource_pack_status_local_rail.contract]
- [ ] [depends:contract] Run baseline targeted packet, matrix, current-bundle, and packet-inventory checks before rail changes. r[mc_compatibility.resource_pack_status_local_rail.baseline]
- [ ] [depends:baseline] Add an isolated owned-local resource-pack offer/status rail or deterministic fixture path. r[mc_compatibility.resource_pack_status_local_rail.rail]
- [ ] [depends:rail] Emit reviewable KV/receipt/log evidence under `docs/evidence/` with row id, packet rows, local fixture identity, status response, no-external-fetch metric, backend/client path, and non-claims. r[mc_compatibility.resource_pack_status_local_rail.evidence]
- [ ] [depends:evidence] Validate resource-pack live evidence through positive and negative checker coverage for wrong status, missing local scope, missing no-external-fetch metric, stale receipt, wrong packet row, and overclaim rejection. r[mc_compatibility.resource_pack_status_local_rail.checker]
- [ ] [depends:checker] Promote only `resource-pack-status` in matrix/current-bundle/packet-inventory docs if live evidence passes; otherwise record blocker evidence and keep the row fixture-bounded. r[mc_compatibility.resource_pack_status_local_rail.promotion]
- [ ] [depends:promotion] Run runner checks, targeted packet checks, matrix/bundle/inventory checks, evidence-manifest/task-evidence checks, Cairn gates, sync, archive, and post-archive validation. r[mc_compatibility.resource_pack_status_local_rail.validation]
