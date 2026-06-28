# mc-compatibility Change Spec: Stevenarella login core

## Requirements

### Requirement: Stevenarella login outcome model

r[mc_compatibility.stevenarella_login.login_outcome_model] Stevenarella login handling SHOULD normalize supported login packet variants into explicit pure login events, decisions, and outcomes before constructing the live `Server` session.

#### Scenario: Login success variants normalize

r[mc_compatibility.stevenarella_login.login_outcome_model.normalize_success]
- GIVEN a supported login success packet variant is observed during login
- WHEN the login core processes the packet summary
- THEN it returns a normalized login outcome with username, UUID, and optional property facts
- AND downstream shell code does not need to duplicate packet-variant-specific success handling.

### Requirement: Stevenarella login shell boundary

r[mc_compatibility.stevenarella_login.login_shell_boundary] Stevenarella login extraction MUST keep network I/O, packet writes, encryption side effects, Mojang session joins, random secret generation, reader spawning, and live `Server` construction outside the pure login core.

#### Scenario: Side effects remain in shell

r[mc_compatibility.stevenarella_login.login_shell_boundary.effects]
- GIVEN the login core indicates compression, encryption, disconnect, or login success
- WHEN the Stevenarella connection shell applies that decision
- THEN only the shell mutates the connection, performs cryptographic/session side effects, spawns readers, or constructs the live server object
- AND the core remains testable with in-memory inputs.

### Requirement: Stevenarella login parity

r[mc_compatibility.stevenarella_login.login_parity] Stevenarella login-core extraction MUST preserve online and offline login behavior, compression propagation, encryption behavior, disconnect handling, wrong-packet diagnostics, milestone output, and protocol-version behavior.

#### Scenario: Existing login semantics remain stable

r[mc_compatibility.stevenarella_login.login_parity.stable]
- GIVEN a supported pre-refactor login path
- WHEN the extracted login core and shell process the same packet sequence
- THEN the resulting connection state, login outcome, milestone output, and error behavior remain equivalent
- AND the refactor does not claim new protocol support.

### Requirement: Stevenarella login positive tests

r[mc_compatibility.stevenarella_login.login_positive_tests] The change MUST include positive tests for offline login success variants, encrypted login decisions, compression-before-success handling, and normalized UUID/property outcomes.

#### Scenario: Supported login paths pass

r[mc_compatibility.stevenarella_login.login_positive_tests.coverage]
- GIVEN representative supported login packet sequences
- WHEN the login core processes them
- THEN tests prove the expected decisions and normalized outcomes are produced.

### Requirement: Stevenarella login negative tests

r[mc_compatibility.stevenarella_login.login_negative_tests] The change MUST include negative tests for disconnects, wrong packets, malformed login outcomes, unsupported FML network versions, and incomplete encryption negotiation.

#### Scenario: Invalid login paths fail closed

r[mc_compatibility.stevenarella_login.login_negative_tests.fail_closed]
- GIVEN invalid or unsupported login inputs
- WHEN the login core or shell processes them
- THEN tests prove the login path returns the existing diagnostic or containment behavior without constructing a corrupt live server session.

### Requirement: Stevenarella login validation

r[mc_compatibility.stevenarella_login.login_validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_login.login_validation.logs]
- GIVEN login-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative login tests plus affected dry-runs and Cairn gates passing.
