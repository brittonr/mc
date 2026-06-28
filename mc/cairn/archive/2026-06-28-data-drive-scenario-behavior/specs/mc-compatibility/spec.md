# mc-compatibility Change Spec: Data-driven scenario behavior

## Requirements

### Requirement: Scenario behavior metadata

r[mc_compatibility.runner_modularity.scenario_metadata] Scenario behavior facts SHOULD be represented as explicit scenario metadata or generated scenario surfaces when the facts are deterministic and declarative.

#### Scenario: Scenario row carries behavior facts

r[mc_compatibility.runner_modularity.scenario_metadata.row]
- GIVEN a scenario has deterministic run strategy, env intent, evidence selector, typed-event edge, or non-claim facts
- WHEN that scenario is represented in the catalog
- THEN those facts are available from scenario metadata or generated scenario surfaces
- AND consumers do not need unrelated large match statements to recover those facts.

### Requirement: Scenario extension path

r[mc_compatibility.runner_modularity.scenario_extension_path] Adding a supported scenario SHOULD require updating a bounded metadata surface plus any explicitly named specialized handler, rather than editing multiple unrelated consumer match statements.

#### Scenario: New scenario has one auditable path

r[mc_compatibility.runner_modularity.scenario_extension_path.auditable]
- GIVEN a new scenario is added
- WHEN reviewers inspect its behavior definition
- THEN names, aliases, milestones, run strategy, env intents, typed-event graph behavior, evidence selectors, and non-claims are discoverable from the scenario metadata path
- AND any custom code hook is explicitly named.

### Requirement: Scenario metadata positive tests

r[mc_compatibility.runner_modularity.scenario_metadata_positive_tests] The change MUST include positive validation for representative single-client, reconnect, multi-client, projectile, inventory, survival, CTF, and MCP scenario metadata.

#### Scenario: Representative metadata validates

r[mc_compatibility.runner_modularity.scenario_metadata_positive_tests.coverage]
- GIVEN representative scenario rows from each behavior family
- WHEN scenario metadata validation runs
- THEN each row produces the expected run strategy, env intents, typed-event edges, evidence selectors, and non-claims.

### Requirement: Scenario metadata negative tests

r[mc_compatibility.runner_modularity.scenario_metadata_negative_tests] The change MUST include negative validation for missing required facts, unknown env intents, invalid graph edges, duplicate aliases, and unsupported handler references.

#### Scenario: Incomplete metadata fails closed

r[mc_compatibility.runner_modularity.scenario_metadata_negative_tests.fail_closed]
- GIVEN a malformed scenario metadata row
- WHEN scenario metadata validation runs
- THEN it rejects the row before runtime execution
- AND the diagnostic names the malformed field.

### Requirement: Scenario metadata validation

r[mc_compatibility.runner_modularity.scenario_metadata_validation] The change MUST record scenario-spec validation, generated-surface freshness checks, runner tests, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.scenario_metadata_validation.logs]
- GIVEN data-driven scenario behavior migration is complete
- WHEN the change is closed
- THEN reviewable logs show metadata parity, positive and negative fixtures, generated freshness checks, and Cairn validation passing.
