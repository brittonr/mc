# mc-compatibility Change Spec: Extended manifest-derived surfaces

## Requirements

### Requirement: Scenario-derived surface inventory

r[mc_compatibility.manifest_surface_expansion.inventory] The repository MUST maintain a reviewable inventory of scenario-derived harness surfaces before extending generation beyond the current generated artifacts.

#### Scenario: Surface ownership is classified

r[mc_compatibility.manifest_surface_expansion.inventory.classified]
- GIVEN a surface repeats scenario names, aliases, wrapper names, receipt names, command examples, milestone metadata, or evidence index rows
- WHEN the inventory is reviewed
- THEN the surface is classified as generated, human-authored, or intentionally duplicated
- AND the owner and freshness check strategy are recorded.

### Requirement: Generator fixture coverage

r[mc_compatibility.manifest_surface_expansion.generator] New generated surface classes MUST have generator fixtures for valid manifests and invalid manifest/output conditions.

#### Scenario: Invalid generation fails closed

r[mc_compatibility.manifest_surface_expansion.generator.negative]
- GIVEN a manifest fixture has duplicate generated names, unsafe output paths, missing required fields, unsupported migration states, or unknown generated-field values
- WHEN the generator evaluates the fixture
- THEN deterministic diagnostics identify the invalid row
- AND no partial generated artifact is accepted.

### Requirement: Manifest-derived wrappers

r[mc_compatibility.manifest_surface_expansion.wrappers] Scenario app/check wrapper metadata SHOULD be derived from the scenario manifest when the output is stable, bounded, and reviewable.

#### Scenario: Wrapper semantics are preserved

r[mc_compatibility.manifest_surface_expansion.wrappers.parity]
- GIVEN wrapper metadata is generated or rendered from manifest rows
- WHEN selected dry-run app/check wrappers execute
- THEN command names, scenario names, backend defaults, receipt names, timeout defaults, and non-claim boundaries match the pre-generation behavior.

### Requirement: Manifest-derived docs/index blocks

r[mc_compatibility.manifest_surface_expansion.docs] README or evidence-index content MAY be generated only inside bounded machine-owned blocks that exclude human-authored evidence interpretation.

#### Scenario: Human prose remains outside generated blocks

r[mc_compatibility.manifest_surface_expansion.docs.boundary]
- GIVEN generated command or index blocks are refreshed
- WHEN a reviewer inspects the diff
- THEN generated content is delimited by ownership markers
- AND prose that interprets compatibility, live coverage, production readiness, or evidence meaning remains human-authored.

### Requirement: Generated surface freshness

r[mc_compatibility.manifest_surface_expansion.freshness] The repository MUST fail checks when newly generated manifest-derived outputs are stale.

#### Scenario: Stale generated wrapper fails

r[mc_compatibility.manifest_surface_expansion.freshness.drift]
- GIVEN the scenario manifest changes without refreshing a generated wrapper, docs block, or index artifact
- WHEN the freshness check runs
- THEN it reports the stale path and fails before evidence promotion or archive.

### Requirement: Manifest surface validation

r[mc_compatibility.manifest_surface_expansion.validation] The expansion MUST record generator fixtures, stale-output rejection, selected wrapper dry-runs, maintained dry-run aggregate output, and Cairn gates before archive.

#### Scenario: Generation expansion closeout is reviewable

r[mc_compatibility.manifest_surface_expansion.validation.log]
- GIVEN manifest-derived surface generation is expanded
- WHEN the change is archived
- THEN reviewable logs show positive generator fixtures, negative generator fixtures, stale-output rejection, selected wrapper dry-runs, maintained dry-run aggregate output, Cairn proposal/design/tasks gates, and Cairn validation.
