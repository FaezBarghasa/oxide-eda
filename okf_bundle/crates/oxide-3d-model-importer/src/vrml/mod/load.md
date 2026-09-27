---
okf_version: "0.2"
type: Function
title: load
description: Parse a VRML97 source file and return the flat mesh list.
resource: crates/oxide-3d-model-importer/src/vrml/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/mod/load
language: rust
---

# load

Parse a VRML97 source file and return the flat mesh list.

## Signature

```rust
pub fn load(path: &PathBuf) -> Result<Vec<VrmlMesh>, ModelImportError>
```

## Visibility

- `pub`

## Docstring

Parse a VRML97 source file and return the flat mesh list.

## Source
Lines 10–31 in `crates/oxide-3d-model-importer/src/vrml/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vrml](/crates/oxide-3d-model-importer/src/vrml/mod.md) |
| calls | [tokenize](/crates/oxide-3d-model-importer/src/vrml/lexer/tokenize.md) |
| calls | [parse](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse.md) |
