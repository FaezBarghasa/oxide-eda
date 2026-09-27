---
okf_version: "0.2"
type: Function
title: handle_browser_delete_row_cancel
description: User dismissed the delete confirm modal without deleting.
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_delete_row_cancel_1
language: rust
---

# handle_browser_delete_row_cancel

User dismissed the delete confirm modal without deleting.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_delete_row_cancel(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

User dismissed the delete confirm modal without deleting.

## Source
Lines 71–79 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
