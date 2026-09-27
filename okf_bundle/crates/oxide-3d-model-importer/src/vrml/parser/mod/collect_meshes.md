---
okf_version: "0.2"
type: Function
title: collect_meshes
description: ─── Mesh collection ──────────────────────────────────────────────────────────
resource: crates/oxide-3d-model-importer/src/vrml/parser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/mod/collect_meshes
language: rust
---

# collect_meshes

─── Mesh collection ──────────────────────────────────────────────────────────

## Signature

```rust
fn collect_meshes(node: &Node, parent_transform: &Transform, out: &mut Vec<VrmlMesh>)
```

## Docstring

─── Mesh collection ──────────────────────────────────────────────────────────

## Source
Lines 118–147 in `crates/oxide-3d-model-importer/src/vrml/parser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-3d-model-importer/src/vrml/parser/mod.md) |
| calls | [build_mesh](/crates/oxide-3d-model-importer/src/vrml/parser/mod/build_mesh.md) |
| called_by | [parse](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse.md) |
