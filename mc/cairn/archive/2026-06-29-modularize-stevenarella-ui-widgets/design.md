# Design: Stevenarella UI widget modules

## Context

The UI module owns several independently reusable widgets and layout primitives. Splitting by widget family should preserve public builders while making widget decisions testable without renderer state.

## Decisions

### 1. Split by widget family

**Choice:** Create modules for layout/regions, container traversal, image/batch elements, text/formatting, button behavior, textbox behavior, and input/focus helpers.

**Rationale:** Widget families have different state and test fixtures.

### 2. Extract pure layout and input cores

**Choice:** Attachment calculations, region resolution, text formatting, button state, textbox editing, and focus transitions should be pure over explicit inputs.

**Rationale:** Widget behavior can be tested without OpenGL or live window events.

### 3. Preserve public builders

**Choice:** Existing UI builder and element APIs remain available through re-exports or compatibility adapters.

**Rationale:** Screens should not need broad rewrites during modularization.
