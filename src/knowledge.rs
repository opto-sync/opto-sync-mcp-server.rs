//! Shared fleet knowledge. This is descriptive and exposes no mutation path.

use serde_json::{Value, json};

pub struct ResourceDocument {
    pub uri: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub mime_type: &'static str,
    pub body: &'static str,
}

pub struct PromptDocument {
    pub name: &'static str,
    pub description: &'static str,
    pub text: &'static str,
}

const RESOURCES: &[ResourceDocument] = &[
    ResourceDocument {
        uri: "docs://opto-sync-protocol",
        name: "Opto Sync protocol invariants",
        description: "Conflict, replay, watermark, tombstone, and compatibility invariants shared by Opto Sync consumers.",
        mime_type: "text/markdown",
        body: "# Opto Sync protocol invariants\n\n- Mutation identities are stable across retries.\n- Replays are idempotent and preserve causal order.\n- A server watermark cannot acknowledge an identity the client has not allocated.\n- Tombstones participate in reconciliation until every relevant replica has observed them.\n- Conflicts are resolved by an explicit strategy; payloads are never silently discarded.\n- Rust, Dart, and TypeScript consumers share versioned JSON-schema contracts.\n",
    },
    ResourceDocument {
        uri: "orgmap://opto-sync",
        name: "Opto Sync organization map",
        description: "Exact repositories, deployment authority, clients, and shared service boundaries for Opto Sync.",
        mime_type: "text/markdown",
        body: "# opto-sync organization\n\n- `syncer.c`: canonical deep-merge engine\n- `syncer.rs`: Rust, C ABI, and WebAssembly engine\n- `opto-sync-interfaces`: transport-neutral schemas\n- `opto-sync-lib`: local-first synchronization semantics\n- `opto-sync-clients`: Dart, TypeScript, and Rust clients\n- `opto-sync-mcp-server.rs`: read-only diagnostics\n- `ORESoftware/k8s-cluster`: deployment authority\n- `shared-auth`: identity authority\n- `ores-otel`: telemetry\n- `zed-pkg`: dependency intent\n",
    },
    ResourceDocument {
        uri: "schema://opto-sync-contract",
        name: "Opto Sync contract schema outline",
        description: "Bounded schema outline for mutation envelopes, watermarks, conflicts, and acknowledgements.",
        mime_type: "application/json",
        body: r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","title":"OptoSyncContractOutline","type":"object","additionalProperties":false,"required":["mutationId","replicaId","schemaVersion","operation"],"properties":{"mutationId":{"type":"string","minLength":1,"maxLength":128},"replicaId":{"type":"string","minLength":1,"maxLength":128},"schemaVersion":{"type":"string","minLength":1,"maxLength":64},"operation":{"enum":["upsert","delete","acknowledge"]},"watermark":{"type":"integer","minimum":0},"conflictStrategy":{"enum":["reject","last_write_wins","deep_merge","manual"]}}}"#,
    },
];

const PROMPTS: &[PromptDocument] = &[
    PromptDocument {
        name: "consumer_compatibility",
        description: "Review cross-language Opto Sync consumer compatibility without changing a contract or deployment.",
        text: "Review opto_sync_consumer_matrix, opto_sync_contract_invariants, zed_dependency_graph, and github_posture. Identify schema-version, generated-client, replay, and watermark incompatibilities across Rust, Dart, and TypeScript. Cite evidence and propose additive remediation only; do not mutate repositories or providers.",
    },
    PromptDocument {
        name: "deploy_readiness",
        description: "Decide whether the Opto Sync MCP and synchronization stack have sufficient evidence to deploy.",
        text: "Evaluate organization_posture, opto_sync_runtime_readiness, opto_sync_lifecycle_state, opto_sync_contract_invariants, and github_posture. Treat not_configured, degraded, unauthorized, and forbidden as distinct states. Report blockers and evidence; never infer provider success from missing configuration.",
    },
    PromptDocument {
        name: "sync_conflict_review",
        description: "Review a synchronization design against Opto Sync conflict and replay invariants.",
        text: "Use opto_sync_plan, opto_sync_contract_invariants, opto_sync_consumer_matrix, and the docs://opto-sync-protocol resource. Check stable mutation identity, idempotent replay, causal ordering, watermarks, tombstones, explicit conflict strategy, and consumer schema compatibility. Produce a read-only review with concrete failed invariants.",
    },
];

#[must_use]
pub fn shared_platform() -> Value {
    json!({
        "oreKubernetes": {
            "role": "GitOps deployment and runtime topology",
            "diagnosticsOnly": true,
            "clusterMutationExposed": false
        },
        "sharedDefinitions": {
            "role": "shared service and infrastructure contracts",
            "consumerMustPinReviewedRevision": true
        },
        "dpm": {
            "role": "declarative migration planning and verification",
            "databaseMutationExposed": false
        },
        "cloudflareSquarespace": {
            "role": "edge, DNS, and site-handoff context",
            "credentials": "environment only",
            "mutationExposed": false
        },
        "supabase": {
            "role": "data and authentication boundary where adopted",
            "credentials": "environment/header only",
            "payloadTelemetry": false
        },
        "fiducia": {
            "role": "secret and lease delivery boundary",
            "credentials": "environment/header only",
            "secretValuesExposed": false
        }
    })
}

#[must_use]
pub fn contract_invariants() -> Value {
    json!({
        "contract": "opto-sync",
        "schemaAuthority": "opto-sync/opto-sync-interfaces",
        "engines": ["opto-sync/syncer.c", "opto-sync/syncer.rs"],
        "invariants": [
            {"id": "stable_mutation_identity", "required": true},
            {"id": "idempotent_replay", "required": true},
            {"id": "causal_order_preserved", "required": true},
            {"id": "watermark_never_acknowledges_unallocated_identity", "required": true},
            {"id": "tombstones_retained_until_observed", "required": true},
            {"id": "conflict_strategy_is_explicit", "required": true},
            {"id": "payloads_excluded_from_telemetry", "required": true}
        ],
        "mutationExposed": false
    })
}

#[must_use]
pub fn consumer_matrix() -> Value {
    json!({
        "schemaAuthority": "opto-sync/opto-sync-interfaces",
        "clientsRepository": "opto-sync/opto-sync-clients",
        "consumers": [
            {"language": "dart", "required": ["json_schema", "offline_queue", "idempotent_replay", "watermark_validation"]},
            {"language": "rust", "required": ["json_schema", "offline_queue", "idempotent_replay", "watermark_validation"]},
            {"language": "typescript", "required": ["json_schema", "offline_queue", "idempotent_replay", "watermark_validation"]}
        ],
        "breakingChangePolicy": "new major schema version plus generated-client compatibility evidence",
        "readOnly": true
    })
}

#[must_use]
pub fn resources() -> &'static [ResourceDocument] {
    RESOURCES
}

#[must_use]
pub fn resource(uri: &str) -> Option<&'static ResourceDocument> {
    RESOURCES.iter().find(|resource| resource.uri == uri)
}

#[must_use]
pub fn prompts() -> &'static [PromptDocument] {
    PROMPTS
}

#[must_use]
pub fn prompt(name: &str) -> Option<&'static PromptDocument> {
    PROMPTS.iter().find(|prompt| prompt.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_knowledge_catalogs_are_exact_sorted_and_bounded() {
        assert_eq!(
            resources().iter().map(|item| item.uri).collect::<Vec<_>>(),
            [
                "docs://opto-sync-protocol",
                "orgmap://opto-sync",
                "schema://opto-sync-contract"
            ]
        );
        assert_eq!(
            prompts().iter().map(|item| item.name).collect::<Vec<_>>(),
            [
                "consumer_compatibility",
                "deploy_readiness",
                "sync_conflict_review"
            ]
        );
        assert!(resources().iter().all(|item| item.body.len() < 16_384));
        assert_eq!(contract_invariants()["mutationExposed"], false);
        assert_eq!(consumer_matrix()["readOnly"], true);
    }
}
