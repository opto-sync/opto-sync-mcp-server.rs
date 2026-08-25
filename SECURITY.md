# Security policy

    Report vulnerabilities privately to the `opto-sync` maintainers. Never include secrets, customer data, source payloads, or exploit material in a public issue.

    ## Runtime boundary

    - stdio is the only transport and stdout is the MCP wire;
    - tools are deterministic, read-only, and fail closed on unknown fields or out-of-range numbers;
    - no tool accepts arbitrary URLs, commands, source payloads, credentials, or mutation instructions;
    - readiness exposes presence booleans only;
    - telemetry excludes arguments, results, identities, secrets, and high-cardinality values.

    - The MCP server validates and plans but never applies reconciliation or writes a record.
- Breaking-contract advice must still follow semantic versioning for real consumers.
- Payloads and conflict contents are excluded from tools and telemetry.
