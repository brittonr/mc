## ADDED Requirements

### Requirement: Furnace smelting behavior card

r[vanilla_composable_plugins.furnace_smelting_card] Furnace smelting follow-on work MUST start from a dedicated behavior card that records source pages, target Java/protocol scope, bounded selected-row claim, non-claims, pure rule-core inputs and outputs, thin Bevy/ECS shell boundaries, positive tests, negative tests, evidence requirements, and stop conditions.

#### Scenario: Furnace card bounds the first slice

r[vanilla_composable_plugins.furnace_smelting_card.bounded]
- GIVEN furnace smelting is selected as the first bounded survival/plugin seam
- WHEN reviewers inspect the behavior card
- THEN it names Java Edition 1.20.1 / protocol 763 scope, selected Minecraft Wiki source pages, target-version recipe/fuel data requirements, selected-row behavior, and future Paper/vanilla parity evidence
- AND it explicitly rejects broad vanilla parity, all-recipe breadth, all block entities, hoppers, XP, smoker/blast-furnace breadth, DefaultPlugins membership changes, public-server safety, and production readiness.

#### Scenario: Furnace card keeps the core testable

r[vanilla_composable_plugins.furnace_smelting_card.core_shell]
- GIVEN a future furnace smelting implementation uses the card
- WHEN its design is reviewed
- THEN furnace semantics are assigned to a pure deterministic rule core over in-memory furnace state, recipe tables, fuel tables, and named constants
- AND Bevy/ECS queries, world mutation, packet/event emission, schedule registration, file reads, data-pack parsing, network access, and logging remain outside the rule core.

### Requirement: Furnace smelting behavior card validation

r[vanilla_composable_plugins.furnace_smelting_card.validation] The furnace smelting card work MUST include focused validation with positive and negative self-tests for required card sections and workflow rules.

#### Scenario: Complete furnace card passes

r[vanilla_composable_plugins.furnace_smelting_card.validation.positive]
- GIVEN the furnace smelting behavior card contains required source, target scope, bounded claim, non-claim, pure core, ECS shell, test, evidence, and stop-condition sections
- WHEN the focused behavior-card validation runs
- THEN it passes with a deterministic success result.

#### Scenario: Incomplete furnace card fails clearly

r[vanilla_composable_plugins.furnace_smelting_card.validation.negative]
- GIVEN a fixture behavior card is missing source scope, target Java/protocol scope, bounded claim, non-claims, pure core boundaries, ECS shell boundaries, positive tests, negative tests, evidence requirements, or stop conditions
- WHEN the focused behavior-card validation runs
- THEN it fails with a diagnostic naming the missing rule.

### Requirement: Furnace smelting behavior card closeout

r[vanilla_composable_plugins.furnace_smelting_card.closeout] Furnace smelting behavior-card work MUST record focused validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, evidence-manifest checks, accepted-spec sync verification, and archive receipts before closeout.

#### Scenario: Furnace card closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_card.closeout.log]
- GIVEN the furnace smelting behavior-card change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused positive/negative validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, evidence-manifest freshness, accepted spec requirement IDs, and archive receipts
- AND the evidence preserves non-claims for Valence implementation, default plugin membership, broad Minecraft compatibility, broad vanilla parity, all recipes, all block entities, public-server safety, and production readiness.
