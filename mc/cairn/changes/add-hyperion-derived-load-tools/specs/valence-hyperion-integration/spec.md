# valence-hyperion-integration Change Spec: Hyperion-derived load and packet tools

## Requirements

### Requirement: Tool inventory

r[valence_hyperion_integration.tools.inventory] The integration MUST inventory Hyperion tools and Valence tools before adapting load or packet diagnostics.

#### Scenario: Tool concepts are classified

r[valence_hyperion_integration.tools.inventory.classified]
- GIVEN tool integration is selected
- WHEN reviewers inspect the inventory
- THEN each load-bot, packet-inspector, capture, and wrapper concept is classified as adopt, port, reference, or reject
- AND public API impact is marked as none unless separately justified.

### Requirement: Tool contract

r[valence_hyperion_integration.tools.contract] Load and packet tools MUST define typed configuration, safe target rules, output contracts, redaction policy, and non-claim boundaries.

#### Scenario: Unsafe target is rejected

r[valence_hyperion_integration.tools.contract.unsafe_target]
- GIVEN a tool config requests a network target outside the documented safe target policy
- WHEN the config validator runs
- THEN it rejects the config with a deterministic diagnostic
- AND no network connection is attempted.

### Requirement: Load bot tooling

r[valence_hyperion_integration.tools.load_bot] Valence MAY include load-bot tooling for loopback smoke and stress evidence, but the tool MUST report failures structurally and avoid compatibility overclaims.

#### Scenario: Load run failure is structured

r[valence_hyperion_integration.tools.load_bot.failure]
- GIVEN a load run cannot connect, times out, or receives an unexpected disconnect
- WHEN the load tool exits
- THEN it records the failing phase, target, configured scenario, and exit status
- AND it does not mark compatibility evidence as passing.

### Requirement: Packet inspector tooling

r[valence_hyperion_integration.tools.packet_inspector] Packet inspection tooling MUST bound capture output, handle malformed captures, and apply the documented redaction policy.

#### Scenario: Malformed capture fails closed

r[valence_hyperion_integration.tools.packet_inspector.malformed]
- GIVEN a capture contains malformed packet bytes
- WHEN the packet inspector decodes it
- THEN it reports the malformed boundary deterministically
- AND it does not emit unbounded or unredacted raw output.

### Requirement: Tool documentation

r[valence_hyperion_integration.tools.docs] Tool documentation SHOULD describe commands, configs, outputs, evidence usage, and non-claims.

#### Scenario: Tool docs separate evidence classes

r[valence_hyperion_integration.tools.docs.non_claims]
- GIVEN tool documentation is published
- WHEN reviewers inspect it
- THEN load, packet-diagnostic, compatibility, and vanilla-reference evidence are described as separate evidence classes.

### Requirement: Tool validation

r[valence_hyperion_integration.tools.validation] Tool work MUST record config tests, malformed capture fixtures, loopback smoke tests, selected dry runs, and Cairn gates before archive.

#### Scenario: Tool closeout is reviewable

r[valence_hyperion_integration.tools.validation.log]
- GIVEN tool integration is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show config positive/negative tests, malformed capture handling, loopback smoke tests, selected load dry runs, docs checks if present, and Cairn validation.
