---
okf_version: "0.2"
type: Function
title: emit_erc_markers
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/emit_erc_markers
language: rust
---

# emit_erc_markers

## Signature

```rust
pub(super) fn emit_erc_markers(
    snapshot: &SchematicSnapshot,
    theme: &ResolvedTheme,
    scene: &mut Scene,
)
```

## Visibility

- `pub(super)`

## Source
Lines 257–306 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| calls | [erc_style_ref](/crates/oxide-renderer/src/schematic/emit/erc_style_ref.md) |
| calls | [erc_color_from_style](/crates/oxide-renderer/src/schematic/emit/erc_color_from_style.md) |
| calls | [erc_marker_vertices](/crates/oxide-renderer/src/schematic/emit/erc_marker_vertices.md) |
| calls | [erc_marker_line](/crates/oxide-renderer/src/schematic/emit/erc_marker_line.md) |
| called_by | [build_scene](/crates/oxide-renderer/src/schematic/mod/build_scene.md) |
