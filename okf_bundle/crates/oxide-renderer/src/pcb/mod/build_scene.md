---
okf_version: "0.2"
type: Function
title: build_scene
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod/build_scene
language: rust
---

# build_scene

## Signature

```rust
impl PcbRenderer { fn build_scene(
        snapshot: &Self::Snapshot,
        theme: &ResolvedTheme,
        dirty: DirtyFlags,
        scene: &mut Scene,
    ) }
```

## Source
Lines 235–258 in `crates/oxide-renderer/src/pcb/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-renderer/src/pcb/mod.md) |
| calls | [emit_traces](/crates/oxide-renderer/src/pcb/emit/emit_traces.md) |
| calls | [emit_vias](/crates/oxide-renderer/src/pcb/emit/emit_vias.md) |
| calls | [emit_static_polygons](/crates/oxide-renderer/src/pcb/emit/emit_static_polygons.md) |
