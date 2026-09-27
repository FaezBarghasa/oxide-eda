---
okf_version: "0.2"
type: Function
title: component_to_library_row
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/component_to_library_row
language: rust
---

# component_to_library_row

## Signature

```rust
pub(super) fn component_to_library_row(row: &ComponentRow) -> Result<LibraryRow, LibraryError>
```

## Visibility

- `pub(super)`

## Source
Lines 105–112 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| calls | [row_to_record](/crates/oxide-library/src/tables/row_to_record.md) |
| called_by | [insert_row](/crates/oxide-library/src/adapters/local_git/adapter/insert_row.md) |
| called_by | [update_row](/crates/oxide-library/src/adapters/local_git/adapter/update_row.md) |
