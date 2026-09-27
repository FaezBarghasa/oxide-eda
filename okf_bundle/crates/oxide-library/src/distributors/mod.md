---
okf_version: "0.2"
type: Module
title: distributors
description: "Community distributor adapters (DigiKey, Mouser, LCSC, JLCPCB)."
resource: crates/oxide-library/src/distributors/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/distributors/mod
language: rust
---

# distributors

Community distributor adapters (DigiKey, Mouser, LCSC, JLCPCB).

## Docstring

Community distributor adapters (DigiKey, Mouser, LCSC, JLCPCB).

Gated behind the `distributors-community` Cargo feature so the core
library crate stays free of `reqwest`/`oauth2`/`keyring` when consumers
don't need vendor lookups (e.g. CI builds of `oxide-app` that ship
without distributor integrations).

See `docs/internal/docs/v0.9-library-plan.md` → §14a Distributor Data Sources.

## Relationships

| Type | Target |
|------|--------|
| related | [keyring](/_dependencies/cargo/keyring.md) |
