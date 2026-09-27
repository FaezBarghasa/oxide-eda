---
okf_version: "0.2"
type: Function
title: pad_from_tsv_row
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/pad_from_tsv_row
language: rust
---

# pad_from_tsv_row

## Signature

```rust
fn pad_from_tsv_row(cells: &[&str]) -> Result<Pad, FootprintFileError>
```

## Source
Lines 273–310 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| calls | [parse_f64_cell_fp](/crates/oxide-library/src/primitive/footprint/serde_tsv/parse_f64_cell_fp.md) |
| calls | [parse_opt_f64_cell_fp](/crates/oxide-library/src/primitive/footprint/serde_tsv/parse_opt_f64_cell_fp.md) |
| calls | [pad_kind_from_token](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_kind_from_token.md) |
| calls | [pad_shape_from_token](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_shape_from_token.md) |
| calls | [layers_from_token](/crates/oxide-library/src/primitive/footprint/serde_tsv/layers_from_token.md) |
| called_by | [pads_from_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv/pads_from_tsv.md) |
