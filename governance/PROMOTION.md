# Promotion governance

A contract or conformance claim may be promoted only when it is tied to implemented behavior and exact-head evidence.

## Evidence rules

- Use the exact PR head SHA. Stale-head, base-only, historical-only, queued, skipped, zero-step, runner-admission, and billing-blocked results are not green evidence.
- The relevant implementation test/check must actually execute. A documentation-only or abstract model cannot certify behavior that is absent from the server.
- Cross-runtime contract parity uses independent authored TypeSpec and Draft 2020-12 JSON Schema with fail-closed TJSV admission. Generated artifacts are evidence only.
- Negative controls are required when a new security-sensitive gate is introduced.

## Mutation gate

The current server is read-only. Before a mutating tool can be promoted, the same change series must establish and test:

1. authenticated identity through the approved Shared Auth boundary;
2. tenant binding before resource selection;
3. tool-specific authorization and explicit write scope;
4. stable mutation identity/idempotency and replay behavior;
5. bounded inputs/outputs and no arbitrary shell/URL execution;
6. ORES OTel audit/telemetry without secret material;
7. deterministic refusal controls for unauthenticated, wrong-tenant, unauthorized, duplicate/replay, and generic-execution cases.

A future-state truth table is useful design material but is not implementation conformance until it refines the real write path.
