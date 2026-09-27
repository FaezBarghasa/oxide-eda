---
okf_version: "0.2"
type: Function
title: handle_browser_delete_row_confirm
description: Confirm step — actually delete the row through
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_delete_row_confirm_1
language: rust
---

# handle_browser_delete_row_confirm

Confirm step — actually delete the row through

## Signature

```rust
pub(super) fn handle_browser_delete_row_confirm(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Confirm step — actually delete the row through
`adapter.delete_row` and refresh the cache.

## Source
Lines 304–372 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
