---
okf_version: "0.2"
type: Function
title: parse_entity_statement
resource: crates/oxide-3d-model-importer/src/step/p21.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/p21/parse_entity_statement
language: rust
---

# parse_entity_statement

## Signature

```rust
fn parse_entity_statement(statement: &str, line: usize) -> Result<(u32, Entity), ParseError>
```

## Source
Lines 166–204 in `crates/oxide-3d-model-importer/src/step/p21.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [p21](/crates/oxide-3d-model-importer/src/step/p21.md) |
| called_by | [parse_entities](/crates/oxide-3d-model-importer/src/step/p21/parse_entities.md) |
