# Delta: Broad protocol-763 coverage

## Requirements

### Requirement: Protocol inventory completeness

r[mc_compatibility.protocol_763_broad_coverage.inventory] Broad protocol-763 coverage MUST be based on a refreshed complete Valence packet inventory and Stevenarella mapping inventory.

#### Scenario: Inventory is complete

r[mc_compatibility.protocol_763_broad_coverage.inventory.complete]
- GIVEN broad protocol-763 coverage is evaluated
- WHEN the inventory checker runs
- THEN every Valence protocol-763 packet row is present
- AND every row has owner, mapping status, parser-shape status, and next action.

### Requirement: Mapping review

r[mc_compatibility.protocol_763_broad_coverage.mapping_review] Promoted protocol packet rows MUST use reviewed mappings and MUST NOT rely on fallback aliases.

#### Scenario: Fallback alias is rejected

r[mc_compatibility.protocol_763_broad_coverage.mapping_review.rejects]
- GIVEN a packet row uses an inherited fallback alias
- WHEN the row is promoted as covered
- THEN the checker fails and names the packet row.

### Requirement: Parser fixtures

r[mc_compatibility.protocol_763_broad_coverage.parser_fixtures] Promoted packet families MUST have parser-shape evidence matching the parser implementation class: structured parsers require positive and negative fixtures; byte-opaque raw consumers require positive byte-preservation fixtures plus explicit semantic non-claim rationale.

#### Scenario: Structured malformed fixture is rejected

r[mc_compatibility.protocol_763_broad_coverage.parser_fixtures.negative]
- GIVEN a malformed packet fixture for a promoted structured packet parser
- WHEN parser-shape tests run
- THEN the malformed packet is rejected without panic or silent acceptance.

#### Scenario: Raw consumer fixture is scoped

r[mc_compatibility.protocol_763_broad_coverage.parser_fixtures.raw_scope]
- GIVEN a promoted packet family is implemented as a byte-opaque raw consumer
- WHEN evidence is reviewed
- THEN a positive byte-preservation fixture is linked
- AND semantic malformed-payload rejection remains an explicit non-claim.

### Requirement: Packet family receipts

r[mc_compatibility.protocol_763_broad_coverage.receipts] Broad protocol coverage MUST include reviewable receipts or deterministic fixture evidence for promoted packet families.

#### Scenario: Receipt backs promoted family

r[mc_compatibility.protocol_763_broad_coverage.receipts.backing]
- GIVEN a packet family is marked covered
- WHEN evidence is reviewed
- THEN a receipt, fixture result, or BLAKE3-backed log is linked for that family.

### Requirement: Broad protocol non-claims

r[mc_compatibility.protocol_763_broad_coverage.nonclaims] Broad protocol-763 packet coverage MUST NOT claim full Minecraft gameplay compatibility.

#### Scenario: Full Minecraft remains separate

r[mc_compatibility.protocol_763_broad_coverage.nonclaims.separate]
- GIVEN broad protocol packet coverage is promoted
- WHEN the evidence doc is reviewed
- THEN full Minecraft compatibility, gameplay correctness, and production readiness remain separate non-claims.
