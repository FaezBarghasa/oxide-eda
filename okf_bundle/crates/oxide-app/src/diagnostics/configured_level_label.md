---
okf_version: "0.2"
type: Function
title: configured_level_label
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/configured_level_label
language: rust
---

# configured_level_label

## Signature

```rust
pub fn configured_level_label() -> &'static str
```

## Visibility

- `pub`

## Source
Lines 93–102 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [configured_level](/crates/oxide-app/src/diagnostics/configured_level.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [sync_diagnostics_panel_ctx](/crates/oxide-app/src/app/runtime/mod/sync_diagnostics_panel_ctx.md) |
| called_by | [refresh_panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx.md) |
