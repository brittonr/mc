# valence-hyperion-integration Change Spec: Hyperion bot packet core

## Requirements

### Requirement: Hyperion bot packet scope

r[valence_hyperion_integration.hyperion_bot_packet.scope] Hyperion bot packet utility work MUST be scoped as Hyperion tool-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion bot ownership is explicit

r[valence_hyperion_integration.hyperion_bot_packet.scope.owned]
- GIVEN Hyperion bot packet work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local tool work
- AND it does not claim Valence adoption, public-server safety, or compatibility evidence.

### Requirement: Hyperion bot packet core

r[valence_hyperion_integration.hyperion_bot_packet.core] Hyperion bot packet utilities SHOULD expose pure cores for packet construction, packet classification, byte-shape validation, and protocol-assumption checks.

#### Scenario: Bot packet decision is testable without network

r[valence_hyperion_integration.hyperion_bot_packet.core.testable]
- GIVEN bot packet input summaries
- WHEN the Hyperion bot packet core processes them
- THEN the result can be tested without socket IO, connection state, async runtime, timing, or logging.

### Requirement: Hyperion bot packet shell boundary

r[valence_hyperion_integration.hyperion_bot_packet.shell_boundary] Hyperion bot packet extraction MUST keep socket IO, connection state, async tasks, sleeps/timing, logging, and bot orchestration outside pure packet cores.

#### Scenario: Bot packet side effects remain in shell

r[valence_hyperion_integration.hyperion_bot_packet.shell_boundary.effects]
- GIVEN the bot packet core returns a packet or classification decision
- WHEN the Hyperion bot shell applies that decision
- THEN only the shell performs IO, mutates connection state, schedules async work, sleeps, or logs diagnostics.

### Requirement: Hyperion bot packet parity

r[valence_hyperion_integration.hyperion_bot_packet.parity] Hyperion bot packet extraction MUST preserve bot tool CLI/API behavior, packet bytes, protocol assumptions, and non-claims.

#### Scenario: Hyperion bot packet behavior remains stable

r[valence_hyperion_integration.hyperion_bot_packet.parity.stable]
- GIVEN a supported pre-refactor bot packet input
- WHEN the extracted packet core and shell process the same input
- THEN packet bytes, classification, public tool behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion bot packet tests

r[valence_hyperion_integration.hyperion_bot_packet.tests] The change MUST include positive and negative tests for valid packet construction, packet classification, malformed packet bytes, unsupported protocol assumptions, closed connections, and missing bot state.

#### Scenario: Bot packet fixtures cover success and failure

r[valence_hyperion_integration.hyperion_bot_packet.tests.coverage]
- GIVEN representative valid and invalid bot packet inputs
- WHEN bot packet tests run
- THEN they prove supported packet paths pass and malformed paths fail closed.

### Requirement: Hyperion bot packet validation

r[valence_hyperion_integration.hyperion_bot_packet.validation] The change MUST record focused Hyperion bot tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_bot_packet.validation.logs]
- GIVEN Hyperion bot packet extraction is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local bot tests plus Cairn gates passing.
