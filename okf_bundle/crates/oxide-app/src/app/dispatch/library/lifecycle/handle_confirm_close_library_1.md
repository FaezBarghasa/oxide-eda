---
okf_version: "0.2"
type: Function
title: handle_confirm_close_library
description: Direct opener for the close-library confirm modal — used when
resource: crates/oxide-app/src/app/dispatch/library/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/lifecycle/handle_confirm_close_library_1
language: rust
---

# handle_confirm_close_library

Direct opener for the close-library confirm modal — used when

## Signature

```rust
pub(super) fn handle_confirm_close_library(
        &mut self,
        library_path: std::path::PathBuf,
        dirty_editors: Vec<EditorAddress>,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Direct opener for the close-library confirm modal — used when
callers already know the dirty list.

## Source
Lines 145–171 in `crates/oxide-app/src/app/dispatch/library/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-app/src/app/dispatch/library/lifecycle.md) |
