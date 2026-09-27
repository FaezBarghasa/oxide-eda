---
okf_version: "0.2"
type: Class
title: SketchEntityId
description: "[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]"
resource: crates/oxide-sketch/src/id.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/id/SketchEntityId
language: rust
---

# SketchEntityId

[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Signature

```rust
pub struct SketchEntityId
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 6–6 in `crates/oxide-sketch/src/id.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [id](/crates/oxide-sketch/src/id.md) |
| called_by | [mirror_delete_pad_drops_constraints_on_the_whole_entity_set](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_constraints_on_the_whole_entity_set.md) |
| called_by | [sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sidecar.md) |
| called_by | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
| called_by | [place_round_rect_then_select_arc_unlink_then_undo_restores_link](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_select_arc_unlink_then_undo_restores_link.md) |
| called_by | [unlink_one_corner_only_that_arc_reads_per_corner_param](/crates/oxide-app/tests/regression/library_cross_track/unlink_one_corner_only_that_arc_reads_per_corner_param.md) |
| called_by | [editing_chamfer_len_propagates_through_solve](/crates/oxide-app/tests/regression/library_pad_geometry/editing_chamfer_len_propagates_through_solve.md) |
| called_by | [unlink_corner_radius_mints_per_corner_param](/crates/oxide-app/tests/regression/library_pad_geometry/unlink_corner_radius_mints_per_corner_param.md) |
| called_by | [entity_id_round_trip](/crates/oxide-sketch/tests/round_trip/entity_id_round_trip.md) |
