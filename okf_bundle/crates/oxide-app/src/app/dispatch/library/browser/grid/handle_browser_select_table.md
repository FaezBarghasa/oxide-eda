---
okf_version: "0.2"
type: Function
title: handle_browser_select_table
description: Active table change inside a Library Browser tab.
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_select_table
language: rust
---

# handle_browser_select_table

Active table change inside a Library Browser tab.

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_select_table(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Active table change inside a Library Browser tab.

## Source
Lines 12–22 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
