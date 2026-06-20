# Tasks

- [x] [serial] Define the structured receipt-test contract, evidence-critical field list, backward-compatibility boundary, and substring-test exception policy. r[mc_compatibility.receipt_schema_tests.contract]
  Evidence: docs/evidence/receipt-schema-hardening-2026-06-20.run.log; docs/evidence/receipt-schema-hardening-2026-06-20.b3
- [x] [depends:contract] Add pure receipt summary parsing/validation helpers with positive fixtures for representative dry-run, typed-event, MCP, and paired-reference receipt shapes. r[mc_compatibility.receipt_schema_tests.parser]
  Evidence: docs/evidence/receipt-schema-hardening-2026-06-20.run.log; docs/evidence/receipt-schema-hardening-2026-06-20.b3
- [x] [depends:parser] Add negative fixtures for missing nonclaims, stale or dirty child revisions, missing typed events, wrong backend, malformed artifact paths, duplicate or wrong-typed fields, and broad overclaim keys. r[mc_compatibility.receipt_schema_tests.negative]
  Evidence: docs/evidence/receipt-schema-hardening-2026-06-20.run.log; docs/evidence/receipt-schema-hardening-2026-06-20.b3
- [x] [depends:negative] Replace substring-only receipt assertions in runner tests with structured assertions where receipt JSON is the contract. r[mc_compatibility.receipt_schema_tests.migration]
  Evidence: docs/evidence/receipt-schema-hardening-2026-06-20.run.log; docs/evidence/receipt-schema-hardening-2026-06-20.b3
- [x] [depends:migration] Update checker/docs wording to describe receipt schema validation and retained CLI text substring checks. r[mc_compatibility.receipt_schema_tests.docs]
  Evidence: docs/evidence/receipt-schema-hardening-2026-06-20.run.log; docs/evidence/receipt-schema-hardening-2026-06-20.b3
- [x] [depends:docs] Run runner tests, affected receipt checkers, affected dry-run checks, evidence manifest checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.receipt_schema_tests.validation]
  Evidence: docs/evidence/receipt-schema-hardening-2026-06-20.run.log; docs/evidence/receipt-schema-hardening-evidence-manifest-2026-06-20.run.log; docs/evidence/receipt-schema-hardening-2026-06-20.b3; docs/evidence/receipt-schema-hardening-evidence-manifest-2026-06-20.b3
