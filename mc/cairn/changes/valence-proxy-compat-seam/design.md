# Design: Valence proxy compatibility seam

## Context

Hyperion commits `1f28877` and `c691c9e` improve multi-proxy and mTLS. For Valence, the ROI is testing proxy compatibility paths before copying architecture.

## Decisions

### 1. Test proxy path before architecture changes

**Choice:** The fork should first make proxy compatibility observable with receipts.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 2. Do not port mTLS wholesale

**Choice:** Hyperion server-to-proxy mTLS is not an immediate Valence fork requirement until proxy acceptance evidence demands it.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 3. Separate direct and proxied claims

**Choice:** Receipts must distinguish direct Stevenarella-to-Valence evidence from evidence through Velocity/ViaVersion or another proxy.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

## Risks / Trade-offs

- Proxy fixtures can require external Java/proxy components and become slow or flaky.
- Forwarding-mode semantics can be security-sensitive; receipts must avoid credentials/secrets.
