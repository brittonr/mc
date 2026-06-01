# Delta: Protocol chunk and biome family coverage rail

## Requirements

### Requirement: Contract

r[mc_compatibility.protocol_chunk_biome_family_coverage.contract] The `chunk/biome packet family` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.protocol_chunk_biome_family_coverage.contract.scope]
- GIVEN `protocol-chunk-biome-family` work starts
- WHEN the evidence contract is reviewed
- THEN it names selected chunk/biome packet rows with reviewed parser fixtures and one live fixture proving client receipt of configured environment data
- AND it states that world generation parity, all chunks, all biomes, lighting/weather, structures, full protocol-763 compatibility, full survival compatibility, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.protocol_chunk_biome_family_coverage.checker] A deterministic checker MUST compare normalized metrics before the `chunk/biome packet family` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.protocol_chunk_biome_family_coverage.checker.rejects]
- GIVEN evidence is missing or mismatches packet name, wire id, chunk position, biome id or environment id, parser fixture id, live receipt path, and malformed fixture status where supported
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.protocol_chunk_biome_family_coverage.checker.standard]
- GIVEN the row requires protocol ledger entries plus live fixture evidence; worldgen and terrain parity remain separate
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.protocol_chunk_biome_family_coverage.rail] The harness MUST expose a `protocol-chunk-biome-family` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.protocol_chunk_biome_family_coverage.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `protocol-chunk-biome-family` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.protocol_chunk_biome_family_coverage.evidence] `chunk/biome packet family` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.protocol_chunk_biome_family_coverage.evidence.reviewable]
- GIVEN the `chunk/biome packet family` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.protocol_chunk_biome_family_coverage.matrix] Acceptance matrix and current-bundle docs MUST promote only the `chunk/biome packet family` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.protocol_chunk_biome_family_coverage.matrix.nonclaims]
- GIVEN `chunk/biome packet family` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `chunk/biome packet family` row is marked covered
- AND world generation parity, all chunks, all biomes, lighting/weather, structures, full protocol-763 compatibility, full survival compatibility, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.protocol_chunk_biome_family_coverage.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.protocol_chunk_biome_family_coverage.validation.log]
- GIVEN the `chunk/biome packet family` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
