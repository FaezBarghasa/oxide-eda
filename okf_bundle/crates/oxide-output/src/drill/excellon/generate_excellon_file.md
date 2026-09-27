---
okf_version: "0.2"
type: Function
title: generate_excellon_file
resource: crates/oxide-output/src/drill/excellon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:33:05Z"
concept_id: crates/oxide-output/src/drill/excellon/generate_excellon_file
language: rust
---

# generate_excellon_file

## Signature

```rust
impl ExcellonExporter { fn generate_excellon_file(
        &self,
        holes: &[DrillHole],
        is_plated: bool,
    ) -> Result<String, DrillError> }
```

## Source
Lines 144–197 in `crates/oxide-output/src/drill/excellon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [excellon](/crates/oxide-output/src/drill/excellon.md) |
