# Evidence directory partitions

Durable review evidence lives under `docs/evidence/`. New artifacts SHOULD use the smallest matching partition so reviewers can identify the artifact class from the path:

| Partition | Artifact class | Pattern |
| --- | --- | --- |
| Run logs | Command output with explicit `exit_status=0` when cited from Cairn tasks | `run-logs/<yyyy-mm-dd>/<change-or-scenario>.run.log` |
| Manifests | BLAKE3 manifests covering durable artifacts | `manifests/<yyyy-mm-dd>/<change-or-scenario>.b3` |
| Receipts | Machine receipts and structured run summaries | `receipts/<yyyy-mm-dd>/<scenario>.receipt.json` |
| Logs | Promoted client/server/typed-event sidecar logs | `logs/<yyyy-mm-dd>/<scenario>.<kind>.log` |
| Oracle notes | Human checkpoints and reviewer decisions | `oracles/<yyyy-mm-dd>/<change-or-scenario>.md` |
| Indexes | Machine-owned navigation surfaces | `indexes/` or root `*.generated.md` when existing checks expect root paths |
| Fixtures | Promoted source snapshots, jars, and fixture inputs | `fixtures/<yyyy-mm-dd>/<name>` |
| Archive-only artifacts | Historical artifacts kept only to preserve old citations | `archive/<yyyy-mm-dd>/<name>` |

Existing flat paths are citation-stable. Do not bulk-move existing artifacts unless the same change updates every Cairn task, accepted spec, evidence note, and BLAKE3 manifest that cites the old path. Flat run logs, receipts, and sidecar logs are inventoried as `migrate-later` until a focused migration can preserve those references.

The generated inventory at `evidence-inventory.generated.md` classifies every current durable artifact as stay-flat, migrate-now, migrate-later, generated index, manifest, or oracle note. The generated index at `evidence-index.generated.md` maps dates and inferred change/scenario keys to durable paths and covering manifests. Refresh both with:

```sh
tools/check_evidence_partitions.rs --write-generated
tools/check_evidence_partitions.rs --root .
```

This partitioning is navigation and durability evidence only; it does not broaden compatibility claims, semantic equivalence, public-server safety, production readiness, full CTF correctness, or full survival correctness.
