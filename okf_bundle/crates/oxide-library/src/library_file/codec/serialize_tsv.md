---
okf_version: "0.2"
type: Function
title: serialize_tsv
description: "Serialize a [`LibraryTable`] back to TSV text. Always ends with `\\n`."
resource: crates/oxide-library/src/library_file/codec.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/codec/serialize_tsv
language: rust
---

# serialize_tsv

Serialize a [`LibraryTable`] back to TSV text. Always ends with `\n`.

## Signature

```rust
fn serialize_tsv(table: &LibraryTable) -> String
```

## Docstring

Serialize a [`LibraryTable`] back to TSV text. Always ends with `\n`.

**Canonical row order.** When the table declares a `row_id` column,
rows are emitted sorted by `row_id` ascending — regardless of the
in-memory `rows` Vec order. This keeps the on-disk file layout
stable across UI sort changes, insertion order, and bulk-import
order, so `git blame` on a row line points at the engineer who last
edited *that row's data* rather than whoever last reordered the
catalog.

Tables without a `row_id` column (rare; legitimate for user-defined
lookup-only tables) preserve insertion order — there's no canonical
key to sort by.

## Source
Lines 230–264 in `crates/oxide-library/src/library_file/codec.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codec](/crates/oxide-library/src/library_file/codec.md) |
| called_by | [write](/crates/oxide-library/src/library_file/codec/write.md) |
