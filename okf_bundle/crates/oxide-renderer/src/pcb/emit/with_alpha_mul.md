---
okf_version: "0.2"
type: Function
title: with_alpha_mul
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/with_alpha_mul
language: rust
---

# with_alpha_mul

## Signature

```rust
pub(super) fn with_alpha_mul(mut color: [f32; 4], alpha_mul: f32) -> [f32; 4]
```

## Visibility

- `pub(super)`

## Source
Lines 187–190 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [emit_overlays](/crates/oxide-renderer/src/pcb/emit/emit_overlays.md) |
| called_by | [emit_static_polygons](/crates/oxide-renderer/src/pcb/emit/emit_static_polygons.md) |
