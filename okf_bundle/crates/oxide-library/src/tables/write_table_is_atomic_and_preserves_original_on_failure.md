---
okf_version: "0.2"
type: Function
title: write_table_is_atomic_and_preserves_original_on_failure
description: "`write_table` must go through `atomic_write`, not `fs::write`:"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/write_table_is_atomic_and_preserves_original_on_failure
language: rust
---

# write_table_is_atomic_and_preserves_original_on_failure

`write_table` must go through `atomic_write`, not `fs::write`:

## Signature

```rust
fn write_table_is_atomic_and_preserves_original_on_failure()
```

## Decorators

- `test`

## Docstring

`write_table` must go through `atomic_write`, not `fs::write`:
a failed save leaves the previous table fully intact.

Discriminator: denying new-file creation in the destination's
parent directory makes `atomic_write`'s `File::create(&tmp)` fail
before it can touch the destination, regardless of the unique
per-writer temp name it picks (#416). A plain `fs::write` would
ignore that and clobber the old rows — so this test fails on a
revert.
[test]

## Source
Lines 634–645 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| calls | [mk_row](/crates/oxide-library/src/tables/mk_row.md) |
