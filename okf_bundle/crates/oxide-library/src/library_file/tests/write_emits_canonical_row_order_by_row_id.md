---
okf_version: "0.2"
type: Function
title: write_emits_canonical_row_order_by_row_id
description: "Canonical row order on write — rows must emit sorted by `row_id`"
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/write_emits_canonical_row_order_by_row_id
language: rust
---

# write_emits_canonical_row_order_by_row_id

Canonical row order on write — rows must emit sorted by `row_id`

## Signature

```rust
fn write_emits_canonical_row_order_by_row_id()
```

## Decorators

- `test`

## Docstring

Canonical row order on write — rows must emit sorted by `row_id`
regardless of in-memory `rows` Vec order, so on-disk file layout
is stable across UI sort changes / bulk insertion order. This is
the load-bearing fix for the "git blame breaks when rows are
reordered" failure mode the architecture critique called out.
[test]

## Source
Lines 421–464 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
| calls | [fixture_manifest](/crates/oxide-library/src/library_file/tests/fixture_manifest.md) |
