---
okf_version: "0.2"
type: Function
title: parse_to_meshes
description: Parse a STEP ISO-10303-21 DATA section into tessellated triangle meshes.
resource: crates/oxide-3d-model-importer/src/step/p21.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/p21/parse_to_meshes
language: rust
---

# parse_to_meshes

Parse a STEP ISO-10303-21 DATA section into tessellated triangle meshes.

## Signature

```rust
pub fn parse_to_meshes(source: &str) -> Result<StepMeshResult, ParseError>
```

## Visibility

- `pub`

## Docstring

Parse a STEP ISO-10303-21 DATA section into tessellated triangle meshes.

Supported paths:
- `ADVANCED_FACE` + `FACE_OUTER_BOUND` + `POLY_LOOP` + `VERTEX_POINT`
- `ADVANCED_FACE` + `FACE_BOUND` + `EDGE_LOOP` + `ORIENTED_EDGE` + `EDGE_CURVE`

## Source
Lines 35–136 in `crates/oxide-3d-model-importer/src/step/p21.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [p21](/crates/oxide-3d-model-importer/src/step/p21.md) |
| calls | [extract_data_section](/crates/oxide-3d-model-importer/src/step/p21/extract_data_section.md) |
| calls | [parse_entities](/crates/oxide-3d-model-importer/src/step/p21/parse_entities.md) |
| calls | [parse_cartesian_point](/crates/oxide-3d-model-importer/src/step/p21/parse_cartesian_point.md) |
| calls | [first_ref](/crates/oxide-3d-model-importer/src/step/p21/first_ref.md) |
| calls | [parse_refs](/crates/oxide-3d-model-importer/src/step/p21/parse_refs.md) |
| calls | [resolve_poly_loop_points](/crates/oxide-3d-model-importer/src/step/p21/resolve_poly_loop_points.md) |
| calls | [resolve_edge_loop_points](/crates/oxide-3d-model-importer/src/step/p21/resolve_edge_loop_points.md) |
| calls | [triangulate_polygon](/crates/oxide-3d-model-importer/src/step/p21/triangulate_polygon.md) |
| called_by | [load](/crates/oxide-3d-model-importer/src/step/mod/load.md) |
| called_by | [parse_minimal_poly_loop_face](/crates/oxide-3d-model-importer/src/step/p21/parse_minimal_poly_loop_face.md) |
| called_by | [parse_missing_data_section_fails](/crates/oxide-3d-model-importer/src/step/p21/parse_missing_data_section_fails.md) |
