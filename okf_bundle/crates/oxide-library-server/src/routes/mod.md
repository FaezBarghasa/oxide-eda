---
okf_version: "0.2"
type: Module
title: routes
description: HTTP route modules — split by resource.
resource: crates/oxide-library-server/src/routes/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library-server/src/routes/mod
language: rust
---

# routes

HTTP route modules — split by resource.

## Docstring

HTTP route modules — split by resource.

The DBLib row tier lives in `routes::rows`; primitives are
`routes::symbols` / `routes::footprints` / `routes::sims`. The
shared `ApiError` envelope lives in `routes::error` so every
module can sit on the same status-code → JSON-body contract.
