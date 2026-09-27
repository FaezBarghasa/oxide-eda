---
okf_version: "0.2"
type: Module
title: distributor_apis
description: Settings → Library → Distributor APIs panel.
resource: crates/oxide-app/src/library/settings/distributor_apis.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/distributor_apis
language: rust
---

# distributor_apis

Settings → Library → Distributor APIs panel.

## Docstring

Settings → Library → Distributor APIs panel.

Spec (v0.9-library-plan.md §14a.2):

- DigiKey: OAuth2 PKCE — Phase 1 stubs the connect button.
- Mouser: API key in keyring — Phase 1 takes the user's key in
memory + a Test button.
- LCSC, JLCPCB: no key required.
- Order-of-preference list — Phase 1 holds it in memory only.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/settings/distributor_apis/view.md) |
| related | [distributor_label](/crates/oxide-app/src/library/settings/distributor_apis/distributor_label.md) |
| related | [divider](/crates/oxide-app/src/library/settings/distributor_apis/divider.md) |
| related | [primary_btn](/crates/oxide-app/src/library/settings/distributor_apis/primary_btn.md) |
| related | [secondary_btn](/crates/oxide-app/src/library/settings/distributor_apis/secondary_btn.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
