---
okf_version: "0.2"
type: Function
title: parse_cartesian_point
resource: crates/oxide-3d-model-importer/src/step/p21.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/p21/parse_cartesian_point
language: rust
---

# parse_cartesian_point

## Signature

```rust
fn parse_cartesian_point(entity: &Entity) -> Result<[f32; 3], ParseError>
```

## Source
Lines 206–215 in `crates/oxide-3d-model-importer/src/step/p21.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [p21](/crates/oxide-3d-model-importer/src/step/p21.md) |
| calls | [parse_numbers](/crates/oxide-3d-model-importer/src/step/p21/parse_numbers.md) |
| called_by | [parse_to_meshes](/crates/oxide-3d-model-importer/src/step/p21/parse_to_meshes.md) |
