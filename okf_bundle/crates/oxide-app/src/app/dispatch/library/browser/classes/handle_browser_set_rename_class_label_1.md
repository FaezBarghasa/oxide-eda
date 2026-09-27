---
okf_version: "0.2"
type: Function
title: handle_browser_set_rename_class_label
description: Live-edit of the rename-class label buffer.
resource: crates/oxide-app/src/app/dispatch/library/browser/classes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/classes/handle_browser_set_rename_class_label_1
language: rust
---

# handle_browser_set_rename_class_label

Live-edit of the rename-class label buffer.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_set_rename_class_label(
        &mut self,
        library_path: std::path::PathBuf,
        value: String,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Live-edit of the rename-class label buffer.

## Source
Lines 184–196 in `crates/oxide-app/src/app/dispatch/library/browser/classes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [classes](/crates/oxide-app/src/app/dispatch/library/browser/classes.md) |
