# valence-hyperion-integration Change Spec: Integration roadmap

## Requirements

### Requirement: Integration roadmap inventory

r[valence_hyperion_integration.roadmap.inventory] Valence/Hyperion roadmap work MUST inventory accepted specs, archived integration Cairns, active Cairns, Hyperion architecture docs, Valence architecture docs, and the Hyperion integration boundary before publishing a convergence sequence.

#### Scenario: Roadmap sources are reviewable

r[valence_hyperion_integration.roadmap.inventory.reviewable]
- GIVEN roadmap work is selected
- WHEN reviewers inspect the inventory
- THEN the relevant accepted specs, archived Cairns, active Cairns, Hyperion docs, Valence docs, boundary rules, known stale evidence, and existing non-claims are recorded.

### Requirement: Integration ownership map

r[valence_hyperion_integration.roadmap.ownership] The roadmap MUST identify Valence-owned, Hyperion-owned, adapter-owned, and reference-only responsibilities before proposing implementation slices.

#### Scenario: Ownership prevents core collapse

r[valence_hyperion_integration.roadmap.ownership.boundaries]
- GIVEN a future merge slice is proposed
- WHEN reviewers compare it with the ownership map
- THEN Valence public API ownership, Hyperion runtime ownership, adapter ownership, and reference-only sources are distinguishable
- AND forbidden core-merge categories remain rejected or reference-only.

### Requirement: Integration dependency sequence

r[valence_hyperion_integration.roadmap.sequence] The roadmap SHOULD define dependency ordering, prerequisites, stop conditions, and evidence gates for ownership audit, adapter contracts, bridge slices, optional backends, optional gameplay plugins, and later consolidation decisions.

#### Scenario: Bridge work waits for ownership decisions

r[valence_hyperion_integration.roadmap.sequence.ownership_first]
- GIVEN a bridge implementation slice depends on shared type or packet semantics
- WHEN its Cairn is reviewed
- THEN ownership and adapter-contract decisions are already recorded or the slice is blocked with an explicit dependency.

### Requirement: Integration decision records

r[valence_hyperion_integration.roadmap.decision_records] Future roadmap-linked implementation Cairns MUST include decision records for inspected Hyperion sources using adopt, port, reference, or reject classifications.

#### Scenario: Source decision includes non-claims

r[valence_hyperion_integration.roadmap.decision_records.non_claims]
- GIVEN a Hyperion source influences a Valence or adapter implementation
- WHEN the implementation Cairn records the source decision
- THEN the record identifies ownership, target, classification, safety notes, required evidence, and unsupported compatibility, production-scale, default-behavior, and vanilla-parity claims.

### Requirement: Integration roadmap reconciliation

r[valence_hyperion_integration.roadmap.reconciliation] The roadmap MUST reconcile new proposed work against existing archived and active Cairns so duplicate packet compose, proxy broadcast, chunk cache, and gameplay-plugin scopes are avoided or explicitly superseded.

#### Scenario: Duplicate scope is caught

r[valence_hyperion_integration.roadmap.reconciliation.duplicate]
- GIVEN a new Cairn proposes work already covered by an archived or active integration Cairn
- WHEN roadmap reconciliation runs
- THEN the new Cairn either cites and narrows the prior scope, declares a supersession path, or removes the duplicate task before implementation.

### Requirement: Integration roadmap validation

r[valence_hyperion_integration.roadmap.validation] Roadmap work MUST record Cairn proposal, design, tasks, repository validation, and evidence-manifest checks for promoted roadmap evidence before archive.

#### Scenario: Roadmap closeout is reviewable

r[valence_hyperion_integration.roadmap.validation.log]
- GIVEN roadmap work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show Cairn proposal/design/tasks gates, Cairn validation, promoted roadmap evidence, evidence manifests when cited, and explicit non-claims for implementation, runtime replacement, default behavior, Hyperion compatibility, production scale, and vanilla parity.
