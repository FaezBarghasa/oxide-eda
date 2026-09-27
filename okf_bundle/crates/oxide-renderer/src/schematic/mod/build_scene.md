---
okf_version: "0.2"
type: Function
title: build_scene
resource: crates/oxide-renderer/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/schematic/mod/build_scene
language: rust
---

# build_scene

## Signature

```rust
impl SchematicRenderer { fn build_scene(
        snapshot: &Self::Snapshot,
        theme: &ResolvedTheme,
        dirty: DirtyFlags,
        scene: &mut Scene,
    ) }
```

## Source
Lines 142–174 in `crates/oxide-renderer/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-renderer/src/schematic/mod.md) |
| calls | [emit_wires](/crates/oxide-renderer/src/schematic/emit/emit_wires.md) |
| calls | [emit_junctions](/crates/oxide-renderer/src/schematic/emit/emit_junctions.md) |
| calls | [emit_arcs](/crates/oxide-renderer/src/schematic/emit/emit_arcs.md) |
| calls | [emit_polygons](/crates/oxide-renderer/src/schematic/emit/emit_polygons.md) |
| calls | [emit_texts](/crates/oxide-renderer/src/schematic/emit/emit_texts.md) |
| calls | [emit_erc_markers](/crates/oxide-renderer/src/schematic/emit/emit_erc_markers.md) |
