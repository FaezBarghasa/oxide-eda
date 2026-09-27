---
okf_version: "0.2"
type: Function
title: handle_browser_begin_rename_class
description: "Sidebar `✎` rename for a class row — flips it into edit mode."
resource: crates/oxide-app/src/app/dispatch/library/browser/classes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/classes/handle_browser_begin_rename_class
language: rust
---

# handle_browser_begin_rename_class

Sidebar `✎` rename for a class row — flips it into edit mode.

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_begin_rename_class(
        &mut self,
        library_path: std::path::PathBuf,
        key: String,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Sidebar `✎` rename for a class row — flips it into edit mode.

## Source
Lines 145–166 in `crates/oxide-app/src/app/dispatch/library/browser/classes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [classes](/crates/oxide-app/src/app/dispatch/library/browser/classes.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
