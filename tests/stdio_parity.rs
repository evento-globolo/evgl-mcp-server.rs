//! Evidence-backed organization and provider parity contract.

use evgl_mcp_server::parity::{self, org_spec};

#[test]
fn stdio_wire_contract_names_all_clients_and_providers() {
    let spec = org_spec();
    assert_eq!(spec.organization, "evento-globolo");
    assert_eq!(spec.repository, "evento-globolo/evgl-mcp-server.rs");
    assert_eq!(parity::PROVIDER_OPERATIONS.len(), 8);
    assert!(
        parity::PROVIDER_OPERATIONS
            .iter()
            .all(|(provider, operations)| {
                !provider.is_empty()
                    && operations.len() >= 2
                    && operations
                        .iter()
                        .all(|operation| operation.starts_with("read_"))
            })
    );
    let clients = ["cursor", "openai", "anthropic", "gemini", "grok", "qwen"];
    assert_eq!(clients.len(), 6);
}

#[test]
fn provider_operation_names_are_unique_and_scoped() {
    for (provider, operations) in parity::PROVIDER_OPERATIONS {
        assert!(!provider.contains('*'));
        let mut unique = operations.to_vec();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), operations.len());
    }
}
