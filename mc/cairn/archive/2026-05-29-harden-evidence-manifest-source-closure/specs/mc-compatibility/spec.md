# Delta: Evidence manifest source closure

## Requirements

### Requirement: Evidence manifest source closure

r[mc_compatibility.evidence_manifest_source_closure.contract] Promoted evidence BLAKE3 manifests MUST cite files present in the parent repo source closure unless the row is explicitly represented by a reviewable oracle document.

#### Scenario: Manifest row is Nix-reviewable

r[mc_compatibility.evidence_manifest_source_closure.contract.reviewable]
- GIVEN a `docs/evidence/*.b3` manifest is promoted
- WHEN the repo is evaluated through Nix
- THEN every manifest path resolves to a file in the parent repo source closure
- AND child-repo or generated-output bytes are copied under `docs/evidence/` before being cited

### Requirement: External artifact closure preservation

r[mc_compatibility.evidence_manifest_source_closure.artifacts] Child-repo source files and generated artifacts referenced by evidence manifests MUST be preserved as durable copied artifacts without changing their BLAKE3 content identity.

#### Scenario: Copied artifact keeps digest

r[mc_compatibility.evidence_manifest_source_closure.artifacts.digest]
- GIVEN a manifest previously cited a nested repo file or `target/` artifact
- WHEN the artifact is copied into `docs/evidence/`
- THEN the manifest cites the copied path
- AND the BLAKE3 digest remains the digest of the copied artifact bytes

### Requirement: Accepted-spec digest refresh

r[mc_compatibility.evidence_manifest_source_closure.spec_digest] Evidence manifests that intentionally include accepted spec files MUST be refreshed after accepted spec edits.

#### Scenario: Spec digest rows match current accepted spec

r[mc_compatibility.evidence_manifest_source_closure.spec_digest.current]
- GIVEN accepted `cairn/specs/*/spec.md` content changes
- WHEN evidence manifest validation runs
- THEN every manifest row that cites the accepted spec records the current BLAKE3 digest or the row is removed in favor of immutable archive evidence

### Requirement: Source-closure validation evidence

r[mc_compatibility.evidence_manifest_source_closure.validation] Source-closure hardening MUST record local and Nix evidence-manifest validation before archive.

#### Scenario: Nix manifest check passes

r[mc_compatibility.evidence_manifest_source_closure.validation.nix]
- GIVEN source-closure hardening is complete
- WHEN validation evidence is recorded
- THEN local manifest self-test/full scan, Nix `mc-compat-evidence-manifests`, Cairn validation, and Cairn gates pass with output copied under `docs/evidence/`
