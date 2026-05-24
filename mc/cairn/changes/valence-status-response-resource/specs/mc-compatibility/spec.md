# Mc Compatibility Delta: Valence status response resource

## Requirements

### Requirement: Status ping response data

r[mc_compatibility.valence_status_response.resource_owned] Valence status ping response data MUST be configurable through a resource or equivalent public server setting.

#### Scenario: Configured status response is used

r[mc_compatibility.valence_status_response.resource_owned.scenario]

- GIVEN an example inserts configured status response data

- WHEN a client sends a status ping

- THEN the response uses the configured description/version/sample fields

### Requirement: The status response resource

r[mc_compatibility.valence_status_response.defaults_stable] The status response resource MUST preserve existing default behavior for examples that do not configure it.

#### Scenario: Default status response remains available

r[mc_compatibility.valence_status_response.defaults_stable.scenario]

- GIVEN an example does not configure custom status data

- WHEN a client sends a status ping

- THEN the response remains valid and compatible with the prior default behavior

### Requirement: Status response behavior

r[mc_compatibility.valence_status_response.test_oracle] Status response behavior MUST be testable as a deterministic compatibility oracle.

#### Scenario: Status probe asserts configured data

r[mc_compatibility.valence_status_response.test_oracle.scenario]

- GIVEN a test or smoke probe sets known status data

- WHEN the status-only probe runs

- THEN the receipt or assertion records the configured values that were observed
