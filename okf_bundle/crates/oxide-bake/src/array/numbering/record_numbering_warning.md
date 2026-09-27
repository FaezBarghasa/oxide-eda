---
okf_version: "0.2"
type: Function
title: record_numbering_warning
description: "Push a numbering warning and mirror it to the log, skipping"
resource: crates/oxide-bake/src/array/numbering.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/array/numbering/record_numbering_warning
language: rust
---

# record_numbering_warning

Push a numbering warning and mirror it to the log, skipping

## Signature

```rust
fn record_numbering_warning(warnings: &mut Vec<String>, message: String)
```

## Docstring

Push a numbering warning and mirror it to the log, skipping
duplicates.

The start / step expressions belong to the array, not to the
individual member, so an unparseable one fails identically for
every member. Deduplicating against the warnings already recorded
for this bake turns what would be one record per array member
into one record per array.

## Source
Lines 102–112 in `crates/oxide-bake/src/array/numbering.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numbering](/crates/oxide-bake/src/array/numbering.md) |
| called_by | [derive_pad_number](/crates/oxide-bake/src/array/numbering/derive_pad_number.md) |
| called_by | [derive_pad_number_2d](/crates/oxide-bake/src/array/numbering/derive_pad_number_2d.md) |
