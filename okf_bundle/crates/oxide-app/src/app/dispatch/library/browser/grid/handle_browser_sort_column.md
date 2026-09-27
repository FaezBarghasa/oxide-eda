---
okf_version: "0.2"
type: Function
title: handle_browser_sort_column
description: Column-header click — toggles sort direction on the matching key.
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_sort_column
language: rust
---

# handle_browser_sort_column

Column-header click — toggles sort direction on the matching key.

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_sort_column(
        &mut self,
        library_path: std::path::PathBuf,
        column_key: String,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Column-header click — toggles sort direction on the matching key.

## Source
Lines 42–51 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
