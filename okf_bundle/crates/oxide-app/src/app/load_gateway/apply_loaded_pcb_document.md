---
okf_version: "0.2"
type: Function
title: apply_loaded_pcb_document
resource: crates/oxide-app/src/app/load_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/load_gateway/apply_loaded_pcb_document
language: rust
---

# apply_loaded_pcb_document

## Signature

```rust
impl Oxide { pub(crate) fn apply_loaded_pcb_document(
        &mut self,
        fit_to_board: bool,
        refresh_panel_ctx: bool,
    ) }
```

## Visibility

- `pub(crate)`

## Source
Lines 339–357 in `crates/oxide-app/src/app/load_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [load_gateway](/crates/oxide-app/src/app/load_gateway.md) |
