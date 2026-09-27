---
okf_version: "0.2"
type: Function
title: handle_browser_cell_edit
description: Live edit of a cell in the browser grid — updates the per-cell
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_cell_edit_1
language: rust
---

# handle_browser_cell_edit

Live edit of a cell in the browser grid — updates the per-cell

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_cell_edit(
        &mut self,
        library_path: std::path::PathBuf,
        row_id: RowId,
        column: String,
        value: String,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Live edit of a cell in the browser grid — updates the per-cell
edit buffer.

## Source
Lines 83–94 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
