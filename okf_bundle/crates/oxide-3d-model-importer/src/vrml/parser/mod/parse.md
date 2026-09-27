---
okf_version: "0.2"
type: Function
title: parse
description: Parse a VRML97 source into a flat list of meshes.
resource: crates/oxide-3d-model-importer/src/vrml/parser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/mod/parse
language: rust
---

# parse

Parse a VRML97 source into a flat list of meshes.

## Signature

```rust
pub fn parse(
    tokens: &[super::lexer::Token],
    line_offsets: &[usize],
) -> Result<Vec<VrmlMesh>, ParseError>
```

## Visibility

- `pub`

## Docstring

Parse a VRML97 source into a flat list of meshes.

# Errors

Returns `(meshes, warnings)` on success; panics are converted to
`VrmlParseError` by the caller layer in `vrml/mod.rs`.

## Source
Lines 41–54 in `crates/oxide-3d-model-importer/src/vrml/parser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-3d-model-importer/src/vrml/parser/mod.md) |
| calls | [collect_meshes](/crates/oxide-3d-model-importer/src/vrml/parser/mod/collect_meshes.md) |
| called_by | [load](/crates/oxide-3d-model-importer/src/vrml/mod/load.md) |
| called_by | [parse_src](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse_src.md) |
