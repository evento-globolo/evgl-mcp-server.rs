//! Shared Auth-protected Streamable HTTP entry point for remote MCP clients.

use ore_mcp_org_server::run_augmented_http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lifecycle = ores_mcp_server_core_libs::state_machine::LifecycleController::new(128)?;
    let server = evgl_mcp_server::server::EventoGloboloMCPServer::new(lifecycle);
    run_augmented_http(server, evgl_mcp_server::parity::org_spec()).await
}
