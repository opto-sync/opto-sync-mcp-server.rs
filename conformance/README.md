# Conformance

Conformance in this repository is implementation-linked. A property is promotable only when the exact repository head executes a test or checker that exercises the implementation or a directly consumed contract.

## Current properties

1. The exposed Opto Sync MCP tool catalog is closed and namespaced.
2. Product tools are read-only, non-destructive, and idempotent according to their MCP annotations.
3. Tool output fails closed when it exceeds the fixed output bound.
4. Unknown resources/prompts are rejected instead of falling through to generic execution.
5. Runtime lifecycle exposure is observational; callers cannot trigger lifecycle transitions through the tool catalog.

The implementation-linked evidence for these claims lives in `src/server.rs` and its Rust tests. Promotion should execute `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo test --locked --all-targets` on the exact PR head, plus applicable contract/source-policy gates.

## Not current conformance claims

This repository does not currently claim a mutating tool surface, write-scope authorization, tenant-scoped mutations, or generic remote execution. If those capabilities are implemented later, conformance must be expanded alongside the implementation and include intentionally broken controls proving the gate detects missing auth, tenant mismatch, missing write scope, replay/idempotency failure, and unsafe generic execution.
