//! Typed, read-only MCP tool routing.

use std::sync::Arc;

use ores_mcp_server_core_libs::observability::{ToolClass, ToolMetrics, ToolOutcome};
use ores_mcp_server_core_libs::state_machine::LifecycleController;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        GetPromptRequestParams, GetPromptResult, Implementation, ListPromptsResult,
        ListResourcesResult, PaginatedRequestParams, Prompt, PromptMessage,
        ReadResourceRequestParams, ReadResourceResult, Resource, ResourceContents, Role,
        ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde_json::Value;

use crate::{
    domain::{self, PlanInput},
    knowledge,
};

pub const SERVER_NAME: &str = "evgl-mcp-server";
pub const SERVER_NAMESPACE: &str = "evento-globolo";
const MAX_TOOL_OUTPUT_BYTES: usize = 512 * 1024;

#[derive(Clone)]
pub struct EventoGloboloMCPServer {
    tool_router: ToolRouter<Self>,
    metrics: ToolMetrics,
    lifecycle: Arc<LifecycleController>,
}

impl EventoGloboloMCPServer {
    #[must_use]
    pub fn new(lifecycle: LifecycleController) -> Self {
        Self {
            tool_router: Self::tool_router(),
            metrics: ToolMetrics::global(),
            lifecycle: Arc::new(lifecycle),
        }
    }
}

#[tool_router]
impl EventoGloboloMCPServer {
    #[tool(
        description = "Return the product-owned repository topology and component roles. Pure, local, and read-only."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "evgl_fleet_map", mcp.tool.class = "inventory"))]
    fn evgl_fleet_map(&self) -> String {
        let timer = self.metrics.start(ToolClass::Inventory);
        let output = render(&domain::fleet_map());
        timer.finish(ToolOutcome::Ok);
        output
    }

    #[tool(
        description = "Calculate a bounded, deterministic product plan from a closed workload enum and numeric units. Never executes or mutates anything."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "evgl_plan", mcp.tool.class = "details"))]
    fn evgl_plan(&self, Parameters(input): Parameters<PlanInput>) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Details);
        let result = domain::plan(input).map(|value| render(&value));
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
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "evgl_runtime_readiness", mcp.tool.class = "health"))]
    fn evgl_runtime_readiness(&self) -> String {
        let timer = self.metrics.start(ToolClass::Health);
        let output = render(&domain::runtime_readiness());
        timer.finish(ToolOutcome::Ok);
        output
    }

    #[tool(
        description = "Return bounded shared knowledge for ORE Kubernetes, shared definitions, dpm, Cloudflare/Squarespace, Supabase, and Fiducia. Descriptive only."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "evgl_shared_platform", mcp.tool.class = "inventory"))]
    fn evgl_shared_platform(&self) -> String {
        let timer = self.metrics.start(ToolClass::Inventory);
        let output = render(&knowledge::shared_platform());
        timer.finish(ToolOutcome::Ok);
        output
    }

    #[tool(
        description = "Return the formal runtime lifecycle state, monotonic revision, and bounded transition audit. Callers cannot trigger transitions."
    )]
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "evgl_lifecycle_state", mcp.tool.class = "health"))]
    fn evgl_lifecycle_state(&self) -> Result<String, String> {
        let timer = self.metrics.start(ToolClass::Health);
        let result = self
            .lifecycle
            .snapshot_and_audit()
            .map(|(snapshot, audit)| {
                render(&serde_json::json!({
                    "state": snapshot.state(),
                    "revision": snapshot.revision(),
                    "transitions": audit,
                    "readOnly": true
                }))
            })
            .map_err(|error| error.to_string());
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
    #[tracing::instrument(name = "mcp.tool", skip_all, fields(mcp.tool.name = "evgl_safety_boundary", mcp.tool.class = "inventory"))]
    fn evgl_safety_boundary(&self) -> String {
        let timer = self.metrics.start(ToolClass::Inventory);
        let output = render(&domain::safety_boundary());
        timer.finish(ToolOutcome::Ok);
        output
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for EventoGloboloMCPServer {
    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let resources = knowledge::resources()
            .iter()
            .map(|resource| {
                Resource::new(resource.uri, resource.name)
                    .with_description(resource.description)
                    .with_mime_type("text/markdown")
            })
            .collect();
        Ok(ListResourcesResult::with_all_items(resources))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let resource = knowledge::resource(&request.uri)
            .ok_or_else(|| McpError::resource_not_found("unknown Evento Globolo resource", None))?;
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(resource.body, resource.uri).with_mime_type("text/markdown"),
        ]))
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        let prompts = knowledge::prompts()
            .iter()
            .map(|prompt| Prompt::new(prompt.name, Some(prompt.description), None))
            .collect();
        Ok(ListPromptsResult::with_all_items(prompts))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let prompt = knowledge::prompt(&request.name)
            .ok_or_else(|| McpError::invalid_params("unknown Evento Globolo prompt", None))?;
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
            .with_server_info(Implementation::new(SERVER_NAME, env!("CARGO_PKG_VERSION")).with_title("Evento Globolo MCP Server"))
            .with_instructions("Read-only MCP diagnostics for events, venues, attendees, providers, and cross-posting. The server is read-only and never logs MCP arguments or results.")
    }
}

fn render(value: &Value) -> String {
    match serde_json::to_string(value) {
        Ok(rendered) if rendered.len() <= MAX_TOOL_OUTPUT_BYTES => rendered,
        Ok(_) => r#"{"error":"bounded output limit exceeded"}"#.to_string(),
        Err(_) => r#"{"error":"serialization failed"}"#.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_prefixed_tool_catalog_is_exposed() {
        let server =
            EventoGloboloMCPServer::new(LifecycleController::new(8).expect("valid lifecycle"));
        let names = server
            .tool_router
            .list_all()
            .into_iter()
            .map(|tool| tool.name.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                "evgl_fleet_map",
                "evgl_lifecycle_state",
                "evgl_plan",
                "evgl_runtime_readiness",
                "evgl_safety_boundary",
                "evgl_shared_platform",
            ]
        );
    }

    #[test]
    fn metadata_is_read_only_and_namespaced() {
        let server =
            EventoGloboloMCPServer::new(LifecycleController::new(8).expect("valid lifecycle"));
        let info = server.get_info();
        assert_eq!(info.server_info.name, SERVER_NAME);
        assert!(
            info.instructions
                .as_deref()
                .is_some_and(|value| value.contains("read-only"))
        );
    }
}
