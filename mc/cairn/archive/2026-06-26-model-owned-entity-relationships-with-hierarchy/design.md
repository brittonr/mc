# Design: Model owned entity relationships with Bevy hierarchy

## Context

Bevy hierarchy is useful when entities form real parent/child structures or need tree traversal. Valence also has many relationships that are not trees: protocol ID indexes, layer membership, team maps, recipient routing, and external identity. This change is an inventory-driven adoption pass, not a blanket conversion.

## Decisions

### 1. Classify relationship semantics first

**Choice:** Each candidate records owner, child lifecycle, traversal needs, cleanup behavior, and whether the relation is a tree, graph, index, or external identity.

**Rationale:** Hierarchy is only appropriate for tree-like ownership or traversal.

### 2. Use hierarchy where ownership is real

**Choice:** Adopt `Parent`/`Children` only for relationships with a clear owner, child lifecycle, and tree traversal need.

**Rationale:** Bevy hierarchy should make ownership clearer, not hide arbitrary references.

### 3. Use explicit relationship components otherwise

**Choice:** Non-tree relationships may use typed components or resources with documented cleanup instead of hierarchy.

**Rationale:** Team/layer/index relationships are often not parent-child ownership.

### 4. Cleanup remains explicit where needed

**Choice:** Hierarchy adoption must document whether cleanup uses explicit systems, recursive despawn helpers, or component lifecycle.

**Rationale:** Do not assume hierarchy alone provides all desired cleanup semantics.

## Risks / Trade-offs

- Hierarchy can be overused for relationships that are really indexes or protocol IDs.
- Recursive cleanup can delete too much if ownership is misclassified; negative tests are required.
- Adding hierarchy dependencies or plugins can affect schedule shape and requires schedule evidence.
