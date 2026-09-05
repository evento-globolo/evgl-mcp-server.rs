//! Immutable organization identity for the shared MCP provider parity layer.

use ore_mcp_org_server::OrgSpec;

/// Operation names supplied by the pinned shared provider adapters.
pub const PROVIDER_OPERATIONS: &[(&str, &[&str])] = &[
    ("github", &["read_organization", "read_latest_workflow_run"]),
    ("aws", &["read_caller_identity", "read_eks_clusters"]),
    ("gcp", &["read_project", "read_enabled_services"]),
    ("supabase", &["read_auth_settings", "read_data_api_schema"]),
    ("neon", &["read_projects", "read_project_branches"]),
    ("cloudflare", &["read_zone", "read_dns_records"]),
    ("k8s_cluster", &["read_deployments", "read_pods"]),
    (
        "nats",
        &["read_service_snapshot", "read_dependency_snapshot"],
    ),
];

const DEPENDENCIES: &[&str] = &[
    "ORESoftware/mcp-rust-libs",
    "ores-otel/ores-mcp-server-core-libs.rs",
    "evento-globolo/evgl-interfaces",
    "evento-globolo/evgl-lib-core",
    "evento-globolo/evgl-api-server.rs",
    "evento-globolo/evgl-infra",
    "evento-globolo/evgl-sync",
    "shared-auth/shared-auth-clients",
    "shared-auth/shared-auth-interfaces",
    "shared-auth/shared-auth-lib",
    "zed-pkg/zed-cli",
];

/// The exact organization identity consumed by all shared provider posture
/// tools. Provider credentials remain process-environment only.
#[must_use]
pub const fn org_spec() -> OrgSpec {
    OrgSpec {
        organization: "evento-globolo",
        repository: "evento-globolo/evgl-mcp-server.rs",
        service_name: "evgl-mcp-server",
        package_name: "evgl-mcp-server",
        dependencies: DEPENDENCIES,
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_and_dependencies_are_org_scoped() {
        let spec = org_spec();
        assert_eq!(spec.organization, "evento-globolo");
        assert_eq!(spec.repository, "evento-globolo/evgl-mcp-server.rs");
        assert!(DEPENDENCIES.contains(&"evento-globolo/evgl-interfaces"));
        assert!(DEPENDENCIES.contains(&"ORESoftware/mcp-rust-libs"));
        assert!(
            DEPENDENCIES
                .iter()
                .all(|dependency| { !dependency.is_empty() && !dependency.contains('*') })
        );
    }
}
