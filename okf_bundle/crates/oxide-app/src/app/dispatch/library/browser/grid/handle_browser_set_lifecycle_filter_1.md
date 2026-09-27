---
okf_version: "0.2"
type: Function
title: handle_browser_set_lifecycle_filter
description: Pick a lifecycle filter mode for the active Library Browser tab.
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_set_lifecycle_filter_1
language: rust
---

# handle_browser_set_lifecycle_filter

Pick a lifecycle filter mode for the active Library Browser tab.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_browser_set_lifecycle_filter(
        &mut self,
        library_path: std::path::PathBuf,
        filter: crate::library::state::LifecycleFilter,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Pick a lifecycle filter mode for the active Library Browser tab.

## Source
Lines 110–124 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
