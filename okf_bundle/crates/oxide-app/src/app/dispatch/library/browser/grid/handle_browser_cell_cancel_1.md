---
okf_version: "0.2"
type: Function
title: handle_browser_cell_cancel
description: Drop the per-cell edit buffer (Esc).
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_cell_cancel_1
language: rust
---

# handle_browser_cell_cancel

Drop the per-cell edit buffer (Esc).

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_cell_cancel(
        &mut self,
        library_path: std::path::PathBuf,
        row_id: RowId,
        column: String,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Drop the per-cell edit buffer (Esc).

## Source
Lines 97–107 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
