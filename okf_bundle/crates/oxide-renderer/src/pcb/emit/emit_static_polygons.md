---
okf_version: "0.2"
type: Function
title: emit_static_polygons
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/emit_static_polygons
language: rust
---

# emit_static_polygons

## Signature

```rust
pub(super) fn emit_static_polygons(
    snapshot: &PcbSnapshot,
    theme: &ResolvedTheme,
    scene: &mut Scene,
)
```

## Visibility

- `pub(super)`

## Source
Lines 229–276 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [with_alpha_mul](/crates/oxide-renderer/src/pcb/emit/with_alpha_mul.md) |
| calls | [pad_alpha_mul](/crates/oxide-renderer/src/pcb/emit/pad_alpha_mul.md) |
| calls | [pad_vertices](/crates/oxide-renderer/src/pcb/emit/pad_vertices.md) |
| called_by | [build_scene](/crates/oxide-renderer/src/pcb/mod/build_scene.md) |
