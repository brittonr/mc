# mc-compatibility Change Spec: Runner config patches

## Requirements

### Requirement: Runner config patch model

r[mc_compatibility.runner_modularity.config_patch_model] The mc-compat runner SHOULD represent partial configuration updates as explicit pure patch data before applying them to a resolved configuration.

#### Scenario: Config source produces a patch

r[mc_compatibility.runner_modularity.config_patch_model.patch]
- GIVEN a supported config source such as defaults, Nickel-exported JSON, restricted Steel config, environment variables, or CLI arguments
- WHEN the runner parses that source
- THEN the parsed result is available as a deterministic config patch or a source-specific diagnostic
- AND parsing the source does not require mutating the final resolved configuration.

### Requirement: Runner config source precedence

r[mc_compatibility.runner_modularity.config_source_precedence] The runner MUST preserve existing config precedence while making the ordered source list explicit and reviewable.

#### Scenario: Later sources override earlier sources

r[mc_compatibility.runner_modularity.config_source_precedence.ordered]
- GIVEN multiple supported config sources set the same configurable value
- WHEN patches are composed in the documented runner source order
- THEN the resolved configuration matches the pre-refactor precedence behavior
- AND the source order can be inspected from the config resolution core.

### Requirement: Runner config validation

r[mc_compatibility.runner_modularity.config_validation] The runner MUST validate cross-field safety and mode constraints after config patches are resolved and before side-effecting execution begins.

#### Scenario: Unsafe resolved config fails before execution

r[mc_compatibility.runner_modularity.config_validation.fail_closed]
- GIVEN config patches resolve to an unsafe path, invalid timeout, missing required value, invalid backend, invalid scenario, or unsupported mode combination
- WHEN config validation runs
- THEN the runner rejects the resolved configuration with an actionable diagnostic
- AND no server, client, receipt, cleanup, or artifact side effect is started.

### Requirement: Runner config positive tests

r[mc_compatibility.runner_modularity.config_positive_tests] The change MUST include positive tests for representative config defaults, file/env/CLI precedence, Steel and Nickel config inputs, mode selection, receipt paths, backend selection, and scenario selection.

#### Scenario: Supported config paths resolve

r[mc_compatibility.runner_modularity.config_positive_tests.coverage]
- GIVEN representative supported config inputs
- WHEN the config patch core resolves them
- THEN tests prove the resulting configuration matches the current runner behavior for those inputs.

### Requirement: Runner config negative tests

r[mc_compatibility.runner_modularity.config_negative_tests] The change MUST include negative tests for unknown flags, missing option values, invalid backend or scenario values, invalid timeouts, unsafe output paths, and conflicting mode or source combinations.

#### Scenario: Invalid config inputs fail closed

r[mc_compatibility.runner_modularity.config_negative_tests.fail_closed]
- GIVEN malformed or unsafe config inputs
- WHEN the config patch core parses, composes, or validates them
- THEN tests prove the inputs are rejected with the expected diagnostic before side effects.

### Requirement: Runner config patch validation evidence

r[mc_compatibility.runner_modularity.config_validation_evidence] The change MUST record focused config tests, dry-run smoke checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.config_validation_evidence.logs]
- GIVEN the config patch refactor is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative config fixtures plus Cairn gates and validation passing.
