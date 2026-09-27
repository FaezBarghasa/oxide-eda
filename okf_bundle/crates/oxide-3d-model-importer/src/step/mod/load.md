---
okf_version: "0.2"
type: Function
title: load
description: Parse a STEP/P21 source file and convert supported planar faces to meshes.
resource: crates/oxide-3d-model-importer/src/step/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/mod/load
language: rust
---

# load

Parse a STEP/P21 source file and convert supported planar faces to meshes.

## Signature

```rust
pub fn load(path: &PathBuf) -> Result<StepMeshResult, ModelImportError>
```

## Visibility

- `pub`

## Docstring

Parse a STEP/P21 source file and convert supported planar faces to meshes.

## Source
Lines 9–37 in `crates/oxide-3d-model-importer/src/step/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [step](/crates/oxide-3d-model-importer/src/step/mod.md) |
| calls | [parse_to_meshes](/crates/oxide-3d-model-importer/src/step/p21/parse_to_meshes.md) |
