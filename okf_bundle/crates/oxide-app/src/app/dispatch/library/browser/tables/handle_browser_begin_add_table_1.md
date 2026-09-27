---
okf_version: "0.2"
type: Function
title: handle_browser_begin_add_table
description: Flip the browser into add-table mode.
resource: crates/oxide-app/src/app/dispatch/library/browser/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/tables/handle_browser_begin_add_table_1
language: rust
---

# handle_browser_begin_add_table

Flip the browser into add-table mode.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_begin_add_table(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Flip the browser into add-table mode.

## Source
Lines 12–20 in `crates/oxide-app/src/app/dispatch/library/browser/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-app/src/app/dispatch/library/browser/tables.md) |
