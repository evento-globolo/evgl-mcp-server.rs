# Evento Globolo MCP Server

    Read-only MCP diagnostics for events, venues, attendees, providers, and cross-posting. The server is a Rust MCP process over stdio or Shared Auth-protected Streamable HTTP at `/mcp`. Stdout is exclusively the JSON-RPC wire; structured diagnostics go to stderr and optional OTLP.

    ## Tools

    - `evgl_fleet_map`
- `evgl_plan`
- `evgl_runtime_readiness`
- `evgl_shared_platform`
- `evgl_lifecycle_state`
- `evgl_safety_boundary`

    Every tool is read-only. Planning accepts a closed workload enum plus bounded numeric fields. The server has no arbitrary URL, command, filesystem, database, GitHub mutation, cluster mutation, or secret-value input.

    ## Fleet parity

    The shared `ore-mcp-org-server` layer adds the same bounded provider posture
    contract used by every hardened organization server: GitHub, AWS, GCP,
    Supabase, NeonDB, Cloudflare, `ORESoftware/k8s-cluster`, and NATS. Each
    provider reports one of `ready`, `not_configured`, `degraded`,
    `unauthorized`, or `forbidden`; missing credentials are evidence gaps, not
    successful health checks. The parity layer also publishes exact ownership,
    Zed dependency, Shared Auth, encrypted-environment, and security resources
    plus deploy-readiness, provider-triage, and dependency-review prompts.

    Remote clients (Cursor, ChatGPT/OpenAI, Anthropic/Claude, Gemini, Grok, and
    Qwen) use the final MCP protocol revision and OAuth 2.1 Shared Auth
    boundary. Provider credentials stay in the process environment; tokens are
    never accepted as tool arguments, forwarded to upstreams, or returned in
    diagnostics. Requests and results are bounded and credentialed HTTP
    redirects and ambient proxies are disabled by the shared runtime.

    ## Product topology

    - `evgl-api` — global event-management and cross-posting API
- `evgl-interfaces` — event, venue, attendee, and provider contracts
- `evento-globolo-libs` — event and cross-posting policy libraries
- `evgl-sync` — offline-first draft, attendee, and venue sync
- `evgl-infra` — Kubernetes and bounded Cloudflare edge infrastructure

    ## Security boundary

    - The MCP server never publishes, cancels, imports, or cross-posts an event.
- Attendee identities, tickets, and provider credentials are excluded.
- Capacity plans are deterministic advisory calculations, not safety certification.

    The shared core is pinned at `c6101656c8227251d1dbd61df54f03a186b42ade`. It provides bounded MCP framing, explicit OTLP/gRPC traces, metrics and logs, JSON stderr diagnostics, redaction, low-cardinality tool metrics, and the formal runtime lifecycle. Each tool also owns an explicit span with `skip_all`; arguments and results are never recorded. Configuration readiness reports environment-variable presence only and performs no authentication or network request.

    This server contains no authenticated HTTP client. If a future tool adds one, it must use fixed or strictly validated HTTP(S) origins, reject credentials/query/fragment/private/metadata targets, disable redirects and ambient proxies, keep credentials in sensitive headers, cap every response, and add adversarial tests before merge.

    ## Shared platform knowledge

    The bounded `shared_platform` tool documents ORE Kubernetes, shared definitions, dpm, Cloudflare/Squarespace, Supabase, and Fiducia without exposing a mutation or credential surface.

    ## Validate

    ```sh
    cargo run --bin evgl-mcp-server
    cargo run --bin evgl-mcp-http
    cargo fmt --all -- --check
    cargo clippy --locked --all-targets --all-features -- -D warnings
    cargo test --locked --all-targets --all-features
    cargo build --locked --release
    cargo audit --deny warnings
    ```
