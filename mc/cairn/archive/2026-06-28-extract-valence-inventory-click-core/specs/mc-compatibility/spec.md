# mc-compatibility Change Spec: Valence inventory click core

## Requirements

### Requirement: Valence inventory click transaction core

r[mc_compatibility.valence_inventory.click_transaction_core] Valence inventory click handling SHOULD expose a pure transaction core that receives explicit inventory, cursor, open-window, click-mode, slot, and slot-change summaries and returns explicit transaction decisions.

#### Scenario: Inventory click decision is testable without Bevy

r[mc_compatibility.valence_inventory.click_transaction_core.testable]
- GIVEN an inventory click packet summary and explicit inventory state summaries
- WHEN the transaction core processes the input
- THEN it returns a decision such as apply transaction, drop cursor, resync invalid, emit click event, ignore, or reject
- AND Bevy queries or live client objects are not required to inspect the decision.

### Requirement: Valence inventory click shell boundary

r[mc_compatibility.valence_inventory.click_shell_boundary] Inventory click extraction MUST keep Bevy queries, packet resync sends, event writers, inventory mutation, cursor mutation, and drop-item event emission outside the pure transaction core.

#### Scenario: Inventory side effects remain in shell

r[mc_compatibility.valence_inventory.click_shell_boundary.effects]
- GIVEN the inventory click core returns a transaction decision
- WHEN the Valence inventory shell applies that decision
- THEN only the shell mutates ECS state, sends resync packets, writes events, or emits dropped items
- AND the core remains testable with in-memory inputs.

### Requirement: Valence inventory click parity

r[mc_compatibility.valence_inventory.click_parity] Inventory click-core extraction MUST preserve existing packet validation, invalid resync behavior, cursor and drop semantics, key/drop mode behavior, regular click flow behavior, emitted event shapes, and evidence non-claims.

#### Scenario: Existing inventory click behavior remains stable

r[mc_compatibility.valence_inventory.click_parity.stable]
- GIVEN a supported pre-refactor inventory click input
- WHEN the extracted transaction core and shell process the same input
- THEN the inventory mutation, cursor state, resync behavior, emitted events, and non-claim boundaries remain equivalent
- AND no new inventory semantic compatibility claim is promoted.

### Requirement: Valence inventory click positive tests

r[mc_compatibility.valence_inventory.click_positive_tests] The change MUST include positive tests for valid regular clicks, outside-window cursor drops, drop-key paths, open-inventory clicks, cursor updates, and emitted click-event plans.

#### Scenario: Supported inventory click paths pass

r[mc_compatibility.valence_inventory.click_positive_tests.coverage]
- GIVEN representative supported inventory click inputs
- WHEN the transaction core processes them
- THEN tests prove the expected transaction decisions, cursor outcomes, and event plans are produced.

### Requirement: Valence inventory click negative tests

r[mc_compatibility.valence_inventory.click_negative_tests] The change MUST include negative tests for invalid packets, unsafe slot indices, malformed slot changes, missing clients, missing open inventories, invalid cursor states, and resync plans.

#### Scenario: Invalid inventory click paths fail closed

r[mc_compatibility.valence_inventory.click_negative_tests.fail_closed]
- GIVEN malformed or unsupported inventory click inputs
- WHEN the transaction core or shell processes them
- THEN tests prove the inputs are rejected, ignored, or resynced according to current behavior without corrupting inventory state.

### Requirement: Valence inventory click validation

r[mc_compatibility.valence_inventory.click_validation] The change MUST record focused Valence inventory tests, affected mc-compat inventory dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_inventory.click_validation.logs]
- GIVEN inventory click-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative inventory click tests plus affected dry-runs and Cairn gates passing.
