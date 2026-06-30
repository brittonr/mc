# valence-hyperion-integration Change Spec: Bridge slice prototype

## Requirements

### Requirement: Bridge slice scope

r[valence_hyperion_integration.bridge_slice.scope] Bridge slice work MUST confirm roadmap and type-ownership prerequisites, record current Valence and Hyperion source revisions, and inventory reusable archived evidence before implementation changes owner code.

#### Scenario: Prototype scope is bounded

r[valence_hyperion_integration.bridge_slice.scope.bounded]
- GIVEN bridge slice work is selected
- WHEN reviewers inspect the scope notes
- THEN roadmap prerequisites, type-ownership decisions, current source revisions, reusable archived evidence, stale evidence, affected crates, optional scope, and non-claims are recorded.

### Requirement: Optional bridge shell

r[valence_hyperion_integration.bridge_slice.optional_shell] Bridge slice work MUST be implemented as an optional plugin, example, or fixture harness with default-disabled behavior, named configuration values, and adapter DTO visibility boundaries.

#### Scenario: Disabled bridge preserves direct behavior

r[valence_hyperion_integration.bridge_slice.optional_shell.disabled]
- GIVEN the bridge plugin, example, or harness is not enabled
- WHEN existing Valence direct-mode networking, packet, chunk, movement, and chat checks run
- THEN their behavior remains compatible with the pre-bridge direct path.

### Requirement: Join and initial chunk plan

r[valence_hyperion_integration.bridge_slice.join_chunk_plan] Bridge slice work MUST model player join and initial chunk delivery as pure deterministic planning over explicit player, session, registry, dimension, layer, chunk/view, packet-order, and diagnostic summaries.

#### Scenario: Valid join plan is deterministic

r[valence_hyperion_integration.bridge_slice.join_chunk_plan.valid]
- GIVEN valid player, session, registry, dimension, layer, chunk/view, and packet-order summaries
- WHEN the join/chunk planning core runs
- THEN it returns a deterministic packet or intent plan with documented ordering and diagnostics.

#### Scenario: Invalid join plan fails closed

r[valence_hyperion_integration.bridge_slice.join_chunk_plan.rejects]
- GIVEN missing player state, stale session mapping, invalid registry, incompatible dimension bounds, missing chunk data, or unsupported packet ordering
- WHEN the join/chunk planning core runs
- THEN it returns a deterministic rejection
- AND no packet send, layer mutation, proxy delivery, or compatibility claim is emitted for that input.

### Requirement: Movement state mapping

r[valence_hyperion_integration.bridge_slice.movement_mapping] Bridge slice work MUST model movement-state mapping as pure deterministic conversion over explicit player/session/entity identifiers, position, rotation, velocity, on-ground, dimension, and stale-state summaries.

#### Scenario: Movement update maps one entity

r[valence_hyperion_integration.bridge_slice.movement_mapping.valid]
- GIVEN a valid movement update for a known player/session/entity in a known dimension
- WHEN the movement mapper runs
- THEN it returns exactly the approved movement intent or state update for that entity.

#### Scenario: Stale movement is rejected

r[valence_hyperion_integration.bridge_slice.movement_mapping.rejects]
- GIVEN a movement update references an unknown session, stale entity, invalid coordinate, unsupported dimension, malformed rotation, or lossy conversion
- WHEN the movement mapper runs
- THEN it rejects the update deterministically
- AND no entity movement, packet echo, or bridge state mutation is applied.

### Requirement: Chat and broadcast route planning

r[valence_hyperion_integration.bridge_slice.chat_broadcast] Bridge slice work MUST model chat and broadcast routing as pure deterministic planning over explicit sender, recipient, channel, local-visibility, exclusion, permission, and malformed-route summaries.

#### Scenario: Authorized chat route is contained

r[valence_hyperion_integration.bridge_slice.chat_broadcast.valid]
- GIVEN an authorized sender, valid route, known recipients, configured local visibility, and explicit exclusions
- WHEN chat or broadcast planning runs
- THEN only eligible recipients are returned in deterministic order with documented route diagnostics.

#### Scenario: Unauthorized route fails closed

r[valence_hyperion_integration.bridge_slice.chat_broadcast.rejects]
- GIVEN an unauthorized sender, malformed route, unknown channel, stale recipient, invalid exclusion, or closed client
- WHEN chat or broadcast planning runs
- THEN no unauthorized recipient is included
- AND the diagnostic does not leak hidden membership, permission, or routing state beyond documented feedback.

### Requirement: Bridge shell integration

r[valence_hyperion_integration.bridge_slice.shell] Bridge slice shell code MUST gather owner-specific Valence or adapter state, call pure bridge cores, apply only approved mutations or sends, and keep runtime side effects outside pure cores.

#### Scenario: Shell applies only approved bridge intents

r[valence_hyperion_integration.bridge_slice.shell.applies]
- GIVEN a bridge core returns approved join, chunk, movement, chat, or broadcast intents
- WHEN shell systems or harness code run
- THEN only the returned owner-specific mutations, packet sends, diagnostics, or evidence records are applied
- AND sockets, ECS mutation, packet writes, logging, clocks, and scheduling remain shell responsibilities.

### Requirement: Bridge fixture coverage

r[valence_hyperion_integration.bridge_slice.tests] Bridge slice work MUST include positive tests for valid join, chunk, movement, chat, broadcast, and plugin-disabled behavior plus negative tests for missing facts, invalid dimensions, stale sessions, malformed movement, unauthorized routes, invalid targets, closed clients, lossy conversions, and disabled behavior.

#### Scenario: Bridge tests cover success and failure

r[valence_hyperion_integration.bridge_slice.tests.coverage]
- GIVEN representative valid and invalid bridge inputs
- WHEN bridge core and shell tests run
- THEN supported bridge plans pass and malformed, stale, unauthorized, lossy, closed, or disabled cases fail closed without default behavior changes.

### Requirement: Bridge slice validation

r[valence_hyperion_integration.bridge_slice.validation] Bridge slice work MUST record baseline direct-mode Valence checks before implementation, focused bridge-core tests, shell/plugin or harness tests, selected packet/chunk/chat dry runs, optional smoke evidence, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Bridge closeout is reviewable

r[valence_hyperion_integration.bridge_slice.validation.log]
- GIVEN bridge slice work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline direct-mode checks, positive bridge tests, negative fail-closed tests, shell or harness checks, selected dry runs, optional smoke evidence if claimed, Cairn gates, Cairn validation, task-evidence validation, evidence manifests, and explicit non-claims for repository merge, runtime replacement, default behavior, Hyperion compatibility, production scale, Bedwars behavior, and vanilla parity.
