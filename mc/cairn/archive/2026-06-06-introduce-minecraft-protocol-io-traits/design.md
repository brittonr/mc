# Design: Introduce Minecraft protocol I/O traits

## Context

The runner only needs a narrow Minecraft protocol helper surface for compatibility status/query and packet writing. The refactor should not become a general protocol crate or duplicate Valence protocol logic. Its goal is testability and explicit wire-format boundaries.

## Decisions

### 1. Use extension traits over `Read` and `Write`

**Choice:** Define `McRead` for VarInt reads and `McWrite` for VarInt, string, and packet writes, implemented for all compatible `Read`/`Write` types.

**Rationale:** `TcpStream` and in-memory cursors can use the same helper surface without coupling tests to sockets.

### 2. Keep encoding constants named and local

**Choice:** Name VarInt segment masks, continuation masks, shift width, and maximum byte counts as constants near the protocol helpers.

**Rationale:** Wire-format numbers are not self-evident and should remain reviewable.

### 3. Separate pure framing from network shell

**Choice:** Pure helper functions build packet bytes and parse VarInts from readers. Network code only connects, writes, reads, and maps I/O errors.

**Rationale:** Protocol correctness can be tested without standing up a server.

### 4. Preserve status-query behavior

**Choice:** Existing status-only and server-wait behavior must keep the same success/failure contract and diagnostics.

**Rationale:** The compatibility harness depends on stable status readiness checks.

## Risks / Trade-offs

- Extension traits can look globally available; keep them module-private unless another owned tool needs them.
- A narrow helper may still duplicate protocol crate behavior; this change is for runner-local status/query logic only.
- Error-message parity matters for existing tests and evidence logs.