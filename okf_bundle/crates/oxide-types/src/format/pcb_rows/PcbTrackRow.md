---
okf_version: "0.2"
type: Class
title: PcbTrackRow
description: "Bulk row for one [`Segment`] in the `[tracks]` block."
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/PcbTrackRow
language: rust
---

# PcbTrackRow

Bulk row for one [`Segment`] in the `[tracks]` block.

## Signature

```rust
pub struct PcbTrackRow
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Bulk row for one [`Segment`] in the `[tracks]` block.
[derive(Debug, Clone, PartialEq)]

## Methods

- `uuid`
- `net`
- `layer`
- `width`
- `start_x`
- `start_y`
- `end_x`
- `end_y`

## Source
Lines 157–166 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
