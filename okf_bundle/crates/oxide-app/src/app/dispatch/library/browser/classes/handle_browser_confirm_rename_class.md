---
okf_version: "0.2"
type: Function
title: handle_browser_confirm_rename_class
description: "Confirm — writes the renamed class via `rename_library_class`."
resource: crates/oxide-app/src/app/dispatch/library/browser/classes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/classes/handle_browser_confirm_rename_class
language: rust
---

# handle_browser_confirm_rename_class

Confirm — writes the renamed class via `rename_library_class`.

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_confirm_rename_class(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Confirm — writes the renamed class via `rename_library_class`.

## Source
Lines 211–261 in `crates/oxide-app/src/app/dispatch/library/browser/classes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [classes](/crates/oxide-app/src/app/dispatch/library/browser/classes.md) |
