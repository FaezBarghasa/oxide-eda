---
okf_version: "0.2"
type: Function
title: handle_browser_cell_commit
description: Commit a per-cell inline edit to the row. Re-hashes + persists.
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_cell_commit
language: rust
---

# handle_browser_cell_commit

Commit a per-cell inline edit to the row. Re-hashes + persists.

## Signature

```rust
impl Oxide { pub(super) fn handle_browser_cell_commit(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
        row_id: RowId,
        column: String,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Commit a per-cell inline edit to the row. Re-hashes + persists.

## Source
Lines 605–744 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [param_value_for_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/param_value_for_commit.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |
