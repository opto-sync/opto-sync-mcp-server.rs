# MCP contract authority

This repository implements the Opto Sync product MCP server. Contract claims must stay tied to the surface that actually exists.

## Current authorities

- `src/server.rs` is the implementation authority for the currently exposed product tool catalog and its MCP annotations.
- `contracts/mcp-fleet.json` records the reviewed fleet/security authority mapping for the server class.
- `ORESoftware/ores-interfaces` and `ORESoftware/api-docs` remain shared interface/RPC authorities where this service consumes those contracts.
- Human-authored TypeSpec and JSON Schema Draft 2020-12 are independent peer authorities for cross-runtime wire contracts. `ORESoftware/typespec-json-schema-validator` is the fail-closed parity/admission mechanism.

Generated schemas, OpenAPI, Contract IR, SDK types, and other projections are evidence or generated outputs. They are not a third authored authority.

## Current MCP capability boundary

The product tool catalog in `src/server.rs` is read-only. Each tool is annotated `readOnly=true`, `destructive=false`, and `idempotent=true`; output is bounded by `MAX_TOOL_OUTPUT_BYTES`. The repository does not currently implement a generic write-tool authorization model.

A future mutating MCP tool requires a separate reviewed implementation of identity/authentication, tenant binding, tool-specific authorization, write scope, idempotency, audit/telemetry, and failure semantics before any conformance claim can be promoted.

An abstract model of a future write path is not evidence that those controls exist in this server.
