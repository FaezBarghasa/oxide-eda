---
okf_version: "0.2"
type: Function
title: parse_src
resource: crates/oxide-3d-model-importer/src/vrml/parser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/mod/parse_src
language: rust
---

# parse_src

## Signature

```rust
fn parse_src(src: &str) -> Vec<VrmlMesh>
```

## Source
Lines 212–215 in `crates/oxide-3d-model-importer/src/vrml/parser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-3d-model-importer/src/vrml/parser/mod.md) |
| calls | [tokenize](/crates/oxide-3d-model-importer/src/vrml/lexer/tokenize.md) |
| calls | [parse](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse.md) |
| called_by | [parse_diffuse_color](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse_diffuse_color.md) |
| called_by | [parse_empty_is_empty](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse_empty_is_empty.md) |
| called_by | [parse_single_triangle](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse_single_triangle.md) |
| called_by | [parse_transform_applies_translation](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse_transform_applies_translation.md) |
