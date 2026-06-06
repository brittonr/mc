# Tasks

- [ ] [serial] Define the bounded chunk biome data packet contract, fixture source, parser expectations, optional live context, and explicit non-claims. r[mc_compatibility.chunk_biome_data_packet_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid packet evidence, missing row id, wrong packet, missing fixture identity, parser omission, protocol mismatch, and broad biome/chunk/worldgen overclaims. r[mc_compatibility.chunk_biome_data_packet_promotion.checker]
- [ ] [depends:checker] Add or select isolated fixture/live-context rails without changing existing chunk/biome rows. r[mc_compatibility.chunk_biome_data_packet_promotion.rail]
- [ ] [depends:rail] Produce reviewable fixture payloads or hashes, normalized inputs, optional receipts, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.chunk_biome_data_packet_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured packet row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.chunk_biome_data_packet_promotion.matrix]
- [ ] [depends:matrix] Run checker, fixture/runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.chunk_biome_data_packet_promotion.validation]
