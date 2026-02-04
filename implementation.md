# Nexus AI — Implementation Guide

## Overview
- **Name**: Nexus AI
- **Type**: Rust binary (systemd)
- **Stack**: Rust, Axum 0.8, Tokio, DashMap, serde_json, mimalloc, rust-embed
- **Subdomain**: nexus.lavescar.com.tr
- **Idle RAM**: ~8MB
- **Disk**: ~3MB binary

## Architecture
Single-binary Rust web server using Axum 0.8. HTML templates are embedded into the binary at compile time via `rust-embed`, eliminating the need for a separate static file server. The application serves both API endpoints and the frontend UI from one process. Rate limiting is handled in-memory using DashMap. SSE (Server-Sent Events) is used for streaming text generation responses. In demo mode, all AI responses come from the built-in mock module; when `DEMO_MODE=false`, requests are forwarded to real OpenAI/HuggingFace APIs.

## Features
- 10 AI tools, each with a dedicated HTML UI:
  - Chat (conversational AI)
  - Content (article/blog generation)
  - Code (code generation and explanation)
  - Email (email drafting)
  - Video (video script generation)
  - SEO (SEO analysis and suggestions)
  - Image (image generation prompts)
  - Voice (text-to-speech / voice synthesis)
  - Resume (resume builder)
  - Bot (chatbot builder)
- Landing page with tool directory
- Hybrid mock/real API mode controlled by `DEMO_MODE` env var
- SSE streaming for text generation endpoints
- Global rate limiting (30 requests/day in demo mode)
- Rate limit status endpoint (`/api/limits`)

## Demo Credentials
No credentials needed. The demo is publicly accessible.

## Demo Safety Measures
- `DEMO_MODE=true` is the default in the systemd service
- Global rate limiting: 30 requests per day per IP in demo mode
- `X-Demo-Mode` response header indicates demo status
- DEMO watermark displayed in the UI
- `noindex` meta tag prevents search engine indexing
- `"demo": true` flag included in all JSON API responses
- systemd resource limits: 64MB memory cap, 25% CPU quota
- Security hardening: `NoNewPrivileges`, `PrivateTmp`, `ProtectSystem=strict`

## Build & Deploy
```bash
# Build
cd demos/ai-platform-rust
cargo build --release
# Binary output: target/release/nexus_ai

# Deploy
sudo cp target/release/nexus_ai /opt/nexus-ai/nexus_ai
sudo cp deploy/nexus-ai.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now nexus-ai

# Nginx reverse proxy on port 8080
```

## File Structure
```
ai-platform-rust/
├── Cargo.toml                  # Dependencies and release profile
├── Cargo.lock
├── deploy/
│   └── nexus-ai.service        # systemd unit file
├── src/
│   ├── main.rs                 # Entry point, Axum router setup
│   ├── config.rs               # Environment and config loading
│   ├── error.rs                # Error types and handling
│   ├── mock.rs                 # Mock AI response generator
│   ├── rate_limiter.rs         # DashMap-based rate limiter
│   ├── state.rs                # Shared application state
│   └── routes/
│       ├── mod.rs              # Route module declarations
│       ├── landing.rs          # Landing page handler
│       ├── limits.rs           # Rate limit status API
│       ├── chat.rs             # Chat tool API + page
│       ├── content.rs          # Content generation
│       ├── code.rs             # Code generation
│       ├── email.rs            # Email drafting
│       ├── video.rs            # Video script generation
│       ├── seo.rs              # SEO analysis
│       ├── image.rs            # Image generation
│       ├── voice.rs            # Voice synthesis
│       ├── resume.rs           # Resume builder
│       └── bot.rs              # Chatbot builder
└── templates/
    ├── index.html              # Landing page
    ├── chat.html               # Chat UI
    ├── content.html            # Content UI
    ├── code.html               # Code UI
    ├── email.html              # Email UI
    ├── video.html              # Video UI
    ├── seo.html                # SEO UI
    ├── image.html              # Image UI
    ├── voice.html              # Voice UI
    ├── resume.html             # Resume UI
    └── bot.html                # Bot UI
```

## Key Design Decisions
- **Single binary**: `rust-embed` compiles all HTML templates into the binary, simplifying deployment to a single file copy.
- **mimalloc allocator**: Chosen for lower memory footprint and better performance over the default allocator.
- **Aggressive release profile**: `opt-level = "z"`, LTO, single codegen unit, and symbol stripping produce a ~3MB binary.
- **DashMap for rate limiting**: Lock-free concurrent hashmap avoids mutex contention under load.
- **SSE over WebSockets**: Simpler protocol for one-way streaming; no bidirectional communication needed.
- **Mock-first design**: Demo mode is the default, ensuring the demo works without any API keys configured.

## Verification Checklist
- [ ] `cargo build --release` completes without errors
- [ ] Binary size is under 5MB
- [ ] `./nexus_ai` starts and listens on port 8080
- [ ] Landing page loads at `/`
- [ ] All 10 tool pages load (`/chat`, `/content`, `/code`, `/email`, `/video`, `/seo`, `/image`, `/voice`, `/resume`, `/bot`)
- [ ] Submitting a request returns a streamed mock response
- [ ] Rate limit kicks in after 30 requests (check `/api/limits`)
- [ ] `X-Demo-Mode: true` header is present in responses
- [ ] DEMO watermark is visible on all pages
- [ ] `robots.txt` or `noindex` meta tag is present
- [ ] systemd service starts and stays running after `systemctl start nexus-ai`
