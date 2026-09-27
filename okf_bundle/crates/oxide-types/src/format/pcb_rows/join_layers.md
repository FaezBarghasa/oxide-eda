---
okf_version: "0.2"
type: Function
title: join_layers
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/join_layers
language: rust
---

# join_layers

## Signature

```rust
fn join_layers(layers: &[String]) -> String
```

## Source
Lines 325–330 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
| called_by | [pad_to_row](/crates/oxide-types/src/format/pcb_rows/pad_to_row.md) |
| called_by | [via_to_row](/crates/oxide-types/src/format/pcb_rows/via_to_row.md) |
