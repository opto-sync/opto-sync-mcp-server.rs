//! Typed, read-only MCP tool routing.

use std::sync::Arc;

use ores_mcp_server_core_libs::observability::{ToolClass, ToolMetrics, ToolOutcome};
use ores_mcp_server_core_libs::state_machine::{LifecycleController, LifecycleEvent};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        GetPromptRequestParams, GetPromptResult, Implementation, ListPromptsResult,
        ListResourcesResult, PaginatedRequestParams, Prompt, PromptMessage,
        ReadResourceRequestParams, ReadResourceResult, Resource, ResourceContents, Role,
        ServerCapabilities, ServerInfo, ToolAnnotations,
    },
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    domain::{self, PlanInput},
    flags::RuntimeConfig,
    knowledge,
};

pub const SERVER_NAME: &str = "opto-sync-mcp-server";
pub const SERVER_NAMESPACE: &str = "opto-sync";
const MAX_TOOL_OUTPUT_BYTES: usize = 40_960;

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EmptyInput {}

#[derive(Clone)]
pub struct OptoSyncMCPServer {
    tool_router: ToolRouter<Self>,
    metrics: ToolMetrics,
    lifecycle: Arc<LifecycleController>,
    lifecycle_audit_capacity: usize,
}

impl OptoSyncMCPServer {
    /// Build one ready, bounded server from immutable boundary configuration.
    ///
    /// # Errors
    ///
    /// Returns an error when the lifecycle controller cannot be created or
    /// its formally checked startup transitions fail.
    pub fn from_config(config: RuntimeConfig) -> Result<Self, String> {
        let lifecycle = LifecycleController::new(config.lifecycle_audit_capacity)
            .map_err(|error| error.to_string())?;
        lifecycle
            .transition(LifecycleEvent::Start)
            .map_err(|error| error.to_string())?;
        lifecycle
            .transition(LifecycleEvent::Started)
            .map_err(|error| error.to_string())?;

        let mut tool_router = Self::tool_router();
        for route in tool_router.map.values_mut() {
            route.attr.annotations = Some(
                ToolAnnotations::new()
                    .read_only(true)
                    .destructive(false)
                    .idempotent(true)
                    .open_world(false),
            );
        }
        Ok(Self {
            tool_router,
            metrics: ToolMetrics::global(),
            lifecycle: Arc::new(lifecycle),
            lifecycle_audit_capacity: config.lifecycle_audit_capacity,
        })
    }
}

#[tool_router]
impl OptoSyncMCPServer {
    #[tool(
        description = "Return the product-owned repository topology and component roles. Pure, local, and read-only."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_fleet_map", mcp.tool.class = "inventory"))]
    fn opto_sync_fleet_map(
        &self,
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Inventory);
        let output = render(&domain::fleet_map());
        timer.finish(if output.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Error
        });
        output
    }

    #[tool(
        description = "Calculate a bounded, deterministic product plan from a closed workload enum and numeric units. Never executes or mutates anything."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_plan", mcp.tool.class = "details"))]
    fn opto_sync_plan(&self, Parameters(input): Parameters<PlanInput>) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Details);
        let result = domain::plan(input).and_then(|value| render(&value));
        timer.finish(if result.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Rejected
        });
        result
    }

    #[tool(
        description = "Report presence-only configuration readiness. Values are never read into output, logged, or authenticated; no network request is made."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_runtime_readiness", mcp.tool.class = "health"))]
    fn opto_sync_runtime_readiness(
        &self,
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Health);
        let output = render(&domain::runtime_readiness(self.lifecycle_audit_capacity));
        timer.finish(if output.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Error
        });
        output
    }

    #[tool(
        description = "Return the exact Opto Sync replay, watermark, tombstone, conflict, schema, and telemetry invariants. Pure, bounded, and read-only."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_contract_invariants", mcp.tool.class = "details"))]
    fn opto_sync_contract_invariants(
        &self,
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Details);
        let output = render(&knowledge::contract_invariants());
        timer.finish(if output.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Error
        });
        output
    }

    #[tool(
        description = "Return the required Dart, Rust, and TypeScript consumer capabilities plus the breaking-contract policy. Pure, bounded, and read-only."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_consumer_matrix", mcp.tool.class = "details"))]
    fn opto_sync_consumer_matrix(
        &self,
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Details);
        let output = render(&knowledge::consumer_matrix());
        timer.finish(if output.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Error
        });
        output
    }

    #[tool(
        description = "Return bounded shared knowledge for ORE Kubernetes, shared definitions, dpm, Cloudflare/Squarespace, Supabase, and Fiducia. Descriptive only."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_shared_platform", mcp.tool.class = "inventory"))]
    fn opto_sync_shared_platform(
        &self,
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Inventory);
        let output = render(&knowledge::shared_platform());
        timer.finish(if output.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Error
        });
        output
    }

    #[tool(
        description = "Return the formal runtime lifecycle state, monotonic revision, and bounded transition audit. Callers cannot trigger transitions."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_lifecycle_state", mcp.tool.class = "health"))]
    fn opto_sync_lifecycle_state(
        &self,
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Health);
        let result = self
            .lifecycle
            .snapshot_and_audit()
            .map_err(|error| error.to_string())
            .and_then(|(snapshot, audit)| {
                render(&serde_json::json!({
                    "state": snapshot.state(),
                    "revision": snapshot.revision(),
                    "transitions": audit,
                    "readOnly": true
                }))
            });
        timer.finish(if result.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Error
        });
        result
    }

    #[tool(
        description = "Return the product-specific safety and privacy boundary. Pure, local, and read-only."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "opto_sync_safety_boundary", mcp.tool.class = "inventory"))]
    fn opto_sync_safety_boundary(
        &self,
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Inventory);
        let output = render(&domain::safety_boundary());
        timer.finish(if output.is_ok() {
            ToolOutcome::Ok
        } else {
            ToolOutcome::Error
        });
        output
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for OptoSyncMCPServer {
    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult::with_all_items(
            knowledge::resources()
                .iter()
                .map(|resource| {
                    Resource::new(resource.uri, resource.name)
                        .with_description(resource.description)
                        .with_mime_type(resource.mime_type)
                })
                .collect(),
        ))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let resource = knowledge::resource(&request.uri)
            .ok_or_else(|| McpError::resource_not_found("unknown Opto Sync resource", None))?;
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(resource.body, resource.uri).with_mime_type(resource.mime_type),
        ]))
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult::with_all_items(
            knowledge::prompts()
                .iter()
                .map(|prompt| Prompt::new(prompt.name, Some(prompt.description), None))
                .collect(),
        ))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let prompt = knowledge::prompt(&request.name)
            .ok_or_else(|| McpError::invalid_params("unknown Opto Sync prompt", None))?;
        Ok(
            GetPromptResult::new(vec![PromptMessage::new_text(Role::User, prompt.text)])
                .with_description(prompt.description),
        )
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
        )
            .with_server_info(Implementation::new(SERVER_NAME, env!("CARGO_PKG_VERSION")).with_title("Opto Sync MCP Server"))
            .with_instructions("Read-only Opto Sync diagnostics for reconciliation, contract invariants, cross-language consumer compatibility, and provider posture. The exact same MCP 2025-11-25 catalog is served to Cursor, ChatGPT/OpenAI, Claude/Anthropic, Gemini, Grok, and Qwen over stdio or Shared-Auth-protected Streamable HTTP. Start with organization_posture for GitHub, AWS, GCP, Supabase, Neon, Cloudflare, ORESoftware/k8s-cluster, and NATS; missing configuration is never success.")
    }
}

fn render(value: &Value) -> Result<String, String> {
    let rendered = serde_json::to_string(value)
        .map_err(|_| "failed to serialize bounded Opto Sync result".to_owned())?;
    truncate_output(rendered)
}

fn truncate_output(rendered: String) -> Result<String, String> {
    if rendered.len() > MAX_TOOL_OUTPUT_BYTES {
        return Err("Opto Sync tool result exceeded the fixed output bound".to_owned());
    }
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_prefixed_tool_catalog_is_exposed() {
        let server = OptoSyncMCPServer::from_config(RuntimeConfig {
            lifecycle_audit_capacity: 8,
        })
        .expect("valid server");
        let names = server
            .tool_router
            .list_all()
            .into_iter()
            .map(|tool| tool.name.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                "opto_sync_consumer_matrix",
                "opto_sync_contract_invariants",
                "opto_sync_fleet_map",
                "opto_sync_lifecycle_state",
                "opto_sync_plan",
                "opto_sync_runtime_readiness",
                "opto_sync_safety_boundary",
                "opto_sync_shared_platform",
            ]
        );
    }

    #[test]
    fn metadata_is_read_only_and_namespaced() {
        let server = OptoSyncMCPServer::from_config(RuntimeConfig {
            lifecycle_audit_capacity: 8,
        })
        .expect("valid server");
        let info = server.get_info();
        assert_eq!(info.server_info.name, SERVER_NAME);
        assert!(
            info.instructions
                .as_deref()
                .is_some_and(|value| value.to_ascii_lowercase().contains("read-only"))
        );
        assert!(server.tool_router.list_all().iter().all(|tool| {
            tool.annotations.as_ref().is_some_and(|annotations| {
                annotations.read_only_hint == Some(true)
                    && annotations.destructive_hint == Some(false)
                    && annotations.idempotent_hint == Some(true)
            })
        }));
    }

    #[test]
    fn output_bound_fails_closed() {
        assert!(truncate_output("x".repeat(MAX_TOOL_OUTPUT_BYTES)).is_ok());
        assert!(truncate_output("x".repeat(MAX_TOOL_OUTPUT_BYTES + 1)).is_err());
    }
}
