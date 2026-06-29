# valence-hyperion-integration Change Spec: Hyperion block loader modules

## Requirements

### Requirement: Hyperion block loader scope

r[valence_hyperion_integration.hyperion_block_loader.scope] Hyperion block-loader modularity work MUST be scoped as Hyperion-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion block-loader ownership is explicit

r[valence_hyperion_integration.hyperion_block_loader.scope.owned]
- GIVEN Hyperion block-loader modularity work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local
- AND it does not claim Valence adoption or compatibility evidence.

### Requirement: Hyperion block loader core

r[valence_hyperion_integration.hyperion_block_loader.core] Hyperion block loading SHOULD expose pure cores for parsing summaries, validation decisions, palette or section plans, and storage update plans.

#### Scenario: Block-loader decision is testable without runtime IO

r[valence_hyperion_integration.hyperion_block_loader.core.testable]
- GIVEN block-loader input summaries
- WHEN the Hyperion block-loader core processes them
- THEN the result can be tested without file IO, decompression, ECS mutation, tracing, or runtime scheduling.

### Requirement: Hyperion block loader shell boundary

r[valence_hyperion_integration.hyperion_block_loader.shell_boundary] Hyperion block-loader extraction MUST keep file/resource reads, decompression, storage mutation, ECS mutation, tracing, and runtime scheduling outside pure block-loader cores.

#### Scenario: Block-loader side effects remain in shell

r[valence_hyperion_integration.hyperion_block_loader.shell_boundary.effects]
- GIVEN the block-loader core returns a parse or update plan
- WHEN the Hyperion shell applies that plan
- THEN only the shell reads resources, decompresses data, mutates storage, mutates ECS, records traces, or wires schedules.

### Requirement: Hyperion block loader parity

r[valence_hyperion_integration.hyperion_block_loader.parity] Hyperion block-loader modularization MUST preserve Hyperion block-loader APIs, world/block behavior, performance-sensitive boundaries, and non-claims.

#### Scenario: Hyperion block-loader behavior remains stable

r[valence_hyperion_integration.hyperion_block_loader.parity.stable]
- GIVEN a supported pre-refactor Hyperion block-loader input
- WHEN the modularized loader processes the same input
- THEN block state, storage-facing output, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion block loader tests

r[valence_hyperion_integration.hyperion_block_loader.tests] The change MUST include positive and negative tests for valid sections, malformed sections, palette edge cases, missing resources, invalid block ids, and rejected update plans.

#### Scenario: Block-loader fixtures cover success and failure

r[valence_hyperion_integration.hyperion_block_loader.tests.coverage]
- GIVEN representative valid and invalid Hyperion block-loader inputs
- WHEN block-loader tests run
- THEN they prove supported inputs pass and malformed inputs fail closed.

### Requirement: Hyperion block loader validation

r[valence_hyperion_integration.hyperion_block_loader.validation] The change MUST record focused Hyperion tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_block_loader.validation.logs]
- GIVEN Hyperion block-loader modularization is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tests plus Cairn gates passing.
