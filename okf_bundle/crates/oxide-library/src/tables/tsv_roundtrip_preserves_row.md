---
okf_version: "0.2"
type: Function
title: tsv_roundtrip_preserves_row
description: Plan §6 step 1.3 — the foundational TSV round-trip.
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/tsv_roundtrip_preserves_row
language: rust
---

# tsv_roundtrip_preserves_row

Plan §6 step 1.3 — the foundational TSV round-trip.

## Signature

```rust
fn tsv_roundtrip_preserves_row()
```

## Decorators

- `test`

## Docstring

Plan §6 step 1.3 — the foundational TSV round-trip.
[test]

## Source
Lines 592–598 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
