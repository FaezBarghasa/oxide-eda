---
okf_version: "0.2"
type: Function
title: mk_row
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/mk_row
language: rust
---

# mk_row

## Signature

```rust
fn mk_row(internal_pn: &str, class: &str) -> ComponentRow
```

## Source
Lines 558–588 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| called_by | [append_row_grows_the_file](/crates/oxide-library/src/tables/append_row_grows_the_file.md) |
| called_by | [delete_row_missing_returns_not_found](/crates/oxide-library/src/tables/delete_row_missing_returns_not_found.md) |
| called_by | [delete_row_removes_only_matching_id](/crates/oxide-library/src/tables/delete_row_removes_only_matching_id.md) |
| called_by | [none_primitive_refs_encode_empty](/crates/oxide-library/src/tables/none_primitive_refs_encode_empty.md) |
| called_by | [update_row_missing_returns_not_found](/crates/oxide-library/src/tables/update_row_missing_returns_not_found.md) |
| called_by | [update_row_replaces_in_place](/crates/oxide-library/src/tables/update_row_replaces_in_place.md) |
| called_by | [write_table_is_atomic_and_preserves_original_on_failure](/crates/oxide-library/src/tables/write_table_is_atomic_and_preserves_original_on_failure.md) |
