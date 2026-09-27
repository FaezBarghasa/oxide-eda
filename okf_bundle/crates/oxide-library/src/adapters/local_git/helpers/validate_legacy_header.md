---
okf_version: "0.2"
type: Function
title: validate_legacy_header
description: "Verify a `[tables.<name>]` block's column order matches the"
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/validate_legacy_header
language: rust
---

# validate_legacy_header

Verify a `[tables.<name>]` block's column order matches the

## Signature

```rust
pub(super) fn validate_legacy_header(table: &str, columns: &[String]) -> Result<(), LibraryError>
```

## Visibility

- `pub(super)`

## Docstring

Verify a `[tables.<name>]` block's column order matches the
legacy schema. Mismatches are loud — a hand-edited `.snxlib`
where someone reordered or renamed columns would silently
miscolumn data otherwise.

## Source
Lines 87–103 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [insert_row](/crates/oxide-library/src/adapters/local_git/adapter/insert_row.md) |
| called_by | [update_row](/crates/oxide-library/src/adapters/local_git/adapter/update_row.md) |
| called_by | [snapshot_table](/crates/oxide-library/src/adapters/local_git/primitives/snapshot_table.md) |
