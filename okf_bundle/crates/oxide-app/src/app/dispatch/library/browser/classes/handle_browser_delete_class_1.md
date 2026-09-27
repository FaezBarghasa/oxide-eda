---
okf_version: "0.2"
type: Function
title: handle_browser_delete_class
description: "Per-row `×` delete — drops the matching class from the library's"
resource: crates/oxide-app/src/app/dispatch/library/browser/classes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/classes/handle_browser_delete_class_1
language: rust
---

# handle_browser_delete_class

Per-row `×` delete — drops the matching class from the library's

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_delete_class(
        &mut self,
        library_path: std::path::PathBuf,
        key: String,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Per-row `×` delete — drops the matching class from the library's
`[[classes]]` block.

## Source
Lines 123–142 in `crates/oxide-app/src/app/dispatch/library/browser/classes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [classes](/crates/oxide-app/src/app/dispatch/library/browser/classes.md) |
