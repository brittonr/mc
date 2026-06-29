# mc-compatibility Change Spec: Stevenarella UI widgets

## Requirements

### Requirement: Stevenarella UI widget boundaries

r[mc_compatibility.stevenarella_ui.widget_boundaries] Stevenarella UI code SHOULD expose cohesive boundaries for layout regions, containers, image and batch elements, text and formatted text, buttons, text boxes, and input or focus helpers.

#### Scenario: UI responsibility has one owner

r[mc_compatibility.stevenarella_ui.widget_boundaries.ownership]
- GIVEN a UI widget responsibility is reviewed
- WHEN maintainers inspect UI modules
- THEN the responsibility is owned by a focused module
- AND unrelated layout, rendering, text, button, textbox, and input concerns are not reintroduced into one module.

### Requirement: Stevenarella UI widget core

r[mc_compatibility.stevenarella_ui.widget_core] UI layout, attachment, focus, text-formatting, button-state, and textbox-edit decisions SHOULD be pure over explicit inputs.

#### Scenario: Widget decision is testable without renderer

r[mc_compatibility.stevenarella_ui.widget_core.testable]
- GIVEN widget state, layout bounds, text, or input summaries
- WHEN the widget core processes them
- THEN the decision can be tested without renderer, clipboard, resource, or window side effects.

### Requirement: Stevenarella UI parity

r[mc_compatibility.stevenarella_ui.parity] UI modularization MUST preserve public UI builders, widget behavior, text formatting, input semantics, renderer boundaries, and evidence non-claims.

#### Scenario: UI behavior remains stable

r[mc_compatibility.stevenarella_ui.parity.stable]
- GIVEN a supported pre-refactor UI input or builder use
- WHEN the modularized UI code processes the same input
- THEN layout, text, widget state, and input behavior remain equivalent.

### Requirement: Stevenarella UI positive tests

r[mc_compatibility.stevenarella_ui.positive_tests] The change MUST include positive tests for layout regions, attachment calculations, formatted text, button state, textbox editing, focus changes, and container traversal.

#### Scenario: Supported UI paths pass

r[mc_compatibility.stevenarella_ui.positive_tests.coverage]
- GIVEN representative supported UI inputs
- WHEN extracted UI cores process them
- THEN tests prove the expected layout, text, focus, or widget decisions are produced.

### Requirement: Stevenarella UI negative tests

r[mc_compatibility.stevenarella_ui.negative_tests] The change MUST include negative tests for invalid layout bounds, empty text, malformed formatting, disabled widgets, invalid cursor positions, focus loss, and unsupported clipboard paths.

#### Scenario: Invalid UI paths fail closed

r[mc_compatibility.stevenarella_ui.negative_tests.fail_closed]
- GIVEN invalid UI inputs
- WHEN extracted UI cores process them
- THEN tests prove the inputs are rejected, ignored, clamped, or contained according to current behavior.

### Requirement: Stevenarella UI validation

r[mc_compatibility.stevenarella_ui.validation] The change MUST record focused Stevenarella UI tests, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_ui.validation.logs]
- GIVEN UI modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative UI tests plus Cairn gates passing.
