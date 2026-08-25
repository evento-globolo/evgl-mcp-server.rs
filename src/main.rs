//! Binary bootstrap. Stdout is reserved exclusively for MCP JSON-RPC.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    evgl_mcp_server::runtime::run_stdio().await
}
