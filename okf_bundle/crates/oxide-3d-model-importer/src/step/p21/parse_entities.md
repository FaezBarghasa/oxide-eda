---
okf_version: "0.2"
type: Function
title: parse_entities
resource: crates/oxide-3d-model-importer/src/step/p21.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/p21/parse_entities
language: rust
---

# parse_entities

## Signature

```rust
fn parse_entities(data: &str) -> Result<HashMap<u32, Entity>, ParseError>
```

## Source
Lines 146–164 in `crates/oxide-3d-model-importer/src/step/p21.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [p21](/crates/oxide-3d-model-importer/src/step/p21.md) |
| calls | [parse_entity_statement](/crates/oxide-3d-model-importer/src/step/p21/parse_entity_statement.md) |
| called_by | [parse_to_meshes](/crates/oxide-3d-model-importer/src/step/p21/parse_to_meshes.md) |
