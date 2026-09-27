---
okf_version: "0.2"
type: Function
title: handle_browser_open_edit_modal
description: Open the Edit Component Details modal for a row. Loads the row
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_open_edit_modal
language: rust
---

# handle_browser_open_edit_modal

Open the Edit Component Details modal for a row. Loads the row

## Signature

```rust
impl Oxide { pub(super) fn handle_browser_open_edit_modal(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Open the Edit Component Details modal for a row. Loads the row
from the library cache and seeds the modal with a working copy.

## Source
Lines 376–403 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
