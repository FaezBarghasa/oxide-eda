---
okf_version: "0.2"
type: Function
title: handle_browser_cancel_rename_table
description: Cancel the inline rename without writing.
resource: crates/oxide-app/src/app/dispatch/library/browser/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/tables/handle_browser_cancel_rename_table
language: rust
---

# handle_browser_cancel_rename_table

Cancel the inline rename without writing.

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_cancel_rename_table(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Cancel the inline rename without writing.

## Source
Lines 127–136 in `crates/oxide-app/src/app/dispatch/library/browser/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-app/src/app/dispatch/library/browser/tables.md) |
