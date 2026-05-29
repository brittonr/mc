# Design: Evidence promotion automation

## Context

The workspace already has many evidence checkers, but producing a complete promoted evidence bundle still requires careful manual steps. Parent `.gitignore` also ignores most of `mc/`, so Nix validation can miss untracked artifacts unless files are added deliberately.

## Decisions

### 1. Rust promotion tool

**Choice:** Implement promotion automation as a repo-owned Rust tool or runner subcommand, not a new Bash/Python script.

**Rationale:** Repo guidance prefers Rust or Steel for new checks/scripts. Rust gives typed artifact plans and safer filesystem mutation.

### 2. Dry-run plan before apply

**Choice:** The tool first emits a promotion plan listing source artifacts, destination paths, BLAKE3 hashes, matrix/bundle edits, non-claim checks, and validation commands. Apply mode mutates only after the plan passes.

**Rationale:** Evidence promotion is review-critical and should be auditable before filesystem writes.

### 3. Functional core, thin shell

**Choice:** A pure core computes artifact plans, matrix updates, bundle updates, and diagnostics from in-memory inputs. The CLI shell performs reads, writes, and command execution.

**Rationale:** Core behavior can be tested with positive and negative fixtures without running live rails.

### 4. Non-claim preservation gate

**Choice:** The tool refuses matrix/current-bundle edits that remove required non-claim phrases unless the rail declares a matching promoted row and checker evidence.

**Rationale:** Prior review failures came from overclaiming or stale evidence; promotion tooling should block that class.

## Risks / Trade-offs

- Automated matrix editing can be brittle with markdown tables; start with constrained table rows and strong fixtures.
- The tool should not hide human/oracle checkpoints; it should require them when receipt schemas cannot machine-record needed facts.
- Running live rails remains expensive and should stay separate from planning/apply mechanics.
