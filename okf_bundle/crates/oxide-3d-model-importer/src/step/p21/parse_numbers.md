---
okf_version: "0.2"
type: Function
title: parse_numbers
resource: crates/oxide-3d-model-importer/src/step/p21.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/step/p21/parse_numbers
language: rust
---

# parse_numbers

## Signature

```rust
fn parse_numbers(params: &str, line: usize) -> Result<Vec<f32>, ParseError>
```

## Source
Lines 217–251 in `crates/oxide-3d-model-importer/src/step/p21.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [p21](/crates/oxide-3d-model-importer/src/step/p21.md) |
| called_by | [parse_cartesian_point](/crates/oxide-3d-model-importer/src/step/p21/parse_cartesian_point.md) |
