//! Stdio-only transport composition and lifecycle ownership.

use ore_mcp_org_server::run_augmented_stdio;
use ores_mcp_server_core_libs::state_machine::LifecycleController;

use crate::server::{EventoGloboloMCPServer, SERVER_NAME, SERVER_NAMESPACE};

/// Initialize telemetry and serve bounded MCP frames on stdio.
///
/// # Errors
///
/// Returns an error when lifecycle initialization or MCP transport
/// startup, service, or shutdown fails.
pub async fn run_stdio() -> anyhow::Result<()> {
    let _telemetry = ores_mcp_server_core_libs::observability::init(SERVER_NAME, SERVER_NAMESPACE);
    let lifecycle = LifecycleController::new(128)?;
    let server = EventoGloboloMCPServer::new(lifecycle);
    run_augmented_stdio(server, crate::parity::org_spec())
        .await
        .map_err(|error| anyhow::anyhow!("MCP runtime failed: {error}"))?;
    Ok(())
}
