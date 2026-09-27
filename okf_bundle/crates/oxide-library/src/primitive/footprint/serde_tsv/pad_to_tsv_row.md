---
okf_version: "0.2"
type: Function
title: pad_to_tsv_row
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/pad_to_tsv_row
language: rust
---

# pad_to_tsv_row

## Signature

```rust
fn pad_to_tsv_row(pad: &Pad) -> Result<String, FootprintFileError>
```

## Source
Lines 184–222 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| calls | [pad_shape_to_token](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_shape_to_token.md) |
| calls | [layers_to_token](/crates/oxide-library/src/primitive/footprint/serde_tsv/layers_to_token.md) |
| calls | [fmt_f64_fp](/crates/oxide-library/src/primitive/footprint/serde_tsv/fmt_f64_fp.md) |
| calls | [pad_kind_token](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_kind_token.md) |
| calls | [fmt_opt_f64_fp](/crates/oxide-library/src/primitive/footprint/serde_tsv/fmt_opt_f64_fp.md) |
| called_by | [pads_to_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv/pads_to_tsv.md) |
