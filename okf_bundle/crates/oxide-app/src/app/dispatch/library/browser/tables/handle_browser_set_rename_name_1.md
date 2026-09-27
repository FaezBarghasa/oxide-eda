---
okf_version: "0.2"
type: Function
title: handle_browser_set_rename_name
description: Live-edit of the inline rename buffer.
resource: crates/oxide-app/src/app/dispatch/library/browser/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/tables/handle_browser_set_rename_name_1
language: rust
---

# handle_browser_set_rename_name

Live-edit of the inline rename buffer.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_set_rename_name(
        &mut self,
        library_path: std::path::PathBuf,
        value: String,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Live-edit of the inline rename buffer.

## Source
Lines 112–124 in `crates/oxide-app/src/app/dispatch/library/browser/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-app/src/app/dispatch/library/browser/tables.md) |
