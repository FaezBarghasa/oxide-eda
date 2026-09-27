---
okf_version: "0.2"
type: Function
title: handle_browser_search_changed
description: Search-buffer edit inside a Library Browser tab.
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_search_changed_1
language: rust
---

# handle_browser_search_changed

Search-buffer edit inside a Library Browser tab.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_search_changed(
        &mut self,
        library_path: std::path::PathBuf,
        value: String,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Search-buffer edit inside a Library Browser tab.

## Source
Lines 25–39 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
| calls | [write_library_browser_search](/crates/oxide-app/src/fonts/misc/write_library_browser_search.md) |
