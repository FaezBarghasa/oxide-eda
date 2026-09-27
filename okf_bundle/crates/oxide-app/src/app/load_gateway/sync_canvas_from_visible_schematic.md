---
okf_version: "0.2"
type: Function
title: sync_canvas_from_visible_schematic
resource: crates/oxide-app/src/app/load_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/load_gateway/sync_canvas_from_visible_schematic
language: rust
---

# sync_canvas_from_visible_schematic

## Signature

```rust
impl Oxide { pub(crate) fn sync_canvas_from_visible_schematic(
        &mut self,
        invalidation: crate::schematic_runtime::RenderInvalidation,
    ) }
```

## Visibility

- `pub(crate)`

## Source
Lines 163–194 in `crates/oxide-app/src/app/load_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [load_gateway](/crates/oxide-app/src/app/load_gateway.md) |
