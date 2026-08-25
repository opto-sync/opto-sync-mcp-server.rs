# Opto Sync MCP Server

    Read-only MCP diagnostics for reconciliation, validation, protocol inspection, and consumer compatibility. The server is a Rust MCP process over stdio. Stdout is exclusively the JSON-RPC wire; structured diagnostics go to stderr and optional OTLP.

    ## Tools

    - `opto_sync_fleet_map`
- `opto_sync_plan`
- `opto_sync_runtime_readiness`
- `opto_sync_shared_platform`
- `opto_sync_lifecycle_state`
- `opto_sync_safety_boundary`

    Every tool is read-only. Planning accepts a closed workload enum plus bounded numeric fields. The server has no arbitrary URL, command, filesystem, database, GitHub mutation, cluster mutation, or secret-value input.

    ## Product topology

    - `syncer.c` — canonical deep-merge engine with cross-language overrides
- `syncer.rs` — Rust-native engine with C ABI and WebAssembly surfaces
- `opto-sync-interfaces` — transport-neutral schemas and generated contracts
- `opto-sync-lib` — local-first synchronization semantics
- `opto-sync-clients` — Dart, TypeScript, and Rust clients

    ## Security boundary

    - The MCP server validates and plans but never applies reconciliation or writes a record.
- Breaking-contract advice must still follow semantic versioning for real consumers.
- Payloads and conflict contents are excluded from tools and telemetry.

    The shared core is pinned at `c6101656c8227251d1dbd61df54f03a186b42ade`. It provides bounded MCP framing, explicit OTLP/gRPC traces, metrics and logs, JSON stderr diagnostics, redaction, low-cardinality tool metrics, and the formal runtime lifecycle. Each tool also owns an explicit span with `skip_all`; arguments and results are never recorded. Configuration readiness reports environment-variable presence only and performs no authentication or network request.

    This server contains no authenticated HTTP client. If a future tool adds one, it must use fixed or strictly validated HTTP(S) origins, reject credentials/query/fragment/private/metadata targets, disable redirects and ambient proxies, keep credentials in sensitive headers, cap every response, and add adversarial tests before merge.

    ## Shared platform knowledge

    The bounded `shared_platform` tool documents ORE Kubernetes, shared definitions, dpm, Cloudflare/Squarespace, Supabase, and Fiducia without exposing a mutation or credential surface.

    ## Validate

    ```sh
    cargo fmt --all -- --check
    cargo clippy --locked --all-targets --all-features -- -D warnings
    cargo test --locked --all-targets --all-features
    cargo build --locked --release
    cargo audit --deny warnings
    ```
