---
okf_version: "0.2"
type: Module
title: error
description: "Shared `ApiError` envelope reused by every route module."
resource: crates/oxide-library-server/src/routes/error.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:46:19Z"
concept_id: crates/oxide-library-server/src/routes/error
language: rust
---

# error

Shared `ApiError` envelope reused by every route module.

## Docstring

Shared `ApiError` envelope reused by every route module.

Sits in its own module so the `tables` / `rows` row tier and the
`symbols` / `footprints` / `sims` primitive routes can share one
status-code → JSON-body contract.

## Relationships

| Type | Target |
|------|--------|
| related | [ApiError](/crates/oxide-library-server/src/routes/error/ApiError.md) |
| related | [fmt](/crates/oxide-library-server/src/routes/error/fmt.md) |
| related | [fmt](/crates/oxide-library-server/src/routes/error/fmt.md) |
| related | [not_found](/crates/oxide-library-server/src/routes/error/not_found.md) |
| related | [bad_request](/crates/oxide-library-server/src/routes/error/bad_request.md) |
| related | [conflict](/crates/oxide-library-server/src/routes/error/conflict.md) |
| related | [unauthorized](/crates/oxide-library-server/src/routes/error/unauthorized.md) |
| related | [internal](/crates/oxide-library-server/src/routes/error/internal.md) |
| related | [not_found](/crates/oxide-library-server/src/routes/error/not_found.md) |
| related | [bad_request](/crates/oxide-library-server/src/routes/error/bad_request.md) |
| related | [conflict](/crates/oxide-library-server/src/routes/error/conflict.md) |
| related | [unauthorized](/crates/oxide-library-server/src/routes/error/unauthorized.md) |
| related | [internal](/crates/oxide-library-server/src/routes/error/internal.md) |
| related | [from](/crates/oxide-library-server/src/routes/error/from.md) |
| related | [from](/crates/oxide-library-server/src/routes/error/from.md) |
| related | [status_code](/crates/oxide-library-server/src/routes/error/status_code.md) |
| related | [error_response](/crates/oxide-library-server/src/routes/error/error_response.md) |
| related | [status_code](/crates/oxide-library-server/src/routes/error/status_code.md) |
| related | [error_response](/crates/oxide-library-server/src/routes/error/error_response.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
