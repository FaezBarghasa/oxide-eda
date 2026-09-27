---
okf_version: "0.2"
type: Function
title: handle_browser_dismiss_delete_error
description: Dismiss the inline delete-table error message.
resource: crates/oxide-app/src/app/dispatch/library/browser/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/tables/handle_browser_dismiss_delete_error
language: rust
---

# handle_browser_dismiss_delete_error

Dismiss the inline delete-table error message.

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_dismiss_delete_error(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Dismiss the inline delete-table error message.

## Source
Lines 88–96 in `crates/oxide-app/src/app/dispatch/library/browser/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-app/src/app/dispatch/library/browser/tables.md) |
