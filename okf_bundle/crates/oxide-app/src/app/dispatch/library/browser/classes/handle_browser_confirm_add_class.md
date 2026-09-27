---
okf_version: "0.2"
type: Function
title: handle_browser_confirm_add_class
description: "Append the new class to the library's `[[classes]]` block via"
resource: crates/oxide-app/src/app/dispatch/library/browser/classes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/classes/handle_browser_confirm_add_class
language: rust
---

# handle_browser_confirm_add_class

Append the new class to the library's `[[classes]]` block via

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_confirm_add_class(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Append the new class to the library's `[[classes]]` block via
`add_library_class` and refresh.

## Source
Lines 66–119 in `crates/oxide-app/src/app/dispatch/library/browser/classes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [classes](/crates/oxide-app/src/app/dispatch/library/browser/classes.md) |
