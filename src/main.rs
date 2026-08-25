//! Binary bootstrap. Stdout is reserved exclusively for MCP JSON-RPC.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    opto_sync_mcp_server::runtime::run_stdio().await
}
