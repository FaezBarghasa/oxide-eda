---
okf_version: "0.2"
type: Function
title: build_scene_with_default_theme
resource: crates/oxide-renderer/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/schematic/mod/build_scene_with_default_theme
language: rust
---

# build_scene_with_default_theme

## Signature

```rust
fn build_scene_with_default_theme(
        snapshot: &SchematicSnapshot,
        dirty: DirtyFlags,
        scene: &mut Scene,
    )
```

## Source
Lines 268–275 in `crates/oxide-renderer/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-renderer/src/schematic/mod.md) |
| calls | [default_theme](/crates/oxide-renderer/src/schematic/mod/default_theme.md) |
| called_by | [arc_emitter_preserves_wraparound_and_tiny_radius_inputs](/crates/oxide-renderer/src/schematic/mod/arc_emitter_preserves_wraparound_and_tiny_radius_inputs.md) |
| called_by | [build_scene_updates_requested_primitive_groups_only](/crates/oxide-renderer/src/schematic/mod/build_scene_updates_requested_primitive_groups_only.md) |
| called_by | [erc_marker_emitter_handles_dense_marker_cluster](/crates/oxide-renderer/src/schematic/mod/erc_marker_emitter_handles_dense_marker_cluster.md) |
| called_by | [overlay_emitter_maps_preview_ghost_lasso_snap](/crates/oxide-renderer/src/schematic/mod/overlay_emitter_maps_preview_ghost_lasso_snap.md) |
| called_by | [polygon_text_emitters_preserve_mapped_fields](/crates/oxide-renderer/src/schematic/mod/polygon_text_emitters_preserve_mapped_fields.md) |
| called_by | [text_emitter_covers_all_text_categories](/crates/oxide-renderer/src/schematic/mod/text_emitter_covers_all_text_categories.md) |
| called_by | [text_emitter_maps_alignment_variants](/crates/oxide-renderer/src/schematic/mod/text_emitter_maps_alignment_variants.md) |
