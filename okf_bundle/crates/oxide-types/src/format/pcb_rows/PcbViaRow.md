---
okf_version: "0.2"
type: Class
title: PcbViaRow
description: "Bulk row for one [`Via`] in the `[vias]` block."
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/PcbViaRow
language: rust
---

# PcbViaRow

Bulk row for one [`Via`] in the `[vias]` block.

## Signature

```rust
pub struct PcbViaRow
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Bulk row for one [`Via`] in the `[vias]` block.
[derive(Debug, Clone, PartialEq)]

## Methods

- `uuid`
- `net`
- `pos_x`
- `pos_y`
- `drill`
- `diameter`
- `layers`
- `via_type`

## Source
Lines 211–220 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
