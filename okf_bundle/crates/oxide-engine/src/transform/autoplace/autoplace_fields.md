---
okf_version: "0.2"
type: Function
title: autoplace_fields
description: "Pick a free side for `symbol`'s reference / value fields and write"
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/autoplace_fields
language: rust
---

# autoplace_fields

Pick a free side for `symbol`'s reference / value fields and write

## Signature

```rust
pub(super) fn autoplace_fields(
    symbol: &mut oxide_types::schematic::Symbol,
    lib: &oxide_types::schematic::LibSymbol,
    document: &oxide_types::schematic::SchematicSheet,
)
```

## Visibility

- `pub(super)`

## Docstring

Pick a free side for `symbol`'s reference / value fields and write
new positions / justifies / rotation into the field `TextProp`s.

## Source
Lines 57–239 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| calls | [graphic_extent_points](/crates/oxide-engine/src/transform/autoplace/graphic_extent_points.md) |
| calls | [transform_local_point](/crates/oxide-engine/src/transform/autoplace/transform_local_point.md) |
| calls | [anchor_obstacle_count](/crates/oxide-engine/src/transform/autoplace/anchor_obstacle_count.md) |
| called_by | [autoplace_all_marked_fields](/crates/oxide-engine/src/transform/autoplace/autoplace_all_marked_fields.md) |
| called_by | [mirror_selected_item](/crates/oxide-engine/src/transform/mod/mirror_selected_item.md) |
| called_by | [rotate_selected_item](/crates/oxide-engine/src/transform/mod/rotate_selected_item.md) |
