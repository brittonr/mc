# Design: Stevenarella control core

## Context

Control commands bridge external MCP requests and internal game-state actions. The core should own command semantics while shells own transport and state mutation.

## Decisions

### 1. Make command semantics pure

**Choice:** Validation, normalization, capability classification, and response classification should be pure over explicit command inputs.

**Rationale:** MCP and game shell can test command behavior without live game state.

### 2. Keep side effects out

**Choice:** Transport parsing, game mutation, packet sends, capture queues, and logging remain in MCP/game shells.

**Rationale:** Control command data should not depend on runtime side effects.

### 3. Preserve schema stability

**Choice:** Existing serialized command/response shapes and names remain stable.

**Rationale:** Runner-controlled MCP checks depend on this vocabulary.
