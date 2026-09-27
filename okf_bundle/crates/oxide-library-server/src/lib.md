---
okf_version: "0.2"
type: Module
title: lib
description: Oxide library DB-flavour server with Actix-Web and optional QUIC/HTTP/3.
resource: crates/oxide-library-server/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:36:00Z"
concept_id: crates/oxide-library-server/src/lib
language: rust
---

# lib

Oxide library DB-flavour server with Actix-Web and optional QUIC/HTTP/3.

## Docstring

Oxide library DB-flavour server with Actix-Web and optional QUIC/HTTP/3.

Exposes a JSON HTTP API over a shared `AppState` (DB pool + lock manager).
Liveness checks (`/health`, `/version`) stay anonymous so process
supervisors don't need credentials. Every other route — the
`/tables` + `/rows` row tier, the primitive
(`/symbols` / `/footprints` / `/sims`) routes, and the advisory
`/rows/:row_id/locks` endpoint — is gated behind a bearer-token
check sourced from the `OXIDE_API_TOKEN` env var.

## Relationships

| Type | Target |
|------|--------|
| related | [health](/crates/oxide-library-server/src/lib/health.md) |
| related | [version](/crates/oxide-library-server/src/lib/version.md) |
| related | [configure_liveness](/crates/oxide-library-server/src/lib/configure_liveness.md) |
| related | [configure_protected](/crates/oxide-library-server/src/lib/configure_protected.md) |
| related | [start_lock_sweeper](/crates/oxide-library-server/src/lib/start_lock_sweeper.md) |
| related | [BearerAuth](/crates/oxide-library-server/src/lib/BearerAuth.md) |
| related | [new](/crates/oxide-library-server/src/lib/new.md) |
| related | [new](/crates/oxide-library-server/src/lib/new.md) |
| related | [new_transform](/crates/oxide-library-server/src/lib/new_transform.md) |
| related | [new_transform](/crates/oxide-library-server/src/lib/new_transform.md) |
| related | [BearerAuthMiddleware](/crates/oxide-library-server/src/lib/BearerAuthMiddleware.md) |
| related | [poll_ready](/crates/oxide-library-server/src/lib/poll_ready.md) |
| related | [call](/crates/oxide-library-server/src/lib/call.md) |
| related | [poll_ready](/crates/oxide-library-server/src/lib/poll_ready.md) |
| related | [call](/crates/oxide-library-server/src/lib/call.md) |
| related | [default_cors](/crates/oxide-library-server/src/lib/default_cors.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
