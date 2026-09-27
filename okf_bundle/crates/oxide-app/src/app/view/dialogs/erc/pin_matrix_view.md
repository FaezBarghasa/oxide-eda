---
okf_version: "0.2"
type: Function
title: pin_matrix_view
description: Pin-connection matrix. Click a cell to cycle Error → Warning →
resource: crates/oxide-app/src/app/view/dialogs/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/erc/pin_matrix_view
language: rust
---

# pin_matrix_view

Pin-connection matrix. Click a cell to cycle Error → Warning →

## Signature

```rust
fn pin_matrix_view(
    tokens: &oxide_types::theme::ThemeTokens,
    overrides: &std::collections::HashMap<(u8, u8), oxide_erc::Severity>,
) -> Element<'static, Message>
```

## Docstring

Pin-connection matrix. Click a cell to cycle Error → Warning →
Info → Off → baseline. Overrides persist via
`write_pin_matrix_overrides`. Currently 6×6 primary pin types;
the full Altium 12-type grid lands when the rule engine learns
the remaining variants (Open Collector, Open Emitter, HiZ, …).

## Source
Lines 252–342 in `crates/oxide-app/src/app/view/dialogs/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/oxide-app/src/app/view/dialogs/erc.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [cell](/crates/oxide-app/tests/command_reference/cell.md) |
