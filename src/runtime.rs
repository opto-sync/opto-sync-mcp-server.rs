//! Transport composition and lifecycle ownership.

use ore_mcp_org_server::run_augmented_stdio;

use crate::{
    flags, observability, parity,
    server::{OptoSyncMCPServer, SERVER_NAME, SERVER_NAMESPACE},
};

/// Initialize telemetry and serve bounded MCP frames on stdio.
///
/// # Errors
///
/// Returns an error when lifecycle initialization or MCP transport
/// startup, service, or shutdown fails.
pub async fn run_stdio() -> anyhow::Result<()> {
    const ROUTINE_ID: &str = "ores-routine-IQEFyT3CGzAfTvByoyKcv";
    let lifecycle = observability::logger();
    let config = match flags::resolve() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("invalid command-line configuration: {error:#}");
            let _ = observability::failure(&lifecycle, "mcp.config.invalid")
                .add_trace("ores-trace-Rytef_9Vu8Ccft8-x6pOV", false)
                .add_routine_id(ROUTINE_ID)
                .send();
            let _ = lifecycle.close();
            return Err(error);
        }
    };
    let _telemetry = ores_mcp_server_core_libs::observability::init(SERVER_NAME, SERVER_NAMESPACE);
    let server = match OptoSyncMCPServer::from_config(config) {
        Ok(server) => server,
        Err(error) => {
            let _ = observability::failure(&lifecycle, "mcp.lifecycle.start_failed")
                .add_trace("ores-trace-Eb2dzDZ1sU0qBgQDjZ_dH", false)
                .add_routine_id(ROUTINE_ID)
                .send();
            let _ = lifecycle.close();
            return Err(anyhow::Error::msg(error));
        }
    };
    if let Err(error) = run_augmented_stdio(server, parity::org_spec()).await {
        let _ = observability::failure(&lifecycle, "mcp.runtime.failed")
            .add_trace("ores-trace-Mjra79Tm9CWiGWtxsXLIZ", false)
            .add_routine_id(ROUTINE_ID)
            .send();
        let _ = lifecycle.close();
        return Err(anyhow::anyhow!("MCP runtime failed: {error}"));
    }
    let _ = observability::event(&lifecycle, "mcp.runtime.stopped")
        .add_trace("ores-trace-qR6Z4dfu0F8P0e8WNrMjV", false)
        .add_routine_id(ROUTINE_ID)
        .send();
    let _ = lifecycle.close();
    Ok(())
}
