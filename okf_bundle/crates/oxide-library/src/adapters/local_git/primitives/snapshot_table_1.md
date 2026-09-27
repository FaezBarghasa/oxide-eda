---
okf_version: "0.2"
type: Function
title: snapshot_table
description: "Read the table named `table` as a snapshot of [`ComponentRow`]s,"
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/snapshot_table_1
language: rust
---

# snapshot_table

Read the table named `table` as a snapshot of [`ComponentRow`]s,

## Signature

```rust
pub(super) fn snapshot_table(&self, name: &str) -> Result<Vec<ComponentRow>, LibraryError>
```

## Visibility

- `pub(super)`

## Docstring

Read the table named `table` as a snapshot of [`ComponentRow`]s,
scoped to whatever the legacy [`TABLE_HEADER`] columns are.
Returns an empty vec for unknown tables (matches the old
`tables/<name>.tsv` "missing file = empty" semantics).

## Source
Lines 282–300 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
| calls | [validate_legacy_header](/crates/oxide-library/src/adapters/local_git/helpers/validate_legacy_header.md) |
| calls | [library_row_to_component](/crates/oxide-library/src/adapters/local_git/helpers/library_row_to_component.md) |
