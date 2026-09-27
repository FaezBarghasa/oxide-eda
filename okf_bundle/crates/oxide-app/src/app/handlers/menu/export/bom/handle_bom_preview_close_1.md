---
okf_version: "0.2"
type: Function
title: handle_bom_preview_close
resource: crates/oxide-app/src/app/handlers/menu/export/bom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_close_1
language: rust
---

# handle_bom_preview_close

## Signature

```rust
pub(crate) fn handle_bom_preview_close(&mut self) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 246–253 in `crates/oxide-app/src/app/handlers/menu/export/bom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-app/src/app/handlers/menu/export/bom.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
