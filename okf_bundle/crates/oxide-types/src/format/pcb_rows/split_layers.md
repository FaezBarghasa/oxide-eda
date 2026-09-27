---
okf_version: "0.2"
type: Function
title: split_layers
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/split_layers
language: rust
---

# split_layers

## Signature

```rust
fn split_layers(s: &str) -> Vec<String>
```

## Source
Lines 332–337 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
| called_by | [row_to_pad](/crates/oxide-types/src/format/pcb_rows/row_to_pad.md) |
| called_by | [row_to_via](/crates/oxide-types/src/format/pcb_rows/row_to_via.md) |
