---
okf_version: "0.2"
type: Function
title: layers_to_token
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/layers_to_token
language: rust
---

# layers_to_token

## Signature

```rust
fn layers_to_token(layers: &[LayerId]) -> Result<String, FootprintFileError>
```

## Source
Lines 144–158 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| called_by | [pad_to_tsv_row](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_to_tsv_row.md) |
