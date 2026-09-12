//! Shared fleet knowledge. This is descriptive and exposes no mutation path.

use serde_json::{Value, json};

#[derive(Clone, Copy)]
pub struct ResourceDocument {
    pub uri: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub body: &'static str,
}

#[derive(Clone, Copy)]
pub struct PromptDocument {
    pub name: &'static str,
    pub description: &'static str,
    pub text: &'static str,
}

const RESOURCES: &[ResourceDocument] = &[
    ResourceDocument {
        uri: "docs://evento-globolo/event-lifecycle",
        name: "Evento Globolo event lifecycle",
        description: "The bounded event-draft, venue-capacity, and cross-posting lifecycle used by Evento Globolo.",
        body: "Evento Globolo keeps event planning advisory and read-only at the MCP boundary. Publishing, cancellation, attendee import, and cross-posting mutations belong to the authenticated product API.",
    },
    ResourceDocument {
        uri: "docs://evento-globolo/provider-routing",
        name: "Evento Globolo provider routing",
        description: "The exact provider planes and delivery boundaries used by the Evento Globolo organization.",
        body: "Provider posture is read-only and exact-scope: GitHub organization, AWS and GCP projects, the shared Supabase namespace, Neon project branches, Cloudflare zone, ORESoftware/k8s-cluster namespace, and two NATS read subjects.",
    },
];

const PROMPTS: &[PromptDocument] = &[
    PromptDocument {
        name: "event_release_readiness",
        description: "Assess Evento Globolo event-release readiness without publishing or mutating an event.",
        text: "Call organization_posture and evgl_plan. Report provider blockers, explicit not_configured evidence gaps, and advisory capacity assumptions. Never publish, cancel, import, or cross-post an event.",
    },
    PromptDocument {
        name: "cross_posting_review",
        description: "Review an Evento Globolo cross-posting plan against provider and safety boundaries.",
        text: "Call evgl_fleet_map, evgl_shared_platform, organization_posture, and evgl_safety_boundary. Verify exact provider readiness and explain which authenticated product API must perform any later mutation.",
    },
];

#[must_use]
pub const fn resources() -> &'static [ResourceDocument] {
    RESOURCES
}

#[must_use]
pub fn resource(uri: &str) -> Option<ResourceDocument> {
    RESOURCES
        .iter()
        .copied()
        .find(|resource| resource.uri == uri)
}

#[must_use]
pub const fn prompts() -> &'static [PromptDocument] {
    PROMPTS
}

#[must_use]
pub fn prompt(name: &str) -> Option<PromptDocument> {
    PROMPTS.iter().copied().find(|prompt| prompt.name == name)
}

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
