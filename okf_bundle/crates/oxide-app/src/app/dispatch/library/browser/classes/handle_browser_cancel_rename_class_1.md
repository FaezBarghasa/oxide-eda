---
okf_version: "0.2"
type: Function
title: handle_browser_cancel_rename_class
description: Cancel the inline rename-class form.
resource: crates/oxide-app/src/app/dispatch/library/browser/classes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/classes/handle_browser_cancel_rename_class_1
language: rust
---

# handle_browser_cancel_rename_class

Cancel the inline rename-class form.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_cancel_rename_class(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Cancel the inline rename-class form.

## Source
Lines 199–208 in `crates/oxide-app/src/app/dispatch/library/browser/classes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [classes](/crates/oxide-app/src/app/dispatch/library/browser/classes.md) |
